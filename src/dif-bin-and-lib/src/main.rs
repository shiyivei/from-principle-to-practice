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

    use std::future::Future;
    // async 真正会返回 Future<Output = u8>, 而不是看上去的u8
    async fn foo() -> u8 {
        5
    }

    // async 块用法，返回 "impl Future<Output = u8>"
    fn bar() -> impl Future<Output = u8> {
        async {
            let x = foo().await;
            x + 5
        }
    }
}
