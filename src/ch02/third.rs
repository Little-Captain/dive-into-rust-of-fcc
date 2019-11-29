#[allow(dead_code)]

/// 复合数据类型
// tuple
pub fn learn_tuple() {
    let a = (1i32, false);
    let b = ("a", (1i32, 2i32));
    println!("{:?}", a);
    println!("{:?}", b);
    let a = (0,);
    let b = (0);
    println!("{:?}", a);
    println!("{:?}", b);
    // 访问元组内部元素有两种方法:
    // 一种: “模式匹配”(pattern destructuring)
    // 一种: “数字索引”
    let p = (1i32, 2i32);
    let (a, b) = p;
    let x = p.0;
    let y = p.1;
    println!("{:?}", p);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", x);
    println!("{:?}", y);
    // 元组内部也可以一个元素都没有。这个类型单独有一个名字，叫 unit (单元类型)
    // 空元组和空结构体 struct Foo 一样，都是占用 0 内存空间
    // 与 C++ 中的空类型不同，Rust 中存在实打实的 0 大小的类型
    let empty: () = ();
    println!("{:?}", empty);
    println!("size of i8 {} byte(s)", std::mem::size_of::<i8>());
    println!("size of char {} byte(s)", std::mem::size_of::<char>());
    println!("size of '()' {} byte(s)", std::mem::size_of::<()>());
}

/// 结构体(struct)与元组类似， 也可以把多个类型组合到一起，作为新的类型
/// 每个元素都有自己的名字
// struct
pub fn learn_struct() {
    struct Point {
        x: i32,
        y: i32,
    }
    // struct 类型的初始化语法类似于 json 的语法，使用“成员一冒号一值”的格式。
    let p = Point { x: 0, y: 0 };
    println!("Point is at {} {}", p.x, p.y);
    // Rust 允许 struct 类型的初始化使用一种简化的写法。如果有局部变量名字和
    // 成员变量名字恰好一致，那么可以省略掉重复的冒号初始化
    let x = 10;
    let y = 20;
    let p = Point { x, y };
    println!("Point is at {} {}", p.x, p.y);
    // 访问结构体内部的元素，也是使用“点”加变量名的方式。当然，我们也可以使用“模式匹配”功能
    let Point { x: px, y: py } = p;
    println!("Point is at {} {}", px, py);
    let Point { x, y } = p;
    println!("Point is at {} {}", x, y);
    // Rust 设计了一个语法糖，允许用一种简化的语法赋值使用另外一个 struct 的部分成员
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    fn default() -> Point3D {
        Point3D { x: 0, y: 0, z: 0 }
    }

    // 可以使用default （） 函数初始化其他的元素
    // ..expr 这样的语法，只能放在初始化表达式中，所有成员的最后最多只能有一个
    let origin = Point3D { x: 5, ..default() };
    let point = Point3D {
        z: 1,
        x: 2,
        ..origin
    };
    println!("{} {} {}", origin.x, origin.y, origin.z);
    println!("{} {} {}", point.x, point.y, point.z);
    // 以下三种都可以， 内部可以没有成员
    struct Fool;
    struct Foo2();
    struct Foo3 {}
}

/// Rust 有一种数据类型叫作 tuple struct，它就像是 tuple 和 struct 的混合
/// 区别在于 tuple struct 有名字，而它们的成员没有名字
// tuple struct
pub fn learn_tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // 可以被想象成这样的结构体
    /*
    struct Color {
        0： i32,
        1: i32,
        2: i32,
    }
    struct Point {
        0: i32,
        1: i32,
        2: i32,
    }
    */
    // tuple struct `tuple struct`
    // 它们除了在取名上有这些区别外，没有其他区别。
    // 它们有一致的内存对齐策略、一致的占用空间规则，
    // 也有类似的语法
    // define struct
    struct T1 {
        v: i32,
    }
    // define tuple struct
    struct T2(i32);
    fn test() {
        let v1 = T1 { v: 1 };
        let v2 = T2(1);
        let v3 = T2 { 0: 1 };

        let i1 = v1.v;
        let i2 = v2.0;
        let i3 = v3.0;

        println!("{} {} {}", i1, i2, i3);
    }

    test();

    // tuple struct 有一个特别有用的场景，那就是当它只包含一个元素的时候，
    // 就是所谓的 newtype idiom。因为它实际上让我们非常方便地在一个类型的
    // 基础上创建了一个新的类型。
    struct Inches(i32);
    fn f1(value: Inches) {}
    fn f2(value: i32) {}

    let v: i32 = 0;
    // f1(v);
    f2(v);

    fn type_alias() {
        type I = i32;

        fn f1(v: I) {
            println!("v I {}", v);
        }
        fn f2(v: i32) {
            println!("v i32 {}", v);
        }

        let v: i32 = 0;
        f1(v);
        f2(v);
    }

    type_alias();
}

/// 如果说 tuple、struct、tuple struct 在 Rust 中代表的是多个类型的"与"关系
/// 那么 enum 类型在 Rust 中代表的就是多个类型的"或"关系
// enum
pub fn learn_enum() {
    enum Number {
        Int(i32),
        Float(f32),
    }
    // Rust 的 enum 中的每个元素的定义语法与 struct 的定义语法类似
    // 可以像空结构体一样，不指定它的类型；也可以像 tuple struct 一样，
    // 用圆括号加无名成员；还可以像正常结构体一样，用大括号加带名字的成员。
    // 用 enum 把这些类型包含到一起之后，就组成了一个新的类型。
    // 要使用 enum ，一般要用到"模式匹配"
    fn read_num(num: &Number) {
        match num {
            // 如果匹配了 Number::Int，那么 value 的类型就是 i32
            &Number::Int(value) => println!("Integer {}", value),
            // 如果匹配了 Number::Float，那么 value 的类型就是 f32
            &Number::Float(value) => println!("Integer {}", value),
        }
    }
    let n: Number = Number::Int(10);
    read_num(&n);
    // Rust 的 enum 与 C/C++ 的 enum 和 union 都不一样
    // 它是一种更安全的类型，可以被称为"tagged union"

    // 输出内存空间大小
    println!("Size of Number: {}", std::mem::size_of::<Number>());
    println!("Size of i32: {}", std::mem::size_of::<i32>());
    println!("Size of f32: {}", std::mem::size_of::<f32>());

    // Rust 里面也支持 union 类型，这个类型与 C 语言中的 union 完全一致。
    // 但在 Rust 里面，读取它内部的值被认为是 unsafe 行为，一般情况下我们
    // 不使用这种类型。它存在的主要目的是为了方便与 C 语言进行交互。

    // 在 Rust 中， enum 和 struct 为内部成员创建了新的名字空间。
    // 如果要访问内部成员，可以使用 :: 符号。因此，不同的 enum 中
    // 重名的元素也不会互相冲突。例如在下面的程序中，两个枚举内部都
    // 有 Move 这个成员，但是它们不会有冲突。
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    let x: Message = Message::Move { x: 3, y: 4 };

    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    enum Animal {
        dog = 1,
        cat = 200,
        tiger,
    }
    let x = Animal::tiger as isize;
    println!("{}", x);

    // Rust 标准库中有一个极其常用的 enum 类型 Option<T>
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // 由于它实在是太常用，标准库将 Option 以及它的成员 Some 、None 都加入到了 Prelude 中，
    // 用户甚至不需要 use 语句声明就可以直接使用。它表示的含义是"要么存在、要么不存在"

    // Rust 的 enum 实际上是一种代数类型系统(Algebraic Data Type, ADT)
    // enum 内部的 variant 只是一个名字而己，恰好我们还可以将这个名字作为
    // `类型构造器`使用。意思是说，我们可以把 enum 内部的 variant 当成一个`函数`使用
    let arr = [1, 2, 3, 4, 5];
    let v: Vec<Option<&i32>> = arr.iter().map(Some).collect();
    println!("{:?}", v);
    // 证明 Some 确实是一个函数类型，
    // 我们把 Some 初始化给一个 unit 变量，产生一个编译错误
    // let _: () = Some;
    // enum 内部的 variant 的类型确实是函数类型
}

// 类型递归定义
pub fn learn_recursive() {
    // Rust 里面的复合数据类型是允许递归定义的
    // struct 里面嵌套同样的 struct 类型，直接嵌套是不行的
    struct Recursive {
        data: i32,
        // rec: Recursive,
        rec: Box<Recursive>, // 把产生了递归的那个成员类型改为了指针
    }
}
