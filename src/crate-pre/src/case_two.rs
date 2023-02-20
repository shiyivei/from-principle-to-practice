// 示例 2: 修复无序问题

use crossbeam_channel;
use rayon;

use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use std::thread;

//消息体，主组件发送至其他组件
enum WorkMsg {
    Work(u8),
    Exit,
}

//消息体，其它组件发送至主组件
enum ResultMsg {
    Result(u8),
    Exited,
}

fn main() {
    // 通道以及守门人
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    // 引入线程池，开两个工作线程

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    // 并行线程
    let _ = thread::spawn(move || loop {
        match work_receiver.recv() {
            Ok(WorkMsg::Work(num)) => {
                let result_sender = result_sender.clone();

                // 执行工作，使用线程池中的线程发送结果
                pool.spawn(move || {
                    let _ = result_sender.send(ResultMsg::Result(num));
                })
            }
            Ok(WorkMsg::Exit) => {
                // 发送退出信息
                let _ = result_sender.send(ResultMsg::Exited);
                break;
            }
            // 收到错误消息 panic
            _ => panic!("Error receiving a WorkMsg"),
        }
    });

    // 主线程创建三个任务
    let _ = work_sender.send(WorkMsg::Work(0));
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Exit);

    let mut counter = 0;

    // 监听其他线程组件发送的消息
    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num)) => {
                //  assert_eq!(num, counter); 无法断言顺序
                println!("task {} received", num);
                counter += 1;
            }
            Ok(ResultMsg::Exited) => {
                //  assert_eq!(2, counter); 无法断言顺序
                println!("exit task received");
                break;
            }

            _ => panic!("Error receiving a ResultMsg"),
        }
    }
}
