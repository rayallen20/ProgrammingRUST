enum IpAddrKind {
    V4,
    V6,
}

enum MyEnum {
    A(String),
    B(i32),
}

fn main() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;

    let s: MyEnum = MyEnum::A(String::from("hello"));
    let i: MyEnum = MyEnum::B(5);
}
