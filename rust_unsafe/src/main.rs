fn main() {
    // unsafe允许你在rust做这几件事不安全的操作（且仅允许你做这几件）
    /*
    解引用裸指针
    调用不安全的函数或方法
    访问或修改可变静态变量
    实现不安全 trait
    访问 union 的字段
    */

    // 解引用裸指针
    let address = 0x012345usize;
    // rust允许你创建裸指针，但是解引用裸指针必须在unsafe块中
    let r = address as *const i32;

    let mut num = 5;

    // 创建不可变引用的裸指针类型
    let r1 = &num as *const i32;
    // 创建可变引用的裸指针类型
    let r2 = &mut num as *mut i32;

    unsafe {
        // 解引用访问裸指针，只能在unsafe中
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 调用不安全函数或方法

    // unsafe表明函数不安全
    unsafe fn dangerous() {
        println!("dangerous");
    }

    // 只能在unsafe块中调用unsafe函数
    unsafe {
        dangerous();
    }

    // 调用c函数
    /*
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        fn main() {
            unsafe {
                println!("Absolute value of -3 according to C: {}", abs(-3));
            }
        }
    */

    // 从其它语言调用 Rust 函数
    #[no_mangle] // 禁用rust对函数名重命名
    pub extern "C" fn call_from_c() {
        // 告诉外部此函数遵循标准c接口
        println!("Just called a Rust function from C!");
    }

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        // 修改static 变量是不安全的
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    // 访问static 变量是不安全的
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    /*
    最后一个只能用在 unsafe 中的操作是实现不安全 trait。当至少有一个方法中包含编译器不能验证的不变量时 trait 是不安全的。
    可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe
    */
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
