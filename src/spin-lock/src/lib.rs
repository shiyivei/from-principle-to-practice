use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt;

// 定义锁
struct Lock<T> {
    locked: RefCell<bool>, // 使用了RefCell容器,在运行时可以改变内部的值
    data: RefCell<T>,
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
    pub fn locked(&self, op: impl FnOnce(&mut T)) {
        while *self.locked.borrow() != false {} // 没拿到锁就一直spin

        // 加锁
        *self.locked.borrow_mut() = true;

        // 做任务
        op(&mut self.data.borrow_mut());

        // 释放
        *self.locked.borrow_mut() = false;
    }
}
