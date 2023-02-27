fn main() {
    // 1. str
    use std::str::Chars;
    let s1: &str = "2023"; // str 的引用
    let bytes: &[u8] = "bors".as_bytes(); // 转bytes
    let s: *const u8 = "Hello".as_ptr(); // 转指针
    let word: Chars = "goodbye".chars(); // 转字符

    // 2. String
    let s2 = "2023".to_string();
    pub struct String {
        vec: Vec<u8>,
    }
}
