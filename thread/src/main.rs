use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() {
    let v = vec![1, 2, 3];

    // spawn创建线程
    let handle = thread::spawn(move || {
        // 使用了外部变量v，所以使用move强制获得所有权
        println!("Here's a vector: {:?}", v);

        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待handle线程结束
    handle.join().unwrap();

    // 线程通信：channel
    // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。
    // 简而言之，Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送（sending）端，但只能有一个消费这些值的 接收（receiving）端
    let (tx, rx) = mpsc::channel();

    // clone多个发送端，可以给不同线程使用
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // 使用tx1发送
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // 使用tx发送
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 可以通过rx.recv()接收，也可以通过迭代器（迭代器next函数里应该调用了recv）
    for received in rx {
        println!("Got: {}", received);
    }

    // mutex（和c++中的mutex有所不同，rust mutex可以带一个数据）
    // 创建一个带i32数据的mutext，并保存在Arc中，Arc是和Rc类似的多所有权智能指针，不同之处是Arc是线程安全，Rc非线程安全
    // Arc实现了Send trait，实现了Send trait的类型，编译器认为所有权可以安全的发送到多线程使用
    // 使用Arc的目的是创建多个多个所有权对象，在不同线程中使用
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {        
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock mutext后返回携带数据的引用（也实现了内部可变性）
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("************mutex*************",);
    println!("Result: {}", *counter.lock().unwrap());

    // 下面两个线程相关的标记trait（不需要实现任何方法，只是用来加强并发相关的不可变性的）

    // Send 标记trait
    /*
    Send trait表明类型的所有权可以在线程间传递。几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 Rc<T>：这是不能 Send 的，
    因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权转移到另一个线程，这两个线程都可能同时更新引用计数。
    为此，Rc<T> 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价。

    因此，Rust 类型系统和 trait bound 确保永远也不会意外的将不安全的 Rc<T> 在线程间发送。而使用标记为 Send 的 Arc<T> 时，就没有问题了。
    任何完全由 Send 的类型组成的类型也会自动被标记为 Send。几乎所有基本类型都是 Send 的，除了第十九章将会讨论的裸指针（raw pointer）
    */

    // Sync 标记trait
    /*
    Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。换一种方式来说，对于任意类型 T，
    如果 &T（T 的引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程。类似于 Send 的情况，
    基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。

    智能指针 Rc<T> 也不是 Sync 的，出于其不是 Send 相同的原因。RefCell<T> 和 Cell<T> 系列类型不是 Sync 的。
    RefCell<T> 在运行时所进行的借用检查也不是线程安全的。Mutex<T> 是 Sync 的，
    */
}
