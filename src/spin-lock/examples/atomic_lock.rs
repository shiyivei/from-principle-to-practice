use std::sync::atomic::{compiler_fence, AtomicBool};
use std::sync::Arc;
use std::thread;

fn main() {
    // 使用原子类型创建一个锁，通过引用计数获得共享所有权
    let spin_lock = Arc::new(AtomicBool::new(false));

    // 引用计数 +1
    let spin_lock_clone = Arc::clone(&spin_lock);
    let sc = Arc::clone(&spin_lock);

    let thread = thread::spawn(move || {
        // 写操作，并指定内存顺序 release 语义：写屏障之前的读写操作不能重排在写屏障之后
        spin_lock_clone.store(true, std::sync::atomic::Ordering::SeqCst);

        println!("spin_lock status a {:?}", sc);
        // 休眠
        let time = std::time::Duration::from_secs(2);
        std::thread::sleep(time);

        compiler_fence(std::sync::atomic::Ordering::Release);
        // 写操作， 并指定内存顺序 release 语义：写屏障之前的读写操作不能重排在写屏障之后
        // 上面有一个写操作，并且下面的指令要求不能在此之后
        spin_lock_clone.store(false, std::sync::atomic::Ordering::SeqCst);
        println!("spin_lock status b {:?}", sc);
    });

    // 读操作 指定内存顺序 acquire 语义 读屏障之后的读写操作不能重排到读写屏障之前
    // 上面的线程中有两条写指令，下面的指令要求之后的读写操作不能在此之前
    while spin_lock.load(std::sync::atomic::Ordering::SeqCst) == false {
        println!("spin_lock status c {:?}", spin_lock)
    }

    println!("spin_lock status d {:?}", spin_lock);

    if let Err(e) = thread.join() {
        println!("Thread had an error {:?}", e);
    }
}
