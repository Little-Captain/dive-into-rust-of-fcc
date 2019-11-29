#[allow(dead_code)]

// 运算表达式
pub fn learn_operator() {
    let x = 100;
    let y = 10;
    println!("{} {} {} {} {}", x + y, x - y, x * y, x / y, x % y);
}

// 位运算符 作用
// !     按位取反(注意不是~符号)
// &     按位与
// |     按位或
// ^     按位异或
// <<    左移
// >>    右移
pub fn learn_bit_op() {
    let num1: u8 = 0b_1010_1010;
    let num2: u8 = 0b_1111_0000;

    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num1 << 4);
    println!("{:08b}", num1 >> 4);
}

// 逻辑运算符 作用
// &&        逻辑与
// ||        逻辑或
// !         逻辑取反
// bool 类型既支持"逻辑与"、"逻辑或"， 也支持"按位与"、"按位或"
// 它们的区别在于，"逻辑与"、"逻辑或"具备"短路"功能
// 逻辑短路，按位不短路
fn f1() -> bool {
    println!("Call f1");
    true
}

fn f2() -> bool {
    println!("Call f2");
    false
}

pub fn learn_bool_op() {
    println!("Bit and: {}\n", f2() & f1());
    println!("Logic and: {}\n", f2() && f1());

    println!("Bit or: {}\n", f1() | f2());
    println!("Logic or: {}\n", f1() || f2());
}

// 赋值表达式
// 一个左值表达式、赋值运算符（＝）和右值表达式，可以构成一个赋值表达式
// 赋值表达式具有"副作用":
// 当它执行的时候，会把右边表达式的值"复制或者移动"(copy or move)到左边的表达式中
// 赋值表达式的类型位 unit, 即空的 tuple ()
pub fn learn_assign() {
    let mut x: i32 = 1;
    x = 2;
    println!("{}", x);
    let y = x = 3;
    println!("{} {:?}", x, y);
    // Rust 支持组合赋值表达式
    // +、-、*、/、%、&、|、^、<<、>> 这几个运算符可以和赋值运算符组合成赋值表达式
    // LEFT OP= RIGHT <=> LEFT = LEFT OP RIGHT
    // Rust 不支持 ++ -- 运算符
    let x = 2;
    let mut y = 4;
    y += x;
    y *= x;
    println!("{} {}", x, y)
}

// 在 Rust 中，语句块也可以是表达式的一部分
// 语句和表达式的区分方式是后面带不带分号(;)
// 如果带了分号，意味着这是一条语句，它的类型是()
// 如果不带分号，它的类型就是表达式的类型
pub fn learn_statement_block() {
    let x: () = {
        println!("Hello.");
    };
    // 最后一个表达式值返回作为语句块的返回值
    let y: i32 = {
        println!("Hello.");
        5
    };
    println!("{:?}", x);
    println!("{:?}", y);
    fn my_func() -> i32 {
        100
    }
    println!("{}", my_func());
}
