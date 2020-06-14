// 移动语义

// 一个变量可以把它拥有的值转移给另外一个变量，称为“所有权转移”
// 赋值语句、函数调用、函数返回等，都有可能导致所有权转移。
// Rust 中所有权转移是所有类型的默认语义
// Rust 中的变量绑定操作，默认是 move 语义，执行了新的变量绑定后，原来的变量就不能再使用了！！！
// Rust vs. C++
// Rust: let v1: Vec<i32> = v2; 移动语义
// C++: std::vector<int> v1 = v2; 复制语义
// 对于“移动语义”，需要强调的一点是，“语义”不代表最终的执行效率。
// “语义”只是规定了什么样的代码是编译器可以接受的，以及它执行后
// 的效果可以用怎样的思维模型去理解。
// 编译器有权在不改变语义的情况下做任何有利于执行效率的优化。
// 语义和优化是两个阶段的事情。
// 可以把移动语义想象成执行了一个 memcpy，但真实的汇编代码未必如此。
pub fn first() {
    fn create() -> String {
        let s = String::from("hello");
        return s; // 所有权转移，从函数内部移动到外部
    }
    fn consume(s: String) { // 所有权转移，从函数外部移动到内部
        println!("{}", s);
    }
    fn test1() {
        let s = create();
        consume(s);
    }
    test1();
}

// 复制语义
// 默认的 move 语义是 Rust 的一个重要设计，
// 但是任何时候需要复制都去调用 clone 函数会显得非常烦琐。
// 对于一些简单类型，让它们在赋值的时候默认采用复制操作会让语言更简单。
pub fn second() {
    fn test1() {
        let v1: isize = 0;
        let mut v2 = v1;
        println!("{} {}", v1, v2);
        v2 = 3;
        println!("{} {}", v1, v2);
        // 在 Rust 中有一部分“特殊照顾”的类型，其变量绑定操作是 copy 语义。
        // 所谓的 copy 语义，是指在执行变量绑定操作的时候，v2 是对 v1 所属
        // 数据的一份复制。v1 所管理的这块内存依然存在，并未失效，而 v2 是
        // 新开辟了一块内存，它的内容是从 v1 管理的内存中复制而来的。和手动调用
        // clone 方法效果一样。
        // let v2 = v1； <=> let v2 = v1.clone()；
    }
    test1();
}

// 使用文件系统进行对比

// copy 语义就像“复制、粘贴”操作。
// 操作完成后，原来的数据依然存在，而新的数据是原来数据的复制品。

// move 语义就像“剪切、粘贴”操作。
// 操作完成后，原来的数据就不存在了，被移动到了新的地方。

// 这两个操作本身是一样的，都是简单的内存复制，
// 区别在于复制完以后，原先那个变量的生命周期是否结束。

// Rust 中，在普通变量绑定、函数传参、模式匹配等场景下，
// 凡是实现了 std::marker::Copy trait 的类型，都会执行 copy 语义。
// 基本类型，比如数字、字符、bool等 ，都实现了 Copy trait，因此具备 copy 语义。
// 对于自定义类型，默认是没有实现 Copy trait 的，但是可以手动添上。
pub fn third() {
    struct Foo {
        data: i32
    }

    impl Clone for Foo {
        fn clone(&self) -> Foo {
            Foo { data: self.data }
        }
    }
    impl Copy for Foo {}

    // Foo 类型也拥有了复制语义。在执行变量绑定、函数参数传递的时候，
    // 原来的变量不会失效，而是会新开辟一块内存，将原来的数据复制过来。

    fn test1() {
        let v1 = Foo { data: 0 };
        let v2 = v1;
        println!("{:?} {:?}", v1.data, v2.data);
    }
    test1();

    // 绝大部分情况下，实现 Copy trait 和 Clone trait 是一个非常机械化的、重复性的工作，
    // clone 方法的函数体要对每个成员调用一下 clone 方法。
    // Rust 提供了一个编译器扩展 derive attribute，来帮助写这些代码，
    // 其使用方式为 #[derive(Copy, Clone)]。
    // 只要一个类型的所有成员都具有 Clone trait，就可以使用这种方法来让编译器实现 Clone trait
    fn test2() {
        #[derive(Copy, Clone)]
        struct Foo {
            data: i32
        }
        let v1 = Foo { data: 10 };
        let v2 = v1;
        println!("{:?} {:?}", v1.data, v2.data);
    }
    test2();
}

// Box 类型
// Box 类型是 Rust 中一种常用的指针类型。
// 它代表“拥有所有权的指针”，类似于 C++ 里面的 unique_ptr
// 严格来说，unique_ptr<T> 更像 Option<Box<T>>。

// Box 类型永远执行的是 move 语义，不能是 copy 语义。
// 原因在于，Rust 中的 copy 语义就是浅复制。
// 对于 Box 这样的类型而言，浅复制必然会造成二次释放问题。

// 对于 Rust 里面的所有变量，在使用前一定要合理初始化，否则会出现编译错误。
// 对于 Box<T> &T &mut T 这样的类型，合理初始化意味着它一定指向了某个具体的对象，
// 不可能是空。如果用户确实需要“可能为空的”指针，必须使用类型 Option<Box<T>>。
pub fn fourth() {
    fn test1() {
        struct T {
            value: i32
        }
        let p = Box::new(T { value: 1 });
        println!("{}", p.value);
    }
    test1();
    // Rust 里面有一个保留关键字 box。它可以用于把变量“装箱”到堆上。
    // 下述写法和 Box::new() 函数调用没有本质区别
    /*
    fn test2() {
        #![feature(box_syntax)]
        struct T {
            value: i32
        }
        let p = box T { value: 1 };
        println!("{}", p.value);
    }
    test2();
    */
}

// Clone vs. Copy

// Clone trait vs. Copy trait

// Copy 的含义
// Copy 的全名是 std::marker::Copy。
// std::marker 模块里面所有的 trait 都是特殊的 trait。
// 目前稳定的有四个，它们是 Copy、Send、Sized、Sync。
// 它们的特殊之处在于，它们是跟编译器密切绑定的，impl 这些 trait 对编译器的行为有重要影响。
// 在编译器看来，它们与其他的 trait 不一样。
// 这几个 trait 内部都没有方法，它们的唯一任务是给类型打一个‘标记”，
// 表明它符合某种约定，这些约定会影响编译器的静态检查以及代码生成。
// 在编译器看来，Copy 这个 trait 代表的是：
// 简单总结就是，如果一个类型 impl 了 Copy trait，意味着任何时候，都可以通过简单的内存复制
// (在 C 语言里按字节复制 memcpy)实现该类型的复制，并且不会产生任何内存安全问题。
// 一旦一个类型实现了 Copy trait，那么它在变量绑定、函数参数传递、函数返回值传递
// 等场景下，都是 copy 语义，而不再是默认的 move 语义。
// 在 Rust 里，move 语义和 copy 语义具体执行的操作，是不允许由程序员自定义的，
// 这是它和 C++ 的巨大区别。
// Rust 没有赋值构造函数或者赋值运算符重载。
// move 语义或者 copy 语义都是执行的 memcpy，无法更改，这个过程中绝对不存在其他副作用。
// 当然，这里一直谈的是“语义”，而没有涉及编译器优化。
// 从语义的角度看，就是，什么样的代码在编译器看来是合法的，什么样的代码是非法的。
// 如果考虑后端优化，在许多情况下，不必要的内存复制已经被彻底优化掉了，
// 所以不必担心执行效率的问题。也没有必要每次都把 move 或者 copy 操作与具体的汇编代码联系起来，
// 因为场景不同，优化结果不同，生成的代码也是不同的。

// Copy 的实现条件
// Rust 规定，对于自定义类型，只有所有成员都实现了 Copy trait，
// 这个类型才有资格实现 Copy trait。
// 常见的数字类型、bool 类型、共享借用指针 ＆，都是具有 Copy 属性的类型。
// Box，Vec、可写借用指针 &mut 等类型都是不具备 Copy 属性的类型。
// 对于数组类型，如果它内部的元素类型是 Copy，那么这个数组也是 Copy 类型。
// 对于元组 tuple 类型，如果它的每一个元素都是 Copy 类型，那么这个 tuple 也是 Copy 类型。
// struct 和 enum 类型不会自动实现 Copy trait。
// 只有当 struct 和 enum 内部的每个元素都是 Copy 类型时，
// 编译器才允许针对此类型实现 Copy trait。

// 可以认为，Rust 中只有 POD (C++ 语言中的 Plain Old Data) 类型才有资格实现 Copy trait。
// 在 Rust 中，如果一个类型只包含 POD 数据类型的成员，并且没有自定义析构函数，那它就是 POD 类型。
// 比如：整数、浮点数、只包含 POD 类型的数组等，都属于 POD 类型；
// Box String Vee 等不能按字节复制的类型，都不属于 POD 类型。
// 反过来，也并不是所有满足 POD 的类型都应该实现 Copy trait，是否实现 Copy 取决于业务需求。
pub fn fifth() {
    /*
    fn test1() {
        struct T(i32);
        let t1 = T(1);
        let t2 = t1; // move
        println!("{} {}", t1.0, t2.0);
    }
    test1();
    */
}

// Clone 的含义
// std::clone::Clone
// clone 方法一般用于“基于语义的复制”操作。所以，它做什么事情，跟具体类型的作用息息相关。
// 比如，
// 对于 Box 类型，clone 执行的是“深复制”；
// 对于 Rc 类型，clone 做的事情就是把引用计数值加 1。
// 可以根据需要在 clone 函数中编写任意的逻辑。
// 对于实现了 copy 的类型，它的 clone 方法应该跟 copy 语义相容，等同于按字节复制。
pub fn sixth() {
    // Rust 提供了一个 attribute，可以利用编译器自动生成这部分代码
    #[derive(Copy, Clone)]
    struct MyStruct(i32);
    let s1 = MyStruct(12);
    let s2 = s1;
    println!("{} {}", s1.0, s2.0);
    // 通过 derive 方式自动实现 Copy 和手工实现 Copy 有微小的区别。
    // 当类型具有泛型参数的时候，比如 struct MyStruct<T＞{}，
    // 通过 derive 自动生成的代码会自动添加一个 T: Copy 的约束。
}

// Copy 和 Clone 两者的区别和联系如下。
// 1. Copy 内部没有方法，Clone 内部有两个方法。
// 2. Copy trait 是给编译器用的，告诉编译器这个类型默认采用 copy 语义，而不是 move 语义。
// Clone trait 是给程序员用的，必须手动调用 clone 方法，它才能发挥作用。
// 3. Copy trait 不是想实现就能实现的，它对类型是有要求的，有些类型不可能 impl Copy。
// 而 Clone trait 则没有什么前提条件，任何类型都可以实现(unsized 类型除外，因为无
// 法使用 unsized 类型作为返回值)。
// 4. Copy trait 规定了这个类型在执行变量绑定、函数参数传递、函数返回等场景下的操作方式。
// 即这个类型在这种场景下，必然执行的是“简单内存复制”操作，这是由编译器保证的，程序员无法控制。
// Clone trait 里面的 clone 方法究竟会执行什么操作，则是取决于程序员自己写的逻辑。
// 一般情况下，clone 方法应该执行一个“深复制”操作，
// 但这不是强制性的，如果愿意，在里面启动一个人工智能程序都是有可能的。
// 5. 如果确实不需要 Clone trait 执行其他自定义操作，编译器提供了一个工具，
// 可以在一个类型上添加 #[derive(Clone)]，来让编译器自动生成那些重复的代码。
// 编译器自动生成的 clone 方法就是依次调用每个成员的 clone 方法。
// 6. Rust 语言规定了在 T: Copy 的情况下，Clone trait 代表的含义。
// 即：当某变量 t:T 符合 T: Copy 时，它调用 t.clone() 方法的含义必须等同于“简单内存复制”。
// 也就是说，clone 的行为必须等同于 let x = std::ptr::read(&t);，也等同于 let x ＝ t;。
// 当 T: Copy 时，不要在 Clone trait 里面乱写自己的逻辑。
// 所以，当需要指定一个类型是 Copy 的时候，最好使用 #[derive(Copy, Clone)] 方式，
// 避免手动实现 Clone 导致错误。
