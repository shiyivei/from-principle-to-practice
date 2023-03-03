use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::sync::Arc;
use std::{fmt, thread};

// 定义锁
struct Lock<T> {
    locked: RefCell<bool>, // 使用了RefCell容器,表示锁的状态
    data: RefCell<T>,      // 实际可以操作的数据
}

// 实现debug
impl<T> fmt::Debug for Lock<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lock<{:?}>", self.data.borrow())
    }
}

// 实现sync，可以在线程中共享
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: RefCell::new(false),
            data: RefCell::new(data),
        }
    }

    // 拿锁执行任务
    pub fn lock(&self, op: impl FnOnce(&mut T)) {
        while *self.locked.borrow() != false {} // 没拿到锁就一直spin

        // 加锁
        *self.locked.borrow_mut() = true;

        // 做任务
        op(&mut self.data.borrow_mut());

        // 释放
        *self.locked.borrow_mut() = false;
    }
}

fn main() {
    let data = Arc::new(Lock::new(1));

    let data1 = data.clone();

    let t1 = thread::spawn(move || data1.lock(|v| *v *= 10));

    let data2 = data.clone();
    let t2 = thread::spawn(move || data2.lock(|v| *v *= 10));

    t1.join();
    t2.join();

    println!("Data :{:?}", data)
}
