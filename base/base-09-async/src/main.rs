/*
    rust异步涉及到的概念：https://zhuanlan.zhihu.com/p/112237024
    rust异步的原理：https://www.rectcircle.cn/posts/rust%E5%BC%82%E6%AD%A5%E7%BC%96%E7%A8%8B/

    // rust只提供了Future等概念的抽象定义，runtime（executor+reactor）需要使用外部库实现

    其他参考：
    深入了解 Rust 异步开发模式：https://cloud.tencent.com/developer/news/686021
    Rust异步实用指南：https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
    Rust：异步代码里的阻塞：https://zhuanlan.zhihu.com/p/147995615    
*/

use futures::executor::block_on;
use futures::prelude::*;
use std::thread;
//use tokio::prelude::*;
use tokio::runtime;
use tokio::task;
use tokio::time::sleep;
use std::time::{Duration, Instant};
use std::pin::Pin;

async fn async_1() {
    println!("async_1 begin :{:?}", thread::current().id());
    sleep(Duration::from_millis(5000)).await;
    println!("async_1 end :{:?}", thread::current().id());
}
async fn async_2() {
    println!("async_2 :{:?}", thread::current().id());
}
async fn async_3() {
    println!("async_3 :{:?}", thread::current().id());
}

// async表明这是一个异步函数，可以使用await中断（实际async改变了函数返回值，改成了future）
async fn async_main() {
    let f1 = async_1();
    let f2 = async_2();

    // 先异步等待async_3执行完成
    async_3().await;

    // 同时异步等待async_1，async_2完成
    futures::join!(f1, f2);
}

// futures::executor::block_on：单线程版本的runtime
#[cfg(feature = "use_rust_futures")]
fn main() {
    println!("main thread: {:?}", thread::current().id());

    let future = async_main();
    // `block_on`会阻塞当前线程，直到提供的future完成为止。
    // 内部实现实际是一个eventloop（单线程实现executor+reactor）
    block_on(future);
}

// tokio：单线程版本的runtime
#[cfg(feature = "use_tokio_s")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main thread: {:?}", thread::current().id());

    let basic_rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = async_main();
    basic_rt.block_on(future);
    Ok(())
}

// tokio：多线程版本的runtime
#[cfg(feature = "use_tokio_m")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main thread: {:?}", thread::current().id());

    let threaded_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = async_main();
    threaded_rt.block_on(future);
    Ok(())
}

// 如何让回调函数支持执行异步代码
type SyncCallback = Box<dyn Fn(u32) -> u32>;
type AsyncCallback = Box<dyn Fn(u32) -> Pin<Box<dyn Future<Output = u32>>>>;

// tokio：使用宏简洁版本的runtime（可以通过flavor指定单线程/多线程）
// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// #[tokio::main(flavor = "current_thread")]

#[cfg(feature = "use_tokio_main")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("main thread: {:?}", thread::current().id());

    let sc : SyncCallback = Box::new(|num| {
        println!("SyncCallback");
        num
    });
    let n = sc(5);
    // 普通Fn肯定不能await
    // let n = sc(5).await.unwrap();
    println!("sc n: {}", n);

    // 在异步环境中，通过回调函数返回Future实现回调函数中可以执行异步代码
    let ac : AsyncCallback = Box::new(|num| {
        println!("AsyncCallback sync");
        // 当前同步环境，不能await
        //async_2().await;
        async_2();
        Box::pin(async move {
            println!("AsyncCallback async");
            // 异步环境可以await
            async_2().await;
            num
        })
    });
    let n = ac(5).await;
    println!("ac n: {}", n);


    ///
    ///     interval.tick().await;
        
    // 创建新的future，并等待
    let join = task::spawn(async {
        println!("spawn: {:?}", thread::current().id());
    });
    // 异步等待，test_spawn执行完再执行下面
    join.await.unwrap();

    // 开启新的线程执行真正同步block的任务
    let join_block = task::spawn_blocking(|| {        
        println!("spawn_blocking: {:?}", thread::current().id());        
    });

    /*
    // 和spawn_blocking的区别是，此闭包是直接在当前线程同步执行，而把runtime转移到其他线程执行
    let result = task::block_in_place(|| {
        // do some compute-heavy work or call synchronous code
        "blocking completed"
    });
    */
        
    join_block.await.unwrap();

    async_main().await;
}


