extern crate cc;

use std::env;
use std::path::PathBuf;


fn main() 
{

    let mut javascript_config = cc::Build::new();
    javascript_config
        .include("tree_sitter")
        .file("lang/javascript/parser.c")
        .file("lang/javascript/scanner.c")
        .compile("javascript");

        //println!("cargo:rustc-link-lib=tree-sitter-javascript");

    /*
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./src/Javascript/src/tree_sitter/javascript.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/Javascript");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");*/
}