// NLL(Non-Lexical-Lifetime)
// Rust 防范“内存不安全”代码的原则极其清晰明了。
// 如果对同一块内存存在多个引用，就不要试图对这块内存做修改；
// 如果需要对一块内存做修改，就不要同时保留多个引用。
// 只要保证了这个原则，就可以保证内存安全。
// 这个原则是 Rust 的立身之本、生命之基、活力之源。

// 这个原则的初始实现版本有一个主要问题，那就是它让借用指针的生命周期规则
// 与普通对象的生命周期规则一样，是按作用域来确定的。
// 所有的变量、借用的生命周期就是从它的声明开始到当前整个语句块结束。
// 这个设计被称为 Lexical Lifetime，生命周期是严格和词法中的作用域范围绑定的。
// 这个策略实现起来非常简单，但它可能过于保守了，某些情况下借用的范围被过度拉长了，
// 以至于某些实质上是安全的代码也被阻止了。在某些场景下，限制了程序员的发挥。
// 因此，Rust 核心组决定引人 Non Lexical Lifetime，
// 用更精细的手段调节借用真正起作用的范围。这就是 NLL。

// NLL 希望解决的问题
pub fn first() {
    fn test1() {
        fn foo() -> Vec<char> {
            let mut data = vec!['a', 'b', 'c'];
            capitalize(&mut data[..]);
            data.push('d');
            data.push('e');
            data.push('f');
            data
        }
        fn capitalize(data: &mut [char]) {
            for c in data {
                c.make_ascii_uppercase();
            }
        }
        let v = foo();
        println!("{:?}", v);
    }
    test1();
}
