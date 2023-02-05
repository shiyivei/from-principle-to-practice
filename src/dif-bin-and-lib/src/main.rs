use std::borrow::Borrow;

#[allow(unused)]
fn main() {
    // move 语义的本质

    let mut a = "42".to_string();
    let b = a;

    // 上面两个表达式的共同作用等价于将a重制为未出始化状态
    // 而不是立即调用drop（析构函数）丢弃
    // 当函数调用结束时才会被都去丢弃
    let mut a: String;

    // 重新给a赋值
    a = "a".to_string();

    // 可以继续使用
    println!("{:?}", a);
}
