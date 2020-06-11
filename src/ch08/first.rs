use std::collections::hash_map::RandomState;
use std::collections::HashMap;

// Rust 的类型系统
fn first() {
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
fn second() {
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
}
