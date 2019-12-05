#![allow(dead_code)]
// trait 约束和继承

// Rust 的 trait 的另外一个大用处是，作为泛型约束使用
use std::fmt::Debug;

// 冒号后面加 trait 名字，就是这个泛型参数的约束条件
// 它要求这个 T 类型实现 Debug 这个 trait。这是因为
// 我们在函数体内，用到了 println! 格式化打印，而且用了
// {:?} 这样的格式控制符，它要求类型满足 Debug 的约束，
// 否则编译不过。
// 泛型约束既是对实现部分的约束，也是对调用部分的约束
fn my_print<T: Debug>(x: T) {
    println!("The value is {:?}.", x);
}

// 泛型约束还有另外一种写法，即 where 子句
fn my_print1<T>(x: T)
where
    T: Debug,
{
    println!("The value is {:?}.", x);
}

// 在某些复杂的情况下，泛型约束只有 where 子句可以表达，泛型参数后面
// 直接加冒号的写法表达不出来，比如涉及关联类型的时候

pub fn learn_trait1() {
    my_print("China");
    my_print(41_i32);
    my_print(true);
    my_print(['a', 'b', 'c']);
    my_print1(['1', 'a', 'b', 'c']);
}

// trait 允许继承
trait Base {}

trait Derived: Base {}
// 满足 Derived 的类型，必然也满足 Base
// 在针对一个具体类型 impl Derived 的时候，
// 编译器也会要求我们同时 impl Base
// 实际上，在编译器的眼中，trait Derived: Base {} 等同于
// trait Derived where Self: Base {} 
// 这两种写法没有本质上的区别，都是给 Derived 这个 trait 加了一个约束条件
// 即实现 Derived trait 的具体类型，也必须满足 Base trait 的约束