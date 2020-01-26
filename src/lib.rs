mod gen;

use memmap::{Mmap, MmapOptions};
use std::{fs::File, io, os::unix::io::AsRawFd, path::Path, ptr};

pub struct Database {
    file: File,
    mapping: Option<Mmap>,
    handle: gen::ZoneDetect,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Database> {
        let file = File::open(path)?;
        let fd = file.as_raw_fd();
        let metadata = file.metadata()?;
        let mapping = unsafe { MmapOptions::new().map(&file) }?;

        // TODO: ZDParseHeader

        Ok(Database {
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
        })
    }

    // TODO: ZDOpenDatabaseFromMemory
}

impl Drop for Database {
    fn drop(&mut self) {
        gen::ZDCloseDatabase(self.database);
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_open() {
        
    }
}
