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
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let use_openssl = env::var("CARGO_FEATURE_WITH_OPENSSL").is_ok();

    println!("cargo:rerun-if-changed=additional_include");
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Set by cargo");
    let temp_include = format!("{}/more_include/", env::var("OUT_DIR").unwrap());
    let path = format!("{crate_dir}/source/");
    let port_path = "src/port/";
    let common_path = "src/common/";
    let pq_path = "src/interfaces/libpq/";

    if !PathBuf::from(&temp_include).exists() {
        fs::create_dir(&temp_include).unwrap();
    }
    if !PathBuf::from(format!("{temp_include}pg_config_os.h")).exists() {
        match target_os.as_str() {
            "linux" => {
                fs::copy(
                    format!("{path}src/include/port/linux.h"),
                    format!("{temp_include}pg_config_os.h"),
                )
                .unwrap();
            }
            "macos" => {
                fs::copy(
                    format!("{path}src/include/port/darwin.h"),
                    format!("{temp_include}pg_config_os.h"),
                )
                .unwrap();
            }
            "windows" => {
                fs::copy(
                    format!("{path}src/include/port/win32.h"),
                    format!("{temp_include}pg_config_os.h"),
                )
                .unwrap();
                println!("cargo:rustc-link-lib=Secur32");
                println!("cargo:rustc-link-lib=Shell32");
            }
            _ => unimplemented(),
        }
    }

    let mut basic_build = cc::Build::new();

    let base_includes = &[
        format!("{path}{port_path}"),
        format!("{path}src/include"),
        format!("{crate_dir}/additional_include"),
        temp_include.clone(),
    ][..];

    let mut includes = if target_os == "windows" {
        let includes_windows = &[
            format!("{path}/src/include/port/win32/"),
            format!("{path}/src/include/port/win32_msvc/"),
        ];
        [base_includes, includes_windows].concat()
    } else {
        base_includes.to_vec()
    };

    if use_openssl {
        includes.push(env::var("DEP_OPENSSL_INCLUDE").unwrap());
    }

    basic_build
        .define("FRONTEND", None)
        .warnings(false)
        .includes(includes);

    if env::var("CARGO_FEATURE_WITH_ASAN").is_ok() {
        basic_build.flag("-fsanitize=address");
    }

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
    // don't even try on windows it's not there
    if std::env::var("CARGO_CFG_WINDOWS").is_err() {
        conditional_define(TEST_FOR_STRCHRNUL, "HAVE_STRCHRNUL", &mut basic_build);
    }
    conditional_define(TEST_FOR_STRSIGNAL, "HAVE_STRSIGNAL", &mut basic_build);

    let libports = LIBPORTS_BASE.iter().chain(libports_os);
    let libcommon = libcommon.iter().chain(libcommon_os);
    let libpq = libpq.iter().chain(libpq_os);

    basic_build
        .files(
            (libports.map(|p| format!("{path}{port_path}{p}")))
                .chain(libcommon.map(|p| format!("{path}{common_path}{p}")))
                .chain(libpq.map(|p| format!("{path}{pq_path}{p}"))),
        )
        .compile("pq");

    let out = env::var("OUT_DIR").expect("Set by cargo");
    let include_path = PathBuf::from(&out).join("include");
    let lib_pq_path = PathBuf::from(format!("{path}/{pq_path}"));
    let postgres_include_path = PathBuf::from(format!("{path}src/include"));
    let additional_includes_path = PathBuf::from(format!("{crate_dir}/additional_include"));
    fs::create_dir_all(&include_path).expect("Failed to create include directory");
    fs::create_dir_all(include_path.join("postgres").join("internal"))
        .expect("Failed to create include directory");
    fs::copy(
        lib_pq_path.join("libpq-fe.h"),
        include_path.join("libpq-fe.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        lib_pq_path.join("libpq-events.h"),
        include_path.join("libpq-events.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        postgres_include_path.join("postgres_ext.h"),
        include_path.join("postgres_ext.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        additional_includes_path.join("pg_config_ext.h"),
        include_path.join("pg_config_ext.h"),
    )
    .expect("Copying headers failed");

    fs::copy(
        lib_pq_path.join("libpq-int.h"),
        include_path
            .join("postgres")
            .join("internal")
            .join("libpq-int.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        lib_pq_path.join("fe-auth-sasl.h"),
        include_path
            .join("postgres")
            .join("internal")
            .join("fe-auth-sasl.h"),
    )
    .expect("Copying headers failed");
    fs::copy(
        lib_pq_path.join("pqexpbuffer.h"),
        include_path
            .join("postgres")
            .join("internal")
            .join("pqexpbuffer.h"),
    )
    .expect("Copying headers failed");

    println!("cargo:include={out}/include");
    println!("cargo:lib_dir={}", out);
}
