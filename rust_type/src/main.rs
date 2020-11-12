fn main() {
    // 数组
    let arr = [0, 1, 2];
    // 元组，元素类型可以不一样
    let tuple = (0, "hello", 3.1415926);
    println!("tuple {}", tuple.1);

    // 结构体
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 构造结构体
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    // 同名变量可以不指定字段名
    let u2 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    // 用另一个对象构造
    let u3 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: u2.active,
        sign_in_count: u2.sign_in_count,
    };
    // 效果同上
    let u4 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        ..u2
    };


    // 元组结构体（有结构体的名字，没有字段名）
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    /*
    注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。
    你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型。
    例如，一个获取 Color 类型参数的函数不能接受 Point 作为参数，即便这两个类型都由三个 i32 值组成。
    在其他方面，元组结构体实例类似于元组：可以将其解构为单独的部分，也可以使用 . 后跟索引来访问单独的值，等等。
    */

    // 类单元结构体，没有字段，只为实现trait，类似没有数据成员的基类
    struct Interface {

    }

    #[derive(Debug)] // 启用Debug trait
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rc = Rectangle {
        width: 1,
        height: 2
    };

    // {}需要实现Display trait
    //println!("Rectangle {}", rc);
    // {:?}需要实现Debug trait
    println!("Rectangle {:?}", rc); // 一行输出
    println!("Rectangle {:#?}", rc); // 可视化输出，可读性更好

    // 为struct增加成员方法
    impl Rectangle {
        // 第一个参数相当于this指针，不过也可以选择&mut self，self（获取所有权）
        fn area(&self) -> u32 {
            self.width * self.height;
            (*self).width * (*self).height // self.的本质是(*self). 某些场合下（访问成员）rust会帮我们自动解引用（自动加*）
        }

        // 第一个参数不是self，静态函数
        fn static_fn() {
            println!("static fn");
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} {}square pixels.",
        rect1.area(),
        (&rect1).area() // rect1.area()的本质就是(&rect1).area()，某些场合下（调用方法）rust会帮我们自动引用（自动加&）
    );

    // 调用静态函数
    Rectangle::static_fn();


    // 普通枚举    
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let e = IpAddrKind::V4;
    println!("IpAddrKind::V4 {:?}", e);

    // 带数据带枚举
    #[derive(Debug)]
    enum IpAddrKindData {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrKindData::V4(127, 0, 0, 1);
    let loopback = IpAddrKindData::V6(String::from("::1"));

    match home {
        IpAddrKindData::V4(x1, x2, x3, x4) => println!("V4: {}.{}.{}.{}", x1, x2, x3, x4),
        _ => println!("not V4"), // _表示其他所有情况
    }
    /*
    if home == IpAddrKindData::V4(u8, u8, u8, u8) {
        println!("home is v4");
    }
    */
    
    println!("home {:?}, loopback {:?}", home, loopback);

    // 枚举花样带数据
    enum Message {
        Quit, // 不带数据
        Move { x: i32, y: i32 }, // 带匿名结构体
        Write(String), // 带String
        ChangeColor(i32, i32, i32), // 带元组
    }

    // 和结构体一样，可以定义方法
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    /*
    Option枚举，为你提供了有和没有的通用解决方案， 你不用手动为每种类型定义null了
    enum Option<T> {
    Some(T),
    None,
    }
    */
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match some_number {
        Some(x) => println!("Some(x): {}", x),
        None => println!("none"),   // rust匹配是穷尽的，如果删除这一行，编译报错，Option+match保证你不会忘记处理空值场景
    }

    // if let和上面等效
    if let Some(x) = some_number {
        println!("Some(x): {}", x);
    } else {
        println!("none")
    }

}
