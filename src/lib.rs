mod gen;

use memmap::{Mmap, MmapOptions};
use std::{
    convert::TryInto, ffi::CStr, fs::File, io, os::unix::io::AsRawFd,
    path::Path, ptr, slice, str::Utf8Error,
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
    #[error("invalid field name")]
    InvalidFieldName(u8, Utf8Error),
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
    handle: gen::ZoneDetect,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
        let file = File::open(path)?;
        let fd = file.as_raw_fd();
        let metadata = file.metadata()?;
        let mapping = unsafe { MmapOptions::new().map(&file) }?;

        let mut db = Database {
            handle: gen::ZoneDetect {
                fd,
                length: metadata.len() as i64,
                mapping: mapping.as_ptr(),

                // Set the rest to zero for now
                tableType: 0,
                version: 0,
                precision: 0,
                notice: ptr::null_mut(),
                fieldNames: Vec::new(),
                bboxOffset: 0,
                metadataOffset: 0,
                dataOffset: 0,
            },
            file,
            mapping: Some(mapping),
        };
        Self::parse_header(&mut db.handle)?;
        Ok(db)
    }

    // TODO: ZDOpenDatabaseFromMemory

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

        db.tableType = unsafe { *db.mapping.offset(3) };
        db.version = unsafe { *db.mapping.offset(4) };
        db.precision = unsafe { *db.mapping.offset(5) };
        let num_fields = unsafe { *db.mapping.offset(6) };

        if db.version >= 2 {
            return Err(Error::InvalidVersion(db.version));
        }

        // Start reading at byte 7
        let mut index = 7;

        db.fieldNames.reserve(num_fields as usize);
        for field_index in 0..num_fields {
            let name = parse_string(db, &mut index)
                .map_err(|err| Error::InvalidFieldName(field_index, err))?;
            db.fieldNames.push(name.into());
        }

        Ok(())
    }

    pub fn simple_lookup(&self, lat: f32, lon: f32) -> Option<String> {
        let result = unsafe {
            gen::ZDLookup(&self.handle, lat, lon, std::ptr::null_mut())
        };
        // TODO
        None
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe { gen::ZDCloseDatabase(&mut self.handle) };
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_open() {}
}
