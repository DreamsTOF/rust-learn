# 01 所有权基础

所有权（Ownership）是 Rust 最核心的概念，它决定了内存管理的安全性和高效性。本章练习将通过 20 道循序渐进的题目，帮助你掌握所有权的转移（move）、克隆（clone）、Copy 语义以及作用域与 Drop 等基础知识。

### 练习 01-01: String 的所有权转移

> 难度：⭐
> 类似 C++ 的 std::move（移动语义），但 Rust 默认就是移动，不需要显式调用

创建一个 `String` 变量，将其赋值给另一个变量，然后尝试使用第一个变量。

```rust
fn main() {
    // TODO: 创建一个 String 变量 s1，内容为 "hello"
    
    // TODO: 将 s1 的所有权转移给 s2
    
    // TODO: 打印 s2 的内容（这样可以）
    
    // TODO: 取消下面的注释，观察编译错误
    // println!("{}", s1);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    // println!("{}", s1); // 编译错误：s1 的所有权已转移
}
```

**说明：** `String` 类型没有实现 `Copy` trait，因此赋值操作会转移（move）所有权。转移后原变量 `s1` 不再有效，编译器会阻止对其访问。
</details>

### 练习 01-02: 多次所有权转移

> 难度：⭐
> 类似 C++ 的移动链（move chain），链中的中间变量同样会失效

让一个 `String` 的所有权经过多次转移，最终打印最后一个变量。

```rust
fn main() {
    // TODO: 创建 String 变量 s1，内容为 "rust"
    
    // TODO: 将 s1 的所有权转移给 s2
    
    // TODO: 将 s2 的所有权转移给 s3
    
    // TODO: 打印 s3 的内容
    
    // TODO: 尝试取消注释下面两行，观察哪个会报错
    // println!("{}", s1);
    // println!("{}", s2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s1 = String::from("rust");
    let s2 = s1;
    let s3 = s2;
    println!("{}", s3);
    // println!("{}", s1); // 编译错误
    // println!("{}", s2); // 编译错误
}
```

**说明：** 所有权链上除了最后一个持有者外，所有之前的变量都已失效。`s1` 的所有权先移给 `s2`，再移给 `s3`，最终只有 `s3` 可以访问数据。
</details>

### 练习 01-03: 基本类型的 Copy 语义

> 难度：⭐
> 类似 C++ 的 int 赋值（复制值），与 Java 的基本类型赋值类似

体验 `i32` 这类实现 `Copy` trait 的类型在赋值时的行为。

```rust
fn main() {
    // TODO: 创建 i32 变量 x，值为 42
    
    // TODO: 将 x 赋值给 y
    
    // TODO: 打印 x 和 y，观察是否都能访问
    // println!("x = {}, y = {}", x, y);
    
    // TODO: 创建 bool 变量 a，值为 true
    
    // TODO: 将 a 赋值给 b
    
    // TODO: 打印 a 和 b
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);
    
    let a = true;
    let b = a;
    println!("a = {}, b = {}", a, b);
}
```

**说明：** `i32` 和 `bool` 都实现了 `Copy` trait，赋值时执行的是拷贝而非移动，因此两个变量都有效。所有实现 `Copy` 的类型在赋值后原变量仍然可用。
</details>

### 练习 01-04: 元组中的 Copy 与 move

> 难度：⭐
> 类似 C++ 的 std::pair 拷贝/移动，取决于元素类型

理解元组中不同类型元素在赋值时的表现差异——Copy 与 move 并存。

```rust
fn main() {
    // TODO: 创建一个元组 (i32, String)，内容为 (10, String::from("hi"))
    
    // TODO: 将元组赋值给另一个变量
    
    // TODO: 尝试打印原元组的第一个元素和第二个元素，观察哪个能访问
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let t1 = (10, String::from("hi"));
    let t2 = t1;
    // println!("{:?}", t1); // 编译错误：String 部分被 move
    println!("{}", t1.0); // i32 实现了 Copy，仍然可用
    println!("{}", t2.1);
}
```

**说明：** 元组是否实现 `Copy` 取决于其所有元素是否都实现了 `Copy`。这里 `String` 没有实现 `Copy`，所以整个元组发生 move，但元组被解构后，`i32` 字段仍可通过原变量访问（因为它是独立 Copy 的）——实际上这里更准确的说是元组整体 move 了，但 `t1.0` 能访问是因为元组被部分移出了字段，而 `i32` 是 Copy 的。
</details>

### 练习 01-05: 综合：move 与 clone 的选择

> 难度：⭐⭐⭐
> 类似 C++ 中区分深拷贝（clone）和移动（move）

给定一个 `String`，要求分别得到两个独立的变量都能访问其内容，同时还要体验 move 的行为。

```rust
fn main() {
    let s = String::from("hello world");
    
    // TODO: 用两种方式让 s1 和 s2 都能独立访问 "hello world"
    // 方式一：使用 clone
    // 方式二：使用 move（但之后还要用 s）
    
    // 最终需要能同时打印 s、s1、s2
    // println!("s: {}, s1: {}, s2: {}", s, s1, s2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s = String::from("hello world");
    let s1 = s.clone();
    let s2 = s1.clone();
    println!("s: {}, s1: {}, s2: {}", s, s1, s2);
}
```

**说明：** `.clone()` 执行深拷贝，堆上的数据被完整复制一份。如果需要原变量和新变量同时有效，就必须使用 clone。注意 `let s2 = s1.clone()` 而不是 `let s2 = s1`，否则 `s1` 的所有权会转移给 `s2`。
</details>

### 练习 01-06: 函数参数——所有权传入

> 难度：⭐
> 类似 C++ 的值传递（pass by value），但 Rust 需要显式关注所有权

编写一个函数接收 `String` 参数，在函数内打印它。

```rust
// TODO: 定义函数 print_string，接收一个 String 参数并打印

fn main() {
    let s = String::from("hello");
    // TODO: 调用 print_string 传入 s
    
    // TODO: 尝试取消注释下面一行，观察编译错误
    // println!("after call: {}", s);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    print_string(s);
    // println!("after call: {}", s); // 编译错误：所有权已转移给函数
}
```

**说明：** 将 `String` 传给函数时，所有权会转移给函数参数。函数调用后原变量不再有效。这是 Rust 所有权规则在函数调用中的体现。
</details>

### 练习 01-07: 函数参数——Copy 类型

> 难度：⭐
> 类似 C++ 的值传递（与 C++ 行为一致，但 Rust 通过 Copy trait 明确区分）

编写一个函数接收 `i32` 参数，体验 Copy 类型在函数调用中的行为。

```rust
// TODO: 定义函数 print_number，接收一个 i32 参数

fn main() {
    let n = 100;
    // TODO: 调用 print_number 传入 n
    
    // TODO: 再次打印 n，观察是否仍然有效
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn print_number(n: i32) {
    println!("n = {}", n);
}

fn main() {
    let n = 100;
    print_number(n);
    println!("after call: {}", n); // 仍然有效！
}
```

**说明：** `i32` 实现了 `Copy` trait，传递给函数时是自动拷贝，原变量的所有权不会转移，因此调用后仍可使用。
</details>

### 练习 01-08: 返回值转移所有权

> 难度：⭐⭐
> 类似 C++ 的返回值优化（RVO），但 Rust 保证移动语义

编写函数返回一个 `String`，体验从函数中转移出所有权。

```rust
// TODO: 定义函数 give_string，返回一个 String，内容为 "given"

fn main() {
    // TODO: 调用 give_string 将返回值赋给 s
    
    // TODO: 打印 s
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn give_string() -> String {
    String::from("given")
}

fn main() {
    let s = give_string();
    println!("{}", s);
}
```

**说明：** 函数返回值的所有权会转移给调用者。这里 `give_string` 内部创建的 `String` 的所有权通过返回值转移给了 `s`，这是 Rust 中常见的所有权传递方式。
</details>

### 练习 01-09: 接收并返回所有权

> 难度：⭐⭐
> 类似 C++ 中函数参数和返回值的所有权传递

编写函数接收一个 `String`，在函数内修改后返回它，从而"归还"所有权。

```rust
// TODO: 定义函数 append_world，接收 String 参数，追加 " world" 后返回

fn main() {
    let s = String::from("hello");
    // TODO: 调用 append_world 并将返回值赋给 s
    
    // TODO: 打印 s
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn append_world(mut s: String) -> String {
    s.push_str(" world");
    s
}

fn main() {
    let s = String::from("hello");
    let s = append_world(s);
    println!("{}", s);
}
```

**说明：** 函数接收所有权，处理后通过返回值将所有权"归还"。调用者需要重新赋值来获取所有权。这种方式虽然可行，但在实际中更常用借用（引用）来避免来回转移所有权。
</details>

### 练习 01-10: 多级传递综合

> 难度：⭐⭐⭐
> 类似 C++ 的复杂移动链，要求跟踪所有权流向

结合函数参数和返回值，完成多级所有权传递。

```rust
// TODO: 定义函数 process1，接收 String，添加 " one" 后返回
// TODO: 定义函数 process2，接收 String，添加 " two" 后返回
// TODO: 定义函数 process3，接收 String，添加 " three" 后返回

fn main() {
    let s = String::from("start");
    // TODO: 依次调用 process1、process2、process3
    // 要求每次调用后 s 仍然"活着"（通过重新绑定）
    
    // TODO: 打印最终的 s
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn process1(s: String) -> String {
    let mut s = s;
    s.push_str(" one");
    s
}

fn process2(s: String) -> String {
    let mut s = s;
    s.push_str(" two");
    s
}

fn process3(s: String) -> String {
    let mut s = s;
    s.push_str(" three");
    s
}

fn main() {
    let s = String::from("start");
    let s = process1(s);
    let s = process2(s);
    let s = process3(s);
    println!("{}", s);
}
```

**说明：** 每次函数调用都转移走所有权，又通过返回值归还。通过变量遮蔽（`let s = ...`）让 `s` 始终持有最新的所有权。这种模式被称为"拿-放-拿"（take-and-give-back），虽然能工作，但实践中更推荐使用引用。
</details>

### 练习 01-11: 栈数据 vs 堆数据

> 难度：⭐
> 类似 C++ 中栈上分配的 int 与堆上分配的 std::string 的区别

直观感受栈上分配的类型（如 `i32`）和堆上分配的类型（如 `String`）在赋值和大小上的区别。

```rust
fn main() {
    // TODO: 创建一个 i32，赋值给另一个变量，两者都可用（Copy）
    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b); // 都能打印
    
    // TODO: 创建一个 String，赋值给另一个变量，观察哪个能访问
    let s1 = String::from("I am on the heap");
    let s2 = s1;
    // TODO: 尝试打印 s1 和 s2，看哪个会报错
    println!("s2 = {}", s2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b);
    
    let s1 = String::from("I am on the heap");
    let s2 = s1;
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // 编译错误
}
```

**说明：** `i32` 是纯栈类型（实现了 Copy），赋值时直接复制值；`String` 的数据在堆上，赋值时只移动了栈上的指针（长度、容量），原变量失效。这就是 Rust 所有权设计的核心：避免双重释放。
</details>

### 练习 01-12: 字面量与 String 的区别

> 难度：⭐
> 类似 C++ 中字符串字面量（const char*）与 std::string 的区别

理解字符串字面量（`&str`）和 `String` 在所有权上的不同。

```rust
fn main() {
    // TODO: 创建一个字符串字面量 &str "hello"
    let s1 = "hello";
    
    // TODO: 将 s1 赋值给 s2
    let s2 = s1;
    
    // TODO: 打印 s1 和 s2——为什么都能访问？
    println!("s1 = {}, s2 = {}", s1, s2);
    
    // TODO: 创建一个 String
    let s3 = String::from("hello");
    let s4 = s3;
    // TODO: 下面这行会报错吗？
    // println!("s3 = {}", s3);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s1 = "hello";
    let s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2);
    
    let s3 = String::from("hello");
    let s4 = s3;
    // println!("s3 = {}", s3); // 编译错误
}
```

**说明：** 字符串字面量 `&str` 是引用类型（实现了 `Copy`），赋值只是复制引用，原变量仍然可用。而 `String` 拥有堆上数据的所有权，赋值会移动所有权。
</details>

### 练习 01-13: Copy 类型清单

> 难度：⭐⭐
> 类似 Java 的基本类型包装类（Integer、Boolean 等）的自动拆装箱，但 Rust 的 Copy 更接近 C++ 的 POD 类型

判断哪些类型实现了 `Copy`，并验证它们的行为。

```rust
fn main() {
    // TODO: 将下方声明补充完整，使每行都能通过编译（所有变量在赋值后仍可用）
    
    // 整数类型
    let x: i32 = 10;
    let y = x;
    println!("x = {}, y = {}", x, y);
    
    // 浮点类型
    // TODO: 创建 f64 变量
    
    // 布尔类型
    // TODO: 创建 bool 变量
    
    // 字符类型
    // TODO: 创建 char 变量
    
    // 元组（仅含 Copy 类型）
    // TODO: 创建 (i32, f64) 元组
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x: i32 = 10;
    let y = x;
    println!("x = {}, y = {}", x, y);
    
    let a: f64 = 3.14;
    let b = a;
    println!("a = {}, b = {}", a, b);
    
    let flag: bool = true;
    let flag2 = flag;
    println!("flag = {}, flag2 = {}", flag, flag2);
    
    let c: char = 'R';
    let d = c;
    println!("c = {}, d = {}", c, d);
    
    let t1: (i32, f64) = (1, 2.5);
    let t2 = t1;
    println!("t1 = {:?}, t2 = {:?}", t1, t2);
}
```

**说明：** Rust 中所有标量类型（整数、浮点、bool、char）以及只包含 Copy 元素的元组都实现了 `Copy` trait。它们赋值时执行的是按位拷贝，原变量仍然有效。
</details>

### 练习 01-14: 自定义类型与 Copy

> 难度：⭐⭐
> 类似 C++ 中可通过 `= default` 生成拷贝构造函数的 POD 类型

理解用户定义的类型如何实现 Copy（需要同时派生 Copy 和 Clone trait）。

```rust
// TODO: 补全派生属性，使 Point 结构体支持 Copy
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    // TODO: 将 p1 赋值给 p2
    
    // TODO: 打印 p1 和 p2 的位置
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}
```

**说明：** 自定义类型默认不实现 `Copy`。需要显式添加 `#[derive(Copy, Clone)]`，且要求所有字段也都实现了 `Copy`。`Clone` 是 `Copy` 的超集——必须先实现 `Clone` 才能实现 `Copy`。
</details>

### 练习 01-15: Copy 与 move 混合场景

> 难度：⭐⭐⭐
> 类似 C++ 中同时存在拷贝和移动构造函数的复杂场景

在一个函数中同时处理 Copy 类型和 non-Copy 类型，理解它们的行为差异。

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// TODO: 给 Point 添加必要的派生属性使其支持 Copy

fn process(point: Point, label: String) {
    println!("Point: {:?}, label: {}", point, label);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let s = String::from("origin");
    
    // TODO: 调用 process，要求调用后 p 和 s 仍可用
    
    // TODO: 打印 p 和 s
    // println!("p = {:?}, s = {}", p, s);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn process(point: Point, label: String) {
    println!("Point: {:?}, label: {}", point, label);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let s = String::from("origin");
    
    process(p, s.clone());
    
    println!("p = {:?}, s = {}", p, s);
}
```

**说明：** `Point` 实现了 `Copy`，传给函数时会自动拷贝，原变量 `p` 仍有效。但 `String` 是 non-Copy 类型，必须调用 `.clone()` 复制一份传入函数，原 `s` 才能继续使用。
</details>

### 练习 01-16: 作用域与 Drop

> 难度：⭐
> 类似 C++ 的 RAII（析构函数在作用域结束时自动调用）

观察变量在离开作用域时自动调用 drop 的行为。

```rust
fn main() {
    // TODO: 创建一个内部作用域
    {
        let s = String::from("temporary");
        println!("inside: {}", s);
        // TODO: 在此处 s 应该被 drop
    }
    
    // TODO: 尝试在此处访问 s，观察编译错误
    // println!("outside: {}", s);
    
    println!("still in main");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    {
        let s = String::from("temporary");
        println!("inside: {}", s);
    }
    // s 在此处已被 drop
    
    // println!("outside: {}", s); // 编译错误
    println!("still in main");
}
```

**说明：** 当变量离开作用域时，Rust 会自动调用 `drop` 释放堆上内存。这类似于 C++ 的 RAII 机制，确保了资源的确定性释放。
</details>

### 练习 01-17: 多个变量的 Drop 顺序

> 难度：⭐
> 类似 C++ 中局部变量的析构顺序（后创建的先析构）

观察多个变量在同一作用域中离开时的 drop 顺序。

```rust
fn main() {
    let s1 = String::from("first");
    let s2 = String::from("second");
    let s3 = String::from("third");
    
    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
    // TODO: 猜测 s1、s2、s3 被 drop 的顺序
    // 提示：与创建顺序相反（后进先出）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s1 = String::from("first");
    let s2 = String::from("second");
    let s3 = String::from("third");
    
    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
}
// drop 顺序：s3 → s2 → s1（后创建的先 drop）
```

**说明：** Rust 中变量的 drop 顺序与创建顺序相反——后创建的变量先被 drop。这是编译器自动生成的析构顺序，与 C++ 的栈对象析构顺序一致。虽然本练习无法直接从输出看出 drop 顺序，但你可通过在类型上实现 `Drop` trait 来观察，后续章节会涉及。
</details>

### 练习 01-18: move 后再赋值

> 难度：⭐⭐
> 类似 C++ 中将移动后的对象重新赋值（但 Rust 编译器会阻止使用已移动的值）

体验所有权转移后，原变量重新赋值会怎样——Rust 允许重新绑定。

```rust
fn main() {
    let mut s1 = String::from("hello");
    
    // TODO: 将 s1 的所有权转移给 s2
    
    // TODO: 此时 s1 已失效，但我们可以重新为 s1 赋值
    // 创建一个新的 String "world" 赋给 s1
    
    // TODO: 打印 s1 和 s2
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1;
    s1 = String::from("world");
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

**说明：** 即使 `s1` 的所有权已经转移，它仍然是有效的变量名（只是未初始化），我们可以重新为其赋值。这相当于让 `s1` 重新获得一个新的 `String` 的所有权。
</details>

### 练习 01-19: 条件分支中的所有权

> 难度：⭐⭐
> 类似 C++ 中 if-else 分支中对象的移动，但 Rust 的编译器检查更严格

在不同条件分支中处理所有权，理解所有权的流动与 if/else 的关系。

```rust
fn main() {
    let s = String::from("hello");
    
    // TODO: 根据条件将 s 的所有权转移到不同的新变量
    let condition = true;
    
    if condition {
        // TODO: 将 s 的所有权移入此分支
        // println!("if branch: {}", s);
    } else {
        // TODO: 这里也尝试使用 s？
        // println!("else branch: {}", s);
    }
    
    // TODO: 在此处还能访问 s 吗？为什么？
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s = String::from("hello");
    let condition = true;
    
    if condition {
        let _s = s;
        println!("if branch: {}", _s);
    } else {
        // 此分支无法访问 s，因为 s 已经在 if 分支中被移动
        // 这里实际上会编译错误，因为 s 可能已被 move
    }
    
    // 此处无法访问 s，因为所有权已在 if 分支中转移
}
```

**说明：** 实际上在 if-else 中这样使用会导致编译错误，因为编译器无法确定 `s` 是否被移走。更好的做法是避免在分支中直接 move 一个可能在多个分支使用的变量，或使用 Clone。这里的示例展示了编译器如何严格防止使用可能已被移动的值。
</details>

### 练习 01-20: 综合所有权流转（含 C++ 对照）

> 难度：⭐⭐⭐
> 类似 C++ 中移动语义与拷贝语义的混合使用，对照理解 Rust 和 C++ 的差异

完成一个综合场景：从输入字符串中提取单词并构建新的字符串集合，体验完整的所有权流转。

```rust
// TODO: 定义函数 extract_word，接收 String，返回 (String, String)
// 要求：将传入的字符串按空格分割，返回第一个单词和剩余部分

fn main() {
    let input = String::from("hello world rust");
    
    // TODO: 调用 extract_word，获取第一个单词和剩余部分
    
    // TODO: 打印提取结果
    
    // TODO: 在调用后还能访问 input 吗？试试看
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn extract_word(s: String) -> (String, String) {
    let parts: Vec<&str> = s.splitn(2, ' ').collect();
    let first = parts[0].to_string();
    let rest = if parts.len() > 1 {
        parts[1].to_string()
    } else {
        String::new()
    };
    (first, rest)
}

fn main() {
    let input = String::from("hello world rust");
    let (first, rest) = extract_word(input);
    println!("first word: {}, rest: {}", first, rest);
    // println!("{}", input); // 编译错误：所有权已转移
}
```

**说明：** `extract_word` 接收了 `input` 的所有权，通过返回值将处理后的字符串所有权归还给调用者。调用者通过解构元组获取两个新字符串的所有权。这展示了 Rust 中典型的所有权传递模式：函数拿走去处理，再通过返回值归还。C++ 中可以使用 `std::move` 实现类似效果，但 C++ 中被移动的对象仍然可以访问（只是处于有效但未指定的状态），而 Rust 编译器会直接阻止使用已移动的变量。
</details>
