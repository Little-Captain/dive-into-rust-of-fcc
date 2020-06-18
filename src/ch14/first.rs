// NLL(Non-Lexical Lifetimes): 非词法作用域生命周期(1.44.0 默认开启)
// Rust 防范“内存不安全”代码的原则极其清晰明了。
// 如果对同一块内存存在多个引用，就不要试图对这块内存做修改；
// 如果需要对一块内存做修改，就不要同时保留多个引用。
// 只要保证了这个原则，就可以保证内存安全。
// 这个原则是 Rust 的立身之本、生命之基、活力之源。

// 这个原则的初始实现版本有一个主要问题，那就是它让借用指针的生命周期规则
// 与普通对象的生命周期规则一样，是按作用域来确定的。
// 所有的变量、借用的生命周期就是从它的声明开始到当前整个语句块结束。
// 这个设计被称为 Lexical Lifetimes，生命周期是严格和词法中的作用域范围绑定的。
// 这个策略实现起来非常简单，但它可能过于保守了，某些情况下借用的范围被过度拉长了，
// 以至于某些实质上是安全的代码也被阻止了。在某些场景下，限制了程序员的发挥。
// 因此，Rust 核心组决定引人 Non-Lexical Lifetimes，
// 用更精细的手段调节借用真正起作用的范围。这就是 NLL。

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

// NLL 解决的问题
pub fn first() {
    fn test1() {
        fn foo1() -> Vec<char> {
            let mut data = vec!['a', 'b', 'c'];
            // 创建一个临时的 &mut 型引用，函数调用结束后，
            // 这个临时借用的生命周期就结束了。
            capitalize(&mut data[..]);
            data.push('d');
            data.push('e');
            data.push('f');
            data
        }
        // 在早期的编译器内部实现中，所有的变量，包括引用，它们的生命周期都是
        // 从声明的地方开始，到当前语句块结束(不考虑所有权转移的情况)。

        // 当前(1.44.0)的编译器实现有所改变：
        // 变量生命周期可能(需要确认)是从声明的地方到最后使用的地方!!!

        // 当前版本(NLL 开启)下，以下原则可能不适用
        // 每个引用的生命周期都是跟代码块(scope)相关联的，它总是从声明的时候被创建，
        // 在退出这个代码块的时候被销毁，因此可以称为 Lexical lifetimes。
        // 而 Non-Lexical lifetimes，就是取消这个关联性，引用的生命周期，
        // 用另外的更智能的方式分析。
        fn foo2() -> Vec<char> {
            let mut data = vec!['a', 'b', 'c'];
            // error: cannot borrow `data` as mutable more than once at a time
            // Rust 规定“共享不可变，可变不共享”，
            // 同时出现两个 &mut 型借用是违反规则的。
            let slice = &mut data[..];
            capitalize(slice);
            data.push('d');
            data.push('e');
            data.push('f');
            // let len = slice.len();
            data
        }
        fn capitalize(data: &mut [char]) {
            for c in data {
                c.make_ascii_uppercase();
            }
        }
        let v = foo1();
        println!("{:?}", v);
        let v = foo2();
        println!("{:?}", v);
    }
    test1();
    // 当前编译器 NLL 打开，编译通过。
    // 在老版本编译器中，无法编译通过。
    // 老版本编译器认为 map 在 match 语句中有多个 &mut 引用存在
    fn process_or_default<K, V: Default>(map: &mut HashMap<K, V>, key: K)
        where K: Eq + Hash, V: Display {
        match map.get_mut(&key) {
            Some(value) => println!("{}", value),
            None => {
                map.insert(key, V::default());
            }
        }
    }
    fn get_default<K, V>(map: &mut HashMap<K, V>, key: K)
                         -> &mut V where K: Eq + Hash, V: Default {
        map
            .entry(key)
            .or_insert_with(|| V::default())
    }
    // 让编译器能更准确地分析借用指针的生命周期，不要简单地与 scope 相绑定，
    // 不论对普通用户还是高阶用户都是一个更合理、更有用的功能。
    // NLL 应运而生！！！
    fn test2() {
        fn process_or_default(map: &mut HashMap<String, String>,
                              key: String) {
            match map.get_mut(&key) {
                Some(value) => println!("{}", value),
                None => {
                    map.insert(key, "value".to_string());
                }
            }
        }
        let mut map = HashMap::<String, String>::new();
        process_or_default(&mut map, "abc".to_string());
        process_or_default(&mut map, "abc".to_string());
    }
    test2();
}

// NLL(Non-Lexical Lifetimes) 的原理
// NLL 的设计目的是让“借用’的生命周期不要过长，适可而止，
// 往往为了避免编译错误，把实际上正确的代码也一起拒绝掉了。
// NLL 的实现，不能是简单地在 AST 上找到某个引用最后一次在哪里使用，
// 在其后就让它的生命周期结束。
pub fn second() {
    fn capitalize(data: &mut [char]) {
        for c in data {
            c.make_ascii_uppercase();
        }
    }
    /*
    fn test1() {
        let mut data = vec!['a', 'b', 'c'];
        let slice = &mut data[..];
        loop {
            capitalize(slice);
            // error: cannot borrow `data` as mutable more than once at a time
            data.push('d');
        }
        data.push('e');
        data.push('f');
    }
    test1();
    */
    // 新版本的借用检查器将不再基于 AST 的语句块来设计，而是将 AST 转换为另外
    // 一种中间表达形式 MIR (middle-level intermediate representation) 之后，
    // 在 MIR 的基础上做分析。
    // 对于复杂一点的程序逻辑，基于 AST 来做生命周期分析是无法达到预期的，
    // 而 MIR 则更适合做这种分析。
    // 在一般情况下，MIR 在编译器内部的表现形式是内存中的一组数据结构。
    // 这些数据结构描述的是一个叫作‘控制流图”(control flow graph)的概念。
    // 所谓控制流图，就是用“图”这种数据结构，描述程序的执行流程。
    // 相比于以前，一个引用的生命周期直接充满整个语句块，
    // NLL 要精细得多，它可以保证引用的生命周期不会被过分拉长。
    fn test2() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(v.len());
        println!("{:?}", v);
    }
    test2();

    fn test3() {
        let mut data = 100;
        let mut p = &data;
        println!("{}", p); // p 的生命周期结束
        data = 101;
        p = &data; // 重新为 p 赋值 &data
        println!("{}", p);
    }
    test3();
}

// NLL 的影响
// 1. 只影响静态分析结果，不影响程序的执行情况；
// 2. 以前能编译通过的程序，以后依然会编译通过，不会影响以前的代码；
// 3. 它依然保证了安全性，只是将以前过于保守的检查规则适当放宽；
// 4. 它依赖的依然是静态检查规则，不会涉及任何动态检查规则；
// 5. 它只影响“引用类型”的生命周期，不影响“对象”的生命周期，
//    即维持现有的析构函数调用时机不变；
// 6. 它不会影响 RAII 语义。

// 总结
// 内存安全是需要一些代码规范来约束才能实现的。
// Rust 中的生命周期是初学者不易理解的难点，
// 而且也确实存在一些情况损害了语言的表达能力。
// 应该做一些更精细的、准确的调整，使它尽可能接近
// “安全”与“不安全”的那条分界线，不偏不倚，宽严皆误。
// NLL 就是为了接近这条分界线，而设计的。
// A rule is worthless if it is not enforced.
// 如果规则不能强制执行，那么这个规则就是没有价值的。
// Rust 对于 segment fault 这一类内存安全问题，
// 可以在静态代码检查阶段完整无遗漏地检查出来。
// Rust 保证内存安全的设计带来了 Rust 独有的新的设计范式。
// Rust 相关的设计范式，需要多读高质量的开源代码来培养感觉。
