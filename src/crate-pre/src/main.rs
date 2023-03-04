use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use std::thread;
use std::time::Duration;

use futures::future::Map;
use std::cell::Cell;
use std::cell::RefCell;

// 枚举描述了一种情况下的所有可能
// 如IP地址种类可能包含IPV6和IPV4
enum IpAddrKind {
    V4,
    V6,
}

// 枚举值的访问

fn main() {}
