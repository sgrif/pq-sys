use std::process::Command;
use std::env;

fn main() {
    if let Ok(lib_dir) = env::var("PQ_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    } else {
        for output in Command::new("pg_config").arg("--libdir").output() {
            if output.status.success() {
                for path in String::from_utf8(output.stdout) {
                    println!("cargo:rustc-link-search=native={}", &path);
                }
            }
        }
    }
}
