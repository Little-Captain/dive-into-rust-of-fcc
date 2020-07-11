// 解引用
// “解引用”(Deref)是“取引用”(Ref)的反操作。
// 取引用，有 &、&mut 等操作符，对应的，解引用，有 * 操作符，跟 C 语言是一样的。

use std::ops::Deref;
use std::rc::Rc;

pub fn first() {
    fn test1() {
        let v1 = 1;
        let p = &v1; // 取引用操作
        let v2 = *p; // 解引用操作
        println!("{} {}", v1, v2);
    }
    test1();
    // 自定义解引用
    // 解引用操作可以被自定义。
    // 方法是，实现标准库中的 std::ops::Deref 或者
    // std::ops::DerefMut 这两个 trait。
    // Deref 和 DerefMut 的唯一区别是：DerefMut返回的是 &mut 型引用
    pub trait Deref {
        // 这个 trait 有一个关联类型 Target，代表解引用之后的目标类型。
        type Target: ?Sized;
        fn deref(&self) -> &Self::Target;
    }
    pub trait DerefMut: Deref {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }
    /*
    impl std::ops::Deref for String {
        type Target = str;
        #[inline]
        fn deref(&self) -> &str {
            unsafe {
                str::from(&self.vec)
            }
        }
    }
    */
    // 注意这里的类型，
    // deref() 方法返回的类型是 &Target，而不是 Target。
    // 如果说有变量 s 的类型为 String，*s 的类型并不等于 s.deref() 的类型。
    // *s 的类型实际上是 Target，即 str。&*s 的类型才是 &str。
    // s.deref() 的类型为 &Target，即 &str。
    // 查看原文有一个表格！！！
    // 关键是要理解，*expr 的类型是 Target，而 deref() 方法返回的类型却是 &Target。

    // 标准库中有许多常见的类型实现了这个 Deref 操作符。
    // 比如 Vec<T>、String、Box<T>、Rc<T>、Arc<T>等。它们都支持“解引用 ”操作。
    // 从某种意义上来说，它们都可以算做特种形式的“指针”(像胖指针一样，是带有额外元数据的指针，
    // 只是元数据不限制在 usize 范围内了)。可以把这些类型都称为“智能指针”。
    // 可以这样理解这几个类型：
    // 1. Box<T> 是“指针”，指向一个在堆上分配的对象；
    // 2. Vec<T> 是“指针”，指向一组同类型的顺序排列的堆上分配的对象，
    // 且携带有当前缓存空间总大小和元素个数大小的元数据；
    // 3. String 是“指针”，指向的是一个堆上分配的字节数组，其中保存的内容是合法的 utf8 字符序列。
    // 且携带有当前缓存空间总大小和字符串实际长度的元数据。
    // 以上几个类型都对所指向的内容拥有所有权，管理着它们所指向的内存空间的分配和释放。
    // 4. Rc<T> 和 Arc<T> 也是某种形式的、携带了额外元数据的“指针”，
    // 它们提供的是一种“共享”的所有权，当所有的引用计数指针都销毁之后，它们所指向的内存空间才会被释放。

    // 自定义解引用操作符可以让用户自行定义各种各样的“智能指针”，完成各种各样的任务。
    // 再配合上编译器的“自动”解引用机制，非常有用。
}

// 自动解引用
pub fn second() {
    fn test1() {
        let s = "hello";
        println!("length: {}", s.len());
        println!("length: {}", (&s).len());
        // 如果使用 &&&&&&&&&&&&&&&&&&&&&&&&&str 类型来调用成员方法，也是可以的。
        // 原因就是，Rust 编译器做了隐式的 deref 调用，当它找不到这个成员方法的时候，
        // 会自动尝试使用 deref 方法后再找该方法，一直循环下去。
        // 编译器在 &&&str 类型里面找不到 len 方法；尝试将它 deref，变成 &&str 类型后
        // 再寻找 len 方法，还是没找到；继续 deref，变成 &str，现在找到 len 方法了，
        // 于是就调用这个方法。
        println!("length: {}", (&&&&&&&&&&&&&&&&&&&&&&&&&s).len());
        println!("length: {}", str::len(&s));
        // 自动 deref 的规则是，如果类型 T 可以解引用为 U，即 T: Deref<U>，
        // 则 &T 可以转为 &U。
    }
    test1();
}

// 自动解引用的用处
pub fn third() {
    fn test1() {
        /*
        impl<T: ?Sized> Deref for Rc<T> {
            type Target = T;

            #[inline(always)]
            fn deref(&self) -> &T {
                &self.inner().value
            }
        }
        */
        let s = Rc::new(String::from("hello"));
        // str 的 bytes 方法
        println!("{:?}", s.bytes());
        // s.deref() -> String
        println!("{:?}", s.deref().bytes());
        // s.deref().deref() -> str
        println!("{:?}", s.deref().deref().bytes());
        // 实际上通过 Rc 类型的变量调用了 str 类型的方法，让这个智能指针透明。
        // 这就是自动 Deref 的意义。
    }
    test1();
    fn test2() {
        let s = Rc::new(String::from("hello"));
        // 下面的写法，在编译器看来没有任何区别
        println!("length: {}", s.len());
        println!("length: {}", s.deref().len());
        println!("length: {}", s.deref().deref().len());
        println!("length: {}", (*s).len());
        println!("length: {}", (&*s).len());
        println!("length: {}", (&**s).len());
        // String 实现 Deref trait，是为了让 &String 类型的变量可以在必要的时候
        // 自动转换为 &str 类型。所以 String 类型的变量可以直接调用 str 类型的方法。
        let s = String::from("hello");
        println!("len: {:?}", s.bytes());
        // Vec<T> 类型实现了 Deref trait，目标类型是 [T]，&Vec<T> 类型的变量就可以
        // 在必要的时候自动转换为 [T] 数组切片类型；
        // Rc<T> 类型实现了 Deref trait，目标类型是 T，Rc<T> 类型的变量可以直接调用 T 类型的方法。
    }
    test2();
    // 注意： & * 两个操作符连写跟分开写是不同的含义。
    fn test3() {
        fn joint() {
            let s = Box::new(String::new());
            let p = &*s;
            println!("{} {}", p, s);
        }
        // 编译不通过
        // fn joint() 是可以直接编译通过的，而 fn separate() 是不能编译通过的。
        // 因为编译器很聪明，它看到 &* 这两个操作连在一起的时候，会直接把  &*s 表达式理解为
        // s.deref()，这时候 p 只是 s 的一个借用而已。
        // 如果把这两个操作分开写，会先执行 *s 把内部的数据 move 出来，再对这个临时变量取引用，
        // 这时候 s 已经被移走了，生命周期已经结束。
        // let p = &{*s}; 这种写法也编译不过。这个花括号的存在创建了一个临时的代码块，
        // 在这个临时代码块内部先执行解引用，同样是 move 语义。
        // 从这里也可以看到，默认的“取引用”、“解引用”操作是互补抵消的关系，互为逆运算。
        // 但是，在 Rust 中，只允许自定义“解引用”，不允许自定义“取引用”。
        // 如果类型有自定义“解引用”，那么对它执行“解引用”和“取引用”就不再是互补抵消的结果了。
        // 先 & 后 * 以及先 * 后 & 的结果是不同的。
        /*
        fn separate() {
            let s = Box::new(String::new());
            let tmp = *s;
            let p = &tmp;
            println!("{} {}", p, s);
        }
        */
        joint();
        // separate();
    }
    test3();
}

// 有时候需要手动处理
pub fn fourth() {
    // 如果智能指针中的方法与它内部成员的方法冲突了，编译器会优先调用当前最匹配的类型，
    // 而不会执行自动 deref，在这种情况下，就只能手动 deref 来表达的需求了。
    fn test1() {
        let s = Rc::new(Rc::new(String::from("hello")));
        let s1 = s.clone();
        let ps1 = (*s).clone();
        let pps1 = (**s).clone();
    }
    test1();
    // 一般情况下，在函数调用的时候，编译器会尝试自动解引用。
    // 但在某些情况下，编译器不会为自动插入自动解引用的代码。
    fn test2() {
        let s = String::new();
        // match s.deref() {
        match s.deref() {
            "" => {}
            _ => {}
        }
        // match 后面的变量类型是 &String，匹配分支的变量类型为 &'static str，
        // 这种情况下就需要手动完成类型转换了。手动将 &String 类型转换为 &str 类型的办法如下。
        // 1) match s.deref()。这个方法通过主动调用 deref() 方法达到类型转换的目的。
        // 此时需要引入 Deref trait 方可通过编译，即加上代码 use std::ops::Deref;。
        // 2) match &*s。 可以通过 *s 运算符， 也可以强制调用 deref() 方法，与上面的做法一样。
        // 3) match s.as_ref()。这个方法调用的是标准库中的 std::convert::AsRef 方法，
        // 这个 trait 存在于 prelude 中，无须手工引人即可使用。
        // 4) match s.borrow()。这个方法调用的是标准库中的 std::borrow::Borrow 方法。
        // 要使用它，需要加上代码 use std::borrow::Borrow;。
        // 5) match &s[..]。这个方案也是可以的，这里利用了 String 重载的 Index 操作。
    }
    test2();
}

// 智能指针
// Rust 语言提供了所有权、默认 move 语义，借用、生命周期、内部可变性等基础概念。
// 但这些并不是 Rust 全部的内存管理方式，在这些概念的基础上，还能继续抽象、
// 封装更多的内存管理方式，而且保证内存安全。
pub fn fifth() {
    // 引用计数
    // 到目前为止，接触到的示例中都是一块内存总是只有唯一的一个所有者。
    // 当这个变量绑定自身消亡的时候，这块内存就会被释放。
    // 引用计数智能指针提供了另外一种选择：一块不可变内存可以有多个所有者，
    // 当所有的所有者消亡后，这块内存才会被释放。
    // Rust 中提供的引用计数指针有 std::rc::Rc<T> 类型和 std::sync::Arc<T> 类型。
    // Rc 类型和 Arc 类型的主要区别是：Rc 类型的引用计数是普通整数操作，只能用在单线程中;
    // Arc 类型的引用计数是原子操作，可以用在多线程中。 这一点是通过编译器静态检查保证的。

    // Rc 智能指针的用法
    fn test1() {
        use std::rc::Rc;
        struct SharedValue {
            value: i32
        }
        let shared_value: Rc<SharedValue> = Rc::new(SharedValue { value: 42 });
        let owner1 = shared_value.clone();
        let owner2 = shared_value.clone();
        println!("value: {} {}", owner1.value, owner2.value);
        println!("address: {:p} {:p}", &owner1.value, &owner2.value);
    }
    // owner1 owner2 里面包含的数据不仅值是相同的,而且地址也是相同的。
    // Rc 指针的创建是调用 Rc::new 静态函数，与 Box 类型一致(将来会允许使用 box 关键字创建)。
    // 如果要创建指向同样内存区域的多个 Rc 指针，需要显式调用 clone 函数。
    // 请注意, Rc 指针是没有实现 Copy trait 的。 如果使用直接赋值方式，会执行 move 语义,
    // 导致前一个指针失效，后一个指针开始起作用,而且引用计数值不变。
    // 如果需要创造新的 Rc 指针,必须手工调用 clone() 函数，此时引用计数值才会加 1。
    // 当某个 Rc 指针失效,会导致引用计数值减 1。当引用计数值减到 0 的时候，共享内存空间才会被释放。
    // 它内部包含的数据是“不可变的”,每个 Rc 指针对它指向的内部数据只有读功能，和共享引用 & 一致，
    // 因此，它是安全的。区别在于，共享引用对数据完全没有所有权，不负责内存的释放，
    // Rc 指针会在引用计数值减到 0 的时候释放内存。
    // Rust 里面的 Rc<T> 类型类似于 C++ 里面的 shared_ptr<const T> 类型，且强制不可为空。
    // Rc 类型重载了“解引用”运算符，而且恰好 Target 类型指定的是 T。
    // 这就意味着编译器可以将 Rc<T> 类型在必要的时候自动转换为 &T 类型，
    // 于是它就可以访问 T 的成员变量，调用 T 的成员方法了。因此，它可以被归类为“智能指针”。
    // 《深入浅出 Rust》 看到 176 页。
    test1();
}