// 响应器
use super::*;

// reactor包含一个epoll，只是对epoll接口调用的一个封装
pub struct Reactor {
    epoll: Epoll,
    wakers: Mutex<HashMap<RawFd, Waker>>,
}

impl Reactor {
    pub fn add_event(&self, fd: RawFd, op: EpollEventType, waker: Waker) -> io::Result<()> {
        info!("adding event: {}", fd);
        self.epoll.add_event(fd, op)?;
        self.wakers.lock().unwrap().insert(fd, waker);
        Ok(())
    }
}

pub fn reactor_main_loop() -> io::Result<()> {
    info!("Start reactor main loop");

    let max_event = 32;

    let event = unsafe { mem::zeroed() };
    let mut events = vec![event; max_event];
    let reactor = &REACTOR;

    loop {
        let nfd = reactor.epoll.wait(&mut events)?;
        info!("(Reactor) wake up. nfd = {}", nfd);
        #[allow(clippy::needless_range_loop)]
        for i in 0..nfd {
            let fd = events[i].u64 as RawFd;

            let waker = reactor
                .wakers
                .lock()
                .unwrap()
                .remove(&fd)
                .unwrap_or_else(|| panic!("not found fd {}", fd));
            info!("(Reactor) delete event: {}", fd);
            reactor.epoll.del_event(fd)?;
            waker.wake();
        }
    }
}
