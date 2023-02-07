// 示例 4: 优化重构
// 把多个状态变量收敛到一个结构体

use crossbeam_channel;
use rayon;

use crossbeam::select;
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

struct WorkState {
    ongoing: i16,
    exiting: bool,
}

impl WorkState {
    fn init() -> Self {
        WorkState {
            ongoing: 0,
            exiting: false,
        }
    }

    fn set_ongoing(&mut self, count: i16) {
        self.ongoing -= count;
    }

    fn set_exiting(&mut self, exit_state: bool) {
        self.exiting = exit_state
    }

    fn is_exiting(&self) -> bool {
        self.exiting == true
    }

    fn is_nomore_work(&self) -> bool {
        self.ongoing == 0
    }
}

fn main() {
    // 通道以及守门人
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    // 在线程池中的线程之间引入通信通道
    let (pool_result_sender, pool_result_receiver) = unbounded();
    // 标示任务状态
    let mut work_state = WorkState::init();

    // 引入线程池，开两个工作线程

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    // 并行线程
    let _ = thread::spawn(move || loop {
        // 使用宏来选择就绪工作

        select! {
          recv(work_receiver)->msg => {
             match msg {
                  Ok(WorkMsg::Work(num)) => {
                       let result_sender = result_sender.clone();
                       let pool_result_sender = pool_result_sender.clone();

                       work_state.set_ongoing(1);

                       // 执行工作，使用线程池中的线程发送结果
                       pool.spawn(move || {
                         // do_something
                            let _ = result_sender.send(ResultMsg::Result(num));
                            let _ = pool_result_sender.send(());

                       })
                  }
                  Ok(WorkMsg::Exit) => {

                       work_state.set_exiting(true);

                       if work_state.is_nomore_work() {
                            // 发送退出信息
                            let _ = result_sender.send(ResultMsg::Exited);
                            break;

                       }
                  }
                  _ => panic!("Error receiving a WorkMsg")
             }
          }
          recv(pool_result_receiver) -> _ => {

               if work_state.is_nomore_work() {
                      panic!("Receving a unexpected pool result")
               }

               work_state.set_ongoing(-1);
               if work_state.is_nomore_work()  && work_state.is_exiting() {
                     // 发送退出信息
                   let _ = result_sender.send(ResultMsg::Exited);
                   break;
               }

          }

        }
    });

    // 主线程创建三个任务
    let _ = work_sender.send(WorkMsg::Work(0));
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Work(2));

    let _ = work_sender.send(WorkMsg::Exit);
    let _ = work_sender.send(WorkMsg::Work(3));
    let _ = work_sender.send(WorkMsg::Work(4));

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
                assert_eq!(5, counter);
                println!("exit task successfully");
                break;
            }

            _ => panic!("Error receiving a ResultMsg"),
        }
    }
}
