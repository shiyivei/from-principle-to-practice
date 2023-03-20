//! 并发编程
//!
/**

```
    // 这里我们为了创建异步线程，所以使用了异步main
#[tokio::main]
async fn main() {
    // 线程的创建

    // 使用标准库创建
    use std::thread;

    for i in 0..=5 {
        thread::spawn(move || {
            println!("from std {}", i + 10);
        });
    }

    // 创建多个异步线程

    use tokio::task;
    for i in 0..=5 {
        task::spawn(handle_task(i));
    }

    async fn handle_task(i: u32) {
        println!("from tokio {}", i + 10);
    }

    let handle = thread::spawn(|| println!("from std too {}", 110));

    // 使用 join 等待所有线程完成
    handle.join().unwrap();

    let v = vec![1, 2, 3, 4, 5];

    // thread::spawn(|| println!("{:?}", v)); // 在线程结束之前main函数有可能先结束，但此时闭包仍旧使用的是main 函数中的v，Rust不允许这种行为
    // 显式使用move 将数据移入闭包
    thread::spawn(move || println!("{:?}", v));

    let x = 4;
    println!("{:p}", &x); // 0x7ff7bfe250b4
                          // thread::spawn(|| println!("{:p}", &x)); // 引用同样需要把数据移入闭包，才能保证闭包使用变量的时候，变量一直存在，而不是被其他线程丢弃
    thread::spawn(move || println!("{:p}", &x)); // 0x7000090dfc64 地址不一样，说明数据本身也发生了复制，而不仅仅是对原来数据的引用

    // 并发安全

    /*

    pub unsafe auto trait Send {
        // empty.
    }

    pub unsafe auto trait Sync {
        // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
        // lands in beta, and it has been extended to check whether a closure is
        // anywhere in the requirement chain, extend it as such (#48534):
        // ```
        // on(
        //     closure,
        //     note="`{Self}` cannot be shared safely, consider marking the closure `move`"
        // ),
        // ```

        // Empty
    }

    */

    // 通过 channel 发送数据

    use std::sync::mpsc;

    // 创建通道
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        // 在线程中创建变量
        let val1 = String::from("hi");
        let val2 = String::from("hello");

        // 将变量发送给别的线程
        tx1.send(val1).unwrap();
        tx2.send(val2).unwrap();

        // println!("{:?}", val); // 不能再使用
    });

    // 别的线程接收数据
    for received in rx {
        println!("Got: {}", received);
    }

    // 使用锁

    use std::sync::{Arc, Mutex};

    // 创建数据
    // 在使用线程时，我们需要将数据移入线程内，但是一旦移入，数据就不可用了，所以使用引用计数容器Arc共享所有权
    // 同时通过Mutex来保证独占访问
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // 拿到锁
            let mut num = counter.lock().unwrap();

            // 修改数据
            *num += 1;

            // 锁释放
            // lock 调用会一个叫做 MutexGuard 的智能指针
            // 这个智能指针实现了 Deref 和 Drop trait
            // 可以自动解引用以及丢弃值
            // 此处自动调用了 drop()
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


```
*/

pub fn threads_and_concurrency() {}
