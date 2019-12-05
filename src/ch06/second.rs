#![allow(dead_code)]

// 字符串
// Rust 的字符串显得有点复杂，主要是跟所有权有关
// Rust 的字符串涉及两种类型，一种是 &str，另外一种是 String

// &str
// str 是 Rust 的内置类型
// &str 是对 str 的借用
// Rust 的字符串内部默认是使用 utf-8 编码格式的
// 内置的 char 类型是 4 字节长度的，存储的内容是 Unicode Scalar Value
// 所以，Rust 里面的字符串不能视为 char 类型的数组，而更接近 u8 类型的数组
// 实际上 str 类型有一种方法：fn as_ptr(&self) -> *const u8
// 它内部无须做任何计算，只需做一个强制类型转换即可
// self as *const str as *const u8
// 设计缺点
// 如果我们要找一个字符串 s 内部的第 n 个字符，不能直接通过 s[n] 得到
// 可以通过这条语句实现: s.chars().nth(n)
// 它的时间复杂度是 O(n)，因为 utf-8 是变长编码，如果我们不从头开始过一遍
// 根本不知道第 n 个字符的地址在什么地方

// [T] 是 DST 类型，对应的 str 是 DST 类型
// &[T] 是数组切片类型，对应的 &str 是字符串切片类型
pub fn learn_str1() {
    let greeting: &str = "Hello";
    let substr: &str = &greeting[2..];
    println!("{}", substr);
    println!("Size of pointer: {}", std::mem::size_of::<*const ()>());
    println!("Size of &str   : {}", std::mem::size_of::<&str>());
    // 它内部实际上包含了一个指向字符串片段头部的指针和一个长度
    // 所以，它跟 C/C++ 的字符串不同： C/C++ 里面的字符串以 '\0' 结尾
    // 而 Rust 的字符串是可以中间包含 '\0' 字符的
}

// String
// String 类型跟 &str 类型的主要区别是它有管理内存空间的权力
// &str 类型是对一块字符串区间的借用，它对所指向的内存空间没有
// 所有权，哪怕 &mut str 也一样
pub fn learn_string1() {
    let mut s = String::from("hello");
    s.push(' ');
    s.push_str("world.");
    println!("{}", s);
}
// 因为 String 类型在堆上动态申请了一块内存空间，它有权对这块内存空间进行扩容，
// 内部实现类似于 std::Vec<u8> 类型。可以把这个类型作为容纳字符串的容器使用
// 这个类型实现了 Deref<Target=str> 的 trait
// 在很多情况下，&String 类型可以被编译器自动转换为 &str 类型
pub fn learn_string2() {
    fn capitalize(substr: &mut str) {
        substr.make_ascii_uppercase();
    }

    let mut s = String::from("hello world");
    capitalize(&mut s);
    println!("{}", s);
}
// Rust 的内存管理方式和 C++ 有很大的相似之处
// 如果用 C++ 来对比，Rust 的 String 类型类似于 std::string，
// 而 Rust 的 &str 类型类似于 std::string_view