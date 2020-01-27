use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};
use tzlookup::{Database, Location, LookupResult, ZoneDetectResult};

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

fn lookup_result_to_string(result: LookupResult) -> &'static str {
    match result {
        LookupResult::Ignore => "Ignore",
        LookupResult::End => "End",
        LookupResult::ParseError => "Parsing error",
        LookupResult::NotInZone => "Not in zone",
        LookupResult::InZone => "In zone",
        LookupResult::InExcludedZone => "In excluded zone",
        LookupResult::OnBorderVertex => "Target point is border vertex",
        LookupResult::OnBorderSegment => "Target point is on border",
    }
}

fn lookup(
    db: &Database,
    location: Location,
) -> (String, Vec<ZoneDetectResult>) {
    let mut output = String::new();
    let (results, safezone) = db.lookup(location);
    for result in &results {
        output +=
            &format!("{}:\n", lookup_result_to_string(result.lookup_result));
        output += &format!("  meta: {}\n", result.meta_id);
        output += &format!("  polygon: {}\n", result.polygon_id);
        output += &format!(
            "  TimezoneIdPrefix: {}\n",
            result.fields.get("TimezoneIdPrefix").unwrap()
        );
        output += &format!(
            "  TimezoneId: {}\n",
            result.fields.get("TimezoneId").unwrap()
        );
        output += &format!(
            "  CountryAlpha2: {}\n",
            result.fields.get("CountryAlpha2").unwrap()
        );
        output += &format!(
            "  CountryName: {}\n",
            result.fields.get("CountryName").unwrap()
        );
    }
    output += &format!("Safezone: {:.6}\n", safezone);
    output += &format!(
        "The simple string is [{}]\n",
        db.simple_lookup(location).unwrap()
    );
    (output, results)
}

/// This test compares the output against that of the demo program in
/// ZoneDetect by feeding both many pairs of random coordinates.
#[test] #[ignore]
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
