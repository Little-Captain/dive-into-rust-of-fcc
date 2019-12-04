#[allow(dead_code)]

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
