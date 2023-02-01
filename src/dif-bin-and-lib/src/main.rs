pub struct S(i32);

impl S {
    fn correlation_function() {
        println!("correlation function");
    }
}

trait T1 {
    // 默认实现
    fn method1(&self) {
        println!("method1");
    }
    // 非默认实现
    fn new(&self, value: i32) -> Self;
}

impl T1 for S {
    fn new(&self, value: i32) -> Self {
        println!("new");
        Self(value)
    }
}

trait T2 {
    fn method2() {
        println!("method2")
    }
}

impl T2 for S {}

fn main() {
    let s = S(42);
    <S as T1>::method1(&s);
    <S as T1>::new(&s, 42);
    S::method2();

    let vec0 = (0..10).collect::<Vec<_>>();
    let vec1 = (0..10).collect::<Vec<u64>>();
    println!("{:?}", vec1);

    let vec2 = Vec::<u64>::with_capacity(1024);

    println!("{:?}", vec2);
}
