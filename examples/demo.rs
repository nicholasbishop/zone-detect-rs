use std::{path::PathBuf, process::exit};
use structopt::StructOpt;
use zone_detect::{Database, Location};

#[derive(StructOpt, Debug)]
#[structopt(name = "demo")]
struct Opt {
    database_path: PathBuf,
    latitude: f32,
    longitude: f32,
}

fn lookup(opt: &Opt) -> Result<(), zone_detect::Error> {
    let database = Database::open(&opt.database_path)?;
    let zones = database.lookup(&Location {
        latitude: opt.latitude,
        longitude: opt.longitude,
    })?;
    for (index, zone) in zones.iter().enumerate() {
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
