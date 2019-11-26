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
}

// 类型递归定义
