pub mod sys;

use std::{
    collections::HashMap,
    ffi::{CStr, CString, NulError},
    os::unix::ffi::OsStrExt,
    path::Path,
    slice,
    str::Utf8Error,
};
use thiserror::Error;

pub struct Database {
    handle: *mut sys::ZoneDetect,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to open database")]
    OpenFailed,
    #[error("path contains nul bytes")]
    PathError(NulError),
    #[error("string is not valid UTF-8")]
    InvalidString(Utf8Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Zone {
    pub polygon_id: u32,
    pub meta_id: u32,
    pub fields: HashMap<String, String>,
}

/// Zip an array of keys and an array of values into a HashMap.
unsafe fn convert_c_strings_to_hashmap(
    keys: &[*mut i8],
    values: &[*mut i8],
) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for (key, value) in keys.iter().zip(values.iter()) {
        let key = CStr::from_ptr(*key)
            .to_str()
            .map_err(Error::InvalidString)?;
        let value = CStr::from_ptr(*value)
            .to_str()
            .map_err(Error::InvalidString)?;
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}

impl Database {
    pub fn open(path: &Path) -> Result<Database> {
        let path_str_c = CString::new(path.as_os_str().as_bytes())
            .map_err(Error::PathError)?;
        let handle;
        unsafe {
            handle = sys::ZDOpenDatabase(path_str_c.as_ptr());
        }
        if handle.is_null() {
            Err(Error::OpenFailed)
        } else {
            Ok(Database { handle })
        }
    }

    pub fn lookup(&self, location: &Location) -> Result<Vec<Zone>> {
        unsafe {
            let mut safezone = 0.0f32;
            let results = sys::ZDLookup(
                self.handle,
                location.latitude,
                location.longitude,
                &mut safezone,
            );
            let mut zones = Vec::new();
            if !results.is_null() {
                let mut index = 0;
                loop {
                    let item = results.offset(index);
                    if (*item).lookupResult == sys::ZDLookupResult_ZD_LOOKUP_END
                    {
                        break;
                    }

                    let num_fields = (*item).numFields as usize;
                    let field_names =
                        slice::from_raw_parts((*item).fieldNames, num_fields);
                    let field_data =
                        slice::from_raw_parts((*item).data, num_fields);
                    let fields =
                        convert_c_strings_to_hashmap(field_names, field_data)?;

                    zones.push(Zone {
                        polygon_id: (*item).polygonId,
                        meta_id: (*item).metaId,
                        fields,
                    });

                    index += 1;
                }
            }
            Ok(zones)
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            sys::ZDCloseDatabase(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
