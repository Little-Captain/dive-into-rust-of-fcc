// 内部可变性
// Rust 的 borrow checker 的核心思想是“共享不可变，可变不共享”。
// 只有这个规则是不够的，在某些情况下，的确需要在存在共享的情况下可变。
// 为了让这种情况是可控的、安全的，Rust 设计了一种“内部可变性”(interior mutability)。
// “内部可变性”的概念，是与“承袭可变性”(inherited mutability)相对应的。
// Rust 中的 mut 关键字不能在声明类型的时候使用，只能跟变量一起使用。
// 类型本身不能规定自己是否是可变的。
// 一个变量是否是可变的，取决于它的使用环境，而不是它的类型。
// 可变还是不可变取决于变量的使用方式，这就叫作“承袭可变性”。
// 如果用 let var: T; 声明，那么 var 是不可变的，同时，var 内部的所有成员也都是不可变的；
// 如果用 let mut var: T; 声明，那么 var 是可变的，相应的，它的内部所有成员也都是可变的。
// 不能在类型声明的时候指定可变性，比如在 struct 中对某部分成员使用 mut 修饰，这是不合法的。
// 只能在变量声明的时候指定可变性。也不能针对变量的某一部分成员指定可变性，其他部分保持不变。
// 常见的具备内部可变性特点的类型有 Cell、RefCell、Mutex、RwLock、Atomic* 等。
// 其中 Cell 和 RefCell 是只能用在单线程环境下的具备内部可变性的类型。

use std::cell::{BorrowError, RefMut, BorrowMutError, Cell};

// Cell
// 如果有共享引用指向一个对象，那么这个对象就不会被更改了。
// 因为在共享引用存在的期间，不能有可变引用同时指向它，因此它一定是不可变的。
// 其实在 Rust 中，这种理解是不准确的。
pub fn first() {
    // Rc 是 Rust 里面的引用计数智能指针。多个 Re 指针可以同时指向同一个对象，
    // 而且有一个共享的引用计数值在记录总共有多少个 Rc 指针指向这个对象。
    // 注意 Rc 指针提供的是共享引用，按道理它没有修改共享数据的能力。
    // 用共享引用调用 clone 方法，引用计数值会发生变化。这就是“内部可变性”。
    // 如果没有内部可变性，标准库中的 Rc 类型是无法正确实现出来的。
    fn test1() {
        use std::rc::Rc;
        let r1 = Rc::new(1);
        println!("reference count {}", Rc::strong_count(&r1));
        let r2 = r1.clone();
        println!("reference count {}", Rc::strong_count(&r2));
    }
    test1();
    // 具备内部可变性的类型，最典型的就是 Cell。
    fn test2() {
        use std::cell::Cell;
        let data: Cell<i32> = Cell::new(100);
        let p = &data;
        data.set(10);
        println!("{}", p.get());
        p.set(20);
        println!("{:?}", data);
        // 需要注意的是，
        // data 这个变量绑定没有用 mut 修饰，
        // p 这个指针也没有用 &mut 修饰，
        // 然而不可变引用竟然可以调用 set 函数，改变变量的值，
        // 而且还有出现任何编译错误。
        // 这就是所谓的内部可变性，这种类型可以通过共享指针修改它内部的值。
        // Cell 类型似乎违反了 Rust 的“唯一修改权”原则。
        // 存在多个指向 Cell 类型的不可变引用，同时还能利用不可变引用改变 Cell 内部的值。
        // 实际上，这个类型是完全符合“内存安全”的。

        // 为什么 Rust 要尽力避免 alias 和 mutation 同时存在？
        // 假如同时有可变指针和不可变指针指向同一块内存，
        // 有可能出现通过一个可变指针修改内存的过程中，
        // 数据结构处于被破坏状态的情况下，被其他的指针观测到。

        // Cell 类型是不会出现这样的情况的。
        // 因为 Cell 类型把数据包裹在内部，用户无法获得指向内部状态的指针，
        // 这意味着每次方法调用都是执行的一次完整的数据移动操作。
        // 每次方法调用之后，Cell 类型的内部都处于一个正确的状态，不可能观察到数据被破坏掉的状态。

        // Cell 类似一个“壳”，它把数据严严实实地包裹在里面，所有的指针只能指向 Cell，
        // 不能直接指向数据。修改数据只能通过 Cell 来完成，用户无法创造一个直接指向数据的指针。

        // Cell 类型公开的主要 API
        // impl<T> Cell<T> {
        //     pub fn get_mut(&mut self) -> &mut T {}
        //     pub fn set(&self, val: T) {}
        //     pub fn swap(&self, other: &Self) {}
        //     pub fn replace(&self, val: T) -> T {}
        //     pub fn into_inner(self) -> T {}
        // }
        // impl<T: Copy> Cell<T> {
        //     pub fn get(&self) -> T {}
        // }
        // 1. get_mut 方法可以从 &mut Cell<T> 类型制造出一个 &mut T 型指针。
        //    因为 &mut 型指针具有“独占性”，所以这个函数保证了调用前，有且仅有一个“可写”指针指向 Cell，
        //    调用后有且仅有一个“可写”指针指向内部数据。它不存在制造多个引用指向内部数据的可能性。
        // 2. set 方法可以修改内部数据。它是把内部数据整个替换掉，不存在多个引用指向内部数据的可能性。
        // 3. swap 方法也是修改内部数据。跟 set 方法一样，也是把内部数据整体替换掉。
        //    与 std::mem::swap 函数的区别在于，它仅要求 & 引用，不要求 &mut 引用。
        // 4. replace 方法也是修改内部数据。跟 set 方法一样，它也是把内部数据整体替换，换出来的数据作为返回值返回。
        // 5. into_inner 方法相当于把这个“壳”剥掉了。它接受的是 Self 类型，即 move 语义，
        //    原来的 Cell 类型的变量会被 move 进入这个方法，会把内部数据整体返回出来。
        // 6. get 方法接受的是 &self 参数，返回的是 T 类型，它可以在保留之前 Cell 类型不变的情况下
        //    返回一个新的 T 类型变量，因此它要求 T:Copy 约束。
        //    每次调用它的时候，都相当于把内部数据 memcpy 了一份, 然后返回出去。
        // 正因为上面这些原因，我们可以看到，
        // Cell 类型虽然违背了“共享不可变，可变不共享”的规则，但它并不会造成内存安全问题。
        // 它把“共享且可变”的行为放在了一种可靠、可控、可信赖的方式下进行。
        // 它的 API 是经过仔细设计过的，绝对不可能让用户有机会通过 &Cell<T> 获得 &T 或者 &mut T。
        // 它是对 alias+mutation 原则的有益补充，而非完全颠覆。
    }
    test2();
}

// RefCell
// RefCell 是另外一个提供了内部可变性的类型。它提供的方式与 Cell 类型有点不一样。
// Cell 类型没办法制造出直接指向内部数据的指针，而 RefCell 可以。
// RefCell 的主要 API
// impl<T: ?Sized> RefCell<T> {
//     pub fn borrow(&self) -> Ref<T> {}
//     pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {}
//     pub fn borrow_mut(&self) -> RefMut<T> {}
//     pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {}
//     pub fn get_mut(&mut self) -> &mut T {}
// }
pub fn second() {
    fn test1() {
        use std::cell::RefCell;
        let shared_vec: RefCell<Vec<isize>> = RefCell::new(vec![1, 2, 3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;
        shared1.borrow_mut().push(4);
        println!("{:?}", shared_vec.borrow());
        shared2.borrow_mut().push(5);
        println!("{:?}", shared_vec.borrow());
    }
    test1();
    // 编译通过，执行失败。
    // 出现 panic 的原因是，RefCell 探测到同时出现了 alias 和 mutation 的情况，
    // 它为了防止更糟糕的内存不安全状态，直接使用了 panic 来拒绝程序继续执行。
    // 如果用 try_ borrow 方法，就会返回 Result::Err，这是一种更友好的错误处理方式。

    // RefCell 的探测原理
    // RefCell 内部有一个“借用计数器”，调用 borrow 方法的时候，
    // 计数器里面的“共享引用计数”值就加 1。当这个 borrow 结束的时候，会将这个值自动减 1。
    // 同理，borrow_mut 方法被调用的时候，它就记录一下当前存在“可变引用”。
    // 如果“共享引用”和“可变引用”同时出现了，就会报错。

    // 从原理上来说，Rust 默认的“借用规则检查器”的逻辑非常像一个在编译阶段执行的
    // “读写锁”(read-write-locker)。
    // 如果同时存在多个“读”的锁，是没问题的；
    // 如果同时存在“读”和“写”的锁，或者同时存在多个“写”的锁，就会发生错误。
    // RefCell 类型并没有打破这个规则，只不过，它把这个检查逻辑从编译阶段移到了执行阶段。
    // RefCell 可以通过共享引用 & 修改内部数据，逃过编译器的静态检查。
    // 但是它依然在兢兢业业地尽可能保证“内存安全”。借用指针必须通过它提供的 API borrow()
    // borrow_mut() 来获得，它实际上是在执行阶段，在内部维护了一套“读写锁”检查机制。
    // 一旦出现了多个“写”或者同时读写，就会在运行阶段报错，
    // 用这种办法来保证写数据时候的执行过程中的内部状态不会被观测到，
    // 任何时候，读操作或者写操作开始的时候，共享的变量都处于一个合法状态。
    // 因此在执行阶段，RefCell 是有少量开销的，它需要维护一个借用计数器来保证内存安全。
    // 所以说，一定不要过于滥用 RefCell 这样的类型。

    // Cell 和 RefCell 用得最多的场景是和多个只读引用相配合。
    // 比如，多个 & 引用或者 Rc 引用指向同一个变量的时候。
    // 不能直接通过这些只读引用修改变量，因为既然存在 alias，就不能提供 mutation。
    // 为了让存在多个 alias 共享的变量也可以被修改，那就需要使用内部可变性。
    // Rust 中提供了只读引用的类型有 &、Rc、Arc等指针，它们可以提供 alias。
    // Rust 中提供了内部可变性的类型有 Cell、RefCell、Mutex、RwLock
    // 以及 Atomic* 系列类型等。这两类类型经常需要配合使用。
    // 如果需要把一个类型 T 封装到内部可变性类型中去，要怎样选择 Cell 和 RefCell 呢？
    // 原则就是，如果只需要整体性地存入、取出 T，那么就选 Cell。
    // 如果需要有个可读写指针指向这个 T 修改它，那么就选 RefCell。
    fn test2() {
        use std::cell::RefCell;

        let shared_vec = RefCell::new(vec![1, 2, 3]);
        let shared1 = &shared_vec;
        let shared2 = &shared1;
        let p1 = shared1.borrow();
        let p2 = &p1[0];

        shared2.borrow_mut().push(4);
        println!("{}", p2);
    }
    // test2();
}

//  UnsafeCell
pub fn third() {
    // 模仿实现 Cell 类型。(只实现 new、get、set 这三个方法)
    // Version 1
    fn test1() {
        /*
        struct CellV1<T> {
            value: T
        }
        impl<T> CellV1<T> {
            fn new(v: T) -> Self where T: Copy {
                CellV1 { value: v }
            }
            fn set(&self, v: T) {
                self.value = v;
            }
            fn get(&self) -> T where T: Copy {
                self.value
            }
        }
        */

        struct CellV2<T> {
            value: T
        }
        impl<T> CellV2<T> {
            fn new(v: T) -> Self where T: Copy {
                CellV2 { value: v }
            }
            fn set(&self, v: T) {
                unsafe {
                    // 此处实际上引入了未定义行为
                    let p = &(self.value) as *const T as *mut T;
                    *p = v;
                }
            }
            fn get(&self) -> T where T: Copy {
                self.value
            }
        }
        let c = CellV2::new(1);
        let p = &c;
        p.set(2);
        println!("{}", c.get());

        struct Table<'arg> {
            cell: CellV2<&'arg isize>
        }
        fn evil<'long, 'short>(t: &Table<'long>, s: &'short isize) where 'long: 'short {
            let u: &Table<'short> = t;
            // 在 'long: 'short 的情况下， &'long 类型的指针向 &'short 类型赋值是没问题的。
            // 这里的 &Table＜'long> 类型的变量赋值给 &Table<'short> 类型是不合理的。
            u.cell.set(s);
        }
        fn innocent<'long>(t: &Table<'long>) {
            let foo: isize = 1;
            evil(t, &foo);
        }
        let local = 100;
        let table = Table { cell: CellV2::new(&local) };
        let p = table.cell.get();
        println!("{}", p);
        innocent(&table);
        let p = table.cell.get();
        println!("{}", p);
        // 这段代码中出现了野指针。分析一下这段测试代码：
        // 在这段测试代码中，我们在 CellV2 类型里面保存了一个引用。
        // main 函数调用了 innocent 函数，继而又调用了 evil 函数。
        // 这里需要特别注意的是：
        // 在 evil 函数中，我们调用了 CellV2 类型的 set 方法，改变了它里面存储的指针。
        // 修改后的指针指向的是 innocent 函数内部的一个局部变量。
        // 最后在 main 函数中，innocent 函数返回后，
        // 再把这个 CellV2 里面的指针拿出来使用，就得到了一个野指针。
    }
    test1();
    // Rust 对于“内存不安全”问题是绝对禁止的。
    // 不像 C/C++，在 Rust 语言中，如果有机会让用户在不用 unsafe 的情况下制造出内存不安全，
    // 这个责任不是由用户来承担，而是应该归因于写编译器或者写库的人。
    // 在 Rust 中，写库的人不需要去用一堆文档来向用户保证内存安全，而是必须要通过编译错误来保证。
    #[lang = "unsafe_cell"] // 这个标记意味着这个类型是个特殊类型，是被编译器特别照顾的类型。
    #[stable(fature = "rust1", since = "1.0.0")]
    pub struct UnsafeCell<T: ?Sized> {
        value: T,
    }
}

// 总结
// 所有具有内部可变性特点的类型都必须基于 UnsafeCell 来实现，否则必然出现各种问题。
// 这个类型是唯一合法的将 &T 类型转为 &mut T 类型的办法。
// 绝对不允许把 &T 直接转换为 &mut T 而获得可变性。因为这是未定义行为。

// Cell 和 RefCell 可以正常工作的关键在于它们都是基于 UnsafeCell 实现的，
// 而 UnsafeCell 本身是编译器特殊照顾的类型。
// 许多时候，的确需要使用 unsafe 代码来完成功能，比如调用 C 代码写出来的库等。
// 但是却有可能一不小心违反了 Rust 编译器的规则。
// 简单地通过裸指针强制类型转换实现 &T 到 &mut T 的类型转换是错误的。
// 这么做会在编译器的生命周期静态检查过程中制造出一个漏洞，
// 而且这个漏洞用简单的测试代码测不出来，只有在某些复杂场景下才会导致内存不安全。
// Rust 代码中写 unsafe 代码最困难的地方其实就在这样的细节中，
// 有些人在没有完全理解掌握 Rust 的 safe 代码和 unsafe 代码之间的界限的情况下，
// 乱写 unsafe 代码，这是不负责任的。
