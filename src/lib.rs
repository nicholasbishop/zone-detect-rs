mod gen;

use memmap::{Mmap, MmapOptions};
use std::{
    convert::TryInto, ffi::CStr, fs::File, io, os::unix::io::AsRawFd,
    path::Path, slice, str::Utf8Error,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("database header is truncated")]
    TruncatedDatabase(i64),
    #[error("invalid magic bytes")]
    InvalidMagic([u8; 3]),
    #[error("invalid version")]
    InvalidVersion(u8),
    #[error("invalid table type")]
    InvalidTableType(u8),
    #[error("invalid field name")]
    InvalidFieldName(u8, Utf8Error),
    #[error("invalid notice")]
    InvalidNotice(Utf8Error),
    #[error("invalid metadata offset")]
    InvalidMetadataOffset,
    #[error("invalid data offset")]
    InvalidDataOffset,
    // TODO: I'm not actually sure what this offset is supposed to be,
    // calling it padding for now
    #[error("invalid padding offset")]
    InvalidPaddingOffset,
    #[error("length mismatch")]
    LengthMismatch(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableType {
    Country,
    Timezone,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_string(
    db: &gen::ZoneDetect,
    index: &mut gen::uint32_t,
) -> std::result::Result<String, Utf8Error> {
    let raw = unsafe { gen::ZDParseString(db, index) };
    let c_str = unsafe { CStr::from_ptr(raw) };
    let s = c_str.to_str()?;
    Ok(s.into())
}

pub struct Database {
    file: File,
    mapping: Option<Mmap>,
    library: gen::ZoneDetect,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
        let file = File::open(path)?;
        let fd = file.as_raw_fd();
        let metadata = file.metadata()?;
        let mapping = unsafe { MmapOptions::new().map(&file) }?;

        let mut db = Database {
            library: gen::ZoneDetect {
                fd,
                length: metadata.len() as i64,
                mapping: mapping.as_ptr(),
                notice: String::new(),

                // Set the rest to zero for now
                tableType: TableType::Country,
                version: 0,
                precision: 0,
                fieldNames: Vec::new(),
                bboxOffset: 0,
                metadataOffset: 0,
                dataOffset: 0,
            },
            file,
            mapping: Some(mapping),
        };
        Self::parse_header(&mut db.library)?;
        Ok(db)
    }

    // TODO: ZDOpenDatabaseFromMemory

    pub fn table_type(&self) -> TableType {
        self.library.tableType
    }

    pub fn notice(&self) -> &str {
        &self.library.notice
    }

    fn parse_header(db: &mut gen::ZoneDetect) -> Result<()> {
        if db.length < 7 {
            return Err(Error::TruncatedDatabase(db.length));
        }

        let expected_magic = b"PLB";
        let actual_magic =
            unsafe { slice::from_raw_parts(db.mapping, expected_magic.len()) };
        if actual_magic != expected_magic {
            return Err(Error::InvalidMagic(
                actual_magic.try_into().unwrap_or([0; 3]),
            ));
        }

        let table_type = unsafe { *db.mapping.offset(3) };
        db.version = unsafe { *db.mapping.offset(4) };
        db.precision = unsafe { *db.mapping.offset(5) };
        let num_fields = unsafe { *db.mapping.offset(6) };

        if table_type == b'T' {
            db.tableType = TableType::Timezone;
        } else if table_type == b'C' {
            db.tableType = TableType::Country;
        } else {
            return Err(Error::InvalidTableType(table_type));
        }

        if db.version >= 2 {
            return Err(Error::InvalidVersion(db.version));
        }

        // Start reading at byte 7
        let mut index = 7;

        db.fieldNames.reserve(num_fields as usize);
        for field_index in 0..num_fields {
            let name = parse_string(db, &mut index)
                .map_err(|err| Error::InvalidFieldName(field_index, err))?;
            db.fieldNames.push(name);
        }

        db.notice =
            parse_string(db, &mut index).map_err(Error::InvalidNotice)?;

        // Read section sizes. Note that bboxOffset is already initialized to zero.
        let mut tmp: gen::uint64_t = 0;
        if unsafe {
            gen::ZDDecodeVariableLengthUnsigned(db, &mut index, &mut tmp)
        } == 0
        {
            return Err(Error::InvalidMetadataOffset);
        }
        db.metadataOffset = tmp as u32 + db.bboxOffset;

        if unsafe {
            gen::ZDDecodeVariableLengthUnsigned(db, &mut index, &mut tmp)
        } == 0
        {
            return Err(Error::InvalidDataOffset);
        }
        db.dataOffset = tmp as u32 + db.metadataOffset;

        if unsafe {
            gen::ZDDecodeVariableLengthUnsigned(db, &mut index, &mut tmp)
        } == 0
        {
            return Err(Error::InvalidPaddingOffset);
        }

        // Add header size to everything
        db.bboxOffset += index;
        db.metadataOffset += index;
        db.dataOffset += index;

        // Verify file length
        let length = (tmp + db.dataOffset as u64) as i64;
        if length != db.length {
            return Err(Error::LengthMismatch(length as usize));
        }

        Ok(())
    }

    pub fn simple_lookup(&self, lat: f32, lon: f32) -> Option<String> {
        let results = unsafe {
            gen::ZDLookup(&self.library, lat, lon, std::ptr::null_mut())
        };

        if let Some(result) = results.first() {
            match self.library.tableType {
                TableType::Country => result.fields.get("Name"),
                TableType::Timezone => {
                    if let Some(prefix) = result.fields.get("TimezoneIdPrefix")
                    {
                        if let Some(id) = result.fields.get("TimezoneId") {
                            return Some(format!("{}{}", prefix, id));
                        }
                    }
                    None
                }
            }
            .cloned()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let db = Database::open("data/timezone21.bin").unwrap();
        assert_eq!(db.library.bboxOffset, 288);
        assert_eq!(db.library.metadataOffset, 33429);
        assert_eq!(db.library.dataOffset, 42557);
        assert_eq!(db.library.notice, "Contains data from Natural Earth, placed in the Public Domain. Contains information from https://github.com/evansiroky/timezone-boundary-builder, which is made available here under the Open Database License \\(ODbL\\).".to_string());
        assert_eq!(db.library.tableType, TableType::Timezone);
        assert_eq!(db.library.precision, 21);
        assert_eq!(
            db.library.fieldNames,
            vec![
                "TimezoneIdPrefix".to_string(),
                "TimezoneId".to_string(),
                "CountryAlpha2".to_string(),
                "CountryName".to_string(),
            ]
        );
    }

    #[test]
    fn test_simple_lookup() {
        let db = Database::open("data/timezone21.bin").unwrap();
        assert_eq!(
            db.simple_lookup(40.7128, -74.0060).unwrap(),
            "America/New_York"
        );
    }
}
