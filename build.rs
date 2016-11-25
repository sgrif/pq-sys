use std::path::PathBuf;
use std::process::Command;
use std::env;

fn main() {
    if let Ok(lib_dir) = env::var("PQ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    } else if let Some(path) = pg_config_output() {
        let path = follow_dylib_symlinks(path.trim().into());
        println!("cargo:rustc-link-search=native={}", &path.display());
    }

    let mode = if env::var_os("PQ_LIB_STATIC").is_some() {
        "static"
    } else {
        "dylib"
    };
    let lib = if cfg!(target_os = "windows") {
        "libpq"
    } else {
        "pq"
    };
    
    println!("cargo:rustc-link-lib={}={}", mode, lib);
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
