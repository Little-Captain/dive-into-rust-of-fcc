#[allow(dead_code)]

/// 基本数据类型
/// bool
pub fn learn_bool() {
    println!("------------------------");

    let x = true;
    let y: bool = !x; // 取反

    let z = x && y; // 逻辑与，短路
    println!("{}", z);

    let z = x || y; // 逻辑或，短路
    println!("{}", z);

    let z = x & y; // 按位与，不带短路
    println!("{}", z);
    let z = x | y; // 按位或，不带短路
    println!("{}", z);

    let z = x ^ y; // 按位异或，不带短路
    println!("{}", z);

    let z = 1 < 2;
    println!("{}", z);
}

pub fn learn_char() {
    println!("------------------------");

    // 字符类型由 char 表示。
    // 它可以描述任何一个符合 unicode 标准的字符值。在代码中，单个的字符字面量用单引号包围
    // 1 个 char 占用 4 个字节
    let love = '❤';
    println!("{}", love);
    let c1 = '\n';
    let c2 = '\x7f';
    let c3 = '\u{7fff}';
    println!("{} {} {}", c1, c2, c3);
    // 对于 ASCII 字符其实只需占用一个字节的空间，
    // 因此 Rust 提供了单字节字符字面量来表示 ASCII 字符
    // 可以使用一个字母 b 在字符或者字符串前面，代表这个字面量存储在 u8 类型数组中
    let x: u8 = 1;
    let y: u8 = b'A';
    let s: &[u8; 5] = b"hello";
    let r: &[u8; 14] = br#"hello \n world"#;
    println!("{} {} {:?} {:?}", x, y, s, r);
}

/// 整数类型
/// 各种整数类型之间的主要区分特征是：有符号/无符号，占据空间大小
/// 所谓有符号／无符号，指的是如何理解内存空间中的bit 表达的含义
/// 如果一个变量是有符号类型，那么它的最高位的那一个bit 就是“符号位”，
/// 表示该数为正值还是负值。如果一个变量是无符号类型，那么它的最高位和
/// 其他位一样，表示该数的大小。
/// 需要特别关注的是 isize 和 usize 类型。它们占据的空间是不定的，
/// 与指针占据的空间一致，与所在的平台相关。如果是 32 位系统上，则是 32 位大小；
/// 如果是 64 位系统上，则是 64 位大小。在 C++ 中与它们相对应的类似类型是 int_ptr
/// 和 uint_ptr。 Rust 的这一策略与 C 语言不同，C 语言标准中对许多类型的大小并没有做强制
/// 规定，比如 int 、long 、double 等类型，在不同平台上都可能是不同的大小，这给许多程
/// 序员带来了不必要的麻烦。相反，在语言标准中规定好各个类型的大小，让编译器针对不同
/// 平台做适配，生成不同的代码，是更合理的选择。
pub fn integer_type() {
    println!("------------------------");

    // 默认为十进制
    // 0x 16
    // 0o 8
    // 0b 2
    let var1: i32 = 32;
    let var2: i32 = 0xFF;
    let var3: i32 = 0o55;
    let var4: i32 = 0b1001;
    println!("{} {} {} {}", var1, var2, var3, var4);
    // 在所有的数字字面量中，可以在任意地方添加任意的下划线，以方便阅读
    let var5 = 0x_1234_ABCD;
    // 字面量后面可以跟后缀，可代表该数字的具体类型，从而省略掉显示类型标记
    let var6 = 123usize;
    let var7 = 0x_ff_u8;
    let var8 = 32;
    println!("{} {} {} {}", var5, var6, var7, var8);
    // 在 Rust 中，我们可以为任何一个类型添加方法，整型也不例外
    // 比如在标准库中，整数类型有一个方法是 pow ，它可以计算 n 次幕
    let x: i32 = 9;
    println!("9 power 3 = {}", x.pow(3));
    // 甚至可以不使用变量，直接对整型字面量调用函数
    println!("9 power 3 = {}", 9_i32.pow(3));
}

pub fn integer_overflow() {
    // 整数溢出
    // Rust 在这个问题上选择的处理方式为：
    // 默认情况下，在 debug 模式下编译器会自动插入整数溢出检查，
    // 一旦发生溢出，则会引发 panic；在 release 模式下，不检查整数溢出，
    // 而是采用自动舍弃高位的方式
    // 加法运算，有溢出风险
    let m: i8 = 123;
    let n: i8 = 113;
    println!("{}", m + n);
    // 编译不通过
    // println!("{}", 123_i8 + 113_i8);
    // 如果在某些场景下，用户确实需要更精细地自主控制整数溢出的行为，可以调用标准库中的
    // checked_*、saturating_* 和 wrapping_* 系列函数
    let i = 100_i8;
    println!("checked {:?}", i.checked_add(i));
    println!("saturating {:?}", i.saturating_add(i));
    println!("wrapping {:?}", i.wrapping_add(i));
    // checke_* 系列函数返回的类型是 Option<_>，当出现溢出的时候，返回值是 None;
    // saturating_* 系列函数返回类型是整数，如果溢出，则给出该类型可表示范围的"最大/最小"值；
    // wrapping_* 系列函数则是直接抛弃已经溢出的最高位，将剩下的部分返回
    // 在对安全性要求非常高的情况下，强烈建议用户尽量使用这几个方法替代默认的算术运算符来做数学运算

    // 在很多情况下，整数溢出应该被处理为截断，即丢弃最高位
    // 标准库还提供了一个叫作 std::num::Wrapping<T> 的类型。
    // 它重载了基本的运算符，可以被当成仅普通整数使用。
    // 凡是被它包裹起来的整数，任何时候出现溢出都是截断行为。
    use std::num::Wrapping;
    let big = Wrapping(std::u32::MAX);
    let sum = big + Wrapping(1_u32);
    println!("{}", sum);
}
