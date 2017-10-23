extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = pkg_config::Config::new()
        .atleast_version("0.28.0")
        .probe("serd-0")
        .unwrap();

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h");

    for ref path in &lib.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .hide_type("max_align_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

