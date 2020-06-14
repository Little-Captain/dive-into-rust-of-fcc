use std::io::Read;

// 析构函数
// 所谓“析构函数”(destructor)是与“构造函数”(constructor)相对应的概念。
// “构造函数”是对象被创建的时候调用的函数。
// “析构函数”是对象被销毁的时候调用的函数。
// Rust 中没有统一的“构造函数”这个语法，对象的构造是直接对每个成员进行初始化完成的，
// 一般将对象的创建封装到普通静态函数中。
// 相对于构造函数，析构函数有更重要的作用。
// 它会在对象消亡之前由编译器自动调用，特别适合承担对象销毁时释放所拥有资源的作用。
// 比如，
// Vec 类型在使用的过程中，会根据情况动态申请内存，当变量的生命周期结束时，就会触发该类型析构函数的调用。
// 在析构函数中，就有机会将所拥有的内存释放掉。在析构函数中，还可以根据需要编写特定的逻辑，从而达到更多的目的。
// 析构函数不仅可以用于管理内存资源，还能用于管理更多的其他资源，如文件、锁、socket 等。
// 在 C++ 中，利用变量生命周期绑定资源的使用周期，已经是一种常用的编程惯例。
// 此手法被称为 RAII(Resource Acquisition Is Initialization)。
// 在变量生命周期开始时申请资源，在变量生命周期结束时利用析构函数释放资源，
// 从而达到自动化管理资源的作用，很大程度上减少了资源的泄露和误用。
// 在 Rust 中编写“析构函数”的办法是 impl std::ops::Drop。
// Drop trait 允许在对象即将消亡之时，自动调用指定代码。
// 对于具有多个局部变量的情况，析构函数的调用顺序是：先构造的后析构，后构造的先析构。
// 因为局部变量存在于一个“栈结构”中，要保持“先进后出”的策略。
pub fn first() {
    fn test1() {
        use std::ops::Drop;
        struct D(i32);
        impl Drop for D {
            fn drop(&mut self) {
                println!("destruct {}", self.0);
            }
        }
        let _x = D(1);
        println!("construct 1");
        {
            let _y = D(2);
            println!("construct 2");
            println!("exit inner scope");
        }
        println!("exit test1 function");
    }
    test1();
}

pub fn second() {
    // 资源管理
    // 在创建变量的时候获取某种资源，在变量生命周期结束的时候释放资源，是一种常见的设计模式。
    // 资源，不仅可以包括内存，还可以包括其他向操作系统申请的资源。
    // 比如 File，会在创建和使用的过程中向操作系统申请打开文件，在它的析构函数中就会去释放文件。
    // 所以，RAII 手法是比 GC 更通用的资源管理手段，GC 只能管理内存，RAII 可以管理各种资源。
    fn test1() {
        use std::fs::File;
        use std::io::Read;
        // 打开文件
        let f = File::open("test.txt");
        if f.is_err() {
            println!("file is not exist.");
            return;
        }
        let mut f = f.unwrap();
        let mut content = String::new();
        // 读取文件
        let result = f.read_to_string(&mut content);
        if result.is_err() {
            println!("read file error.");
            return;
        }
        println!("{}", result.unwrap());
        println!("{}", content);
        // 关闭文件
        // 不需要手动关闭文件，因为在 File 类型的析构函数中已经处理好了关闭文件的相关操作
        // 再比如标准库中的各种复杂数据结构(如 Vec LinkedList HashMap 等)，
        // 它们管理了很多在堆上动态分配的内存。它们也是利用“析构函数”这个功能，
        // 在生命终结之前释放了申请的内存空间，因此无须像 C 语言那样手动调用 free 函数。
    }
    test1();
    // 主动析构
    // 一般情况下，局部变量的生命周期是从它的声明开始，到当前语句块结束。
    // 然而，也可以手动提前结束它的生命周期。
    // 请注意，主动调用对象的析构函数是非法的。
    // 不过，可以调用标准库中的 std::mem::drop 函数
    fn test2() {
        let p = Box::new(22);
        // p.drop(); // 非法
        std::mem::drop(p);
        // println!("{}", p); // 错误的调用，因为在上一句代码已经把 p 析构了。
        // drop 函数的实现为空
        // drop 函数不需要任何的函数体，只需要参数为“值传递”即可。
        // 将对象的所有权移入函数中，什么都不用做，编译器就会自动释放掉这个对象。
        // 这个 drop 函数的关键在于使用 move 语义把参数传进来，使得变量的所有权从调用
        // 方移动到 drop 函数体内，参数类型一定要是 T ，而不是 &T 或者其他引用类型。
        // 函数体本身其实根本不重要，重要的是把变量的所有权 move 进入这个函数体中，
        // 函数调用结束的时候该变量的生命周期结束，变量的析构函数会自动调用，管理的内存空间也会自然释放。
        // 这个过程完全符合生命周期、move 语义，无须编译器做特殊处理。

        // 其次，对于 Copy 类型的变量，对它调用 std::mem::drop 函数是没有意义的。
        let x = 1;
        println!("before drop {}", x);
        std::mem::drop(x);
        println!("after drop {}", x);
        // Copy 类型在函数参数传递的时候执行的是复制语义，原来的那个变量依然存在，
        // 传入函数中的只是一个复制品，因此原变量的生命周期不会受到影响。

        // 变量遮蔽(Shadowing)不会导致变量生命周期提前结束，它不等同于 drop。
        struct D(i32);
        impl Drop for D {
            fn drop(&mut self) {
                println!("destructor for {}", self.0);
            }
        }
        let x = D(1);
        println!("construct first variable");
        let x = D(2);
        println!("construct second variable");
        // 在第二个 x 出现的时候，虽然将第一个 x 遮蔽起来了，
        // 但是第一个 x 的生命周期并未结束，它依然存在，直到函数退出。
        // 这说明了，虽然这两个变量绑定了同一个名字，但在编译器内部依然将它们视为两个不同的变量。

        // 注意，下划线这个特殊符号。
        // 如果用下划线来绑定一个变量，那么这个变量会当场执行析构，
        // 而不是等到当前语句块结束的时候再执行。下划线是特殊符号，不是普通标识符。
        let _x = D(11);
        let _ = D(12); // 当场析构
        let _y = D(13);
        // 用下划线绑定的那个变量当场就执行了析构，而其他两个变量等到语句块结束了才执行析构，
        // 而且析构顺序和初始化顺序刚好相反(栈)。

        // 区分 std::mem::drop() 函数和 std::ops::Drop::drop() 方法
        // 1. std::mem::drop() 函数是一个独立的函数，不是某个类型的成员方法，
        // 它由程序员主动调用，作用是使变量的生命周期提前结束；
        // std::ops::Drop::drop() 方法是一个 trait 中定义的方法，
        // 当变量的生命周期结束的时候，编译器会自动调用，手动调用是不允许的。
        // 2. std::mem::drop<T>(_ x: T) 的参数类型是 T，采用的是 move 语义；
        // std::ops::Drop::drop(&mut self) 的参数类型是 &mut Self，采用的是可变借用。
        // 在析构函数调用过程中，还有机会读取或者修改此对象的属性。
    }
    test2();

    // Drop vs. Copy
    // 要想实现 Copy trait，类型必须满足一定条件。
    // 这个条件是：如果一个类型可以使用 memcpy 的方式执行复制操作，
    // 且没有内存安全问题，那么它才能被允许实现 Copy trait。
    // 反过来，所有满足 Copy trait 的类型，在需要执行 move 语义的时候，
    // 使用 memcpy 复制一份副本，不删除原件是完全不会产生安全问题的。
    // 注意：带有析构函数的类型都是不能满足 Copy 语义的。
    // 因为不能保证，对于带析构函数的类型，使用 memcpy 复制一个副本一定不会有内存安全问题。
    // 所以对于这种情况，编译器直接禁止了。
    // 带有析构函数的类型是不能 Copy 的。这两个身份是不能同时存在于一个类型上的。
    /* 编译错误
    fn test3() {
        struct T;
        impl Drop for T {
            fn drop(&mut self) {}
        }
        impl Copy for T {}
    }
    test3();
    */

    // 析构标记
    // 在 Rust 里面，析构函数是在变量生命周期结束的时候被调用的。
    // 然而，既然可以手动提前终止变量的生命周期，那么就说明，变量的生命周期并不是简单地
    // 与某个代码块一致，生命周期何时结束，很可能是由运行时的条件决定的。
    // 变量的析构函数调用时机是有可能在运行阶段发生改变的。
    fn test4() {
        use std::ops::Drop;
        use std::mem::drop;
        struct D(&'static str);
        impl Drop for D {
            fn drop(&mut self) {
                println!("destructor {}", self.0);
            }
        }
        // 获取 DROP 环境变量的值，并转换为整数
        fn condition() -> Option<u32> {
            std::env::var("DROP")
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .ok()
        }
        let var = (D("first"), D("second"), D("third"));
        match condition() {
            Some(1) => drop(var.0),
            Some(2) => drop(var.1),
            Some(3) => drop(var.2),
            _ => {}
        }
        println!("test4 end");
        // 编译器对析构标记的实现
        // 首先判断一个变量是否可能会在多个不同的路径上发生析构，
        // 如果是，那么它会在当前函数调用栈中自动插入一个 bool 类型的标记，
        // 用于标记该对象的析构函数是否已经被调用。
        // 原理是在析构函数被调用的时候，把标记设置一个状态，
        // 在各个可能调用析构函数的地方都先判断一下状态再调用析构函数。
        // 这样，编译阶段确定生命周期和执行阶段根据情况调用就统一起来了。
        // 具体实现看原书代码
    }
    test4();
}