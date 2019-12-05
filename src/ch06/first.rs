#![allow(dead_code)]

// 数组
// 数组是一个容器，它在一块连续空间内存中，存储了一系列的同样类型的数据
// 数组中元素的占用空间大小必须是编译期确定的。数组本身所容纳的元素个数
// 也必须是编译期确定的，执行阶段不可变。如果需要使用变长的容器，可以使用
// 标准库中的 Vec/LinkedList 等。数组类型的表示方式为 [T; n]。其中 T
// 代表元素类型；n 代表元素个数；它必须是编期常量整数；中间用分号隔开
pub fn learn_array1() {
    // 定长数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素初始化同样的数据
    let ys: [i32; 500] = [0; 500];
    println!("{:?}", xs);
    // println!("{:?}", ys);
}

// 在 Rust 中，对于两个数组类型，只有元素类型和元素个数都完全相同，这两个数组
// 才是同类型的。数组与指针之间不能隐式转换。同类型的数组之间可以互相赋值
pub fn learn_array2() {
    let mut xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 5] = [6, 7, 8, 9, 10];
    xs = ys;
    println!("new array {:?}", xs);
    // 把数组 xs 作为参数传给一个函数，这个数组并不会退化成一个指针
    // 而是会将这个数组完整复制进这个函数。函数体内对数组的改动不会影响到外面的数组
}

// 对数组内部元素的访问，可以使用中括号索引的方式。Rust 支持 usize 类型的索引的数
// 组，索引从 0 开始计数

// 内置方法
// Rust 的数组类型拥有一些内置方法，可以很方便地完成一些任务
pub fn learn_array3() {
    let v1 = [1, 2, 3];
    let v2 = [1, 2, 4];
    println!("{:?}", v1 < v2);

    // 遍历数组
    // 在目前的标准库中，数组本身没有实现 Irttolterator trait，但是数组切片是实现了的
    // 所以我们可以直接在 for in 循环中使用数组切片，而不能直接使用数组本身
    for i in &v1 {
        println!("{:?}", i);
    }
}

// 多维数组
// 既然 [T; n] 是一个合法的类型，那么它的元素 T 当然也可以是数组类型，
// 因此 [[T; m]; n] 类型自然也是合法类型
pub fn learn_array4() {
    let v: [[i32; 2]; 3] = [[0, 0], [0, 0], [0, 0]];
    for i in &v {
        println!("{:?}", i);
    }
}

// 数组切片
// 对数组取借用 borrow 操作，可以生成一个"数组切片"(Slice)。数组切片对数组没有"所有权"，
// 我们可以把数组切片看作专门用于指向数组的指针，是对数组的另外一个"视图"。
// 比如，我们有一个数组 [T; n]，它的借用指针的类型就是 &[T; n]
// 它可以通过编译器内部魔法转换为数组切片类型 &[T]
// 数组切片实质上还是指针，它不过是在类型系统中丢弃了编译阶段定长数组类型的长度信息，
// 而将此长度信息存储为运行期的值
pub fn learn_array5() {
    fn mut_array(a: &mut [i32]) {
        a[2] = 5;
    }

    println!("size of &[i32; 3]: {:?}", std::mem::size_of::<&[i32; 3]>());
    println!("size of &[i32]   : {:?}", std::mem::size_of::<&[i32]>());

    let mut v: [i32; 3] = [1, 2, 3];
    {
        let s: &mut [i32; 3] = &mut v;
        mut_array(s);
    }
    println!("{:?}", v);
}

// DST 和胖指针
// 数组切片是指向一个数组的指针，而它比指针又多了一点东西
// 它不止包含有一个指向数组的指针，切片本身还含带长度信息

// Slice 与普通的指针是不同的，它有一个非常形象的名字：胖指针(fat pointer)
// 与这个概念相对应的概念是"动态大小类型"(Dynamic Sized Type, DST)
// 所谓的 DST 指的是编译阶段无法确定占用空间大小的类型
// 为了安全性，指向 DST 的指针一般是胖指针

// 对于不定长数组类型 [T]，有对应的胖指针 &[T] 类型
// 对于不定长字符串 str 类型有对应的胖指针 &str 类型
// 以及在后文中会出现的 Trait Object；等等

// 由于不定长数组类型 [T] 在编译阶段是无法判断该类型占用空间的大小的，
// 目前我们不能在栈上声明一个不定长大小数组的变量实例，也不能用它作为
// 函数的参数、返回值。但是，指向不定长数组的胖指针的大小是确定的，
// &[T] 类型可以用做变量实例、函数参数、返回值。
// &[T] 类型占用了两个指针大小的内存空间
fn raw_slice(arr: &[i32]) {
    unsafe {
        let (val1, val2): (usize, usize) = std::mem::transmute(arr);
        println!("Value in raw pointer:");
        println!("value1: {:x}", val1);
        println!("value2: {:x}", val2);
    }
}
// 胖指针内部的数据既包含了指向源数组的地址，又包含了该切片的长度
// 对于 DST 类型，Rust 有如下限制:
// 1. 只能通过指针来间接创建和操作 DST 类型，&[T] Box<[T]> 可以，[T] 不可以
// 2. 局部变量和函数参数的类型不能是 DST 类型，因为局部变量和函数参数必须在编译
//    阶段知道它的大小因为目前 unsized rvalue 功能还没有实现
// 3. enum 中不能包含 DST 类型，struct 中只有最后一个元素可以是 DST，其他地方不行，
//    如果包含有 DST 类型，那么这个结构体也就成了 DST 类型

// Rust 设计出 DST 类型，使得类型暂时系统更完善，也有助于消除一些 C/C++ 中容易
// 出现的bug。这一设计的好处有:
// 1. 首先，DST 类型虽然有一些限制条件，但我们依然可以把它当成合法的类型看待，
//    比如，可以为这样的类型实现 trait、添加方法、用在泛型参数中等
// 2. 胖指针的设计，避免了数组类型作为参数传递时自动退化为裸指针类型，丢失了
//    长度信息的问题，保证了类型安全
// 3. 这一设计依然保持了与"所有权""生命周期"等概念相容的特点
// 数组切片不只是提供了"数组到指针"的安全转换，配合上 Range 功能，它还能提供数组的局部切片功能
pub fn learn_array6() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let address: &[i32; 5] = &arr;
    println!("Address of arr: {:p}", address);
    raw_slice(address as &[i32]);
}

// Range
// Rust 中的 Range 代表一个"区间"，点 ..
pub fn learn_array7() {
    // 生成的是一个 std::ops::Range<_> 类型的变量
    // let r = 1..10; // [1, 10)
    let r = std::ops::Range { start: 1, end: 10 };
    for i in r {
        print!("{:?} ", i);
    }
    println!("-----------------");
    let r = (1i32..11).rev().map(|i| i * 10);
    for i in r {
        print!("{:?} ", i);
    }
}

// Rust 中的几种 Range
// 1. std::ops::Range 代表前闭后开的区间，语法为 start..end
//    含义是 [start, end)
// 2. std::ops::RangeFrom 代表只有起始没有结束的范围，语法为 start..
//    含义是[start, +∞)
// 3. std::ops::RangeTo 代表没有起始只有结束的范围，语法为 ..end
//    对有符号数的含义是(-∞, end)，对无符号数的含义是[0, end)
// 4. std::ops::RangeFull 代表没有上下限制的范围，语法为 ..
//    对有符号数的含义是(-∞，+∞)，对无符号数的含义是[0, +∞)
// 5. std::ops::RangeInclusive，语法为 start..=end
//    含义是 [start, end]
// 6. std::ops::RangeToInclusive，语法为 ..=end
//    对有符号数的含义是(-∞, end]，对无符号数的含义是[0, end]
pub fn learn_array8() {
    let r = 1..10;
    let rf = 1..;
    let rt = ..10;
    let rfull = ..;
    let rc = 1..=10;
    let rtc = ..=10;
    println!("{:?}", r);
    println!("{:?}", rf);
    println!("{:?}", rt);
    println!("{:?}", rfull);
    println!("{:?}", rc);
    println!("{:?}", rtc);
    for i in rc {
        print!("{:?} ", i);
    }
}

// 数组和 Range 之间最常用的配合就是使用 Range 进行索引操作
fn print_slice(arr: &[i32]) {
    println!("Length: {}", arr.len());

    for item in arr {
        print!("{}\t", item);
    }

    println!("");
}

pub fn learn_array9() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    print_slice(&arr[..]);

    let slice = &arr[2..];
    print_slice(slice);

    let slice2 = &arr[..2];
    print_slice(slice2);

    let slice3 = &arr[..=2];
    print_slice(slice3);
    // 在许多时候，使用数组的一部分切片作为被操作对象在函数间传递，既保证了效率(避
    // 免直接复制大数组)，又能保证将所需要执行的操作限制在一个可控制的范围内(有长度信
    // 息，有越界检查)，还能控制其读写权限，非常有用
}

// 边界检查
// 在 Rust 中，"索引"操作也是一个通用的运算符，是可以自行扩展的
// 如果希望某个类型可以执行"索引"读操作，就需要该类型实现 std::ops::Index trait
// 如果希望某个类型可以执行"索引"写操作，就需要该类型实现 std::ops::IndexMut trait

// 为了防止索引操作导致程序崩惯，如果我们不确定使用的"索引"是否合法，应该使用 get() 方法
// 调用来获取数组中的元素，这个方法不会引起 panic!，它的返回类型是 Option<T>

// 在 Rust 里面，靠编译阶段静态检查是无法消除数组越界的行为的
pub fn learn_array10() {
    let v = [10i32, 20, 30, 40, 50];
    let first = v.get(0);
    let tenth = v.get(10);
    println!("{:?} {:?}", first, tenth);
}

// 一般情况下，Rust 不鼓励大量使用“索引”操作。正常的“索引”操作都会执行一次“边界检查”
// 从执行效率上来说，Rust 比 C/C++ 的数组索引效率低一点，因为 C/C++ 的索引操作是
// 不执行任何安全性检查的，它们对应的 Rust 代码相当于调用 get_unchecked() 函数
// 在 Rust 中，更加地道的做法是尽量使用“迭代器”方法
pub fn learn_array11() {
    use std::iter::Iterator;

    let v = &[10i32, 21, 30, 40, 50];
    for (index, value) in v.iter().enumerate() {
        println!("{} {}", index, value);
    }

    let item = v.iter().filter(|&x| *x % 2 == 0).nth(2);
    println!("{:?}", item);
}
