use anyhow::anyhow;
use anyhow::{Ok, Result};

use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, Arc, Condvar, Mutex},
};

pub struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared
            .senders
            .fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receiver left"));
        }

        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);

            empty
        };

        if was_empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        self.shared
            .receivers
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn total_senders(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared
            .senders
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

const INITIAL_SIZE: usize = 12;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(t) => return Ok(t),
                None => {
                    if self.total_senders() == 0 {
                        return Err(anyhow!("no sender left"));
                    }
                }
                None => {
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("lock poisoned"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared
            .senders
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared
            .receivers
            .fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;
    use std::time::Duration;

    // 测试一：生产者能生产，消费者能消费
    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world");
    }

    // 测试二：允许多个生产者生产数据

    #[test]
    fn multi_channel_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();

        let t1 = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t2 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t3 = thread::spawn(move || {
            s2.send(3).unwrap();
        });

        // 等待完成

        for handle in [t1, t2, t3] {
            handle.join().unwrap()
        }

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];

        result.sort();
        assert_eq!(result, [1, 2, 3])
    }

    // 测试三： receiver 会在线程位空时阻塞
    //     #[test]
    //     fn receiver_should_be_locked_when_nothing_to_read() {
    //         let (mut s, mut r) = unbounded();
    //         let mut s1 = s.clone();

    //         thread::spawn(move || {
    //             for (dx, i) in r.into_iter().enumerate() {
    //                 assert_eq!(dx, i)
    //             }
    //             assert!(false)
    //         });

    //         thread::spawn(move || {
    //             for i in 0..100usize {
    //                 s.send(i).unwrap();
    //             }
    //         });

    //         thread::sleep(Duration::from_millis(1));

    //         assert_eq!(s1.total_queued_items(), 0)
    //     }

    // 队列为空时阻塞，使用 condvar

    // 测试四：最后一个sender退出时发出错误
    //     #[test]
    //     fn last_sender_drop_should_error_when_receive() {
    //         let (mut s, mut r) = unbounded();
    //         let mut s1 = s.clone();

    //         let senders = [s, s1];
    //         let total = senders.len();

    //         for mut sender in senders {
    //             thread::spawn(move || {
    //                 sender.send("hello").unwrap();
    //             })
    //             .join()
    //             .unwrap();
    //         }

    //         for _ in 0..total {
    //             r.recv().unwrap();
    //         }

    //         assert!(r.recv().is_err());
    //     }

    // 测试五：没有receiver也报错
    //     #[test]
    //     fn receiver_drop_should_error_when_send() {
    //         let (mut s1, mut s2) = {
    //             let (s, _) = unbounded();
    //             let s1 = s.clone();
    //             let s2 = s.clone();

    //             (s1, s2)
    //         };

    //         assert!(s1.send(1).is_err());
    //         assert!(s2.send(2).is_err());
    //     }
}
