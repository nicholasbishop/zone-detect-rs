mod gen;

use memmap::{Mmap, MmapOptions};
use std::{
    convert::TryInto, fs::File, io, os::unix::io::AsRawFd, path::Path, ptr,
    slice,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("database header is truncated")]
    TruncatedDatabase(i64),
    #[error("invalid magic bytes")]
    InvalidMagic([u8; 3]),
}

pub type Result<T> = std::result::Result<T, Error>;

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

        // TODO: ZDParseHeader

        let mut db = Database {
            handle: gen::ZoneDetect {
                fd,
                length: metadata.len() as i64,
                mapping: mapping.as_ptr(),

                // Set the rest to zero for now
                tableType: 0,
                version: 0,
                precision: 0,
                numFields: 0,
                notice: ptr::null_mut(),
                fieldNames: ptr::null_mut(),
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

        Ok(())
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
