extern crate bindgen;
extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;

fn main() {
    // Compile assimp from source
    // Disable unnecessary stuff, it takes long enough to compile already
    let dst = Config::new("assimp")
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("ASSIMP_INSTALL_PDB", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_SUPPRESS_DEVELOPER_WARNINGS", "ON")
        .define("LIBRARY_SUFFIX", "")
        .define("CMAKE_C_COMPILER", "clang")
        .define("CMAKE_C_FLAGS", "-fPIC")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    // Link to assimp and its dependencies
    let debug_postfix = if env::var("DEBUG").unwrap() == "true" {
        "d"
    } else {
        ""
    };
    println!("cargo:rustc-link-lib=static=assimp{}", debug_postfix);
    println!("cargo:rustc-link-lib=static=IrrXML{}", debug_postfix);

    let minizip = pkg_config::probe_library("minizip").unwrap();
    for path in minizip.link_paths {
        println!("cargo:rustc-link-path={}", path.to_str().unwrap());
    }
    for lib in minizip.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    let zlib = pkg_config::probe_library("zlib").unwrap();
    for path in zlib.link_paths {
        println!("cargo:rustc-link-path={}", path.to_str().unwrap());
    }
    for lib in zlib.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-F", "assimp/include", "-F", "assimp/contrib/irrXML"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_debug(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .unwrap_or_else(|e| panic!("Couldn't write to {:?}: {:?}", bindings_path, e));

    // Link to libstdc++ on GNU
    let target = env::var("TARGET").unwrap();
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
