extern crate pkg_config;

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;

fn main() {
    let link_flag = "pq";
    let pkg_name = "libpq";

    if let Ok(lib_dir) = env::var("PQ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);

    } else if pkg_config::probe_library(pkg_name).is_ok() {
        return // pkg_config does everything for us, including output for cargo

    } else if let Some(path) = pg_config_output() {
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

fn pg_config_output() -> Option<String> {
    Command::new("pg_config")
        .arg("--libdir")
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
