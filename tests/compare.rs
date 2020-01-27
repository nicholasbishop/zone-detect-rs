use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};
use zone_detect::{Database, Location, ZoneMatch, ZoneMatchKind};

fn random_location(rng: &mut StdRng) -> Location {
    Location {
        latitude: rng.gen_range(-90.0, 90.0),
        longitude: rng.gen_range(-180.0, 180.0),
    }
}

fn get_demo_path() -> PathBuf {
    let path_var = env::var("ZONEDETECT_DEMO").expect("the ZONEDETECT_DEMO env var must be set to the path of the ZoneDetect demo");
    let path = Path::new(&path_var);
    assert!(path.exists(), "ZoneDetect demo path is invalid");
    path.into()
}

fn lookup_result_to_string(result: ZoneMatchKind) -> &'static str {
    match result {
        ZoneMatchKind::InZone => "In zone",
        ZoneMatchKind::InExcludedZone => "In excluded zone",
        ZoneMatchKind::OnBorderVertex => "Target point is border vertex",
        ZoneMatchKind::OnBorderSegment => "Target point is on border",
    }
}

fn lookup(db: &Database, location: Location) -> (String, Vec<ZoneMatch>) {
    let mut output = String::new();
    let result = db.lookup(location);
    for result in &result.matches {
        output += &format!("{}:\n", lookup_result_to_string(result.kind));
        output += &format!("  meta: {}\n", result.zone.meta_id);
        output += &format!("  polygon: {}\n", result.zone.polygon_id);
        output += &format!(
            "  TimezoneIdPrefix: {}\n",
            result.zone.fields.get("TimezoneIdPrefix").unwrap()
        );
        output += &format!(
            "  TimezoneId: {}\n",
            result.zone.fields.get("TimezoneId").unwrap()
        );
        output += &format!(
            "  CountryAlpha2: {}\n",
            result.zone.fields.get("CountryAlpha2").unwrap()
        );
        output += &format!(
            "  CountryName: {}\n",
            result.zone.fields.get("CountryName").unwrap()
        );
    }
    output += &format!("Safezone: {:.6}\n", result.safezone);
    output += &format!(
        "The simple string is [{}]\n",
        db.simple_lookup(location).unwrap()
    );
    (output, result.matches)
}

/// This test compares the output against that of the demo program in
/// ZoneDetect by feeding both many pairs of random coordinates.
#[test]
#[ignore]
fn test_compare() {
    let path = get_demo_path();
    let db_path = "data/timezone21.bin";
    let db = Database::open(db_path).unwrap();

    let mut rng = StdRng::from_seed([0; 32]);

    let num_iterations = 10000;
    for i in 0..num_iterations {
        let loc = random_location(&mut rng);
        let output = Command::new(&path)
            .arg(db_path)
            .arg(loc.latitude.to_string())
            .arg(loc.longitude.to_string())
            .output()
            .unwrap();
        assert!(output.stderr.is_empty());
        let expected = std::str::from_utf8(&output.stdout).unwrap();
        let (actual, results) = lookup(&db, loc);
        if actual != expected {
            println!("expected: {}", expected);
            println!("actual: {}", actual);
            dbg!(&results);
        }
        assert_eq!(
            actual, expected,
            "failed on i={}, lat={}, lon={}, results={:?}",
            i, loc.latitude, loc.longitude, results,
        );
    }
}
