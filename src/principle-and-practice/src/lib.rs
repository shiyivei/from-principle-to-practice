#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]

pub mod expression;
pub mod lexical_structure;
pub mod trait_impl;
pub mod type_system;
