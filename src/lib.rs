//! Example:
//!
//! ```
//! let database = zone_detect::Database::open("data/timezone21.bin")
//!     .expect("failed to open database");
//! let s = database.simple_lookup(zone_detect::Location {
//!     latitude: 35.0715,
//!     longitude: -82.5216,
//! }).unwrap();
//! assert_eq!(s, "America/New_York");
//! ```

#![deny(missing_docs)]

mod gen;
use std::{
    collections::HashMap, convert::TryInto, fs, io, path::Path,
    string::FromUtf8Error,
};

/// Latitude and longitude.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    /// Latitude.
    pub latitude: f32,
    /// Longitude.
    pub longitude: f32,
}

impl Location {
    /// Create a new Location.
    pub fn new(latitude: f32, longitude: f32) -> Location {
        Location {
            latitude,
            longitude,
        }
    }
}

/// Zone retrieved from the database.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Zone {
    /// Polygon ID.
    pub polygon_id: u32,
    /// Metadata ID.
    pub meta_id: u32,
    /// Zone information. The keys will vary depending on the database.
    pub fields: HashMap<String, String>,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZoneMatchKind {
    InZone,
    InExcludedZone,
    OnBorderVertex,
    OnBorderSegment,
}

impl ZoneMatchKind {
    fn from_point_lookup(r: gen::PointLookupResult) -> Option<ZoneMatchKind> {
        match r {
            gen::PointLookupResult::InZone => Some(ZoneMatchKind::InZone),
            gen::PointLookupResult::InExcludedZone => {
                Some(ZoneMatchKind::InExcludedZone)
            }
            gen::PointLookupResult::OnBorderVertex => {
                Some(ZoneMatchKind::OnBorderVertex)
            }
            gen::PointLookupResult::OnBorderSegment => {
                Some(ZoneMatchKind::OnBorderSegment)
            }
            _ => None,
        }
    }
}

/// Zone retrieved from the database, along with the type of result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZoneMatch {
    /// Type of match.
    pub kind: ZoneMatchKind,
    /// Zone information.
    pub zone: Zone,
}

/// Matching zones and safezone from a database lookup.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneLookup {
    /// List of matching zones.
    pub matches: Vec<ZoneMatch>,
    /// TODO: not sure what this value is
    pub safezone: f32,
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("database header is truncated")]
    TruncatedDatabase(usize),
    #[error("invalid magic bytes")]
    InvalidMagic([u8; 3]),
    #[error("invalid version")]
    InvalidVersion(u8),
    #[error("invalid table type")]
    InvalidTableType(u8),
    #[error("invalid field name")]
    InvalidFieldName(u8, StringParseError),
    #[error("invalid notice")]
    InvalidNotice(StringParseError),
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

/// Database type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableType {
    /// Country-name database.
    Country,
    /// Timezone database.
    Timezone,
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum StringParseError {
    #[error("encoding error")]
    EncodingError,
    #[error("invalid UTF-8")]
    InvalidUtf8(#[from] FromUtf8Error),
}

fn parse_string(
    db: &Database,
    index: &mut u32,
) -> std::result::Result<String, StringParseError> {
    if let Some(bytes) = gen::parse_string(db, index) {
        let string = String::from_utf8(bytes)?;
        Ok(string)
    } else {
        Err(StringParseError::EncodingError)
    }
}

/// Zone database.
pub struct Database {
    bbox_offset: u32,
    data_offset: u32,
    mapping: Vec<u8>,
    metadata_offset: u32,

    /// Names of all the fields in the database.
    pub field_names: Vec<String>,
    /// Database notice text (e.g. for licensing information).
    pub notice: String,
    /// Precision of the data.
    pub precision: u8,
    /// Type of data (country or timezone).
    pub table_type: crate::TableType,
    /// Encoding version.
    pub version: u8,
}

impl Database {
    /// Open a zone database.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
        let mapping = fs::read(path)?;
        Self::from_vec(mapping)
    }

    /// Load the database from a byte vector.
    pub fn from_vec(mapping: Vec<u8>) -> Result<Database> {
        let mut db = Database {
            mapping,
            notice: String::new(),
            table_type: TableType::Country,
            version: 0,
            precision: 0,
            field_names: Vec::new(),
            bbox_offset: 0,
            metadata_offset: 0,
            data_offset: 0,
        };
        Self::parse_header(&mut db)?;
        Ok(db)
    }

    fn parse_header(db: &mut Database) -> Result<()> {
        if db.mapping.len() < 7 {
            return Err(Error::TruncatedDatabase(db.mapping.len()));
        }

        let expected_magic = b"PLB";
        let actual_magic = &db.mapping[0..3];
        if actual_magic != expected_magic {
            return Err(Error::InvalidMagic(
                actual_magic.try_into().unwrap_or([0; 3]),
            ));
        }

        let table_type = db.mapping[3];
        db.version = db.mapping[4];
        db.precision = db.mapping[5];
        let num_fields = db.mapping[6];

        if table_type == b'T' {
            db.table_type = TableType::Timezone;
        } else if table_type == b'C' {
            db.table_type = TableType::Country;
        } else {
            return Err(Error::InvalidTableType(table_type));
        }

        if db.version >= 2 {
            return Err(Error::InvalidVersion(db.version));
        }

        // Start reading at byte 7
        let mut index = 7;

        db.field_names.reserve(num_fields as usize);
        for field_index in 0..num_fields {
            let name = parse_string(db, &mut index)
                .map_err(|err| Error::InvalidFieldName(field_index, err))?;
            db.field_names.push(name);
        }

        db.notice =
            parse_string(db, &mut index).map_err(Error::InvalidNotice)?;

        // Read section sizes. Note that bboxOffset is already initialized to zero.
        let mut tmp: u64 = 0;
        if gen::decode_variable_length_unsigned(db, &mut index, &mut tmp) == 0 {
            return Err(Error::InvalidMetadataOffset);
        }
        db.metadata_offset = tmp as u32 + db.bbox_offset;

        if gen::decode_variable_length_unsigned(db, &mut index, &mut tmp) == 0 {
            return Err(Error::InvalidDataOffset);
        }
        db.data_offset = tmp as u32 + db.metadata_offset;

        if gen::decode_variable_length_unsigned(db, &mut index, &mut tmp) == 0 {
            return Err(Error::InvalidPaddingOffset);
        }

        // Add header size to everything
        db.bbox_offset += index;
        db.metadata_offset += index;
        db.data_offset += index;

        // Verify file length
        let length = (tmp + db.data_offset as u64) as usize;
        if length != db.mapping.len() {
            return Err(Error::LengthMismatch(length as usize));
        }

        Ok(())
    }

    /// Get a simple description of a location.
    ///
    /// For a country database this will be the country name, for the
    /// timezone database it will be the timezone ID.
    pub fn simple_lookup(&self, location: Location) -> Option<String> {
        let results = gen::lookup(self, location, None);

        if let Some(result) = results.first() {
            match self.table_type {
                TableType::Country => result.zone.fields.get("Name"),
                TableType::Timezone => {
                    if let Some(prefix) =
                        result.zone.fields.get("TimezoneIdPrefix")
                    {
                        if let Some(id) = result.zone.fields.get("TimezoneId") {
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

    /// Perform a full database lookup for a location.
    pub fn lookup(&self, location: Location) -> ZoneLookup {
        let mut safezone: f32 = 0.0;
        let results = gen::lookup(self, location, Some(&mut safezone));
        let matches = results
            .iter()
            .map(|r| {
                ZoneMatch {
                    // Unwrapping should be OK here since the lookup
                    // function already filters out other kinds of results
                    kind: ZoneMatchKind::from_point_lookup(r.result)
                        .expect("invalid match kind"),
                    zone: r.zone.clone(),
                }
            })
            .collect::<Vec<_>>();
        ZoneLookup { matches, safezone }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let db = Database::open("data/timezone21.bin").unwrap();
        assert_eq!(db.bbox_offset, 288);
        assert_eq!(db.metadata_offset, 33429);
        assert_eq!(db.data_offset, 42557);
        assert_eq!(db.notice, "Contains data from Natural Earth, placed in the Public Domain. Contains information from https://github.com/evansiroky/timezone-boundary-builder, which is made available here under the Open Database License \\(ODbL\\).".to_string());
        assert_eq!(db.table_type, TableType::Timezone);
        assert_eq!(db.precision, 21);
        assert_eq!(
            db.field_names,
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
        // Beijing
        assert_eq!(
            db.simple_lookup(Location::new(39.9042, 116.4074)).unwrap(),
            "Asia/Shanghai"
        );
        // Buenos Aires
        assert_eq!(
            db.simple_lookup(Location::new(-34.6037, -58.3816)).unwrap(),
            "America/Argentina/Buenos_Aires"
        );
        // Canberra
        assert_eq!(
            db.simple_lookup(Location::new(-35.2809, 149.1300)).unwrap(),
            "Australia/Sydney"
        );
        // New York City
        assert_eq!(
            db.simple_lookup(Location::new(40.7128, -74.0060)).unwrap(),
            "America/New_York"
        );
    }
}
