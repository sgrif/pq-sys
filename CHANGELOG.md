# Change Log
All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## [0.2.6] 2016-12-10

### Added

- We will attempt to use `pkg-config` to locate libpq before falling back to
  `pg_config`.

## [0.2.5] 2016-12-10

- No changes. Accidental release on the wrong commit.

## [0.2.4] 2016-11-22

### Added

- `pq` will be statically linked if the environment variable `PQ_LIB_STATIC` is
  set.

## [0.2.3] 2016-08-12

### Changed

- On Mac if `pg_config` points to a directory where `libpq.dylib` is a symlink,
  we will now attempt to find the canonical directory to link against. This
  means that for installations using homebrew,
  `/usr/local/Cellar/postgresql/version/lib` will be added to `DYLD_LIBRARY_PATH`
  instead of `/usr/local/lib`

## [0.2.2] 2016-07-30

### Added

- The directory containing libpq for linking can now be specified via the
  `PQ_LIB_DIR` environment variable.
