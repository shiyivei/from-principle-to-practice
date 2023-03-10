// 一、一般的 echo server 模型

// 1. establish tcp connect
// 2. handle tcp stream: read/write
// 3. poll/select

// 二、异步 echo server 模型 // 区别在与选择 epoll 还是 poll

// 1. establish tcp connect
// 2. handle tcp stream: read/write
// 3. epoll

use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::io;
use std::io::Write;
use std::mem;
use std::os::unix::io::RawFd;
use tcp_listener::{Ipv4Addr, TcpListener, TcpStream};

use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use lazy_static::lazy_static;
use log::info;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[macro_use]
mod util;

mod epoll;
mod executor;
mod reactor;
mod tcp_listener;

use libc;

use epoll::*;
use executor::*;
use reactor::*;
use tcp_listener::*;

// 创建常量
lazy_static! {
    static ref REACTOR: Reactor = {
        std::thread::spawn(move || reactor_main_loop());

        Reactor {
            epoll: Epoll::new().expect("Failed to create epoll"),
            wakers: Mutex::new(HashMap::new()),
        }
    };
}

fn init_log() {
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{:>3}] {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

async fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];

    info!("(handle_client) {}", stream.0);

    loop {
        let n = stream.read(&mut buf).await?;

        if n == 0 {
            break;
        }
        stream.write(&buf[..n]).await?;
    }
    Ok(())
}

fn main() {
    init_log();

    let (executor, spawner) = new_executor_and_spawner();
    let spawner_clone = spawner.clone();

    let main_loop = async move {
        let addr = Ipv4Addr::new(127, 0, 0, 1);

        let port = 8080;

        let listener = TcpListener::bind(addr, port)?;

        let incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            spawner.spawn(handle_client(stream))
        }
        Ok(())
    };
    spawner_clone.spawn(main_loop);
    drop(spawner_clone);
    executor.run()
}
