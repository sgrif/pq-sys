// Fix for #25
#[cfg(feature = "openssl-static")]
extern crate openssl_sys;

mod bindgen;

pub use bindgen::*;
