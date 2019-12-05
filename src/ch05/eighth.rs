#![allow(dead_code)]

// 标准库中常见的 trait

// Display Debug
pub fn learn_display_debug() {
    use std::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    struct T {
        field1: i32,
        field2: i32,
    }

    impl Display for T {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{{ field1: {}, field2: {} }}", self.field1, self.field2)
        }
    }

    let var = T {
        field1: 1,
        field2: 2,
    };
    println!("{}", var);
    println!("{:?}", var);
    println!("{:#?}", var);
}

// 只有实现了 Display trait 的类型，才能用 {} 格式控制打印出来
// 只有实现了 Debug trait 的类型，才能用 {:?} {:#?} 格式控制打印出来
// {} {:?} {:#?} 的区别如下

// Display 假定了这个类型可以用 utf-8 格式的字符串表示，它是准备给最终用户看的
// 并不是所有类型都应该或者能够实现这个 trait。这个 trait 的 fmt 应该如何格式化
// 字符串，完全取决于程序员自己，编译器不提供自动 derive 的功能

// 标准库中还有一个常用 trait 叫作 std::string::ToString，对于所有实现了
// Display trait 的类型，都自动实现了这个 ToString trait
// 它包含了一个方法 to_string(&self) -> String 任何一个实现了 Display
// trait 的类型，我们都可以对它调用 to_string() 方法格式化出一个字符串

// Debug 则是主要为了调试使用，建议所有的作为 API 的"公开"类型都应该实现
// 这个 trait，以方便调试。它打印出来的字符串不是以"美观易读"为标准
// 编译器提供了自动 derive 的功能

// PartialOrd / Ord / PartialEq / Eq
// 因为 NaN 的存在，浮点数是不具备"total order(全序关系)"的

// 对于集合 X 中的元素 a, b, c,
// * 如果 a < b 则一定有 !(a > b)；反之， 若 a > b，则一定有 !(a < b)，称为反对称性
// * 如果 a < b 且 b < c 则 a < c，称为传递性
// * 对于 X 中的所有元素，都存在 a < b 或 a > b 或者 a == b， 三者必居其一，称为完全性
// 偏序: 如果集合 X 中的元素只具备上述前两条特征
// 全序: 同时具备以上所有特征

pub fn learn_float() {
    let nan = std::f32::NAN;
    let x = 1.0f32;
    println!("{}", nan < x);  // false
    println!("{}", nan > x);  // false
    println!("{}", nan == x); // false
}
// Rust 设计了两个 trait 来描述这样的状态:
// 一个是 std::cmp::PartialOrd，表示"偏序"
// 一个是 std::cmp::Ord，表示"全序"

// PartialEq 和 Eq 两个 trait 也就可以理解了
// 它们的作用是比较相等关系，与排序关系非常类似
// 偏等 全等

// Sized
// 这个 trait 定义在 std::marker 模块中，它没有任何的成员方法
// 它有#[lang = "sized"] 属性，说明它与普通 trait 不同，编译器对它有特殊的处理
// 用户也不能针对自己的类型 impl 这个 trait
// 一个类型是否满足 Sized 约束是完全由编译器推导的，用户无权指定

// 我们知道，在 C/C++ 这一类的语言中，大部分变量、参数、返回值都应该
// 是编译阶段固定大小的。在 Rust 中，但凡编译阶段能确定大小的类型，
// 都满足 Sized 约束。那还有什么类型是不满足 Sized 约束的呢？
// 比如 C 语言里的不定长数组(Variable-length Array)。
// 不定长数组的长度在编译阶段是未知的，是在执行阶段才确定下来的
// Rust 里面也有类似的类型[T]。 在 Rust 中 VLA 类型已经通过了 RFC 设计，
// 只是暂时还没有实现而已。不定长类型在使用的时候有一些限制，
// 比如不能用它作为函数的返回类型，而必须将这个类型藏到指针背后才可以
// 但它作为一个类型，依然是有意义的，我们可以为它添加成员方法，
// 用它实例化泛型参数，等等

// Rust 中对于动态大小类型专门有一个名词 Dynamic Sized Type。
// 我们后面将会看到的[T], str 以及 dyn Trait 都是 DST

// Default
// Rust 里面并没有 C++ 里面的"构造函数"的概念
// 大家可以看到，它只提供了类似 C 语言的各种复合类型各自的初始化语法
// 主要原因在于，相比普通函数，构造函数本身并没有提供什么额外的抽象能力
// 所以 Rust 里面推荐使用普通的静态函数作为类型的"构造器"
// 对于那种无参数、无错误处理的简单情况，标准库中提供了 Default trait
// 来做这个统一抽象
// 它只包含一个"静态函数" default() 返回 Self 类型
// 标准库中很多类型都实现了这个 trait，它相当于提供了一个类型的默认值
// 在 Rust 中，单词 new 并不是一个关键字。所以我们可以看到，很多类型中都使用了
// new 作为函数名，用于命名那种最常用的创建新对象的情况。因为这些 new 函数差别甚大，
// 所以并没有一个 trait 来对这些 new 函数做一个统一抽象

// 总结
// 除了上面介绍的之外，trait 还有许多用处:
// 1 trait 可以携带泛型参数
// 2 trait 可以用在泛型参数的约束中
// 3 trait 可以为一组类型 impl，也可以单独为某一个具体类型 impl，而且它们可以同时存在
// 4 trait 可以为某个 trait impl，而不是为某个具体类型 impl
// 5 trait 可以包含关联类型，而且还可以包含类型构造器，实现高阶类型的某些功能
// 6 trait 可以实现泛型代码的静态分派，也可以通过 trait object 实现动态分派
// 0 trait 可以不包含任何方法，用于给类型做标签(marker)以此来描述类型的一些重要特性
// 0 trait 可以包含常量

// trait 这个概念在 Rust 语言中扮演了非常重要的角色，承担了各种各样的功能，在写代码的时候会经常用到
