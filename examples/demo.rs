use clap::AppSettings;
use std::{path::PathBuf, process::exit};
use structopt::StructOpt;
use zone_detect::{Database, Location};

#[derive(StructOpt, Debug)]
#[structopt(name = "demo", global_settings(&[AppSettings::AllowNegativeNumbers]))]
struct Opt {
    database_path: PathBuf,
    latitude: f32,
    longitude: f32,
}

fn lookup(opt: &Opt) -> Result<(), zone_detect::Error> {
    let database = Database::open(&opt.database_path)?;
    let result = database.lookup(Location {
        latitude: opt.latitude,
        longitude: opt.longitude,
    });
    for (index, zone) in result.matches.iter().enumerate() {
        println!("zone {}: {:#?}", index, zone);
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = lookup(&opt) {
        eprintln!("error: {}", err);
        exit(1);
    }
}
