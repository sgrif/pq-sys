#[cfg(feature="pkg-config")]
extern crate pkg_config;
extern crate bindgen;

use self::bindgen::Builder;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;

fn main() {
    generate_bindgen_file();

    let link_flag = "pq";

    if let Ok(lib_dir) = env::var("PQ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);

    } else if configured_by_pkg_config() {
        return // pkg_config does everything for us, including output for cargo

    } else if let Some(path) = pg_config_output("--libdir") {
        let path = follow_dylib_symlinks(path.trim().into());
        println!("cargo:rustc-link-search=native={}", &path.display());
    }

    let mode = if env::var_os("PQ_LIB_STATIC").is_some() {
        "static"
    } else {
        "dylib"
    };

    println!("cargo:rustc-link-lib={}={}", mode, link_flag);
}

#[cfg(feature="pkg-config")]
fn configured_by_pkg_config() -> bool {
    pkg_config::probe_library("libpq").is_ok()
}

#[cfg(not(feature="pkg-config"))]
fn configured_by_pkg_config() -> bool {
    false
}

fn pg_config_output(command: &str) -> Option<String> {
    Command::new("pg_config")
        .arg(command)
        .output()
        .ok()
        .into_iter()
        .filter(|output| output.status.success())
        .flat_map(|output| String::from_utf8(output.stdout).ok())
        .next()
}

#[cfg(target_os = "macos")]
fn follow_dylib_symlinks(libdir: PathBuf) -> PathBuf {
    fs::canonicalize(libdir.join("libpq.dylib"))
        .ok()
        .and_then(|dir| dir.parent().map(|parent| parent.into()))
        .unwrap_or(libdir)
}

#[cfg(not(target_os = "macos"))]
fn follow_dylib_symlinks(libdir: PathBuf) -> PathBuf {
    libdir
}

fn generate_bindgen_file() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut builder = Builder::default()
        .no_unstable_rust()
        .header("wrapper.h");

    if let Some(path) = pg_config_output("--includedir") {
        builder = builder.clang_arg(format!("-I{}", path));
    }

    builder.generate()
        .expect("Unable to generate bindings for libpq")
        .write_to_file(PathBuf::from(out_dir).join("bindings.rs"))
        .expect("Unable to write bindings to file");
}
