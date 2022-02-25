extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main () {

    println!("cargo:rustc-link-lib=dylib=stdc++");
    let bindings = bindgen::Builder::default()
        .header("UDBM/include/dbm/dbm.h")
        .clang_arg("-IUDBM/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("{:?}", out_path.join("bindings.rs"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    let src = [
        "UDBM/dbm/dbm.c",
        "UDBM/base/intutils.cpp",
        "UDBM/dbm/print.cpp",
        "UDBM/debug/utils.cpp",
    ];
    let includes = [
        "UDBM/include",
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .cpp(true)
        .files(src.iter())
        .includes(includes.iter());

    build.compile("UDBM");
}
