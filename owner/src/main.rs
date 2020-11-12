fn main() {    

    /*
    所有权+Copy trait+Drop trait解决了浅拷贝内存泄漏问题

    所有权指的是变量和对应内存的所有权：
    对于实现了Copy trait的变量赋值是会copy的，
    所以赋值以后新变量和旧变量各自对应自己的内存有所有权
    对于实现了Drop trait的变量赋值是会move的（所有权转移），
    所以赋值以后，旧变量对应内存被move到了新变量中，旧变量没有所有权了，新变量持有对原始内存的所有权
    */

    /*
    Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。
    如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。
    Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。
    */
    // 普通变量的赋值就是简单的copy（因为实现了Copy trait）
    let mut x = 5;
    let y = x;
    x = 3;
    println!("x: {}, y: {}", x, y);

    {
        let s = String::from("aaaaa");
        println!("s: {}", s);
        // 作用域结束后s不会内存泄漏，因为String实现了Drop trait
        // 当变量s离开作用域，Rust为我们调用一个特殊的函数。这个函数叫做 drop
        // 在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。
    }

    // 包含堆数据变量赋值其实是move（为了防止多次drop）
    let s1 = String::from("hello");
    let mut s2 = s1;    // 在此以后，s1内容move到s2中，s1不可用了（s1不会被drop）
    //s1.push_str(", abc"); // 这里编译会报错s1被move
    s2.push_str(", abc");
    println!("test move: s2: {}", s2);

    // 可以使用clone函数来实现copy
    let mut s1 = String::from("hello");
    let s2 = s1.clone(); // s2是s1的克隆，后面s1和s2都可用
    s1.push_str(", abc");
    println!("copy test s1 = {}, s2 = {}", s1, s2);

    // 当函数参数类型是String时，传值会导致所有权转移，后面就不能使用s1了
    let s1 = String::from("hello");
    use_string(s1);
    //s1.push_str(", abc"); // 这里编译报错，s1被转移所有权了，无效了

    // 引用（其实就是指针）解决上面的问题
    let mut s1 = String::from("hello");
    use_string2(&s1); // 通过&传递String的引用进去，不会发生所有权转移
    s1.push_str(", abc"); // 后面都可以照常使用s1
    println!("use_string2 after s = {}", s1);

    // 引用其实就是指针，那么指针会有野指针，读写指针同时访问导致脏数据问题，rust引用如何避免？
    // 1. 野指针？ 直接编译报错    
    let s1 = String::from("hello1");    
    let mut s_ref = &s1;
    {
        let s2 = String::from("hello2");
        //s_ref = &s2; // 引用了s2，但是s2一会就销毁，编译报错
    }    
    println!("s_ref = {}", s_ref);

    // 读写指针脏数据？编译报错
    let mut s = String::from("hello read");
    let s1 = &s;
    let s2 = &s; // 多个读指针没问题，不会脏数据，允许编译
    // let s_mut = &mut s; // 突然来个写指针不安全了，不允许编译
    let s3 = &s;

    // 如果没有这一行，上面s_mut不会编译报错的，引用的生命周期是到最后一次使用
    // 如果没有这一行，s1 s2的生命周期在s_mut之前就结束了，所以s_mut不会报错，你说人性不人性
    println!("test s = {}", s1); 

    /* rust引用总结：
    在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    引用必须总是有效的。
    */

    // slice类型：连续集合中的连续片段的不可变引用
    // 例如&str就是String集合中的连续片段的不可变引用
    let s = String::from("hello slice");
    let s_slice = &s[..6]; // &str类型
    println!("s_slice: {}", s_slice);
    // 字符串常量也是&str类型
    let s = "slice"; // &str类型

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let a_slice = &a[..5]; // &[i32]类型
    println!("a_slice: {:?}", a_slice);

}

fn use_string(s: String) {  // 参数为String，会发生所有权转移
    println!("use_string s = {}", s);
}

fn use_string2(s: &String) {  // 参数为&String，不会发生所有权转移
    println!("use_string2 s = {}", s);
}
