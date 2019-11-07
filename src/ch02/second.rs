#[allow(dead_code)]

/// 基本数据类型
/// bool
pub fn learn_bool() {
    println!("------------------------");

    let x = true;
    let y: bool = !x; // 取反

    let z = x && y; // 逻辑与，短路
    println!("{}", z);

    let z = x || y; // 逻辑或，短路
    println!("{}", z);

    let z = x & y; // 按位与，不带短路
    println!("{}", z);
    let z = x | y; // 按位或，不带短路
    println!("{}", z);

    let z = x ^ y; // 按位异或，不带短路
    println!("{}", z);

    let z = 1 < 2;
    println!("{}", z);
}

pub fn learn_char() {
    println!("------------------------");

    // 字符类型由 char 表示。
    // 它可以描述任何一个符合 unicode 标准的字符值。在代码中，单个的字符字面量用单引号包围
    // 1 个 char 占用 4 个字节
    let love = '❤';
    println!("{}", love);
    let c1 = '\n';
    let c2 = '\x7f';
    let c3 = '\u{7fff}';
    println!("{} {} {}", c1, c2, c3);
    // 对于 ASCII 字符其实只需占用一个字节的空间，
    // 因此 Rust 提供了单字节字符字面量来表示 ASCII 字符
    // 可以使用一个字母 b 在字符或者字符串前面，代表这个字面量存储在 u8 类型数组中
    let x: u8 = 1;
    let y: u8 = b'A';
    let s: &[u8; 5] = b"hello";
    let r: &[u8; 14] = br#"hello \n world"#;
    println!("{} {} {:?} {:?}", x, y, s, r);
}
