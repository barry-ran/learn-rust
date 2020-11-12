use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { // 成功返回(),失败返回Box<dyn Error>表示任何类型的错误（实际是一个trait 对象）
    // crash 并且打印堆栈
    //panic!("crash and backtrace"); 
    
    let v = vec![1, 2, 3];
    // vec实现中访问越界会触发panic
    // 通过设置环境变量RUST_BACKTRACE=1可以查看详细堆栈，例如RUST_BACKTRACE=1 cargo run    
    //v[99];

    // open返回Relust，成功返回文件句柄，失败返回io error
    let f = File::open("hello.txt");

    // 处理不同错误
    /*
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    */

    // unwrap 成功返回文件句柄，失败调用panic
    //let f = File::open("hello.txt").unwrap();

    // expect 成功返回文件句柄，失败调用panic，并打印给定信息
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // ? 成功返回文件句柄，失败直接返回错误 return er;
    // main函数返回值为(),为了适配？，需要改成Result<(), Box<dyn Error>>
    let f = File::open("hello.txt")?;
    /*
    ?的作用：
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    */
    // 成功返回()
    Ok(())
}
