extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=bluetooth");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("btlepasvscan_open")
        .whitelist_function("btlepasvscan_close")
        .whitelist_function("btlepasvscan_read")
        .whitelist_type("btlepasvscan_ctx")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        // .write_to_file(out_path.join("bindings.rs"))
        .write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("c/btlepasvscan.c")
        .compile("btlepasvscan.o")
}
