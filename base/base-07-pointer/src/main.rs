fn main() {
    // Box<T> 是智能指针，提供在堆上保存数据的能力，相当于unique ptr
    // Box<T> 实现了Deref trait（解引用）以便可以向使用引用一样使用Box<T>
    // Box<T> 实现了Drop trait以便可以清理堆上数据

    // 0 保存在堆上，b是指向0所在内存的指针
    let b = Box::new(0);
    println!("box: {}", b);

    // 像引用一样使用智能指针
    let x = 5;
    let y = Box::new(x);
    // z获取所有权以后，y就无效了，所以Box相当于unique ptr
    // let z = y;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 利用Box<T>定义链表
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 自定义智能指针
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;

    /*
    没有 Deref trait 的话，编译器只会解引用 & 引用类型。
    deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值，
    并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力
    */
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            // 返回MyBox元组中元素的引用
            &self.0
        }
    }

    let y = MyBox::new(5);
    // 编译器自动帮我们把*y替换为 *(y.deref())
    assert_eq!(5, *y);

    // 解引用强制多态：将“实现了 Deref 的类型的引用”转换为“原始类型通过Deref所能够转换的类型的引用”
    // 解引用强制多态的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    /*
    使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。
    因为MyBox<T>实现了 Deref trait，Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
    标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
    Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了
    */
    // 如果没哟解引用强制多态，我们要这么写：hello(&(*m)[..]);
    hello(&m);

    // Deref trait实现了不可变引用的转换，同理DerefMut trait实现了可变引用的转换
    /*
    Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：

    当 T: Deref<Target=U> 时从 &T 到 &U。
    当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    当 T: Deref<Target=U> 时从 &mut T 到 &U。
    */

    // Drop trait 相当于智能指针析构函数
    struct CustomSmartPointer {
        data: String,
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer { data: String::from("c") };
    let d = CustomSmartPointer { data: String::from("d") };
    let e = CustomSmartPointer { data: String::from("e") };

    // 想要提前释放我们不能直接调用drop函数，而是通过系统std::mem::drop调用，参数为想要释放的对象
    // 提前释放的使用场景：当使用智能指针管理锁时；你可能希望强制运行 drop 方法来释放锁以便作用域中的其他代码可以获取锁
    drop(e);
    println!("CustomSmartPointers created.");

    // Box<T>相当于unique ptr，那什么对应shared ptr呢？
    // Rc<T>相当于shared ptr，通过引用计数实现多所有权，
    // 注意 Rc<T> 只能用于单线程场景
    use std::rc::Rc;
    let p1 = Rc::new(0);
    println!("count {}", Rc::strong_count(&p1));
    {
        // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数
        let p2 = Rc::clone(&p1);
        println!("count {}", Rc::strong_count(&p2));
        // 不必像调用 Rc::clone 增加引用计数那样调用一个函数来减少计数；Drop trait 的实现当 Rc<T> 值离开作用域时自动减少引用计数
    }
    println!("count {}", Rc::strong_count(&p1));

    use std::cell::RefCell;
    struct TestRefCell1{
        s: String,
    }
    let s1 = TestRefCell1 {
        s: String::from("s")
    };
    // s1是un mut的，所以这里编译不过
    // s1.s.push_str("aaa");

    struct TestRefCell2{
        s: RefCell<String>,
    }
    let s2 = TestRefCell2 {
        s: RefCell::new(String::from("s"))
    };
    // s2是un mut的，但是s是RefCell类型，所以我们通过RefCell borrow_mut可以改变s的值
    // 这就是RefCell的作用：内部可变性
    s2.s.borrow_mut().push_str("aaa");

    /*
    当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，
    这属于 RefCell<T> 安全 API 的一部分。borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut 类型的智能指针。
    这两个类型都实现了 Deref，所以可以当作常规引用对待。

    RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。
    当 Ref<T> 值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。

    如果我们尝试违反这些规则，相比引用时的编译时错误，RefCell<T> 的实现会在运行时出现 panic
    */


    //let a = ::new(0);

    /*
    如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：

    Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
    Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
    因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
    */

    // Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问
    // 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者并且可以修改的值了
    let p = Rc::new(RefCell::new(0));
    let p1 = Rc::clone(&p);
    let p2 = Rc::clone(&p);
    *p1.borrow_mut() = 2;
    println!("p: {:?}", p);

    /*
    Weak<T>
    
    到目前为止，我们已经展示了调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，
    和只在其 strong_count 为 0 时才会被清理的 Rc<T> 实例。
    你也可以通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。
    调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。
    不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 会将 weak_count 加1。
    Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。
    其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

    强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，
    因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

    因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。
    为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，
    则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。因为 upgrade 返回一个 Option<T>，
    我们确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。
    */


    /*
    这一章涵盖了如何使用智能指针来做出不同于 Rust 常规引用默认所提供的保证与取舍。
    Box<T> 有一个已知的大小并指向分配在堆上的数据。
    Rc<T> 记录了堆上数据的引用数量以便可以拥有多个所有者。
    RefCell<T> 和其内部可变性提供了一个可以用于当需要不可变类型但是需要改变其内部值能力的类型，并在运行时而不是编译时检查借用规则。

    我们还介绍了提供了很多智能指针功能的 trait Deref 和 Drop。同时探索了会造成内存泄漏的引用循环，以及如何使用 Weak<T> 来避免它们。
    */
}
