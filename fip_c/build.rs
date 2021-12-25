//! TODO: documentation

// use std::env;
// use std::path::Path;
// use std::process::Command;
// #![no_std]
#![no_main]

#[allow(unsafe_code)]
#[no_mangle]
fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let _command = Command::new("clang")
    //     .args(&["src/hello.c", "-c", "-fPIC", "-o"])
    //     .arg(&format!("{}/hello.o", out_dir))
    //     .status()
    //     .unwrap();
    // let _command = Command::new("ar")
    //     .args(&["crus", "libhello.a", "hello.o"])
    //     .current_dir(&Path::new(&out_dir))
    //     .status()
    //     .unwrap();
    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=hello");

    cc::Build::new().file("src/hello.c").compile("hello");

    println!("cargo:rerun-if-changed=src/hello.c");
}
