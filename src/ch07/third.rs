#![allow(dead_code)]

// if-let 和 while-let
// Rust 不仅能在 match 表达式中执行‘模式解构”，在 let 语句中， 也可以应用同样的模式
// Rust 还提供了 if-let 语法糖。它的语法为 if let PATTERN = EXPRESSION { BODY }
// 后面可以跟一个可选的 else 分支
pub fn first() {
    let opt = Some(1);
    match opt {
        Some(x) => println!("{}", x),
        _ => println!("None"),
    }

    println!("{:?}", opt);

    if opt.is_some() {
        let x = opt.unwrap();
        println!("{}", x);
    }

    // if-let 语法
    if let Some(x) = opt {
        println!("{}", x);
    }

    println!("-------------------");

    // 这其实是一个简单的语法糖，其背后执行的代码与 match 表达式相比，并无效率上的差别
    // 它跟 match 的区别是：match 一定要完整匹配，if-let 只匹配感兴趣的某个特定的分支
    // 这种情况下的写法比 match 简单点
    // 同理，while-let 与 if-let 一样，提供了在 while 语句中使用“模式解构”的能力
    // if-let 和 while-let 还支持模式的“或”操作（此功能目前尚未在编译器中实现）
    enum E<T> {
        A(T),
        B(T),
        C,
        D,
        E,
        F,
    }

    let x: E<i32> = E::C;
    let r = if let E::C | E::D = x { 1 } else { 2 };
    println!("{}", r);
    let r = match x {
        E::C | E::D => 1,
        _ => 2,
    };
    println!("{}", r);

    let expr = E::A(32i32);
    // 在这个匹配过程中还可以有变量绑定
    let mut count = 0;
    while let E::A(x) | E::B(x) = expr {
        println!("{}", x);
        count += 1;
        if count >= 10 {
            break;
        }
    }

    println!("--------------");

    match expr {
        E::A(x) | E::B(x) => println!("{}", x),
        _ => {}
    }
}

// 函数和闭包参数做模式解构
// 一个函数接受一个结构体参数，可以直接在参数这里做模式解构
pub fn second() {
    println!("-----------------");

    struct T {
        item1: char,
        item2: bool,
    }

    fn test(
        T {
            item1: arg1,
            item2: arg2,
        }: T,
    ) {
        println!("{} {}", arg1, arg2);
    }

    let x = T {
        item1: 'A',
        item2: false,
    };

    test(x);
}

// 总结
// “模式解构” 是 Rust 中较为复杂的一个功能，但是非常实用
// 1. Rust 的“模式解构”功能在语法上具有良好的一致性和扩展性
// 2. Rust 的“模式解构”功能不仅出现在 match 语句中，还可以
//    出现在 let、if-let、while-let、函数调用、闭包调用等情景中
// 3. Rust 的“模式解构”功能可以应用于各种数据类型，包括但不限于
//    tuple、struct、enum 等， 暂时在稳定版中不支持 slice 的模式匹配
// 4. Rust 的“模式解构”功能要求“无遗漏”的分析(exhaustive case analysis)
//    确保不会因为不小心而漏掉某些情况
// 5. Rust 的“模式解构”与 Rust 的核心所有权管理功能完全相容