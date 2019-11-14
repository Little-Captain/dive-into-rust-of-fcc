#[allow(dead_code)]

/// 复合数据类型
// tuple
pub fn learn_tuple() {
    let a = (1i32, false);
    let b = ("a", (1i32, 2i32));
    println!("{:?}", a);
    println!("{:?}", b);
    let a = (0,);
    let b = (0);
    println!("{:?}", a);
    println!("{:?}", b);
    // 访问元组内部元素有两种方法:
    // 一种: “模式匹配”(pattern destructuring)
    // 一种: “数字索引”
    let p = (1i32, 2i32);
    let (a, b) = p;
    let x = p.0;
    let y = p.1;
    println!("{:?}", p);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", x);
    println!("{:?}", y);
    // 元组内部也可以一个元素都没有。这个类型单独有一个名字，叫 unit (单元类型)
    // 空元组和空结构体 struct Foo 一样，都是占用 0 内存空间
    // 与 C++ 中的空类型不同，Rust 中存在实打实的 0 大小的类型
    let empty: () = ();
    println!("{:?}", empty);
    println!("size of i8 {} byte(s)", std::mem::size_of::<i8>());
    println!("size of char {} byte(s)", std::mem::size_of::<char>());
    println!("size of '()' {} byte(s)", std::mem::size_of::<()>());
}

// struct

// tuple struct

// enum

// 类型递归定义
