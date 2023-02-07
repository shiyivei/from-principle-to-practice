// 示例 1: 模拟并运行组件

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

    let _ = thread::spawn(move || loop {
        match work_receiver.recv() {
            Ok(WorkMsg::Work(num)) => {
                //执行工作，并发送结果
                let _ = result_sender.send(ResultMsg::Result(num));
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
                assert_eq!(num, counter);
                println!("task {} received", num);
                counter += 1;
            }
            Ok(ResultMsg::Exited) => {
                assert_eq!(2, counter);
                break;
            }

            _ => panic!("Error receiving a ResultMsg"),
        }
    }
}
