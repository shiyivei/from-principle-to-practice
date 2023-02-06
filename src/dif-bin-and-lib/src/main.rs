#[allow(unused)]

fn main() {
    // 在主线程和子线程之间通过消息来共享数据

    use crossbeam_channel;
    use rayon;

    use crossbeam_channel::unbounded;
    //use parking_lot::{Condvar, Mutex}; // 比标准库更小、更快更安全
    //use std::sync::Arc; // 但未实现 Arc
    use std::collections::HashMap;

    use std::sync::{Arc, Condvar, Mutex};

    use std::thread;

    // 主组件和其他组件并行运行

    // 发送消息体
    enum WorkMsg {
        Work(u8),
        Exit,
    }

    // 返回消息体
    enum ResultMsg {
        Result(u8),
        Exited,
    }

    // 创建两个通道

    let (worker_sender, worker_receiver) = unbounded();
    let (receiver_sender, receiver_receiver) = unbounded();

    // 创建子线程

    let _ = thread::spawn(move || loop {
        // 循环收消息
        match worker_receiver.recv() {
            //收到工作消息
            Ok(WorkMsg::Work(num)) => {
                // 回消息在工作
                let _ = receiver_sender.send(ResultMsg::Result(num));
            }
            // 收到停止消息
            Ok(WorkMsg::Exit) => {
                // 回消息已停止
                let _ = receiver_sender.send(ResultMsg::Exited);
                break;
            }
            _ => panic!("Error receiving a WorkMsg"),
        }
    });

    // 主线程发送三个消息
    let _ = worker_sender.send(WorkMsg::Work(0));
    let _ = worker_sender.send(WorkMsg::Work(1));
    let _ = worker_sender.send(WorkMsg::Exit);

    let mut counter = 0;

    // 主线程循环
    loop {
        match receiver_receiver.recv() {
            Ok(ResultMsg::Result(num)) => {
                //断言，保证顺序执行
                assert_eq!(num, counter);
                counter += 1;
            }
            Ok(ResultMsg::Exited) => {
                //断言，保证顺序执行
                assert_eq!(2, counter);
                break;
            }

            _ => panic!("Error receiving a ResultMsg"),
        }
    }
}
