extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::CargoCallbacks;

fn main() {
    // === RESOLVE VIPS ===

    let vips_pkg_config = pkg_config::Config::new()
        .probe("vips").unwrap();
    
    // === RUN BINDGEN ===

    println!("{:?}", vips_pkg_config);

    let vips_header_path = &vips_pkg_config.include_paths[0];
    let vips_entry_header_path = vips_header_path.join("vips/vips.h");
    let vips_entry_header_path_str = vips_entry_header_path.to_str().unwrap();


    let bindings = bindgen::Builder::default()
        .header(vips_entry_header_path_str)
        .clang_args(vips_pkg_config.include_paths.iter()
            .map(|p| format!("-I{}", p.to_str().unwrap()))
        )
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
