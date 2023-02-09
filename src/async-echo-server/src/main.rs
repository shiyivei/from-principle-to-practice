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

mod tcp_listener;
mod executor;
fn main() {
    println!("Hello, world!");
}
