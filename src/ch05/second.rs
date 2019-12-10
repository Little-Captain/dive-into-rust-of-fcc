#![allow(dead_code)]

// 静态方法
// 没有 receiver 参数的方法(第一个参数不是 self 参数的方法)称作"静态方法"
// 静态方法可以通过 Type::FunctionName() 的方式调用
// 需要注意的是，即便我们的第一个参数是 Self 相关类型，只要变量名字不是 self，
// 就不能使用小数点的语法调用函数
struct T(i32);

impl T {
    // 这是一个静态方法
    fn func(this: &Self) {
        println!("value {}", this.0);
    }
}

pub fn learn_static_method() {
    let x = T(42);
    // x.func();
    T::func(&x);
}

// trait 中也可以定义静态函数
trait Default1 {
    fn default() -> Self;
}
// Rust 中没有"构造函数"的概念
// Default trait 实际上可以看作一个针对无参数构造函数的统一抽象
// 在标准库中，Vee::default() 就是一个普通的静态函数

// 跟 C++ 相比，在 Rust 中，定义静态函数没必要使用 static 关键字，
// 因为它把 self 参数显式在参数列表中列出来了。作为对比，C++ 里面
// 成员方法默认可以访问 this 指针，因此它需要用 static 关键字来标记静态方法
// Rust 不采取这个设计，主要原因是 self 参数的类型变化太多，
// 不同写法语义差别很大，选择显式声明 self 参数更方便指定它的类型。
