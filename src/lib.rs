#[cfg(feature = "bundled")]
extern crate pq_src;

#[cfg(not(feature = "buildtime_bindgen"))]
#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[cfg_attr(target_os = "windows", path = "windows_bindings.rs")]
mod bindings;

#[cfg(feature = "buildtime_bindgen")]
#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
