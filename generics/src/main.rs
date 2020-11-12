// 范型相当于模版

// 函数名中的<T>表示声明范型T，声明以后才可以在参数中使用T
// T:后面表示对T的约束，T必须实现PartialOrd（用于比较）和Copy（用于赋值）的类型
fn largest<T : PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 结构体中使用范型
    // x,y两个参数使用了不同的范型
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

{
    // 方法中使用范型
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // 必须在impl后面声明<T>，用于表明Point<T>中的T是一个范型，而不是一个具体类型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // impl后面没有T，表明Point<f32>中的f32是一个具体类型，含义为：为Point<f32>类型单独实现的函数
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

}

    // 函数和结构体可以使用不同的范型
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    /*
    p1是一个有 i32 类型的 x（其值为 5）和 f64 的 y（其值为 10.4）的 Point。
    p2 则是一个有着字符串 slice 类型的 x（其值为 "Hello"）和 char 类型的 y（其值为c）的 Point。
    在 p1 上以 p2 作为参数调用 mixup 会返回一个 p3，它会有一个 i32 类型的 x，因为 x 来自 p1，
    并拥有一个 char 类型的 y，因为 y 来自 p2。println! 会打印出 p3.x = 5, p3.y = c
    */
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    // trait 相当于接口
    // 定义名字为Summary的trait
    pub trait Summary {
        // trait中声明的函数，需要具体的类型自己去实现
        fn summarize(&self) -> String;
        // 当然也可以提供默认实现
        fn summarize_def(&self) -> String {
            // 默认实现中接着去调用了summarize
            format!("(Read more from {}...)", self.summarize())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    // 为结构体Tweet实现Summary trait
    impl Summary for Tweet {
        // 实现trait具体的函数
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // 调用实现的trait函数
    println!("1 new tweet: {}", tweet.summarize());
    // 调用trait默认实现的函数
    println!("1 new tweet def: {}", tweet.summarize_def());

    // 函数参数中使用trait

    // 限制item参数为实现了Summary trait的某种类型（可以通过+指定多个trait：Summary + Display）
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound 语法，效果同上（可以通过+指定多个trait：Summary + Display）
    pub fn notify2<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(tweet);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify2(tweet);


    // where从句简化Trait Bound

    // 长Trait Bound
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

    /* where简化，效果同上
    fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
    {
    */

    // 函数返回值也可以加trait限制
    //fn returns_summarizable(switch: bool) -> impl Summary {}


    // 有条件地只为那些实现了特定 trait 的类型实现方法
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    // 为所有Pair<T>类型实现new方法
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    // 只为实现了Display + PartialOrd trait的Pair<T>类型实现cmp_display方法
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // 也可以对任何实现了特定 trait 的类型有条件地实现 trait
    // 例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。这个 impl 块看起来像这样
    /*
    // 为实现了Displaytrait的T类型实现ToString trait
    impl<T: Display> ToString for T {
        // --snip--
    }
    */

    
}