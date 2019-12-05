#![allow(dead_code)]
/// # 变量声明
pub fn def_variable() {
    let variable: i32 = 100;
    println!("var = {}", variable);
    // let 语句在此处引入了一个模式解构, 不能把 let mut 视为一个组合
    // 而应该将 mut x 视为一个组合
    let mut x = 5;
    x = 10;
    println!("x = {}", x);
    // mut x 是一个"模式", 可以用这种方式同时声明多个变量
    let (mut a, mut b) = (1, 2);
    a = 12;
    b = 13;
    println!("a = {} b = {}", a, b);
    // 在 Rust 中, 一般把声明的局部变量并初始化的语句称为"变量绑定"
    // 强调的是"绑定"的含义, 与 C/C++ 中的"赋值初始化"语句有所区别

    // Rust 中, 每个变量必须被合理初始化之后才能被使用, 使用未初始化变量这样的错误
    // 在 Rust 中是不可能出现的(利用 unsafe 做 hack 除外)
    test(true);
    // 类型没有"默认构造函数", 变量没有"默认值"

    // Rust 里面的下划线是一个特殊的标识符, 在编译器内部它是被特殊处理的
    // 它跟其他标识符有许多重要区别
    // 不能只使用下划线做标识符
    // let _ = "hello";
    // println!("{}", _);
    // 下划线表达的含义是"忽略这个变量绑定, 后面不会再用到了"
}

fn test(condition: bool) {
    let x: i32; // 声明 x, 不必使用 mut 修饰
    if condition {
        x = 1; // 初始化 x, 不需要 x 是 mut 的, 因为这是初始化, 不是修改
        println!("{}", x);
    }
    // 如果条件不满足, x 没有被初始化
    // 但是没关系, 只要这里不使用 x 就没事
}

pub fn shadowing_var() {
    // Rust 允许在同一个代码块中声明同样名字的变量
    // 后面声明的变量会将前面声明的变量"遮蔽"(Shadowing)起来
    let x = "hello";
    println!("x is {}", x);

    let x = 5;
    println!("x is {}", x);
}

// 一个"不可变绑定"依然是一个"变量"
// 虽然没办法通过这个"变量绑定"修改变量的值, 但是重新使用"可变绑定"之后, 还是有机会修改的
// 这样做并不会产生内存安全问题, 因为我们对这块内存拥有完整的所有权, 且此时没有任何其他引用
// 指向这个变量, 对这个变量的修改是完全合法的. Rust 的可变性控制规则与其他语言不一样
// 实际上, 传统编程语言 C/C++ 中也存在类似的功能, 只不过它们只允许嵌套的区域内部的变量出现遮蔽
// 而 Rust 在这方面放得稍微宽一点, 同一个语句块内部声明的变量也可以发生遮蔽
pub fn shadowing_var1() {
    // 变量遮蔽的一种用法
    // 对一个可变数组执行初始化, 希望此时它是可读写的, 但是初始化完成后, 我们希望它是只读的
    let mut v = Vec::new(); // v 必须是 mut 修饰, 因为我们需要对它写入数据
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let v = v; // 从这里往下, v 成了只读变量, 可读写变量 v 已经被遮蔽, 无法再访问
    for i in &v {
        println!("{}", i);
    }

    // 反过来, 如果一个变量是不可变的, 我们也可以通过变量遮蔽创建一个新的、可变的同名变量
    let v = Vec::new();
    let mut v = v;
    v.push(1);
    println!("{:?}", v);
}

/// # 类型推导
/// Rust 的类型推导功能是比较强大的
/// 它不仅可以从变量声明的当前语句中获取信息进行推导, 而且还能通过上下文信息进行推导
pub fn infer_type() {
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);
    // 到后面调用了 push 函数, 通过 elem 变量的类型
    // 编译器可以推导出 vec 的实际类型是 Vec<u8>

    println!("{:?}", vec);
}

pub fn infer_type2() {
    let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];

    // players 是动态数组, 内部成员的类型没有指定, 交给编译器自动推导
    let players: Vec<_> = player_scores
        .iter()
        .map(|&(player, _scores)| player)
        .collect();

    println!("{:?}", players);
    // 自动类型推导和"动态类型系统"是两码事
    // Rust 依然是静态类型的. 一个变量的类型必须在编译阶段确定, 且无法更改
    // 只是某些时候不需要在源码中显式写出来而已. 这只是编译器给我们提供的一个辅助工具
    // Rust 只允许"局部变量/全局变量"实现类型推导, 而函数签名等场景下是不允许的, 这是故意这样设计的
    // 这是因为局部变量只有局部的影响, 全局变量必须当场初始化而函数签名具有全局性影响
    // 函数签名如果使用自动类型推导, 可能导致某个调用的地方使用方式发生变化,
    // 它的参数、返回值类型就发生了变化, 进而导致远处另一个地方的编译错误
}

type Age = u32;
type Double<T> = (T, Vec<T>); // Double<i32> <=> (i32, Vec<i32>)
fn grow(age: Age, year: u32) -> Age {
    age + year
}

pub fn type_alias() {
    let x: Age = 20;
    println!("20 years later: {}", grow(x, 20))
}

/// 静态变量
/// Rust 中可以用 static 关键字声明静态变量
/// 与 let 语句一样, static 语句同样也是一个模式匹配.
/// 与 let 语句不同的是, 用 static 声明的变量的生命周期是整个程序, 从启动到退出
/// static 变量的生命周期永远是 'static, 它占用的内存空间也不会在执行过程中回收
/// 这也是 Rust 中唯一的声明全局变量的方法
/// 由于 Rust 非常注重内存安全, 因此全局变量的使用有许多限制
/// 这些限制都是为了防止程序员写出不安全的代码:
/// 1. 全局变量必须在声明的时候马上初始化
/// 2. 全局变量的初始化必须是编译期可确定的常量, 不能包括执行期才能确定的表达式、语句和函数调用
/// 3. 带有 mut 修饰的全局变量, 在使用的时候必须使用 unsafe 关键字
static GLOBAL: i32 = 0;
pub fn use_global() {
    println!("{}", GLOBAL);
    // 局部变量声明, 可以留待后面初始化, 只要保证使用前已经初始化即可
    let x;
    let y = 1_i32;
    x = 2_i32;
    println!("{} {}", x, y);
    // 全局变量必须声明的时候初始化, 因为全局变量可以写到函数外面, 被任意一个函数使用
    static G1: i32 = 3;
    println!("{}", G1);
    // 可变全局变量无论读写都必须用 unsafe 修饰
    static mut G2: i32 = 4;
    unsafe {
        G2 = 5;
        println!("{}", G2);
    }
    // 全局变量的内存不是分配在当前函数栈上, 函数退出的时候, 并不会销毁全局变量占用的内存空间,
    // 程序退出才会回收

    // Rust 禁止在声明 static 变量的时候调用普通函数, 或者利用语句块调用其他非 const 代码
    // static vec: Vec<i32> = { let mut v = Vec::new(); v.push(1); v };
    // 调用 const fn 是允许的
    use std::sync::atomic::AtomicBool;
    static FLAG: AtomicBool = AtomicBool::new(true);
    // const fn 是编译期执行的
    // Rust 不允许用户在 main 函数之前或者之后执行自己的代码
    // 所以, 比较复杂的 static 变量的初始化一般需要使用 lazy 方式, 在第一次使用的时候初始化
    // 在 Rust 中, 如果用户需要使用比较复杂的全局变量初始化, 推荐使用 lazy_ static 库
}

pub fn const_var() {
    // 在 Rust 中可以用 const 关键字做声明
    const GLOBAL: i32 = 0;
    // 使用 const 声明的是常量, 而不是变量
    // 因此一定不允许使用 mut 关键字修饰这个变量绑定
    // 常量的初始化表达式也一定要是一个编译期常量, 不能是运行期的值
    // 它与 static 变量的最大区别在于: 编译器并不一定会给 const 常量
    // 分配内存空间, 在编译过程中, 它很可能会被内联优化
    // 用户千万不要用 hack 的方式, 通过 unsafe 代码去修改常量的值, 这么做是没有意义的
    // 以 const 声明一个常量, 也不具备类似 let 语句的模式匹配功能
    println!("{}", GLOBAL);
}
