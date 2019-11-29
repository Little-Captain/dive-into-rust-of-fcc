#[allow(dead_code)]

fn func(n: i32) -> bool {
    if n < 0 {
        print!("{} is negative", n);
        false
    } else if n > 0 {
        print!("{} is positive", n);
        true
    } else {
        print!("{} is zero", n);
        true
    }
    // 如果 else 分支省略掉了，那么编译器会认为 else 分支的类型默认为 ()
}

// if-else 表达式
pub fn learn_if_else() {
    println!("\n{}", func(12));
}

// loop
// 在 Rust 中，使用 loop 表示一个无限死循环
pub fn learn_loop() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // 不再继续执行后面的代码，跳转到 loop 开头继续循环
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // 跳出循环
            break;
        }
    }

    // loop 结构也可以作为表达式的一部分
    let v = loop {
        break 10;
    };
    println!("{}", v);
    // 在 loop 内部 break 的后面可以跟一个表达式，这个表达式就是最终的 loop 表达式的值
    // 如果一个 loop 永远不返回，那么它的类型就是"发散类型"
    // let v = loop {};
    // 编译器可以判断出 v 的类型是发散类型，而后面的打印语句是永远不会执行的死代码。
    // println!("{}", v);
}

// continue break
// continue; 语句表示本次循环内，后面的语句不再执行，直接进入下一轮循环
// break; 语句表示跳出循环，不再继续
// break 语句和 continue 语句还可以在多重循环中选择跳出到哪一层的循环
pub fn learn_break_continue() {
    let mut m = 1;
    let n = 1;

    // 可以在 loop while for 循环前面加上"生命周期标识符"
    // 该标识符以单引号开头，在内部的循环中可以使用 break 语句选择跳出到哪一层
    'a: loop {
        if m < 100 {
            m += 1;
        } else {
            'b: loop {
                if m + n > 50 {
                    println!("break");
                    break 'a;
                } else {
                    continue 'a;
                }
            }
        }
    }
}

// while
// while 语句是带条件判断的循环语句
pub fn learn_while() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

// for
// Rust 中的 for 循环实际上是许多其他语言中的 for-each 循环
// Rust 中没有类似 C/C++ 的三段式 for 循环语句
pub fn learn_for() {
    let array = &[1, 2, 3, 4, 5];
    for i in array {
        println!("The number is {}", i);
    }
}
