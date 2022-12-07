#![allow(dead_code)]

use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("src");

    // Add the files we build
    let source_files = ["vendor/xatlas.cpp"];

    for source_file in &source_files {
        build.file(&source_file);
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-sign-compare")
            .flag("-Wno-deprecated")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build.flag("-std=c++11").cpp_link_stdlib("stdc++").cpp(true);
    }

    build.debug(false).flag("-DNDEBUG").cpp(true);

    build.compile("xatlas");

    generate_bindings("src/bindings.rs")
}

fn generate_bindings(output_file: &str) {
    let bindings = bindgen::builder()
        .header("header.h")
        .enable_cxx_namespaces()
        .rustfmt_bindings(true)
        .clang_args(&["-xc++", "-std=c++11"])
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}
