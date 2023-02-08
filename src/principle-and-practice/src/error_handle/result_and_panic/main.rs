// 6 ？操作符
// 还在构建

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    use std::panic;

    let result = panic::catch_unwind(|| println!("hello"));
    assert!(result.is_ok());
    let result = panic::catch_unwind(|| panic!("oh no"));
    assert!(result.is_err());

    println!("{}", sum(1, 2));
}
