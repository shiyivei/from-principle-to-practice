#[allow(unused)]
fn main() {
    unsafe {
        let mut a = "hello".to_string();
        let b = &mut a;
        // let c = &mut a;

        b;
        // c;
    }

    // 解引用静态变量
    static mut COUNTER: u32 = 0;
    let inc = 3;

    unsafe {
        COUNTER += inc;
        println!("Counter: {}", COUNTER);
    }
}
