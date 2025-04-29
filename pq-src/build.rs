use std::path::PathBuf;
use std::{env, fs};

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPORTS_BASE: &[&str] = &[
    "strlcat.c",
    "strlcpy.c",
    "snprintf.c",
    "pg_crc32c_sb8.c",
    "bsearch_arg.c",
    "chklocale.c",
    "inet_net_ntop.c",
    "noblock.c",
    "pg_bitutils.c",
    "pg_strong_random.c",
    "pgcheckdir.c",
    "pgmkdirp.c",
    "pgsleep.c",
    "pgstrcasecmp.c",
    "pgstrsignal.c",
    "pqsignal.c",
    "qsort.c",
    "quotes.c",
    "strerror.c",
    "tar.c",
    "explicit_bzero.c",
];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPORTS_LINUX: &[&str] = &["getpeereid.c", "user.c"];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPORTS_MACOS: &[&str] = &["user.c"];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPORTS_WINDOWS: &[&str] = &[
    "getpeereid.c",
    "win32common.c",
    "win32dlopen.c",
    "win32env.c",
    "win32error.c",
    "win32fdatasync.c",
    "win32fseek.c",
    "win32getrusage.c",
    "win32gettimeofday.c",
    "win32link.c",
    "win32ntdll.c",
    "win32pread.c",
    "win32pwrite.c",
    "win32security.c",
    "win32setlocale.c",
    "win32stat.c",
    "win32gai_strerror.c",
    "open.c",
    "dirmod.c",
    "inet_aton.c",
];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBCOMMON_BASE: &[&str] = &[
    "file_perm.c",
    "encnames.c",
    "base64.c",
    "scram-common.c",
    "ip.c",
    "jsonapi.c",
    "kwlookup.c",
    "link-canary.c",
    "md5_common.c",
    "percentrepl.c",
    "pg_get_line.c",
    "pg_lzcompress.c",
    "pg_prng.c",
    "pgfnames.c",
    "psprintf.c",
    "rmtree.c",
    "saslprep.c",
    "string.c",
    "stringinfo.c",
    "unicode_norm.c",
    "username.c",
    "wait_error.c",
    "wchar.c",
    "fe_memutils.c",
    "restricted_token.c",
    "sprompt.c",
    "logging.c",
];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBCOMMON_OPENSSL: &[&str] = &[
    "cryptohash_openssl.c",
    "hmac_openssl.c",
    "protocol_openssl.c",
];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBCOMMON_NOT_OPENSSL: &[&str] = &["cryptohash.c", "hmac.c", "md5.c", "sha1.c", "sha2.c"];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBCOMMON_NOT_WINDOWS: &[&str] = &[];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBCOMMON_WINDOWS: &[&str] = &["wchar.c"];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPQ_BASE: &[&str] = &[
    "fe-auth-scram.c",
    "fe-auth.c",
    "fe-cancel.c",
    "fe-connect.c",
    "fe-exec.c",
    "fe-lobj.c",
    "fe-misc.c",
    "fe-print.c",
    "fe-protocol3.c",
    "fe-secure.c",
    "fe-trace.c",
    "legacy-pqsignal.c",
    "libpq-events.c",
    "pqexpbuffer.c",
];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPQ_OPENSSL: &[&str] = &["fe-secure-common.c", "fe-secure-openssl.c"];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPQ_NOT_WINDOWS: &[&str] = &[];

// If you update this file list also
// update the list of includes files in
// the Cargo.toml file
const LIBPQ_WINDOWS: &[&str] = &["fe-secure.c", "pthread-win32.c", "win32.c"];

fn unimplemented() -> ! {
    unimplemented!(
        "Building a bundled version of libpq is currently not supported for this OS\n\
        If you are interested in support for using a bundled libpq we are happy to accept patches \
        at https://github.com/sgrif/pq-sys/"
    );
}

macro_rules! make_test_for {
    ($name: ident, $test: literal) => {
        const $name: &str = concat!(
            r#"
#include<string.h>

int main() {"#,
            $test,
            r#"
}"#
        );
    };
}

make_test_for!(TEST_FOR_STRCHRNUL, r#"strchrnul("", 42);"#);
make_test_for!(TEST_FOR_STRSIGNAL, r#"strsignal(32);"#);

fn check_compiles(test: &str) -> bool {
    let test_path = std::env::var("OUT_DIR").expect("Set by cargo") + "/test.c";
    std::fs::write(&test_path, &test).expect("Failed to write test");
    let r = cc::Build::new().file(&test_path).try_compile("test");
    std::fs::remove_file(test_path).expect("Failed to remove test file");
    if let Err(ref e) = r {
        println!("{e}");
    }
    r.is_ok()
}

fn conditional_define(test: &str, define: &str, command: &mut cc::Build) {
    if check_compiles(test) {
        command.define(define, None);
    }
}

fn main() {
    // Get build information from environment
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let use_openssl = env::var("CARGO_FEATURE_WITH_OPENSSL").is_ok();

    println!("cargo:rerun-if-changed=additional_include");

    // === Get and define various paths ===

    // Path to pq-src (where this build.rs is located)
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Set by cargo");
    // Path to where this build script shall place it's output
    let out = env::var("OUT_DIR").expect("Set by cargo");

    // Path to additional includes that are shipped with pq-src
    let additional_includes_path = PathBuf::from(&crate_dir).join("additional_include");

    // Path to PostgreSQL source code
    let psql_source_path = PathBuf::from(&crate_dir).join("source");

    // Paths to relevant components within the PostgreSQL source code
    let psql_include_path = psql_source_path.join("src").join("include");
    let port_path = psql_source_path.join("src").join("port");
    let common_path = psql_source_path.join("src").join("common");
    let libpq_path = psql_source_path
        .join("src")
        .join("interfaces")
        .join("libpq");

    // For includes that are created during build time
    let temp_include = PathBuf::from(&out).join("more_include");
    if !temp_include.exists() {
        fs::create_dir(&temp_include).unwrap();
    }

    // Select port header based on target OS
    if !temp_include.join("pg_config_os.h").exists() {
        match target_os.as_str() {
            "linux" => {
                fs::copy(
                    psql_include_path.join("port/linux.h"),
                    temp_include.join("pg_config_os.h"),
                )
                .unwrap();
            }
            "macos" => {
                fs::copy(
                    psql_include_path.join("port/darwin.h"),
                    temp_include.join("pg_config_os.h"),
                )
                .unwrap();
            }
            "windows" => {
                fs::copy(
                    psql_include_path.join("port/win32.h"),
                    temp_include.join("pg_config_os.h"),
                )
                .unwrap();
                println!("cargo:rustc-link-lib=Secur32");
                println!("cargo:rustc-link-lib=Shell32");
            }
            _ => unimplemented(),
        }
    }

    // Include paths for all builds
    let base_includes = &[
        port_path.clone(),
        psql_include_path.clone(),
        additional_includes_path.clone(),
        temp_include.clone(),
    ][..];

    // Add additional include paths for windows builds
    let mut includes = if target_os == "windows" {
        let includes_windows = &[
            psql_include_path.join("port").join("win32"),
            psql_include_path.join("port").join("win32_msvc"),
        ];
        [base_includes, includes_windows].concat()
    } else {
        base_includes.to_vec()
    };

    // Add includes for openssl (if required)
    if use_openssl {
        let openssl_include_path = PathBuf::from(env::var("DEP_OPENSSL_INCLUDE").unwrap());
        includes.push(openssl_include_path);
    }

    // Create "compiler" and add previously determined includes
    let mut basic_build = cc::Build::new();
    basic_build
        .define("FRONTEND", None)
        .warnings(false)
        .includes(includes);

    if env::var("CARGO_FEATURE_WITH_ASAN").is_ok() {
        basic_build.flag("-fsanitize=address");
    }

    // Add necessary defines based on OS and collect OS-specific files for compilation
    let (libports_os, libcommon_os, libpq_os) = match target_os.as_str() {
        "linux" => {
            basic_build.define("_GNU_SOURCE", None);
            (LIBPORTS_LINUX, LIBCOMMON_NOT_WINDOWS, LIBPQ_NOT_WINDOWS)
        }
        "macos" => {
            // something is broken in homebrew
            // https://github.com/Homebrew/legacy-homebrew/pull/23620
            basic_build.define("_FORTIFY_SOURCE", Some("0"));
            (LIBPORTS_MACOS, LIBCOMMON_NOT_WINDOWS, LIBPQ_NOT_WINDOWS)
        }
        "windows" => {
            basic_build.define("WIN32", None);
            basic_build.define("_WINDOWS", None);
            basic_build.define("__WIN32__", None);
            basic_build.define("__WINDOWS__", None);
            basic_build.define("HAVE_SOCKLEN_T", Some("1"));
            (LIBPORTS_WINDOWS, LIBCOMMON_WINDOWS, LIBPQ_WINDOWS)
        }
        _ => unimplemented(),
    };

    // Add defines for openssl (if needed) and collect files for compilation
    let (libcommon, libpq) = if use_openssl {
        // Define to 1 to build with OpenSSL support. (--with-ssl=openssl)
        basic_build.define("USE_OPENSSL", "1");
        (
            [LIBCOMMON_BASE, LIBCOMMON_OPENSSL].concat(),
            [LIBPQ_BASE, LIBPQ_OPENSSL].concat(),
        )
    } else {
        (
            [LIBCOMMON_BASE, LIBCOMMON_NOT_OPENSSL].concat(),
            LIBPQ_BASE.to_vec(),
        )
    };

    // Check if strchrnul and/or strsignal are supported, if so add defines for them
    conditional_define(TEST_FOR_STRCHRNUL, "HAVE_STRCHRNUL", &mut basic_build);
    conditional_define(TEST_FOR_STRSIGNAL, "HAVE_STRSIGNAL", &mut basic_build);

    // Collect files for compilation
    let libports = LIBPORTS_BASE.iter().chain(libports_os);
    let libcommon = libcommon.iter().chain(libcommon_os);
    let libpq = libpq.iter().chain(libpq_os);

    // Compile code into statically linked library
    // Note: The compilation will output to $OUT_DIR/libpq.a
    basic_build
        .files(
            libports
                .map(|p| port_path.join(p))
                .chain(libcommon.map(|p| common_path.join(p)))
                .chain(libpq.map(|p| libpq_path.join(p))),
        )
        .compile("pq");

    // Directory that shall hold the relevant headers for next step(s)
    let include_path = PathBuf::from(&out).join("include");
    // Same as include_path, but for postgres internals
    let postgres_internal_path = include_path.join("postgres").join("internal");

    // Create out/include/postgres/internal directory (incl. parent directories)
    fs::create_dir_all(&postgres_internal_path).expect("Failed to create include directory");

    // Copy over relevant headers for next step(s)
    fs::copy(
        libpq_path.join("libpq-fe.h"),
        include_path.join("libpq-fe.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        libpq_path.join("libpq-events.h"),
        include_path.join("libpq-events.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        psql_include_path.join("postgres_ext.h"),
        include_path.join("postgres_ext.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        additional_includes_path.join("pg_config_ext.h"),
        include_path.join("pg_config_ext.h"),
    )
    .expect("Copying headers failed");

    fs::copy(
        libpq_path.join("libpq-int.h"),
        postgres_internal_path.join("libpq-int.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        libpq_path.join("fe-auth-sasl.h"),
        postgres_internal_path.join("fe-auth-sasl.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        libpq_path.join("pqexpbuffer.h"),
        postgres_internal_path.join("pqexpbuffer.h"),
    )
    .expect("Copying headers failed");

    println!("cargo:include={out}/include");
    println!("cargo:lib_dir={out}");
}
