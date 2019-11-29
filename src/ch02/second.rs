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

pub fn float_type() {
    let f1 = 123.0f64;
    let f2 = 0.1f64;
    let f3 = 0.1f32;
    let f4 = 12E+99_f64;
    let f5: f64 = 2.;
    println!("{} {} {} {} {}", f1, f2, f3, f4, f5);
    // 与整数类型相比， Rust 的浮点数类型相对复杂得多
    // 浮点数的麻烦之处在于：它不仅可以表达正常的数值，还可以表达不正常的数值。
    // 在标准库中，有一个 std::num::FpCategory 枚举，表示了浮点数可能的状态：
    // enum FpCategory {
    //     Nan,
    //     Infinite,
    //     Zero,
    //     Subnormal,
    //     Normal,
    // }
    // 演示 Subnormal
    // 变量 small 初始化为一个非常小的浮点数
    let mut small = std::f32::EPSILON;
    // 不断循环， 让 small 越来越趋近于 0 ， 直到最后等于 0 的状态
    while small > 0.0 {
        small = small / 2.0;
        println!("{} {:?}", small, small.classify());
    }
    // 演示 Infinite Nan
    let x = 1.0f32 / 0.0;
    let y = 0.0f32 / 0.0;
    println!("{} {}", x, y);
    // inf 数学运算
    let inf = std::f32::INFINITY;
    println!("{} {} {}", inf * 0.0, 1.0 / inf, inf / inf);
    // NaN 这个特殊值有个特殊的麻烦，主要问题还在于它不具备"全序"的特点
    let nan = std::f32::NAN;
    println!("{} {} {}", nan < nan, nan > nan, nan == nan)
    // 一个数字可以不等于自己
    // 因为 NaN 的存在，浮点数是不具备"全序关系"(total order)的
    // 全序/偏序 Ord/PartialOrd
}

pub fn point_type() {
    // 无 GC 的编程语言，如 C 、C++ 以及 Rust，对数据的组织操作有更多的自由度，具体表现为：
    // 1. 同一个类型，某些时候可以指定它在栈上，某些时候可以指定它在堆上。
    //    内存分配方式可以取决于使用方式，与类型本身无关。
    // 2. 既可以直接访问数据，也可以通过指针间接访问数据。可以针对任何一个对象取得指向它的指针。
    // 3. 既可以在复合数据类型中直接嵌入别的类型的实体，也可以使用指针，间接指向别的类型。
    // 4. 甚至可能在复合数据类型末尾嵌入不定长数据构造出不定长的复合数据类型。
    // Rust 里面也有指针类型，而且不止一种指针类型。
    // 1. Box<T>        指向类型 T 的、具有所有权的指针，有权释放内存
    // 2. &T            指向类型 T 的借用指针，也称为引用，无权释放内存，无权写数据
    // 3. &mut T        指向类型 T 的 mut 型借用指针，无权释放内存，有权写数据
    // 4. *const T      指向类型 T 的只读裸指针，没有生命周期信息，无权写数据
    // 5. *mut T        指向类型 T 的可读写裸指针，没有生命周期信息，有权写数据
    // 在标准库中还有一种封装起来的可以当作指针使用的类型，叫"智能指针"(smart pointer)。
    // 1. Rc<T>         指向类型 T 的引用计数指针，共享所有权，线程不安全
    // 2. Arc<T>        指向类型 T 的原子型引用计数指针，共享所有权，线程安全
    // 3. Cow<'a, T>    Clone-on-write，写时复制指针。可能是借用指针也可能是具有所有权的指针
}

pub fn type_case() {
    // 类型转换
    // Rust 对不同类型之间的转换控制得非常严格
    // Rust 提供了一个关键字 as
    let var1: i8 = 41;
    let var2: i16 = var1 as i16;
    // as 关键字也不是随便可以用的，它只允许编译器认为合理的类型转换。任意类型转换是不允许的
    let a = "some string";
    // let b = a as u32; // 编译错误
    // 有些时候， 甚至需要连续写多个as 才能转成功
    let i = 42;
    // 先转为＊ c onst i32 ， 再转为＊ mut i32
    let p = &i as *const i32 as *mut i32;
    println!("{:p}", p);
    // 如果需要更复杂的类型转换，一般是使用标准库的 From Into 等 trait
    // e as U
    // e: 表达式; U: 目标类型
    // Type of e                           U
    // Integer of Float type               Integer or Float type
    // C-like enum                         Integer type
    // bool or char                        Integer type
    // u8                                  char
    // *T                                  *V where V: Sized *
    // *T where T: Sized                   Numeric type
    // Integer type                        *V where V: Sized
    // &[T; n]                             *const T
    // Function pointer                    *V where T: Sized
    // Function pointer                    Integer
}
