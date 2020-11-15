fn main() {
    /*
    // 闭包和函数对比（相当于lambda表达式）
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // 函数定义
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 完整闭包定义
    let add_one_v3 = |x|             { x + 1 }; // 省略类型注解的闭包
    let add_one_v4 = |x|               x + 1  ; // 简化闭包定义
    */
    println!("Hello, world!");

// 闭包实现惰性求值
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32 // Fn是一个trait，表明T类型是一个闭包，参数u32，返回值u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    //thread::sleep(Duration::from_secs(2));
    num
});

// 只有第一次会调用闭包计算，后面都使用缓存值
println!("expensive_result {}", expensive_result.value(1));
println!("expensive_result {}", expensive_result.value(1));
println!("expensive_result {}", expensive_result.value(1));

fn test_fn<F:Fn()>(f: &F){
    f();
    f();
}

fn test_fnmut<F:FnMut()>(f: &mut F){
    f();
    f();
}

fn test_fnonce<F:FnOnce()>(f:F){
    f();
    //f();
}

let mut x = String::from("xxxxx");
let mut f_fn = || { 
    println!("fn {}", x);
    //x.push_str("pppppp");
 };
f_fn();
f_fn();

//test_fn(&f_fn);
test_fnmut(&mut f_fn);
test_fnonce(f_fn);
test_fnonce(f_fn);
}
