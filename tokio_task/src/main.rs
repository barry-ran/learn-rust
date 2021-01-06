use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use tokio::sync::Notify;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

lazy_static! {
    pub static ref TOKIO_TASK: Mutex<TokioTask> = Mutex::new(TokioTask::default());
}
#[derive(Default)]
pub struct TokioTask {
    tokio_thread: Option<JoinHandle<()>>,
    values: Arc<Mutex<VecDeque<i32>>>,
    notify: Arc<Notify>,
    quit: Arc<Mutex<bool>>,
}

impl TokioTask {
    pub fn start(&mut self) {
        let notify_send = Arc::new(Notify::new());
        let notify_recv = notify_send.clone();
                
        let quit = Arc::new(Mutex::new(false));
        self.quit = quit.clone();        
        
        let values: Arc<Mutex<VecDeque<i32>>> = Arc::new(Mutex::new(VecDeque::new()));
        self.values = values.clone();

        let handle = thread::spawn(move || {
            println!("tokio thread begin");

            let basic_rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            basic_rt.block_on(async move {
                println!("tokio feature begin");

                loop {
                    notify_recv.notified().await;
                    if (*quit.lock()) {
                        break;
                    }

                    while let Some(value) = values.lock().pop_front() {
                        if (*quit.lock()) {
                            break;
                        }
                        println!("recv task {}", value);
                        thread::sleep(Duration::from_millis(1000));
                    }
                }                

                println!("tokio feature end");  
            });

            println!("tokio thread end");
        });

        self.tokio_thread = Some(handle);
        self.notify = notify_send;
    }

    pub fn stop(&mut self) {
        if self.tokio_thread.is_none() {
            return;
        }
        (*self.quit.lock()) = true;
        self.notify.notify_one();
        self.tokio_thread.take().unwrap().join().unwrap();
    }

    pub fn add_task(&mut self, i: i32) {
        self.values.lock().push_back(i);
        self.notify.notify_one();
    }

    pub fn cancel_all_task(&mut self) {
        self.values.lock().clear();
        self.notify.notify_one();
    }
}

fn main() {
    TOKIO_TASK.lock().start();

    for i in 1..100 {
        println!("send task {}", i);
        TOKIO_TASK.lock().add_task(i);  
        //thread::sleep(Duration::from_millis(1000));
    }

    thread::sleep(Duration::from_millis(10000));
    TOKIO_TASK.lock().cancel_all_task();

    for i in 100..110 {
        println!("send task {}", i);
        TOKIO_TASK.lock().add_task(i);  
        thread::sleep(Duration::from_millis(1000));
    }

    println!("begin stop");    
    TOKIO_TASK.lock().stop();
    println!("end stop");
}
