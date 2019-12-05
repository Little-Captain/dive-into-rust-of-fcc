#![allow(dead_code)]

// Fully Qualified Syntax 提供一种无歧义的函数调用语法，允许程序员精确地
// 指定想调用的是那个函数。以前也叫 UFCS(universal function call syntax)，
// 也就是所谓的"通用函数调用语法"。

// 这个语法可以允许使用类似的写法精确调用任何方法，包括成员方法和静态方法
// 其他一切函数调用语法都是它的某种简略形式
// 它的具体写法为 <T as TraitName>::item

trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook::start");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash::start");
    }
}

pub fn learn_call_func() {
    let me = Chef;

    // me.start();

    // 必要使用完整的函数调用语法来进行方法调用，只有这样写，
    // 才能清晰明白且无歧义地表达清楚期望调用的是哪个函数
    <dyn Cook>::start(&me);
    <Chef as Wash>::start(&me);
}

// 由此我们也可以看到， 所谓的"成员方法"也没什么特殊之处，它跟普通的静态方法
// 的唯一区别是，第一个参数是 self，而这个 self 只是一个普通的函数参数而已
// 只不过这种成员方法也可以通过变量加小数点的方式调用
// 变量加小数点的调用方式在大部分情况下看起来更简单更美观，完全可以视为一种语法糖

// 需要注意的是，通过小数点语法调用方法调用，有一个"隐藏着"的"取引用"步骤
// 虽然我们看起来源代码长的是这个样子 me.start()，但是大家心里要清楚，
// 真正传递给 start() 方法的参数是 &me 而不是 me，这一步是编译器自动帮我们做的
// 不论这个方法接受的 self 参数究竟是 Self、&Self 还是 &mut Self ，最终在源码上，
// 我们都是统一的写法: variable.method()。而如果用 UFCS 语法来调用这个方法，
// 我们就不能让编译器帮我们自动取引用了，必须手动写清楚。

struct T(usize);

impl T {
    fn get1(&self) -> usize {
        self.0
    }

    fn get2(&self) -> usize {
        self.0
    }
}

fn get3(t: &T) -> usize {
    t.0
}

fn check_type(_: fn(&T) -> usize) {
}

pub fn learn_call_func1() {
    // get1、get2 和 get3 都可以自动转成 fn(&T）-> usize 类型
    check_type(T::get1);
    check_type(T::get2);
    check_type(get3);
}
