extern crate bindgen;

use bindgen::EnumVariation;
use std::{env, path::PathBuf};

fn main() {
    let lib = pkg_config::Config::new().probe("serd-0").unwrap();

    let mut builder = bindgen::Builder::default().header("wrapper.h");

    for ref path in &lib.include_paths {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder
        .default_enum_style(EnumVariation::Rust { non_exhaustive: true })
        .blocklist_type("max_align_t")
        .blocklist_item("SERD_URI_NULL")
        .blocklist_item("SERD_NODE_NULL")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
