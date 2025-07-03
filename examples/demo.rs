use clap::Parser;
use std::{path::PathBuf, process::exit};
use zone_detect::{Database, Location};

#[derive(Parser)]
#[command(name = "demo")]
struct Opt {
    database_path: PathBuf,
    #[arg(allow_negative_numbers = true)]
    latitude: f32,
    #[arg(allow_negative_numbers = true)]
    longitude: f32,
}

fn lookup(opt: &Opt) -> Result<(), zone_detect::Error> {
    let database = Database::open(&opt.database_path)?;
    let result = database.lookup(Location {
        latitude: opt.latitude,
        longitude: opt.longitude,
    });
    for (index, zone) in result.matches.iter().enumerate() {
        println!("zone {index}: {zone:#?}");
    }
    Ok(())
}

fn main() {
    let opt = Opt::parse();
    if let Err(err) = lookup(&opt) {
        eprintln!("error: {err}");
        exit(1);
    }
}
