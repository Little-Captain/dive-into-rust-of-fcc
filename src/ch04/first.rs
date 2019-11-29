#[allow(dead_code)]

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

// 函数的参数列表与 let 语句一样，也是一个"模式解构"
fn add2((x, y): (i32, i32)) -> i32 {
    x + y
}

pub fn first() {
    println!("add 1 {}", add1((1, 2)));
    println!("add 2 {}", add2((1, 2)));

    // 函数可以当成头等公民(first class value)被复制到一个值中，这个值可以像函数一样被调用
    let p = (1, 3);
    let func = add2;
    // func 可以被当成普通函数一样被调用
    println!("evaluation output {}", func(p));

    // 在 Rust 中，每一个函数都具有自己单独的类型，但是这个类型可以自动转换到 fn 类型
    // 虽然 add1 和 add2 有同样的参数类型和同样的返回值类型，但它们是不同类型，直接赋值会报错
    // 写法 1
    // let mut func = add1 as fn((i32, i32)) -> i32;
    // 写法 2
    let mut func: fn((i32, i32)) -> i32 = add1;
    func = add2;
}

// Rust 的函数体内允许定义其他 item，包括静态变量、常量、函数、trait、类型、模块等
pub fn test_inner() {
    static INNER_STASIC: i64 = 42;

    // 函数内部定义的函数
    fn internal_incr(x: i64) -> i64 {
        x + 1
    }

    struct InnerTemp(i64);

    impl InnerTemp {
        fn incr(&mut self) {
            self.0 = internal_incr(self.0);
        }
    }

    // 函数体，执行语句
    let mut t = InnerTemp(INNER_STASIC);
    t.incr();
    println!("{}", t.0);
    // 当你需要一些 item 仅在此函数内有用的时候，可以把它们直接定义到函数体内
    // 以避免污染外部的命名空间。
}

pub fn learn_diverging_func() {
    // Rust 支持一种特殊的发散函数(Diverging functions)，它的返回类型是感叹号 !
    // 这个函数根本就不能正常返回
    fn diverges() -> ! {
        // 因为 panic! 会直接导致栈展开，所以这个函数调用后面的代码都不会继续执行，
        // 它的返回类型就是一个特殊的 ! 符号，这种函数也叫作发散函数
        // 发散类型的最大特点就是，它可以被转换为任意一个类型
        panic!("This function never returns!");
    }

    let x: i32 = diverges();
    let y: String = diverges();

    let p = if x != 100 {
        panic!("error");
    } else {
        100
    };
    // 对于分支结构的表达式，它的每条分支的类型必须一致
    // 那么这条 panic! 宏应该生成一个什么类型呢？
    // 这就是 ! 类型的作用了。因为它可以与任意类型相容，所以编译器的类型检查才能通过。

    // 在Rust 中，有以下这些情况永远不会返回，它们的类型就是 !
    // 1. panic! 以及基于它实现的各种函数/宏，比如 unimplemented!、unreachable!
    // 2. 死循环 loop {}
    // 3. 进程退出函数 std::process::exit 以及类似的 libc 中的 exec 一类函数
}

pub fn learn_main_fn() {
    // Rust 的设计稍微有点不一样，传递参数和返回状态码都由单独的 API 来完成
    for arg in std::env::args() {
        match std::env::var(&arg) {
            Ok(val) =>println!("{}: {:?}", &arg, val),
            Err(e) => println!("couldn't find environment {}, {}", &arg, e),
        }
        println!("Arg: {}", arg);
    }
    println!("All environment varible count {}", std::env::vars().count());
    // std::process::exit(0);
    // 如果要读取环境变量，可以用 std::env::var() 以及 std::env::vars() 函数获得
    // var() 函数可以接受一个字符串类型参数，用于查找当前环境变量中是否存在这个名字的
    // 环境变量， vars() 函数不携带参数，可以返回所有的环境变量

    // 关于 main 函数的签名
    // Rust 的 main 函数只支持无参数、无返回值类型的声明方式，即 main 函数的签名固定为:
    // fn main() -> ()。但是，在引入了 ? 符号作为错误处理语法糖之后，就变得不那么优雅了，
    // 因为 ? 符号要求当前所在的函数返回的是 Result 类型，这样一来，问号就无法直接在 main 函数
    // 中使用了。为了解决这个问题，Rust 设计组扩展了 main 函数的签名，使它变成了一个泛型函数，
    // 这个函数的返回类型可以是任何一个满足 Termination trait 约束的类型，其中()、bool Result
    // 都是满足这个约束的，它们都可以作为 main 函数的返回类型。
    // fn main() -> () or bool or Result
}

pub fn learn_const_fn() {
    // 函数可以用 const 关键字修饰，这样的函数可以在编译阶段被编译器执行
    // 返回值也被视为编译期常量
    const fn cube(num: usize) -> usize {
        num * num * num
    }

    const DIM: usize = cube(2);
    const ARR: [i32; DIM] = [0; DIM];

    println!("{:?}", ARR);
}

pub fn learn_recursion() {
    // 函数递归调用
    // 所谓递归调用，指的是函数直接或者间接调用自己
    fn fib(index: u32) -> u64 {
        if index == 1 || index == 2 {
            1
        } else {
            fib(index - 1) + fib(index - 2)
        }
    }

    let f8 = fib(8);
    println!("{}", f8);
}
