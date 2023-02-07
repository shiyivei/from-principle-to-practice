// 示例 f: 确保从缓存中取共享数据的行为是确定的
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
    Result(u8, WorkPerformed),
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

#[derive(Debug, Eq, PartialEq)]
enum WorkPerformed {
    FromCache,
    New,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct CacheKey(u8);

#[derive(Debug, Eq, PartialEq)]
enum CacheState {
    Ready,
    WorkInProgress,
}

fn main() {
    // 通道以及守门人
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    // 在线程池中的线程之间引入通信通道
    let (pool_result_sender, pool_result_receiver) = unbounded();
    // 标示任务状态
    let mut work_state = WorkState::init();

    let cache: Arc<Mutex<HashMap<CacheKey, u8>>> = Arc::new(Mutex::new(HashMap::new()));
    // 增加缓存状态，指示对于给定的key，缓存是否已经准备好被读取。
    let cache_state = Arc::new(Mutex::new(HashMap::new()));

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

                       // 使用缓存
                       let cache = cache.clone();
                       let cache_state = cache_state.clone();

                       work_state.set_ongoing(1);

                       // 执行工作，使用线程池中的线程发送结果
                       pool.spawn(move || {

                         let num = {

                              let (cache_state_lock,cvar) = {

                                   //  `cache_state` 临界区开始
                                   let mut state_map = cache_state.lock().unwrap();
                                   &*state_map
                                       .entry(CacheKey(num.clone()))
                                       .or_insert_with(|| {
                                           Arc::new((
                                               Mutex::new(CacheState::Ready),
                                               Condvar::new(),
                                           ))
                                       })
                                       .clone()
                                    //  `cache_state` 临界区结束

                              };
                              //  `state` 临界区开始
                              let mut state = cache_state_lock.lock().unwrap();

                              // 注意：使用while循环来防止条件变量的虚假唤醒
                              while let CacheState::WorkInProgress = *state {
                                  // 阻塞直到状态是 `CacheState::Ready`.
                                  //
                                  // 当唤醒时会自动释放锁
                                  let current_state = cvar
                                      .wait(state)
                                      .unwrap();
                                  state = current_state;
                              }

                              // 循环外可以认为state 已经是 Ready 的了
                              assert_eq!(*state, CacheState::Ready);

                              let (num, result) = {
                                   // 缓存临界区开始
                                   let cache = cache.lock().unwrap();
                                   let key = CacheKey(num);
                                   let result = match cache.get(&key) {
                                       Some(result) => Some(result.clone()),
                                       None => None,
                                   };
                                   (key.0, result)
                                   // 缓存临界区结束
                               };

                               if let Some(result) = result {
                                   // 从缓存中获得一个结果，并将其发送回去，
                                   // 同时带有一个标志，表明是从缓存中获得了它
                                   let _ = result_sender.send(ResultMsg::Result(result, WorkPerformed::FromCache));
                                   let _ = pool_result_sender.send(());

                                   // 不要忘记通知等待线程
                                   cvar.notify_one();
                                   return;
                               } else {
                                   // 如果缓存里没有找到结果，那么切换状态
                                   *state = CacheState::WorkInProgress;
                                   num
                               }
                               // `state` 临界区结束

                         };

                         // 返回结果，表明我们必须执行work
                         let _ = result_sender.send(ResultMsg::Result(num.clone(), WorkPerformed::New));

                         {

                         // 在缓存中存储“昂贵”的work.
                         let mut cache = cache.lock().unwrap();
                         let key = CacheKey(num.clone());
                         cache.insert(key, num);

                         }

                         let (lock, cvar) = {
                              let mut state_map = cache_state.lock().unwrap();
                              &*state_map
                                  .get_mut(&CacheKey(num))
                                  .expect("Entry in cache state to have been previously inserted")
                                  .clone()
                          };
                          // 重新进入 `state` 临界区
                          let mut state = lock.lock().unwrap();

                          // 在这里，由于已经提前设置了state，并且任何其他worker都将等待状态切换回ready，可以确定该状态是“in-progress”。
                          assert_eq!(*state, CacheState::WorkInProgress);

                          // 切换状态为 Ready
                          *state = CacheState::Ready;

                          // 通知等待线程
                          cvar.notify_one();

                         let _ = pool_result_sender.send(());

                       }) ;
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
    let _ = work_sender.send(WorkMsg::Work(1));

    let _ = work_sender.send(WorkMsg::Work(3));
    let _ = work_sender.send(WorkMsg::Work(4));
    let _ = work_sender.send(WorkMsg::Exit);

    let mut counter = 0;

    // 当work 是 1 的时候重新计数
    let mut work_one_counter = 0;

    // 监听其他线程组件发送的消息
    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num, cached)) => {
                //  assert_eq!(num, counter); 无法断言顺序
                println!("task {} received", num);
                counter += 1;

                if num == 1 {
                    work_one_counter += 1;
                }

                // 现在我们可以断言，当收到 num 为 1 的第二个结果时，它已经来自缓存。
                if num == 1 && work_one_counter == 2 {
                    assert_eq!(cached, WorkPerformed::FromCache);
                }
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
