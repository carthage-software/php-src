#![allow(non_upper_case_globals)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod ext;
pub mod zend;
