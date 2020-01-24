use std::env;
use std::path::PathBuf;

fn generate_bindings() {
    let header = "wrapper.h";

    // Rebuild if the header changes
    println!("cargo:rerun-if-changed={}", header);

    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_library() {
    cc::Build::new()
        .file("ZoneDetect/library/zonedetect.c")
        .compile("ZoneDetect");
}

fn main() {
    generate_bindings();
    build_library();
}
