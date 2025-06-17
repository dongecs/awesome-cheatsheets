/*******************************************************************************
 * 1. 基础语法
 *******************************************************************************/
// 1.1 工具链命令
rustc file.rs                                    // 编译单个 Rust 文件
cargo new project_name                           // 创建新的 Rust 项目
cargo build                                      // 编译项目
cargo run                                        // 编译并运行项目
cargo test                                       // 运行测试
cargo doc                                        // 生成文档
cargo clean                                      // 清理编译产物
cargo update                                     // 更新依赖
cargo check                                      // 检查代码但不生成可执行文件
cargo fmt                                        // 格式化代码
cargo clippy                                     // 运行 linter

// 1.2 环境变量
RUST_BACKTRACE=1                                // 启用详细的错误回溯
RUST_LOG=debug                                  // 设置日志级别
CARGO_HOME                                      // Cargo 主目录
RUSTUP_HOME                                     // Rustup 主目录

// 1.3 基本数据类型
// 标量类型
let x: i8 = 42;                                 // 8位有符号整数
let y: u8 = 42;                                 // 8位无符号整数
let z: i16 = 42;                                // 16位有符号整数
let w: u16 = 42;                                // 16位无符号整数
let v: i32 = 42;                                // 32位有符号整数
let u: u32 = 42;                                // 32位无符号整数
let t: i64 = 42;                                // 64位有符号整数
let s: u64 = 42;                                // 64位无符号整数
let r: i128 = 42;                               // 128位有符号整数
let q: u128 = 42;                               // 128位无符号整数
let p: isize = 42;                              // 指针大小的有符号整数
let o: usize = 42;                              // 指针大小的无符号整数
let n: f32 = 3.14;                              // 32位浮点数
let m: f64 = 3.14;                              // 64位浮点数
let b: bool = true;                             // 布尔值
let c: char = '中';                             // Unicode 字符
let byte: u8 = b'A';                            // 字节字面量

// 复合类型
let tup: (i32, f64, char) = (1, 2.0, 'a');      // 元组
let arr: [i32; 5] = [1, 2, 3, 4, 5];            // 数组
let slice: &[i32] = &arr[1..3];                 // 切片

// 1.4 变量和可变性
let x = 5;                                      // 不可变变量
let mut y = 5;                                  // 可变变量
const MAX_POINTS: u32 = 100_000;                // 常量
static PROGRAM_NAME: &str = "Rust";             // 静态变量

// 1.5 循环控制
// for 循环
for i in 0..5 {                                 // 范围循环
    println!("{}", i);
}

for (i, val) in arr.iter().enumerate() {        // 带索引的迭代
    println!("{}: {}", i, val);
}

for (_, val) in arr.iter().enumerate() {        // 只关心值，忽略索引
    println!("{}", val);
}

for val in arr.iter() {                         // 只有一个参数
    println!("{}", val);
}

// while 循环
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}

// loop 循环
let mut i = 0;
loop {
    if i >= 5 {
        break;
    }
    println!("{}", i);
    i += 1;
}

// 1.6 切片
let s = String::from("hello world");
let hello = &s[0..5];                           // 字符串切片
let world = &s[6..11];

let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4];                         // 数组切片

/*******************************************************************************
 * 2. 内存安全
 *******************************************************************************/
// 2.1 所有权系统
// 所有权规则
let s1 = String::from("hello");                 // s1 拥有字符串的所有权
let s2 = s1;                                    // s1 的所有权移动到 s2
// println!("{}", s1);                          // 错误！s1 已经不再有效

// 克隆
let s1 = String::from("hello");
let s2 = s1.clone();                            // 深拷贝

// 引用
let s1 = String::from("hello");
let s2 = &s1;                                   // 不可变引用
let mut s3 = String::from("hello");
let s4 = &mut s3;                               // 可变引用

// 2.2 智能指针
// Box<T> - 在堆上分配数据
let b = Box::new(5);                            // 在堆上存储值 5

// Rc<T> - 引用计数
use std::rc::Rc;
let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);                          // 增加引用计数

// Arc<T> - 原子引用计数（线程安全）
use std::sync::Arc;
let a = Arc::new(String::from("hello"));
let b = Arc::clone(&a);                         // 线程安全的引用计数

// RefCell<T> - 内部可变性
use std::cell::RefCell;
let data = RefCell::new(5);
*data.borrow_mut() = 6;                         // 运行时借用检查

// 2.3 Pin
use std::pin::Pin;
use std::marker::PhantomPinned;

struct Unmovable {
    data: String,
    _pin: PhantomPinned,
}

let unmovable = Unmovable {
    data: String::from("hello"),
    _pin: PhantomPinned,
};
let pinned = Pin::new(&unmovable);

/*******************************************************************************
 * 3. 类型系统
 *******************************************************************************/
// 3.1 结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建实例
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 实现方法
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
}

// 3.2 枚举和模式匹配
// 定义枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 模式匹配
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change color to: {}, {}, {}", r, g, b),
}

// if let
let some_value = Some(3);
if let Some(3) = some_value {
    println!("three");
}

// while let
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}

// 3.3 泛型和特征
// 泛型函数
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 特征定义
trait Summary {
    fn summarize(&self) -> String;
}

// 实现特征
impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} ({})", self.username, self.email)
    }
}

// 3.4 生命周期
// 基本生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
struct Excerpt<'a> {
    part: &'a str,
}

// 静态生命周期
let s: &'static str = "hello";                  // 字符串字面量具有 'static 生命周期

// 3.5 模块系统
// 模块定义
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 使用模块
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// 可见性
pub struct PublicStruct;                         // 公开结构体
struct PrivateStruct;                           // 私有结构体

pub enum PublicEnum {                           // 公开枚举
    PublicVariant,
    PrivateVariant,
}

// 3.6 模式匹配
// match 表达式
match some_value {
    Some(3) => println!("three"),
    Some(n) => println!("number: {}", n),       // 绑定值
    None => println!("nothing"),
    _ => println!("other"),                     // 通配模式
}

// 解构
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
let Point { x, y } = p;                         // 结构体解构

// 参数解构
fn print_coordinates(&(x, y): &(i32, i32)) {    // 函数参数解构
    println!("Current location: ({}, {})", x, y);
}

// 3.7 错误处理
// panic!
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

// 3.8 注解
#[derive(Debug, Clone, Copy)]                   // 派生宏
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]                                    // 条件编译
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*******************************************************************************
 * 4. 并发和异步
 *******************************************************************************/
// 4.1 线程和并发
// 线程
use std::thread;
let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});

// 消息传递
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
tx.send(String::from("hello")).unwrap();
let received = rx.recv().unwrap();

// 共享状态
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

// 4.2 异步编程
// 使用 async/await
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://example.com").await?;
    let text = resp.text().await?;
    Ok(text)
}

// 运行异步代码
#[tokio::main]
async fn main() {
    let result = fetch_data().await.unwrap();
    println!("{}", result);
}

// 4.3 Stream
use futures::stream::{self, StreamExt};

async fn stream_example() {
    let mut stream = stream::iter(1..=3);
    while let Some(x) = stream.next().await {
        println!("Got: {}", x);
    }
}

// 4.4 并行迭代器
use rayon::prelude::*;
let sum: i32 = (0..1000).par_iter().sum();

/*******************************************************************************
 * 5. 高级特性
 *******************************************************************************/
// 5.1 闭包
// 基本闭包
let add = |x, y| x + y;
let result = add(1, 2);

// 捕获环境
let x = 4;
let equal_to_x = |z| z == x;

// 闭包类型
let closure = |x| x + 1;                         // Fn
let mut counter = 0;
let mut closure = || counter += 1;               // FnMut
let closure = move || counter;                   // FnOnce

// 作为参数
fn apply<F>(f: F) where F: FnOnce() {
    f();
}

// 5.2 宏
// 声明宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// 属性宏
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 过程宏
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// 5.3 Unsafe Rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}

// 解引用裸指针
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    *r2 = 10;
}

/*******************************************************************************
 * 6. 测试和文档
 *******************************************************************************/
// 6.1 单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "panic message")]
    fn test_panic() {
        panic!("panic message");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// 6.2 集成测试
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_external() {
        assert_eq!(crate::add(2, 2), 4);
    }
}

// 6.3 基准测试
#[bench]
fn bench_add(b: &mut test::Bencher) {
    b.iter(|| {
        add(2, 2)
    });
}

/*******************************************************************************
 * 7. 错误处理
 *******************************************************************************/
// 7.1 Result 类型
fn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("file.txt")
}

// 使用 ? 运算符
fn read_file_short() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}

// 7.2 Option 类型
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("user1"))
    } else {
        None
    }
}