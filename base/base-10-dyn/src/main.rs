fn main() {
    // trait和范型可以实现静态多态，那么rust中有没有动态多态？
    // 答案是有的，dyn关键字定义的trait对象就是为使用不同类型的值而设计的

    // 有这样一个trait
    trait Draw {
        fn draw(&self);
    }
    // 有多个不同struct都实现了这个trait
    struct A {

    }

    impl Draw for A {
        fn draw(&self) {
            println!("A draw");
        }
    }

    struct B {

    }

    impl Draw for B {
        fn draw(&self) {
            println!("B draw");
        }
    }

    // 使用trait对象保存实现了Draw trait的对象
    let v: Vec<Box<dyn Draw>> = vec![Box::new(A{}), Box::new(B{})];

    // 这效果，多像C++的动态多态
    for i in v.iter() {
        i.draw();
    }
}
