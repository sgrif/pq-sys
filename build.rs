#[cfg(feature="pkg-config")]
extern crate pkg_config;

#[cfg(target_env = "msvc")]
extern crate vcpkg;

use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=PQ_LIB_DIR");
    println!("cargo:rerun-if-env-changed=PQ_LIB_STATIC");
    
    if let Ok(lib_dir) = env::var("PQ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    } else if configured_by_pkg_config() {
        return // pkg_config does everything for us, including output for cargo
    } else if configured_by_vcpkg() {
        return // vcpkg does everything for us, including output for cargo
    } else if let Some(path) = pg_config_output("--libdir") {
        let path = replace_homebrew_path_on_mac(path);
        println!("cargo:rustc-link-search=native={}", path);
    }

    if cfg!(all(windows, target_env="msvc")) {
        println!("cargo:rustc-link-lib=dylib=libpq");
    } else if env::var_os("PQ_LIB_STATIC").is_some() {
        println!("cargo:rustc-link-lib=static=pq");
    } else {
        println!("cargo:rustc-link-lib=pq");
    }
}

#[cfg(feature = "pkg-config")]
fn configured_by_pkg_config() -> bool {
    pkg_config::probe_library("libpq").is_ok()
}

#[cfg(not(feature = "pkg-config"))]
fn configured_by_pkg_config() -> bool {
    false
}

#[cfg(target_env = "msvc")]
fn configured_by_vcpkg() -> bool {
    vcpkg::probe_package("libpq").map(|_| {

        // found libpq which depends on openssl
        vcpkg::Config::new()
            .lib_name("libeay32")
            .lib_name("ssleay32")
            .probe("openssl").ok();

        println!("cargo:rustc-link-lib=crypt32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=secur32");
    }).is_ok()
}

#[cfg(not(target_env = "msvc"))]
fn configured_by_vcpkg() -> bool {
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
        .map(|output| output.trim().to_string())
        .next()
}

#[cfg(not(target_os = "macos"))]
fn replace_homebrew_path_on_mac(path: String) -> String {
    path
}

#[cfg(target_os = "macos")]
fn replace_homebrew_path_on_mac(path: String) -> String {
    if path == "/usr/local/lib" {
        Command::new("brew")
            .arg("--prefix")
            .arg("postgres")
            .output()
            .ok()
            .into_iter()
            .filter(|output| output.status.success())
            .flat_map(|output| String::from_utf8(output.stdout).ok())
            .map(|output| format!("{}/lib", output.trim()))
            .next()
            .unwrap_or(path)
    } else {
        path
    }
}
