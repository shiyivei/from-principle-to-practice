use serde::Deserialize;
use std::borrow::Cow;

fn main() {
    let s = String::from("hello world");
    let t = &s;

    let b = &s[..];
    let x: &str = s.as_ref();

    let s = s.to_string();
}
