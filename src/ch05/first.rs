#![allow(dead_code)]

// trait
// trait 中可以包含：函数、常量、类型等

// 成员方法
// trait 中可以定义函数
trait Shape {
    fn area(&self) -> f64;
}
// 所有的 trait 中都有一个隐藏的类型 Self，代表当前这个实现了此 trait 的具体类型
// trait 中定义的函数，也可以称作关联函数(associated function)。函数的第一个参数
// 如果是 Self 相关的类型，且命名为 self，这个参数可以被称为"receiver"(接收者)。
// 具有 receiver 参数的函数，我们称为"方法"(method)，可以通过变量实例使用小数点来调用。
// 没有 receiver 参数的函数，我们称为"静态函数"(static function)，可以通过类型加双冒号::
// 的方式来调用。在 Rust 中，函数和方法没有本质区别。

// Rust 中 Self 和 self 都是关键字，大写 S 的是类型名，小写 s 的是变量名
// self 参数同样也可以指定类型，当然这个类型是有限制的，必须是包装在 Self 类型之上的类型
// 对于第一个 self 参数，常见的类型有 self: Self、self: &Self、self: &mut Self 等类型
// 对于以上这些类型， Rust 提供了一种简化的写法，我们可以将参数简写为 self、&self、&mut self
// self 参数只能用在第一个参数的位置

trait T1 {
    fn method1(self: Self);
    fn method2(self: &Self);
    fn method3(self: &mut Self);
}

trait T2 {
    fn method1(self);
    fn method2(&self);
    fn method3(&mut self);
}

// 以上两种写法完全等价

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    // self 的类型是 &Self, 即 &Circle
    fn area(&self) -> f64 {
        // 访问成员变量，需要用 self.radius
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub fn trait1() {
    let c = Circle { radius: 2f64 };
    println!("The area is {}", c.area());
}

// 针对一个类型，我们可以直接对它 impl 来增加成员方法，无须 trait 名字
// 可以把这段代码看作是为 Circle 类型 impl 了一个匿名的 trait
// 用这种方式定义的方法叫作这个类型的"内在方法"(inherent methods)
impl Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

// trait 中可以包含方法的默认实现。如果这个方法在 trait 中已经有了方法体，
// 那么在针对具体类型实现的时候，就可以选择不用重写。当然，如果需要针对特殊
// 类型作特殊处理，也可以选择重新实现来"override"默认的实现方式
// self 参数甚至可以是 Box 指针类型 self: Box<Self>
trait Shape1 {
    fn area1(self: Box<Self>) -> f64;
}

impl Shape1 for Circle {
    fn area1(self: Box<Self>) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub fn trait2() {
    let c = Circle { radius: 2f64 };
    // c.area1(); // 编译错误
    let b = Box::new(Circle { radius: 4f64 });
    println!("The area is {}", b.area1());
}

// impl 的对象甚至可以是 trait
trait Round {
    fn get_radius(&self) -> f64;
}

impl Round for Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Shape for dyn Round {
    fn area(&self) -> f64 {
        println!("dyn Round");
        std::f64::consts::PI * self.get_radius() * self.get_radius()
    }
}

pub fn trait3() {
    let c = Circle { radius: 2f64 };
    let b = Box::new(Circle { radius: 4f64 }) as Box<dyn Round>;
    println!("The area is {}", b.area());
}

// 注意这里的写法，impl Shape for Round 和 impl<T: Round> Shape for T 是不一样的
// 在前一种写法中，self 是 &Round 类型，它是一个 trait object，是胖指针
// 在后一种写法中，self 是凹类型，是具体类型
// 前一种写法是为 trait object 增加一个成员方法，而后一种写法是为所有的满足 T: Round 的具体类型增加一个成员方法
// 所以上面的示例中，我们只能构造一个 trait object 之后才能调用 area() 成员方法
// trait object 和"泛型"之间的区别请参考本书第三部分。
// 题外话，impl Shape for Round 这种写法确实是很让初学者纠结的， Round 既是 trait 又是 type
// 在将来，trait object 的语法会被要求加上 dyn 关键字，所以
// 在 Rust 2018 edition 以后应该写成 impl Shape for dyn Round
