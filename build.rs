extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use bindgen::CargoCallbacks;

fn main() {
    // === BUILD VIPS ===

    let libvips_path = PathBuf::from("ext/vips")
        .canonicalize()
        .expect("cannot canonicalize path");

    // TODO: make this system-dependent
    //CC=/usr/local/opt/llvm/bin/clang CXX=/usr/local/opt/llvm/bin/clang++ LDFLAGS="-L/usr/local/opt/llvm/lib/c++ -Wl,-no_fixup_chains,-rpath,/usr/local/opt/llvm/lib/c++" ./configure CC=/usr/local/opt/llvm/bin/clang CXX=/usr/local/opt/llvm/bin/clang++ LDFLAGS="-L/usr/local/opt/llvm/lib/c++ -Wl,-rpath,/usr/local/opt/llvm/lib/c++"
    let meson_env = HashMap::from([
        ("CC", "/usr/local/opt/llvm/bin/clang"),
        ("CXX", "/usr/local/opt/llvm/bin/clang++"),
        ("LDFLAGS", "-L/usr/local/opt/llvm/lib/c++ -Wl,-no_fixup_chains,-rpath,/usr/local/opt/llvm/lib/c++")
    ]);

    let meson_args = vec![
        "setup",
        "--buildtype=release",
        "--default-library=static",
        "build"
    ];

    let vips_meson_setup_result = std::process::Command::new("meson")
        .current_dir(&libvips_path)
        .envs(&meson_env)
        .args(meson_args)
        .output().expect("could not spawn `meson`").status.success();

    if !vips_meson_setup_result {
        panic!("Could not `meson setup` vips");
    }

    let vips_meson_compile_result = std::process::Command::new("meson")
        .arg("compile")
        .current_dir(libvips_path.join("build"))
        .output().expect("could not spawn `meson`").status.success();

    if !vips_meson_compile_result {
        panic!("Could not `meson compile` vips");
    }

    // === SET UP LINK/INCLUDE PATHS ===

    println!("cargo:rustc-link-search={}", libvips_path.join("build/libvips").to_str().unwrap());
    println!("cargo:rustc-link-lib=vips");
    // TODO: rerun if changed?

    // === RUN BINDGEN ===

    let vips_header_path = libvips_path.join("libvips/include/vips");
    let vips_header_path_str = vips_header_path.to_str().unwrap();
    let vips_entry_header_path = vips_header_path.join("vips.h");
    let vips_entry_header_path_str = vips_entry_header_path.to_str().unwrap();

    let bindings = bindgen::Builder::default()
        .header(vips_entry_header_path_str)
        .clang_args([
            format!("-I{vips_header_path_str}"),
            format!("-I{}", libvips_path.join("build/libvips/include").to_str().unwrap()),
            // -- TODO: discover these paths dynamically --
            "-I/usr/local/opt/glib/include/glib-2.0/".to_string(),
            "-I/usr/local/opt/glib/lib/glib-2.0/include/".to_string()
        ])
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
