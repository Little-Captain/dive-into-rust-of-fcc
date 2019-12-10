#![allow(dead_code)]

// 扩展方法

// 可以利用 trait 给其他的类型添加成员方法，哪怕这个类型不是我们自己写的

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> i32 {
        *self * 2
    }
}

pub fn learn_extension() {
    let x: i32 = 10.double();
    println!("{}", x);
}

// 在声明 trait 和 impl trait 的时候，Rust 规定了一个
// Coherence Rule (一致性规则)或称为 Orphan Rule(孤儿
// 规则): impl 块要么与 trait 的声明在同一个的 crate 中，
// 要么与类型的声明在同一个 crate 中

// 也就是说，如果 trait 来自于外部 crate，而且类型也来自于外部 crate，
// 编译器不允许你为这个类型 impl 这个 trait
// 它们之中必须至少有一个是在当前 crate 中定义的。因为在其他的 crate 中，
// 一个类型没有实现一个 trait ，很可能是有意的设计
// 如果我们在使用其他的 crate 的时候，强行把它们“拉郎配”，是会制造出 bug 的
// 这也意味着，上游开发者在给别人写库的时候，尤其要注意，一些比较常见的标准库
// 中的 trait ，如 Display Debug ToString Default 等，应该尽可能地提供好
// 否则，使用这个库的下游开发者是没办法帮我们把这些 trait 实现的

// 同理，如果是匿名 impl，那么这个 impl 块必须与类型本身存在于同一个 crate 中

// 其他语言中的 interface 和 trait 之间有很大的不同

// Rust 是一种用户可以对内存有精确控制能力的强类型语言
// 我们可以自由指定一个变量是在栈里面，还是在堆里面，变量和指针也是不同的类型
// 类型是有大小(Size)的。有些类型的大小是在编译阶段可以确定的，
// 有些类型的大小是编译阶段无法确定的
// 在函数参数传递、返回值传递等地方，都要求这个类型在编译阶段有确定的大小
// 否则，编译器就不知道该如何生成代码了。

// 而 trait 本身既不是具体类型，也不是指针类型，它只是定义了针对类型的、抽象的“约束”
// 不同的类型可以实现同一个 trait，满足同一个 trait 的类型可能具有不同的大小
// 因此，trait 在编译阶段没有固定大小，目前我们不能直接使用 trait 作为实例变量、参数、返回值
// 请一定要记住，trait 的大小在编译阶段是不固定的
