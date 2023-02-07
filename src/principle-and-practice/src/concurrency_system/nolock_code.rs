//! 无锁并发

/**

### 自旋锁

```
 use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    // 实现一个自旋锁

    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);

    let thread = thread::spawn(move || {
        spinlock_clone.store(1, Ordering::SeqCst);
        let t = Duration::from_secs(3);
        std::thread::sleep(t);

        spinlock_clone.store(0, Ordering::SeqCst);
    });

    while spinlock.load(Ordering::SeqCst) != 0 {}

    if let Err(panic) = thread.join() {
        println!("Thread had an error {:?}", panic)
    }
 ```
### 使用原子类型实现轻量级的锁

 ```
    use core::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    struct LightLock(AtomicBool);

    impl LightLock {
        pub fn new() -> LightLock {
            LightLock(AtomicBool::new(false))
        }

        pub fn try_lock<'a>(&'a self) -> Option<LightGuard<'a>> {
            let was_locked = self.0.swap(true, Ordering::Acquire);

            if was_locked {
                None
            } else {
                Some(LightGuard { lock: self })
            }
        }
    }

    struct LightGuard<'a> {
        lock: &'a mut LightLock,
    }

    impl<'a> Drop for LightGuard<'a> {
        fn drop(&mut self) {
            self.lock.0.store(false, Ordering::Release)
        }
    }

 ```
 */

pub fn spinlock() {}
