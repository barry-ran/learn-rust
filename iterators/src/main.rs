fn main() {
    println!("Hello, world!");
    /*
    迭代器是通过trait实现的，只需要实现next方法即可
    pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
    */
    
    // 在迭代器上调用next方法
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1_iter = v1.iter();
    // 迭代器是惰性的，只有真正用到迭代器的时候才会去真正生成（例如这里的sum函数）
    // 类似sum函数被称为消费适配器
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // 迭代器分三种：
    // iter 方法生成一个不可变引用的迭代器。
    // into_iter 获取所有权并返回拥有所有权的迭代器。
    // iter_mut 可变引用迭代器

    // 迭代器适配器：允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。
    // 不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果

    // map 方法使用闭包来调用每个元素以生成新的迭代器
    println!("v1: {:?}", v1);
    let v2:Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);
    let f:i32 = 3;
    // filter 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器
    let v3:Vec<i32> = v1.into_iter()
        .filter(|i| *i != f)
        .collect();
    println!("v3: {:?}", v3);

    // 为自定义结构实现迭代器（只实现next方法即可）
    struct Counter {
        count: u32,
    }
    
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
    
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
    
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 通过定义 next 方法实现 Iterator trait，
    // 我们现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了
    // 因为他们都使用了 next 方法的功能

    /*
    例如，出于某种原因我们希望获取 Counter 实例产生的值，
    将这些值与另一个 Counter 实例在省略了第一个值之后产生的值配对，
    将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加

    注意 zip 只产生四对值；理论上第五对值 (5, None) 从未被产生，因为 zip 在任一输入迭代器返回 None 时也返回 None。
    所有这些方法调用都是可能的，因为我们指定了 next 方法如何工作，而标准库则提供了其它调用 next 的方法的默认实现。
    */

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);

}
