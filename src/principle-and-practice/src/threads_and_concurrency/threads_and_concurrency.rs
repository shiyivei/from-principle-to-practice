//! 线程与并发
//!
/**
 ###  Rust中的线程
```
    // 时间间隔
    let duration = std::time::Duration::from_millis(30000);

    println!("main thread ");

    use std::thread;

    // 使用 thread
    let handle = thread::spawn(move || {
        println!("sub thread 1");

        let handle2 = thread::spawn(move || {
            println!("sub thread 2");
            thread::sleep(duration)
        });

        handle2.join().unwrap();
        thread::sleep(duration)
    });

    handle.join().unwrap();
    thread::sleep(duration)

    // rust 并不保证线程之间的引用之间的生命周期关系
    // rust线程由操作系统调度

```
### 在线程间共享数据

```
// 在线程间共享数据

    // 案例 1 通过借用检查，消除数据竞争
    use std::thread;
    let mut v = vec![1, 2, 3, 4];
    //     thread::spawn(move || v.push(5)); // v 只能使用1次，无法使用for 循环迭代加入元素

    // 借用规则要求可变借用只能有一次，避免了数据竞争（多个线程同时使用 v ）
    //     for i in 0..10 {
    //         thread::spawn(move || v.push(i));
    //     }

    // 案例 2 通过函数来传递数据，也不被允许
    // 线程中没法传递引用，因为不知道线程执行顺序
    // 如果线程封装在函数中，不知道函数会被在哪里调用以及调用多少次

    //     fn inner_func(vref: &mut Vec<u32>) {
    //         std::thread::spawn(move || vref.push(3));
    //     }

    // 案例 4 只读也不能通过函数传递吗？ 不能，可能存在悬垂指针

    //     fn inner_func(vref: &Vec<u32>) {
    //         std::thread::spawn(move || println!("{:?}", vref));
    //     }

    // 案例 5 如何在线程间传递引用

    // 5.1 不使用第三方库的实现（加‘static类型）
    use std::fmt;
    struct Foo {
        string: String,
        v: Vec<f64>,
    }

    impl fmt::Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}: {:?}", self.string, self.v)
        }
    }

    // 封装到函数中
    // 但是存在约束条件
    fn test<T: Send + Sync + fmt::Display + 'static>(val: T) {
        thread::spawn(move || println!("{}", val));
    }

    test("hello"); //&'static str
    test(String::from("hello")); // String，因为它是所有权的数据，与程序同生同灭
    test(5); // i32

    // 内部的数据是由所有权的，所以也可以作为参数传递
    let foo = Foo {
        string: String::from("hello"),
        v: vec![1.2, 2.2, 3.2, 42.2],
    };
    test(foo);
    //     test(foo); 不能使用第二次

    use std::time::Duration;
    thread::sleep(Duration::new(1, 0));

    // 5.2 使用第三方库crossbeam 的实现
    // crossbeam::scope 共享数据
    use crossbeam;
    let mut vec = vec![1, 2, 3, 4, 5];

    crossbeam::scope(|scope| {
        // scope 出来的子线程会在主线程关闭之前回收
        // 保证不会出现悬垂指针
        scope.spawn(move |_| {
            for e in &vec {
                println!("{:?}", e);
            }
        });
    })
    .expect("a child thread panicked");

    let mut v = vec![1, 2, 3, 4, 5];

    crossbeam::scope(|scope| {
        // scope 出来的子线程会在主线程关闭之前回收
        // 不出现数据竞争
        for e in &mut v {
            scope.spawn(move |_| thread::sleep(Duration::from_secs(1)));
        }
    })
    .expect("a child thread panicked");

    use std::sync::{Arc, Mutex};
    // 5.3 也可以使用Arc和Mutex实现共享所有权
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));

    // 每次都克隆一个
    for i in 0..3 {
        let cloned_v = v.clone();
        thread::spawn(move || {
            cloned_v.lock().unwrap().push(i);
        });
    }
```
*/

pub fn threads_and_concurrency() {}
