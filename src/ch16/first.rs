// 解引用
// “解引用”(Deref)是“取引用”(Ref)的反操作。
// 取引用，有 &、&mut 等操作符，对应的，解引用，有 * 操作符，跟 C 语言是一样的。

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
pub  fn second() {
    fn test1() {

    }
    test1();
}
