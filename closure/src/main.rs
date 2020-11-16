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
    where
        T: Fn(u32) -> u32, // Fn是一个trait，表明T类型是一个闭包，参数u32，返回值u32
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
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
                }
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

    /*
    闭包的实现原理是：创建一个结构体，结构体字段保存要捕获的环境变量，为结构体实现Fn/FnMut/FnOnce trait，
    系统就是通过调用Fn系的trait中的函数来实现函数/闭包调用的

    pub trait Fn<Args>: FnMut<Args> {
        extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    }

    pub trait FnMut<Args>: FnOnce<Args> {
        extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    }

    pub trait FnOnce<Args> {
        extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    }

    Fn trait的函数是call，参数是&self
    FnMut trait的函数是call_mut，参数是&mut self
    FnOnce trait的函数是call_once，参数是self


    那么在闭包的实现中有3个问题：
    1. 以何种方式保存环境变量？不可变引用/可变引用/所有权
    2. 调用闭包结构体实现的哪种trait 函数？Fn(call)/FnMut(call_mut)/FnOnce(call_once)
    3. Fn系列trait的继承关系为什么是 Fn继承FnMut，FnMut继承FnOnce？
    */

    /*
    1. 以何种方式保存环境变量？不可变引用/可变引用/所有权
    rust会按照最小影响的原则自动推导每个环境变量使用哪种方式保存

    */

    // 闭包中仅仅读取了环境变量的值，则用不可变引用的方式保存环境变量
    let mut s = String::from("ssssss");
    let l = || {
        println!("l {}", s);
    };
    l();
    println!("s {}", s);

    // 闭包中修改了环境变量的值，则用可变引用的方式保存环境变量
    let mut s = String::from("ssssss");
    let mut l = || {
        println!("l {}", s);
        s.push_str("aaaa")
    };
    l();
    println!("s {}", s);

    // 闭包中使用了环境变量所有权的函数，则用所有权的方式保存环境变量
    let mut s = String::from("ssssss");
    let mut s2 = String::from("ssssss");
    let mut l = || {
        println!("l {} {}", s, s2);
        drop(s);
    }; // drop函数获取String的所有权
    l();
    //println!("l {}", s); // s所有权被闭包捕获，这里无法再使用s
    println!("s2 {}", s2); // 仅仅对s2进行了读取，所以是以可变引用保存对s2

    /*
    2. 调用闭包结构体实现的哪种trait 函数？Fn(call)/FnMut(call_mut)/FnOnce(call_once)
    具体调用哪个trait的函数由第一个问题可以推导出来，第一个问题中：假设闭包以可变引用保存的环境变量a(&mut a)
    那如果调用Fn(call)肯定不合理，因为Fn(call)的参数是&self，调用self.a访问环境变量编译不过（self是不可变引用，a是可变引用）
    所以编译器按照Fn(call)>FnMut(call_mut)>FnOnce(call_once)的顺序尝试，哪个编译过调用哪个函数
    */

    /*
    3. 为什么Fn系的函数继承关系是Fn继承FnMut，FnMut继承FnOnce？
    在C++中，函数参数是基类指针的时候，我们可以传子类对象的指针，也就是：
    子类可以替换基类，基类不可以替换子类
    所以Fn系的继承关系决定了：
    如果一个函数的参数是FnOnce类型，那么我可以传FnOnce/FnMut/Fn类型的闭包
    如果一个函数的参数是FnMut类型，那么我可以传FnMut/Fn类型的闭包
    如果一个函数的参数是Fn类型，那么我可以传Fn类型的闭包
    */

    // 参数是FnOnce类型
    fn test_fnonce<F: FnOnce()>(f: F) {
        f();
    }
    // 参数是FnMut类型
    fn test_fnmut<F: FnMut()>(mut f: F) {
        f();
    }
    // 参数是Fn类型
    fn test_fn<F: Fn()>(f: F) {
        f();
    }

    {
        let f = || println!("hello f");
        let mut n = 1;
        let f_mut = || {
            println!("hello f");
            n = 2
        };
        let mut s = String::from("sssssss");
        let f_once = || {
            println!("hello f");
            drop(&s);
        };

        // FnOnce传Fn
        test_fnonce(f);
        // FnOnce传FnMut
        test_fnonce(f_mut);
        // FnOnce传FnOnce
        test_fnonce(f_once);
    }

    {
        let f = || println!("hello f");
        let mut n = 1;
        let f_mut = || {
            println!("hello f");
            n = 2
        };
        let s = String::from("sssssss");
        let f_once = || {
            println!("hello {:?}", s);
            drop(s);
        };

        // FnMut传FnOnce，编译报错
        // test_fnmut(f_once);
        // FnMut传FnMut
        test_fnmut(f_mut);
        // FnMut传Fn
        test_fnmut(f);        
    }

    {
        let f = || println!("hello f");
        let mut n = 1;
        let f_mut = || {
            println!("hello f");
            n = 2
        };
        let s = String::from("sssssss");
        let f_once = || {
            println!("hello {:?}", s);
            drop(s);
        };

        // Fn传FnOnce，编译报错
        //test_fn(f_once);
        // Fn传FnMut，编译报错
        //test_fn(f_mut);
        // Fn传Fn
        test_fn(f);        
    }



    // move 强制闭包使用所有权方式捕获环境变量
    {
        let s = String::from("ssss");
        let f = move ||{println!("{}", s);};
        //let f = ||{println!("{}", s);};
        //println!("{}", s);
    }

    /*
    fn test_fn<F:Fn()>(f: &F){
        f();
    }
    let f: fn() = || println!("hello f");
    test_fn(&f);

    */

    /*

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

    */
}
