use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::mem::size_of;

// Rust 的类型系统
pub fn first() {
    // Rust 的类型系统是一种代数类型系统。数学上严格定义，非常严谨的一套理论
    // 代数类型系统与代数的对比
    // 类型：类比为代数中的变量
    // 类型间的组合关系：类比为代数中数学运算

    // 一个类型所有取值的可能性叫作这个类型的“基数”(Cardinality)
    // unit: 1, ()
    // bool: 2, true/false
    // i32: 2^32 Cardinality(i32) 代表 i32 的基数

    // 把多个类型组合到一起形成新的复合类型，这个新的类型就会有新的基数
    // 如果两个类型的基数是一样的，那么可以说它们携带的信息量是一样的，也可以说它们是“同构”的
    type T1 = [i32; 2];
    type T2 = (i32, i32);
    struct T3(i32, i32);
    struct T4 {
        field1: i32,
        field2: i32,
    }
    // 上面的4个类型，不是同一个类型，无法通用
    // 从数学上看，这四个类型表达出来的信息量是完全一样的，它们都只能装下两个 i32 类型的成员
    // 基数都是 Cardinality(i32) * Cardinality(i32)
    // tuple、struct、tuple struct 拥有同样的内存布局，区别仅仅在于是否给类型及成员起了名字
    // tuple、struct、tuple struct 的基数计算都可以类比为代数中的“求积”运算
    // array 的基数计算可以类比为代数中的乘方运算. 次数为数组的长度
    // enum 的基数计算相当于代数中的“求和”运算
    enum Direction {
        North,
        East,
        South,
        West,
    }
    // Option<T>
    // Cardinality(Option<T>) = 1 + Cardinality(T)
    enum Option<T> {
        None,
        Some(T),
    }
    // 进一步类比
    // 空的 enum -> 0
    // unit、空的结构体 -> 1
    // enum 类型 Cardinality 计算 -> 求和
    // tuple、struct 类型 Cardinality 计算 -> 求积
    // array 类型 Cardinality 计算 -> 乘方
    // ...
}

// Never Type
pub fn second() {
    // 考虑一个类型在机器层面的表示方式：一个类型占用的 bit 位数可以决定它能携带多少信息
    //  2^bits_of(T) = Cardinality(T)
    //  bits_of(T) = log2(Cardinality(T))
    // 空的 enum : 基数 0 -> -∞ bit
    // bool : 基数 2 -> 1 bit
    // unit、空 struct: 基数 1 -> 0 bit。这样的类型实际上是 0 大小的类型
    pub struct HashSet<T, S = RandomState> {
        // 在定义 HashSet 时，只需要将 HashMap 中键值对的“值”指定为 unit 类型即可
        // 所谓的 Hash Set，就是只有 key 没有 value 的 HashMap
        map: HashMap<T, (), S>,
    }
    enum Never {}
    // 在 Rust 类型系统中称无法构造出对象的类型为 never type
    // never type 具备的独特属性
    // 1. 它们在运行时根本不可能存在，因为没有语法可以构造出这样的对象
    // 2. Cardinality(Never) = 0
    // 3. bits_of(Never) = log2(0) = -∞
    // 4. 处理这种类型的代码，根本不可能执行
    // 5. 返回这种类型的代码，根本不可能返回
    // 6. 它们可以被转换为任意类型
    loop {
        let cond = false;
        let x: i32 = if cond { 1 } else { continue; };
    }
    // 在 Rust 中，if-else 是表达式，而且每个分支表达式类型必须一致。
    // 这种有 continue 的情况，类型检查是怎样通过的呢？
    // 如果把 continue 语句的类型指定为 never 类型，那么一切就都顺理成章了。
    // 因为 never 类型可以转换为任意类型，所以，它符合 if 分支类型一致的规定。
    // 它不可能返回，因为执行到 else 分支的时候，接下来不会执行对变量 x 的赋值操作，会进入下一次的循环。
    // 如果这个分支大括号内部 continue 后面还有其他代码，编译器可以很容易地判断出它后面的代码是永远不会执行的死代码。
    // 一切都在类型系统层面得到了统一
    // 综上，never 类型是 Rust 类型系统中不可缺少的一部分。
    // 与 unit 类型类似，一般用空 tuple () 代表 unit 类型
    // Rust 里面也有一个专门的类型来表示 never， 使用感叹号 !。
    // 所谓的 "diverging function(发散函数)" 就是一个返回 never type 的函数。

    // 一个完整的 never type 对于 Rust 还有一些其他的现实意义
    // 场景一：可以使得泛型代码兼寄 diverging function
    fn test1() {
        fn call_fn<T, F: Fn(i32) -> T>(f: F, arg: i32) -> T { f(arg) }
        // exit 返回 !
        // 如果不把 ! 当成一个类型，那么下面这句代码就会出现编译错误
        // 因为只有类型才能替换类型参数
        call_fn(std::process::exit, 0);
    }
    // 场景二：更好的死代码检查
    fn test2() {
        // 如果有完整的 never 类型支持，那么编译器应该可以推理出闭包的返回类型是 ! 而不是 ()，
        // 因此 t.join().unwrap() 会产生一个 ! 类型，
        // 进而编译器可以检查出 println! 永远不可能执行。
        let t = std::thread::spawn(|| panic!("nope"));
        let tt = t.join().unwrap();
        println!("hello");
    }
    // 场景三：可以用更好的方式表达 “不可能出现的情况”
    /* 1.44.0 无法正常编译通过
    trait FromStr {
        type Err;
        fn from_str(s: &str) -> Result<Self, Self::Err>;
    }
    struct T(String);
    impl FromStr for T {
        // 如果某些类型调用 from_str 方法永远不会出错，那么这个 Err 类型可以指定为 !
        type Err = !;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(T(String::from(s)))
        }
    }

    // 对于错误处理可以让 Result 退化成没有错误的情况
    struct T1(String);
    impl FromStr for T1 {
        type Err = !;

        fn from_str(s: &str) -> Result<T1, !> {
            Ok(T1(String::from(s)))
        }
    }
    unsafe  fn test() {
        let r: Result<T1, !> = T1::from_str("hello");
        println!("size of T1: {}", size_of::<T1>());
        println!("size of Result: {}", size_of_val(&r));
        // 根本不需要考虑 Err 的情况，因为 Err 的类型是!，
        // 所以 match 语句中只有 Ok 分支，编译器可以判定其为“完整匹配”。
        let Ok(T1(ref s)) = r;
        println!("{}", s);
    }

    unsafe {
        test();
    }
    */
}

// Option 类型
pub fn third() {
    // Rust 中 的 Option 类型解决了许多编程语言中的“空指针”问题。
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // Option 类型参数可以是常见的指针类型，
    // 也可以是非指针类型，
    // 它的表达能力已经超过了“可空的指针”这种类型
    // Option 的常用操作方法
    // map: Option<U> -> Option<V>
    let maybe_some_string = Some(String::from("Hello World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(12));
    // and_then 方法可以把一系列操作串联起来
    fn sq(x: u32) -> Option<u32> { Some(x * x) }
    fn nope(_: u32) -> Option<u32> { None }
    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    // unwrap 方法可以从 Option<T> 中提取 T
    // 如果为 None, 执行会导致 panic，所以不推荐使用它
    // 推荐使用 expect 方法，它在发生 panic 时，会打印一条字符串
    fn test() {
        println!("size of isize: {}", size_of::<isize>());
        println!("size of Option<isize>: {}", size_of::<Option<isize>>());
        // size of isize: 8
        // size of Option<isize>: 16

        println!("size of &isize: {}", size_of::<&isize>());
        println!("size of Box<isize>: {}", size_of::<Box<isize>>());
        // size of &isize: 8
        // size of Box<isize>: 8

        println!("size of Option<&isize>: {}", size_of::<Option<&isize>>());
        println!("size of Option<Box<isize>>: {}", size_of::<Option<Box<isize>>>());
        // size of Option<&isize>: 8
        // size of Option<Box<isize>>: 8

        // 说明
        // 根据 Rust 的设计，借用指针＆和所有权指针 Box 从语义上来说，都是不可能为“0”的状态。
        // 有些数值是不可能成为这几个指针指向的地址的，它们的取值范围实际上小于 isize 类型的取值范围。
        // 因此 Option<&isize＞和 Option<Box<isize》 类型可以利用这个特点，使用“0”值代表当前状态为“空”。
        // 这意味着，在编译后的机器码层面，使用 Option 类型对指针的包装，与 C/C++ 的指针完全没有区别 。

        println!("size of *const isize: {}", size_of::<* const isize>());
        println!("size of Option<* const isize>: {}", size_of::<Option<* const isize>>());
        // size of *const isize: 8
        // size of Option<* const isize>: 16

        // 说明
        // 对于 *const T 类型，它本身是有可能取值为 0 的，因此这种类型无法执行优化
        // Option<* const T＞的大小就变成了两个指针大小。
    }

    test();

    // Rust Option 类型总结：

    // 1. 如果从逻辑上说，需要一个变量确实是可空的，那么就应该显式标明其类型为 Option<T>，
    // 否则应该直接声明为 T 类型 。
    // 从类型系统的角度来说，这二者有本质区别，不可混为一谈。

    // 2. 不要轻易使用 unwrap 方法。这个方法可能会导致程序发生 panic。
    // 在正式项目中，最好是使用 lint 工具强制禁止调用这个方法。

    // 3. 相对于裸指针，使用 Option 包装的指针类型的执行效率不会降低，这是“零开销抽象”带来的好处。

    // 4. 不必担心这样的设计会导致大量的 match 语句，使得程序可读性变差。
    // 因为 Option<T> 类型有许多方便的成员函数，再配合上闭包功能，实际上在表达能力和可读性
    // 要更胜一筹。
}
