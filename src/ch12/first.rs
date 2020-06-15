// 借用和生命周期

// 生命周期
// 一个变量的生命周期就是它从创建到销毁的整个过程。
pub fn first() {
    let v = vec![1, 2, 3, 4, 5]; // v 的生命周期开始
    {
        let center = v[2]; // center 的生命周期开始
        println!("{}", center);
    } // center 的生命周期结束
    println!("{:?}", v);
} // v 的生命周期结束
// 如果一个变量永远只能有唯一一个入口可以访问的话，那就太难使用了。
// 因此，所有权还可以借用。

// 所有权借用
// 变量对其管理的内存拥有所有权。这个所有权不仅可以被转移(move)，还可以被借用(borrow)。
// 借用指针的语法使用 & 符号或者 &mut 符号表示。前者表示只读借用，后者表示可读写借用。
// 借用指针(borrow pointer)也可以称作“引用”(reference)。借用指针与普通指针的内
// 部数据是一模一样的，唯一的区别是语义层面上的。它的作用是告诉编译器，它对指向的这
// 块内存区域没有所有权。
// 借用指针在编译后，就是一个普通的指针，它的意义只能在编译阶段的静态检查中体现。
pub fn second() {
    // 形参类型: 可变的借用指针
    fn foo(v: &mut Vec<i32>) {
        // Vec::push 函数的作用是对动态数组添加元素，它的签名是 pub fn push(&mut self, value: T)
        // 它要求 self 参数是一个 &mut Self 类型。
        v.push(5);
    }
    fn test1() {
        // 需要动态数组本身是可变的
        let mut v = vec![];
        // 获取可变借用指针
        foo(&mut v);
        println!("{:?}", v);
    }
    test1();

    // 对于 &mut 型指针，不要混淆它与变量绑定之间的语法。
    // 如果 mut 修饰的是变量名，那么它代表这个变量可以被重新绑定；
    // 如果 mut 修饰的是 “借用指针 &”，那么它代表的是被指向的对象可以被修改。
    fn test2() {
        let mut var = 0;
        {
            // p1 指针本身不能被重新绑定，可以通过 p1 改变变量 var 的值
            let p1 = &mut var;
            *p1 = 1;
        }
        {
            let temp = 2;
            // 不能通过 p2 改变变量 var 的值，但 p2 指针本身指向的位置可以被改变
            let mut p2 = &var;
            p2 = &temp;
        }
        {
            let mut temp = 3;
            // 既可以通过 p3 改变变量 var 的值，
            // 而且 p3 指针本身指向的位置也可以改变
            let mut p3 = &mut var;
            *p3 = 13;
            p3 = &mut temp;
        }
        println!("{}", var);
    }
    test2();
}

// 借用规则
// 关于借用指针，有以下几条规则：
// 1. 借用指针不能比它指向的变量存在的时间更长。
// 2. &mut 型借用只能指向本身具有 mut 修饰的变量，对于只读变量，不可以有 &mut 型借用。
// 3. &mut 型借用指针存在的时候，被借用的变量本身会处于“冻结”状态。
// 4. 如果只有 & 型借用指针，那么能同时存在多个；如果存在 &mut 型借用指针，那么只能存在一个；
// 如果同时有其他的 & 或者 &mut 型借用指针存在，那么会出现编译错误。
// 5. 借用指针只能临时地拥有对这个变量读或写的权限，没有权力管理这个变量的生命周期。
// 因此，借用指针的生命周期绝对不能大于它所引用的原来变量的生命周期，
// 否则就是悬空指针，会导致内存不安全。
pub fn third() {
    // 参数采用的“引用传递”，实参并未丢失对内存的管理权
    fn borrow_semantics(v: &Vec<i32>) {
        // 打印参数占用空间的大小，在 64 位系统上， 结果为 8，
        // 表明该指针与普通裸指针的内部表示方法相同
        println!("size of param: {}", std::mem::size_of::<&Vec<i32>>());
        for item in v {
            print!("{} ", item);
        }
        println!();
    }
    // 参数采用的“值传递”，而 Vec 没有实现 Copy trait，意味着它将执行 move 语义
    fn move_semantics(v: Vec<i32>) {
        // 打印参数占用空间的大小，结果为 24，
        // 表明实参中栈上分配的内存空间复制到了函数的形参中
        println!("size of param: {}", std::mem::size_of::<Vec<i32>>());
        for item in v {
            print!("{} ", item);
        }
        println!();
    }
    fn test1() {
        let array = vec![1, 2, 3, 4, 5];
        // 需要注意的是，如果使用引用传递，不仅在函数声明的地方需要使用 & 标记
        // 函数调用的地方同样需要使用 & 标记，否则会出现语法错误
        // 小数点方式的成员函数调用，对于 self 参数，会“自动转换”，不必显式借用
        borrow_semantics(&array);
        // 在使用引用传递给上面的函数后，array 本身依然有效，还能在下面的函数中使用
        move_semantics(array);
        // 在使用 move 语义传递后，array 在这个函数调用后，它的生命周期就已经完结
    }
    test1();

    // 一般情况下，函数参数使用引用传递的时候，不仅在函数声明这里要写上类型参数，
    // 在函数调用这里也要显式地使用引用运算符。
    // 但是，有一个例外，那就是当参数为 self、&self、&mut self 等时，
    // 若使用小数点语法调用成员方法，在函数调用这里不能显式写出借用运算符。
    fn test2() {
        let mut x: String = "hello".into();
        // len(&self) -> usize
        // 完整调用形式：String::len(&x)
        println!("length of String {}", x.len());
        // push(&mut self, ch: char)
        // 完整调用形式：String::push(&mut x, '!')
        x.push('!');

        println!("length of String {}", x.len());

        // into_bytes(self) -> Vec<u8>
        // 注意 self 的类型，此处发生了所有权转移
        // 完整调用形式：String::into_bytes(x)
        let v = x.into_bytes();

        // 再次调用 len()，编译失败，因为此处已经超过了 x 的生命周期
        // println!("length of String {}", x.len());
    }
    test2();

    // 任何借用指针的存在，都会导致原来的变量被“冻结”(Frozen)。
    /*
    fn test3() {
        let mut x = 1;
        let p = &mut x;
        // 因为 p 的存在，此时对 x 的改变被认为是非法的。
        x = 2;
        println!("value of pointed: {}", p);
    }
    test3();
    */
}

// 生命周期标记
// 对一个函数内部的生命周期进行分析，Rust 编译器可以很好地解决。
// 但是，当生命周期跨函数的时候，就需要一种特殊的生命周期标记符号了。
pub fn fourth() {
    // 函数的生命周期标记
    fn test1() {
        struct T {
            member: i32,
        }
        // 生命周期符号使用单引号开头，后面跟一个合法的名字。
        // 生命周期标记和泛型类型参数是一样的，都需要先声明后使用。
        // 在上面这段代码中，尖括号里面的 'a 是声明一个生命周期参数，
        // 它在后面的参数和返回值中被使用。
        // 在做局部变量的时候，生命周期参数是可以省略的。
        // 生命周期之间有重要的包含关系。如果生命周期 'a 比 'b 更长或相等，则记为 'a : 'b，
        // 对于借用指针类型来说，
        // 如果 &'a 是合法的，那么 'b 作为 'a 的一部分， &'b 也一定是合法的。???
        // 'static 是一个特殊的生命周期，它代表的是这个程序从开始到结束的整个阶段，
        // 所以它比其他任何生命周期都长。任意一个生命周期 'a 都满足 'static : 'a。
        fn test<'a>(arg: &'a T) -> &'a i32 {
            &arg.member
        }
        let t = T { member: 0 };
        let x = test(&t);
        println!("{:?}", x);
    }
    test1();
}
