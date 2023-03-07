use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::{Arc, Condvar, Mutex};

use std::thread;
use std::time::Duration;

use futures::future::Map;
use std::cell::Cell;
use std::cell::RefCell;
use std::pin::Pin;

// 枚举描述了一种情况下的所有可能
// 如IP地址种类可能包含IPV6和IPV4
enum IpAddrKind {
    V4,
    V6,
}

// 枚举值的访问

fn main() {
    // 一 Vec原理

    // pub struct Vec<T, A: Allocator = Global> {
    //     buf: RawVec<T, A>,
    //     len: usize,
    // }

    // pub struct RawVec<T, A: Allocator = Global> {
    //     ptr: Unique<T>,
    //     cap: usize,
    //     alloc: A,
    // }

    // 二 Vec应用

    // 1. 与 &str, String转换

    let send_message = String::from("hello Rust");

    let msg_buffer = send_message.into_bytes();

    let receive_message = from_utf8(&msg_buffer).unwrap().to_string();

    println!("Received message :{:?}", receive_message);

    // 2. 当做队列

    let mut queue = vec![];
    queue.push(1);
    queue.pop();
    assert_eq!(queue.len(), 0);

    // 3. 收缩容量

    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    assert_eq!(vec.capacity(), 10);

    println!("address of vec :{:?}", vec.as_ptr()); // 0x6000036fc210

    vec.shrink_to(4);
    assert!(vec.capacity() >= 4);

    println!("address of vec :{:?}", vec.as_ptr()); // 0x600003af8070 收缩时内存重新进行了分配

    vec.shrink_to_fit(); // 收缩到恰好能容纳三个元素
    assert!(vec.capacity() == 3);

    println!("address of vec :{:?}", vec.as_ptr()); // 0x600003af8070
}
