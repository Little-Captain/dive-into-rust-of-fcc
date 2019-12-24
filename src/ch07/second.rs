#![allow(dead_code)]

// match

pub fn first() {
    // #[non_exhaustive]
    enum Direction {
        East,
        West,
        South,
        North,
    }

    fn print(x: Direction) {
        // 当一个类型有多种取值可能性的时候，特别适合使用 match 表达式
        match x {
            Direction::East => {
                println!("East");
            }
            Direction::West => {
                println!("West");
            }
            Direction::South => {
                println!("South");
            }
            Direction::North => {
                println!("North");
            } // _ => {
              //     println!("North");
              // }
        }
    }

    let x = Direction::East;
    print(x);
}

// exhaustive
// exhaustive 是 Rust 模式匹配的重要特点
// 有些时候我们不想把每种情况一一列出，可以用一个下划线来
// 表达"除了列出来的那些之外的其他情况"
// 上游库作者可以用一个叫作"non_exhaustive"的 attribute 来标记一个 enum
// 或者 struct，这样在另外一个项目中使用这个类型的时候，无论如何都没办法在
// match 表达式中通过列举所有的成员实现完整匹配，必须使用下划线才能完成编译
// 这样，以后上游库里面为这个类型添加新成员的时候，就不会导致下游项目中的编译
// 错误了因为它已经存在一个默认分支匹配其他情况

// 下划线
// 下划线还能用在模式匹配的各种地方，用来表示一个占位符
// 虽然匹配到了但是忽略它的值的情况
// 下划线更像是一个"关键字"，而不是普通的"标识符"(identifier)，
// 把它当成普通标识符使用是会有问题的
// 如果把下划线后面跟上字母、数字或者下划线，那么它就可以成为一个正常的标识符了
// 比如，连续两个下划线 `__`，就是一个合法的、正常的"标识符"
// let _ = x; 和 let _y = x; 具有不一样的意义。后面的“析构函数”部分还会继续强调
// 如果变量 x 是非Copy 类型，let _ ＝ x; 的意思是“忽略绑定”，此时会直接调用 x 的
// 析构函数，不能在后面使用下划线`_`读取这个变量的内容；而 let _y = x; 的意思是
// “所有权转移”，_y 是一个正常的变量名，x 的所有权转移到了 _y 上，y 在后面可以使用
// 下划线在 Rust 里面用处很多
// 1. 在 match 表达式中表示"其他分支"
// 2. 在模式中作为占位符
// 3. 在类型中做占位符
// 4. 在整数和小数字面量中做连接符
// 5. ...
pub fn second() {
    struct P(f32, f32, f32);

    fn calc(arg: P) -> f32 {
        let P(x, _, y) = arg;
        x * x + y * y
    }

    let t = P(1.0, 2.0, 3.0);
    println!("{}", calc(t));

    // 函数参数本身就具备"模式解构"功能，我们可以直接在参数中完成解构
    fn calc1(P(x, _, y): P) -> f32 {
        x * x + y * y
    }

    let t = P(1.0, 2.0, 3.0);
    println!("{}", calc1(t));
}

// 除了下划线可以在模式中作为“占位符”，还有两个点`..`也可以在模式中作为“占位符”使用
// `_` : 省略一个元素
// `..` : 省略多个元素
pub fn third() {
    let x = (1, 2, 3);
    // let (a, ..) = x;
    let (a, _, _) = x;
    println!("{}", a);

    let x = (1, 2, 3, 4, 5);
    let (a, .., b) = x;
    println!("{} {}", a, b);
}

// match 也是表达式
// 跟 Rust 中其他流程控制语法一样，match 语法结构也同样可以是表达式的一部分
pub fn fourth() {
    enum Direction {
        East,
        West,
        South,
        North,
    }

    fn direction_to_int(x: Direction) -> i32 {
        match x {
            Direction::East => 10,
            Direction::West => 20,
            Direction::South => 30,
            Direction::North => 40,
        }
    }

    // match 表达式的每个分支可以是表达式，它们要么用大括号包起来，要么用逗号分开
    // 每个分支都必须具备同样的类型

    let x = Direction::East;
    let s = direction_to_int(x);
    println!("{}", s);

    // match 除了匹配“结构”，还可以匹配“值”
    fn category(x: i32) {
        match x {
            -1 => println!("negative"),
            0 => println!("zero"),
            1 => println!("positive"),
            _ => println!("error"),
        }
    }
    let x = 1;
    category(x);

    // 使用 | 来匹配多个条件
    fn category1(x: i32) {
        match x {
            -1 | 1 => println!("true"),
            0 => println!("false"),
            _ => println!("error"),
        }
    }
    let x = 1;
    category1(x);

    // 使用范围作为匹配条件，使用`..`表示一个前闭后开区间范围，使用`..=`表示一个闭区间范围
    let x = 'X';
    match x {
        'a'..='z' => println!("lowercase"),
        'A'..='Z' => println!("lowercase"),
        _ => println!("something else"),
    }
}

// Guards
// 可以使用 if 作为“匹配看守”(match guards)
// 当匹配成功且符合 if 条件，才执行后面的语句
pub fn fifth() {
    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    // 在对变量的“值”进行匹配的时候，编译器依然会保证“完整无遗漏”检查
    // 但是这个检查目前做得并不是很完美，某些情况下会发生误报的情况
    // 因为毕竟编译器内部并没有一个完整的数学解算功能

    let x = 10;

    match x {
        i if i > 5 => println!("bigger than five"),
        i if i <= 5 => println!("small or equal to five"),
        // 从 if 条件中可以看到，实际上我们已经覆盖了所有情况，可惜还是出现了编译错误
        // 编译器目前还无法完美地处理这样的情况
        _ => unreachable!(),
    }

    // 编译器会保证 match 的所有分支合起来一定覆盖了目标的所有可能取值范围
    // 但是并不会保证各个分支是否会有重叠的情况(毕竟编译器不想做成一个完整的数学解算器)
    // 如果不同分支覆盖范围出现了重叠，各个分支之间的先后顺序就有影响了

    let x = -1;

    // 如果我们进行匹配的值同时符合好几条分支，那么总会执行第一条匹配成功的分支，忽略其他分支
    match x {
        i if i * i < 1000 => println!("case 3"),
        i if i < 10 => println!("case 2"),
        i if i < 0 => println!("case 1"),
        _ => println!("default case"),
    }
}

// 变量绑定
// 可以使用 @ 符号绑定变量。@ 符号前面是新声明的变量，后面是需要匹配的模式
pub fn sixth() {
    let x = 1;

    match x {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
        match v {
            // r 绑定到的是第一层 Option 内部，r 的类型是 Option<i32>
            // 与这种写法含义不一样：Some(Some(r)) if (1..10).contains(r)
            Some(r @ Some(1..=9)) => r,
            _ => None,
        }
    }

    let x = Some(Some(5));
    println!("{:?}", deep_match(x));

    let y = Some(Some(105));
    println!("{:?}", deep_match(y));

    // 如果在使用自的同时使用｜，需要保证在每个条件上都绑定这个名字
    let x = 5;
    match x {
        e @ 1..=4 | e @ 8..=9 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

// ref 和 mut
// 如果我们需要绑定的是被匹配对象的引用，则可以使用 ref 关键字

pub fn seventh() {
    let x = 5_i32;

    match x {
        // 之所以在某些时候需要使用 ref，是因为模式匹配的时候有可能发生变量的所有权转移
        // 使用 ref 就是为了避免出现所有权转移
        ref r => println!("Got a reference to {}", r), // 此时 r 的类型是 `&i32`
    }

    fn type_id(_: ()) {}
    // ref 和 & 的关系
    // ref 是“模式”的一部分，它只能出现在赋值号左边
    // ref 关键字是“模式”的一部分，不能修饰赋值号右边的值
    // & 是借用运算符是表达式的一部分，它只能出现在赋值号右边
    let x = 5_i32; // i32
    let x = &5_i32; // &i32
    let ref x = 5_i32; // &i32
    // type_id(x);
    let ref x = &5_i32; // &&i32
    // type_id(x);

    // mut 关键字也可以用于模式绑定中
    // mut 关键字和 ref 关键字一样，是“模式”的一部分
    // Rust 中，所有的变量绑定默认都是“不可更改”的
    // 只有使用了 mut 修饰的变量绑定才有修改数据的能力
    // 使用了 mut 修饰的变量绑定，可以重新绑定到其他同类型的变量
    let mut v = vec![1i32, 2, 3];
    v = vec![4i32, 5, 6];
    // v = vec![1.0f32, 2, 3]; // 类型不匹配，不能重新绑定
    println!("{:?}", v);

    // 重新绑定与前面提到的“变量遮蔽”（ shadowing ）是完全不同的作用机制
    // “重新绑定”要求变量本身有 mut 修饰，并且不能改变这个变量的类型
    // “变量遮蔽”要求必须重新声明一个新的变量，这个新变量与老变量之间的类型可以毫无关系

    // Rust 在"可变性"方面，默认为不可修改
    // mut 关键字不仅可以在模式用于修饰变量绑定，还能修饰指针（引用）
    // mut 修饰变量绑定，与 &mut 型引用，是完全不同的意义
    let mut x: &mut i32;
    //  ^1     ^2
    // 第 1 处 mut，代表这个变量 x 本身可变，因此它能够重新绑定到另外一个变量上去
    // 具体到这个示例来说，就是指针的指向可以变化
    // 第 2 处 mut, 修饰的是指针，代表这个指针对于所指向的内存具有修改能力，因此可以
    // 用 *x = 1; 这样的语句，改变它所指向的内存的值


    // 至于为什么有些场景下，我们必须使用 ref 来进行变量绑定
    // 背后的原因跟 “move” 语义有关
    let mut x: Option<String> = Some("hello".into());
    // match x {
    match &mut x {
        // Some(ref mut i) => i.push_str(" world"),
        Some(i) => i.push_str(" world"),
        None => println!("None"),
    }

    println!("{:?}", x);
}
