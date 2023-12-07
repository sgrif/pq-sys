use std::path::PathBuf;

const LIBPORTS: &'static [&'static str] = &[
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    "getpeereid.c",
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
    #[cfg(not(target_os = "windows"))]
    "thread.c",
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    "explicit_bzero.c",
    #[cfg(target_os = "windows")]
    "win32common.c",
    #[cfg(target_os = "windows")]
    "win32dlopen.c",
    #[cfg(target_os = "windows")]
    "win32env.c",
    #[cfg(target_os = "windows")]
    "win32error.c",
    #[cfg(target_os = "windows")]
    "win32fdatasync.c",
    #[cfg(target_os = "windows")]
    "win32fseek.c",
    #[cfg(target_os = "windows")]
    "win32getrusage.c",
    #[cfg(target_os = "windows")]
    "win32gettimeofday.c",
    #[cfg(target_os = "windows")]
    "win32link.c",
    #[cfg(target_os = "windows")]
    "win32ntdll.c",
    #[cfg(target_os = "windows")]
    "win32pread.c",
    #[cfg(target_os = "windows")]
    "win32pwrite.c",
    #[cfg(target_os = "windows")]
    "win32security.c",
    #[cfg(target_os = "windows")]
    "win32setlocale.c",
    #[cfg(target_os = "windows")]
    "win32stat.c",
    #[cfg(target_os = "windows")]
    "open.c",
    #[cfg(target_os = "windows")]
    "dirmod.c",
];

const LIBCOMMON: &'static [&'static str] = &[
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
    #[cfg(not(target_os = "windows"))]
    "cryptohash_openssl.c",
    #[cfg(target_os = "windows")]
    "cryptohash.c",
    #[cfg(not(target_os = "windows"))]
    "hmac_openssl.c",
    #[cfg(target_os = "windows")]
    "hmac.c",
    #[cfg(not(target_os = "windows"))]
    "protocol_openssl.c",
    "fe_memutils.c",
    "restricted_token.c",
    "sprompt.c",
    "logging.c",
    #[cfg(target_os = "windows")]
    "md5.c",
    #[cfg(target_os = "windows")]
    "sha1.c",
    #[cfg(target_os = "windows")]
    "sha2.c",
    #[cfg(target_os = "windows")]
    "wchar.c",
];

const LIBPQ: &'static [&'static str] = &[
    "fe-auth-scram.c",
    "fe-auth.c",
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
    #[cfg(not(target_os = "windows"))]
    "fe-secure-common.c",
    #[cfg(not(target_os = "windows"))]
    "fe-secure-openssl.c",
    #[cfg(target_os = "windows")]
    "fe-secure.c",
    #[cfg(target_os = "windows")]
    "pthread-win32.c",
    #[cfg(target_os = "windows")]
    "win32.c",
];

fn main() {
    println!("cargo:rerun-if-changed=additional_include");
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let temp_include = format!("{}/more_include/", std::env::var("OUT_DIR").unwrap());
    let path = format!("{crate_dir}/source/");
    let port_path = "src/port/";
    let common_path = "src/common/";
    let pq_path = "src/interfaces/libpq/";

    if !PathBuf::from(&temp_include).exists() {
        std::fs::create_dir(&temp_include).unwrap();
    }
    if !PathBuf::from(format!("{temp_include}pg_config_os.h")).exists() {
        if cfg!(target_os = "linux") {
            std::fs::copy(
                format!("{path}src/include/port/linux.h"),
                format!("{temp_include}pg_config_os.h"),
            )
            .unwrap();
        } else if cfg!(target_os = "macos") {
            std::fs::copy(
                format!("{path}src/include/port/darwin.h"),
                format!("{temp_include}pg_config_os.h"),
            )
            .unwrap();
        } else if cfg!(target_os = "windows") {
            std::fs::copy(
                format!("{path}src/include/port/win32.h"),
                format!("{temp_include}pg_config_os.h"),
            )
            .unwrap();
            println!("cargo:rustc-link-lib=Secur32");
            println!("cargo:rustc-link-lib=Shell32");
        } else {
            unimplemented!(
                "Building a bundled version of libpq is currently not supported for this OS\n\
                 If you are interested in support for using a bundled libpq we are happy to accept patches \n
                 at https://github.com/sgrif/pq-sys/"
            );
        }
    }
    #[cfg(not(target_os = "windows"))]
    let openssl = std::env::var("DEP_OPENSSL_INCLUDE").unwrap();

    let mut basic_build = cc::Build::new();

    basic_build
        .define("FRONTEND", None)
        .warnings(false)
        .includes([
            format!("{path}{port_path}"),
            format!("{path}src/include"),
            format!("{crate_dir}/additional_include"),
            temp_include.clone(),
            #[cfg(not(target_os = "windows"))]
            openssl.clone(),
            #[cfg(target_os = "windows")]
            format!("{path}/src/include/port/win32/"),
            #[cfg(target_os = "windows")]
            format!("{path}/src/include/port/win32_msvc/"),
        ]);

    if cfg!(target_os = "linux") {
        basic_build.define("_GNU_SOURCE", None);
    }
    if cfg!(target_os = "macos") {
        // something is broken in homebrew
        // https://github.com/Homebrew/legacy-homebrew/pull/23620
        basic_build.define("_FORTIFY_SOURCE", Some("0"));
    }
    if cfg!(target_os = "windows") {
        basic_build.define("WIN32", None);
        basic_build.define("_WINDOWS", None);
        basic_build.define("__WIN32__", None);
        basic_build.define("__WINDOWS__", None);
    }

    basic_build
        .clone()
        .files(LIBPORTS.iter().map(|p| format!("{path}{port_path}{p}")))
        .compile("ports");

    basic_build
        .clone()
        .files(LIBCOMMON.iter().map(|p| format!("{path}{common_path}{p}")))
        .compile("pgcommon");

    basic_build
        .files(LIBPQ.iter().map(|p| format!("{path}{pq_path}{p}")))
        .compile("pq");
}
