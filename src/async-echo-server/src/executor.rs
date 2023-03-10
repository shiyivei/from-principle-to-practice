// 一个调度器，一个ready队列
// waker 负责把 pending队列中的 任务放到 executor 中

use super::*;

// 维护了一个ready 任务队列，是一个运行时
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

// task本身携带了sender,异步安全，有唤醒机制
pub struct Task {
    future: Mutex<Option<BoxFuture<'static, io::Result<()>>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    // 通过发送消息实现唤醒
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("failed to send")
    }
}

// 启动一个运行时，读取队列，把没有准备好的队列放回去
impl Executor {
    pub fn run(&self) {
        // 使用 while let 循环拿到 task
        while let Ok(task) = self.ready_queue.recv() {
            // 上锁
            let mut future_slot = task.future.lock().unwrap();

            // 拿到需要计算的任务
            if let Some(mut future) = future_slot.take() {
                // 拿到唤醒器
                let waker = waker_ref(&task);

                // 创建上下文
                let mut context = Context::from_waker(&*waker);
                // 使用 poll执行系统调用
                if let Poll::Pending = future.as_mut().poll(&mut context) {
                    //如果pending,把future放回
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

// 一个调度器可以spawn一个task，就像spawn一个线程一样

impl Spawner {
    // 根据future创建task，并放入队列，运行时读取
    pub fn spawn(&self, fut: impl Future<Output = io::Result<()>> + 'static + Send) {
        let fut = fut.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(fut)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("failed to send");
    }
}

// 创建一个执行器和分发器

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, ready_queue) = sync_channel(10000);
    (Executor { ready_queue }, Spawner { task_sender })
}
