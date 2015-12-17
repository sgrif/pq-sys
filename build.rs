use std::process::Command;

fn main() {
    for output in Command::new("pg_config").arg("--libdir").output() {
        if output.status.success() {
            for path in String::from_utf8(output.stdout) {
                println!("cargo:rustc-link-search=native={}", &path);
            }
        }
    }
}
