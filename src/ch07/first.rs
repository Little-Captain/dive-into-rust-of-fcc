#![allow(dead_code)]

// "Pattern Destructure"是 Rust 中一个重要且实用的设计
// "Destructure" 的意思是把原来的结构肢解为单独的、局部的、原始的部分
pub fn destructure1() {
    let tuple = (1_i32, false, 3f32);
    let (head, center, tail) = tuple;
    println!("{} {} {}", head, center, tail);
}

// 构造和解构遵循类似的语法，怎么把一个数据结构组合起来的，就怎么把它拆解开来
pub fn destructure2() {
    struct T1(i32, char);

    struct T2 {
        item1: T1,
        item2: bool,
    }

    let x = T2 {
        item1: T1(0, 'A'),
        item2: false,
    };

    // 完全可以一次性解构多个层次，直接把这个对象内部深处的元素拆解出来
    let T2 {
        item1: T1(value1, value2),
        item2: value3,
    } = x;

    println!("{} {} {}", value1, value2, value3);
}

// match
// Rust 的"模式解构"功能不仅出现在 let 语句中，还可以出现在
// match、if let、while let、函数调用、闭包调用等情景中
// match 具有功能最强大的模式匹配
