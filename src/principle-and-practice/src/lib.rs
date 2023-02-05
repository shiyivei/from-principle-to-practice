#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    test(no_crate_inject, attr(deny(warnings))),
    test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]

pub mod expression;
pub mod function_and_closure;
pub mod iterator;
pub mod language_structure;
pub mod lexical_structure;
pub mod mod_member;
pub mod ownership;
pub mod pattern_match;
pub mod smart_pointer;
pub mod trait_impl;
pub mod type_system;
