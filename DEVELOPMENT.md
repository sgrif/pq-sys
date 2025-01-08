# DEVELOPMENT NOTES

Use the following command to generate a new bindings file:
```sh
# keep the command in sync with the command in `src/make_bindings.rs"
bindgen wrapper.h \
    --rustified-enum ".*" \
    --no-derive-default \
    --generate "functions,types,vars,methods,constructors,destructors" \
    --allowlist-var "PG_.*" \
    --allowlist-var "LIBPQ_.*" \
    --allowlist-var "PQ.*" \
    --allowlist-type "Oid" \
    --allowlist-type "ConnStatusType"  \
    --allowlist-type "Postgres.*" \
    --allowlist-type "pg.*" \
    --allowlist-type "PG.*" \
    --allowlist-type "PQ.*" \
    --allowlist-type "pq.*" \
    --allowlist-function "PQ.*" \
    --allowlist-function "lo_.*" \
    --allowlist-function "pg_.*" \
    --opaque-type "FILE" \
    --blocklist-type "FILE" \
    --raw-line "use libc::FILE;" \
    -- -I pq-src/source/src/interfaces/libpq/ -I pq-src/source/src/include/ -I pq-src/additional_include/
```

It is required to generate bindings for the following targets:

* Linux 64 bit
* Linux 32 bit (different field sizes, compilation fails otherwise due to const checks)
* Windows (MSVC) 64 bit (uses `#[repr(i32)]` instead of `#[repr(u32)]` for enums,
  can simply replace that in the generated linux bindings)
* Windows (MSVC) 32 bit (same as 64 bit windows + 32 bit linux)
