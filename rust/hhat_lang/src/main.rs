#![allow(
    dead_code,
    unused,
    clippy::upper_case_acronyms,
    clippy::large_enum_variant,
    clippy::enum_variant_names
)]

// CI demo: rust workflow trigger

use std::collections::HashMap;

mod backends;
mod config;
mod ir;
mod jit;
mod parse;
mod passes;
mod runtime;
mod semantics;
mod subcompilers;
mod toolchain;
mod utils;

fn main() {}
