# 01-快速入门:Rust中有哪些你不得不了解的基础语法?

## PART1. Rust的基础类型

### 1.1. 字符

Rust的字符类型是`char`,它是Unicode散列值,它的大小总是4个字节

```rust
fn main() {
    let c = 'z';
}
```

### 1.2 字符串

String内部存储的是Unicode字符串de UTF8编码,而char存储的是Unicode Scalar Value.换言之,**String不是char的数组**.

Rust中的String不能通过下标访问.

### 1.3 字节串

字节串是一个字节的数组,它的类型是`&[u8]`.字节串是一个不可变的引用,它的长度是固定的.

```rust
fn main() {
    let byte_string: &[u8] = b"this is a byte string";
    println!("A byte string: {:?}", byte_string);
}
```

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/byte_string`
A byte string: [116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103]
```

### 1.4 数组

#### 1.4.1 固定长度数组

Rust中的数组是固定长度的,也就是说在编译阶段就能知道它占用的字节数,并且在运行阶段,不能改变它的长度(尺寸).

Rust中区分固定尺寸数组和动态数组.之所以做这种区分是因为Rust语言在设计时就要求适应不同的场合,要有足够的韧性能在不同的场景中都达到最好的性能.

因为固定尺寸的数据类型是可以直接放栈上的,创建和回收都比在堆上动态分配的动态数组性能要好.

是否能在编译期计算出某个数据类型在运行过程中占用内存空间的大小,这个指标很重要,Rust的类型系统就是按这个对类型进行分类的

数组常用于开辟一个固定大小的Buffer(缓冲区),用来接收IO输入输出等.也常用已知元素个数的字面量集合来初始化,比如表达一年有12个月

#### 1.4.2 动态数组

Rust中的动态数组类型是Vec(Vector),也就是向量,中文翻译成动态数组.它用来存储同一类型的多个值,容量可在程序运行的过程中动态地扩大或缩小,因此叫做动态数组

#### 1.4.3 面对索引越界的不同表现

- 固定长度数组的索引越界:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = a[5];
    println!("{}", b)
}
```

```bash
cargo build
   Compiling array v0.1.0 (/code/array)
error: this operation will panic at runtime
 --> src/main.rs:3:13
  |
3 |     let b = a[5];
  |             ^^^^ index out of bounds: the length is 5 but the index is 5
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `array` (bin "array") due to 1 previous error
```

未通过编译.因为数组的长度是确定的,因此Rust在编译时就能知道数组的长度,当索引超出数组的长度时,编译器就会报错.阻止了越界访问.

- 动态数组的索引越界

```rust
fn main() {
    let s1: String = String::from("s1");
    let s2: String = String::from("s2");
    let s3: String = String::from("s3");
    let s4: String = String::from("s4");

    let v: Vec<String> = vec![s1, s2, s3, s4];
    println!("{:?}", v[4]);
}
```

```bash
cargo build
   Compiling vector v0.1.0 (/code/vector)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
```

```bash
 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/vector`
thread 'main' panicked at src/main.rs:8:23:
index out of bounds: the len is 4 but the index is 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

可以看到,动态数组在运行时才会检查索引是否越界,当索引超出数组的长度时,程序会直接崩溃,这是因为动态数组的长度是在运行时才能确定的,因此Rust在编译时无法检查索引是否越界.

### 1.5 哈希表

Rust中的哈希表是一个键值对的集合,它的类型是`HashMap<K, V>`,其中K是键的类型,V是值的类型.

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

## PART2. Rust中的复合类型

### 2.1 元组

元组是一个固定(元素)长度的列表,每个元素类型可以不一样:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} {} {}", five, six_point_four, one);
}
```

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/tuple`
500 6.4 1
```

元组通常用于函数的返回值

### 2.2 枚举

枚举也是一种复合类型,它允许你定义一个类型,它的值是几个可能的变体之一.

枚举类型里面的选项叫做此枚举的变体(variants),变体是其所属枚举类型的一部分

```rust
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
```

与结构体不同,结构体类型是里面的所有字段(所有类型)同时起作用,来产生一个具体的实例

而枚举类型是其中的一个变体起作用,来产生一个具体实例

**枚举类型是Rust中最强大的复合类型**

## PART3. Rust中的控制流

### 3.1 循环

#### 3.1.1 loop

```rust
fn main() {
    let mut counter = 0;

    // result用于接收从loop中返回的值
    let result = loop {
        counter += 1;

        if counter == 10 {
            // 跳出循环时 从循环中返回一个值
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

```bash
 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/loops`
The result is 20
```

这种**返回一个值到外面对一个变量初始化的方式**,是Rust中的习惯用法,这能让代码更紧凑

#### 3.1.2 for

for循环在Rust中基本只用于迭代器的遍历.Rust中没有C语言中的for循环,因为那被认为是一种不好的设计

一定要记住,在Rust中,for循环是用来遍历迭代器的,而不是用来遍历数字的.因此哪怕你真的要循环n次,也应该使用`0..n`这种迭代器

```rust
fn main() {
    for number in 1..4 {
        println!("{}!", number);
    }

    for number2 in (1..4).rev() {
        println!("{}!", number2);
    }
}
```

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/fors`
1!
2!
3!
3!
2!
1!
```

## PART4. Rust中的函数与模块

### 4.1 函数

函数对于几乎所有语言都非常重要,实际上各种编程语言在实现时,都是以函数作为基本单元来组织栈上的内存分配和回收的,这个基本的内存单元就是所谓的**栈帧**(frame)

### 4.2 闭包

```rust
fn main() {
    let a: u32 = 10;

    fn add_v1(x: u32) ->u32 {
            x + a
    }

    let add_v2 = |x: u32| -> u32 {
        x + a
    };

    let result1 = add_v1(10);
    let result2 = add_v2(10);

    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}
```

```bash
cargo build
   Compiling closure v0.1.0 (/code/closure)
error[E0434]: can't capture dynamic environment in a fn item
 --> src/main.rs:5:17
  |
5 |             x + a
  |                 ^
  |
  = help: use the `|| { ... }` closure form instead

For more information about this error, try `rustc --explain E0434`.
error: could not compile `closure` (bin "closure") due to 1 previous error
```

闭包与函数的一个显著不同在于:闭包可以捕获函数中的局部变量使用,而函数不行

### 4.3 模块

现有目录结构如下:

```bash
tree ./    
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

2 directories, 5 files
```

`src/garden/vegetables.rs`:

```rust
pub(crate) fn echo() {
    println!("vegetables");
}
```

`src/garden.rs`:

```rust
pub(crate) mod vegetables;
```

`src/main.rs`:

```rust
mod garden;

fn main() {
    garden::vegetables::echo();
}
```

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/backyard`
vegetables
```