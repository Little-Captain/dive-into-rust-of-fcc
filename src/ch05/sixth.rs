#[allow(dead_code)]
// Derive

// Rust 里面为类型 impl 某些 trait 的时候，逻辑是非常机械化的
// 为许多类型重复而单调地 impl 某些 trait，是非常枯燥的事情
// 为此，Rust 提供了一个特殊的 attribute，它可以帮我们自动 impl 某些 trait
#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Foo {
    data: i32,
}
// 在你希望 impl trait 的类型前面写 #[derive(…)]，
// 括号里面是你希望 impl 的 trait 的名字
// 然后，编译器就帮你自动加上了 impl 块
// impl Copy for Foo { ... }
// ...

// 目前， Rust 支持的可以自动 derive 的 trait 有以下这些
// Debug Clone Copy Hash RustcEncodable RustcDecodable
// PartialEq Eq ParialOrd Ord Default FromPrimitive Send Sync

pub fn learn_derive() {
    let v1 = Foo { data: 0 };
    let v2 = v1;
    println!("{:?}", v2);
}

// trait 别名
// 跟 type alias 类似的，trait 也可以起别名(trait alias)
// pub trait Service {
//     type Request;
//     type Response;
//     type Error;
//     type Future: Future<Item=Self::Response, Error=Self::Error>;
//     fn call(&self, req : Self::Request) -> Self::Future;
// }
// trait HttpService = Service<Request = http::Request,
//         Response= http::Response,
//         Error= http::Error>;
