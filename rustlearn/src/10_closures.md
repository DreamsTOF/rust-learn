
# 10 闭包与函数式

闭包（closure）是 Rust 中支持函数式编程的核心特性，类似于 C++ 的 lambda 表达式或 Java 的 lambda，但 Rust 闭包通过三种 trait（`Fn`、`FnMut`、`FnOnce`）精细控制捕获方式。本章练习将帮助你掌握闭包的定义、环境捕获、`move` 语义、作为参数和返回值的用法，以及高阶编程技巧。

### 练习 10-01: 定义简单闭包

> 难度：⭐
> 类似 C++ 的 `[](int x) { return x + 1; }`，Rust 闭包用 `|x| x + 1`

补全代码，定义三个不同风格的闭包：显式类型、隐式类型和多语句体。

```rust
fn main() {
    // TODO: 定义一个闭包 add_one，接受 i32 返回 i32（显式类型标注）
    // let add_one = ...;

    // TODO: 定义一个闭包 square，自动推断类型
    // let square = ...;

    // TODO: 定义一个闭包 factorial，计算 n 的阶乘（多语句体需要花括号）
    // let factorial = ...;

    println!("5 + 1 = {}", add_one(5));
    println!("6 的平方 = {}", square(6));
    println!("5! = {}", factorial(5));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };
    let square = |x| x * x;

    let factorial = |n: u32| -> u32 {
        (1..=n).product()
    };

    println!("5 + 1 = {}", add_one(5));
    println!("6 的平方 = {}", square(6));
    println!("5! = {}", factorial(5));
}
```

**说明：** Rust 闭包用 `|参数列表| 表达式` 定义。单表达式可以省略花括号，多语句必须用 `{}`。参数和返回值可以显式标注类型，也可以让编译器推断。`(1..=n).product()` 展示了闭包体内可以使用迭代器方法。
</details>

### 练习 10-02: 闭包的类型推断

> 难度：⭐
> 类似 C++ 的 `auto lambda = [](auto x) { ... }`，但 Rust 类型推断在第一次调用后就固定

补全代码，让编译器自动推断闭包参数和返回值类型。

```rust
fn main() {
    // TODO: 定义 doubler 闭包，将输入乘以 2（不写类型，让编译器推断）
    // let doubler = ...;

    // TODO: 定义 concat 闭包，将两个字符串拼接（不写类型，让编译器推断）
    // let concat = ...;

    println!("{}", doubler(21));       // 42
    println!("{}", doubler(3.14));     // 6.28
    // println!("{}", doubler("x"));   // 取消注释看看编译器报错
    println!("{}", concat("Hello, ", "Rust!"));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let doubler = |x| x * 2;
    let concat = |a, b| format!("{}{}", a, b);

    println!("{}", doubler(21));
    println!("{}", doubler(3.14));
    // println!("{}", doubler("x"));   // 编译错误：类型已推断为 f64
    println!("{}", concat("Hello, ", "Rust!"));
}
```

**说明：** 闭包的类型在**第一次调用**时被固定下来。`doubler` 先传入 `21`（整数），立即推断为整数乘法，下一行 `3.14`（浮点数）会触发隐式类型转换问题——实际上第一次调用 `21` 推断为 `i32`，后续 `3.14` 会造成编译错误，因为 `i32` 和 `f64` 不匹配。这体现了 Rust 类型推断的严格性：闭包参数类型一旦确定就不可更改。
</details>

### 练习 10-03: 函数转闭包

> 难度：⭐⭐
> 类似 C++ 中函数指针可以赋给 `std::function`，Rust 中普通函数可以赋值给闭包变量

定义一个普通函数，然后将其赋值给一个闭包类型的变量，并调用它。

```rust
// TODO: 定义一个普通函数 add，接受两个 i32 返回 i32

fn main() {
    // TODO: 将 add 函数赋值给闭包变量 closure
    // let closure = ...;

    println!("{}", closure(3, 4)); // 7
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let closure = add;
    println!("{}", closure(3, 4));
}
```

**说明：** 在 Rust 中，普通函数实现了所有三种闭包 trait（`Fn`、`FnMut`、`FnOnce`），因此函数名可以直接赋值给期望闭包类型的变量。函数可以看作是"不捕获任何环境"的特殊闭包。这种统一性使得函数和闭包在 Rust 中可以互换使用，简化了高阶函数的设计。
</details>

### 练习 10-04: 闭包转函数指针

> 难度：⭐⭐
> 类似 C++ 的无捕获 lambda 可以转换为函数指针，Rust 同理

如果一个闭包不捕获任何环境变量，它可以被转换为函数指针。补全代码，将一个无捕获闭包转换为 `fn` 类型并调用。

```rust
fn main() {
    // TODO: 定义一个无捕获闭包 multiply，然后将其转换为函数指针 fn(i32, i32) -> i32
    // let multiply = ...;
    // let func_ptr: fn(i32, i32) -> i32 = ...;

    println!("{}", func_ptr(6, 7)); // 42

    // TODO: 直接将闭包作为参数传递给接受 fn 的函数
    fn apply_func(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        f(a, b)
    }
    // println!("{}", apply_func(???, 10, 20));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let multiply = |a, b| a * b;
    let func_ptr: fn(i32, i32) -> i32 = multiply;

    println!("{}", func_ptr(6, 7));

    fn apply_func(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        f(a, b)
    }
    println!("{}", apply_func(multiply, 10, 20));
}
```

**说明：** 只有**不捕获环境**的闭包才能转换为函数指针（`fn` 类型）。一旦闭包捕获了外部变量，它就不再是"普通函数"，不能转换为 `fn`。这个限制与 C++ 中无捕获 lambda 可转换为函数指针的规则一致。
</details>

### 练习 10-05: 闭包基础综合

> 难度：⭐⭐⭐
> 综合运用闭包定义、类型推断、函数指针转换

实现一个简单的计算器，使用闭包来表示不同的运算。

```rust
// TODO: 定义一个函数 make_calculator，根据运算符字符串返回对应的闭包
// 支持的运算符: "+", "-", "*", "/"
// 对于除法，除数为 0 时返回 f64::NAN
fn make_calculator(op: &str) -> impl Fn(f64, f64) -> f64 {
    // TODO: 返回对应的闭包
    unimplemented!()
}

fn main() {
    let add = make_calculator("+");
    let sub = make_calculator("-");
    let mul = make_calculator("*");
    let div = make_calculator("/");

    println!("加法: {}", add(10.0, 3.0));    // 13.0
    println!("减法: {}", sub(10.0, 3.0));    // 7.0
    println!("乘法: {}", mul(10.0, 3.0));    // 30.0
    println!("除法: {}", div(10.0, 3.0));    // 3.333...
    println!("除零: {}", div(10.0, 0.0));    // NaN

    // TODO: 检查 make_calculator 返回的闭包是否可以作为 fn 指针传递
    fn apply(f: fn(f64, f64) -> f64, x: f64, y: f64) -> f64 {
        f(x, y)
    }
    // 下面的代码是否能编译？为什么？
    // println!("{}", apply(make_calculator("+"), 1.0, 2.0));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn make_calculator(op: &str) -> impl Fn(f64, f64) -> f64 {
    match op {
        "+" => |a, b| a + b,
        "-" => |a, b| a - b,
        "*" => |a, b| a * b,
        "/" => |a, b| if b == 0.0 { f64::NAN } else { a / b },
        _ => panic!("不支持的运算符: {}", op),
    }
}

fn main() {
    let add = make_calculator("+");
    let sub = make_calculator("-");
    let mul = make_calculator("*");
    let div = make_calculator("/");

    println!("加法: {}", add(10.0, 3.0));
    println!("减法: {}", sub(10.0, 3.0));
    println!("乘法: {}", mul(10.0, 3.0));
    println!("除法: {}", div(10.0, 3.0));
    println!("除零: {}", div(10.0, 0.0));

    fn apply(f: fn(f64, f64) -> f64, x: f64, y: f64) -> f64 {
        f(x, y)
    }
    // 不能编译：make_calculator 返回 impl Fn，不是 fn 指针
    // 因为这些闭包捕获了 op 字符串的引用（match 分支中的字符串字面量）
    // 但等等——它们实际上没有捕获任何变量，只是返回不同的闭包
    // 然而 impl Fn 在编译期是不透明类型，编译器无法保证它是 fn 指针
    // println!("{}", apply(make_calculator("+"), 1.0, 2.0));
}
```

**说明：** `impl Fn(f64, f64) -> f64` 是返回闭包 trait 对象的写法。注意 `make_calculator` 返回的闭包不捕获环境（字符串字面量是 `&'static str`，但 `op` 参数已被 match 消耗），但从类型系统角度看，`impl Fn` 不透明，不能自动转换为 `fn` 指针。如果希望返回 `fn` 指针，需要明确指定返回类型为 `fn(f64, f64) -> f64`。
</details>

### 练习 10-06: 不可变捕获——读取环境变量

> 难度：⭐
> 类似 C++ 的 `[&]` 捕获，Rust 默认不可变借用

闭包可以捕获其定义作用域中的变量。补全代码，让闭包以不可变借用方式读取外部变量。

```rust
fn main() {
    let base: i32 = 10;
    let numbers = vec![1, 2, 3, 4, 5];

    // TODO: 定义一个闭包 add_base，捕获 base 并将每个数字加上 base
    // let add_base = ...;

    let result: Vec<i32> = numbers.into_iter().map(add_base).collect();
    println!("{:?}", result); // [11, 12, 13, 14, 15]

    // base 在这里还能用吗？验证一下
    println!("base 仍然可用: {}", base);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let base: i32 = 10;
    let numbers = vec![1, 2, 3, 4, 5];

    let add_base = |x| x + base;

    let result: Vec<i32> = numbers.into_iter().map(add_base).collect();
    println!("{:?}", result);

    println!("base 仍然可用: {}", base);
}
```

**说明：** 默认情况下，闭包以**不可变借用**（`&T`）的方式捕获环境变量。由于只是读取 `base`，闭包实现了 `Fn` trait。`base` 在闭包定义后仍然可用，因为不可变借用不影响所有者的使用权。这是最常用的捕获方式，不会转移所有权或修改变量。
</details>

### 练习 10-07: 不可变捕获——字符串拼接

> 难度：⭐
> 类似 C++ 的 `[&]` 捕获字符串，Rust 闭包捕获 &String

补全代码，让闭包捕获外部变量并生成问候语。

```rust
fn main() {
    let greeting = String::from("你好");
    let names = vec!["Alice", "Bob", "Charlie"];

    // TODO: 定义一个闭包 greet，捕获 greeting，为每个名字生成问候语
    // let greet = ...;

    let greetings: Vec<String> = names.into_iter().map(greet).collect();
    println!("{:?}", greetings); // ["你好, Alice!", "你好, Bob!", "你好, Charlie!"]

    println!("greeting 仍然可用: {}", greeting);

    // TODO: 能否捕获 greeting 的引用以节省克隆？尝试修改闭包使其借用 greeting
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let greeting = String::from("你好");
    let names = vec!["Alice", "Bob", "Charlie"];

    let greet = |name| format!("{}, {}!", greeting, name);

    let greetings: Vec<String> = names.into_iter().map(greet).collect();
    println!("{:?}", greetings);

    println!("greeting 仍然可用: {}", greeting);
}
```

**说明：** 闭包 `greet` 以不可变借用捕获 `greeting`。注意 `format!` 宏只借用 `greeting`，不取得所有权，所以 `greeting` 仍然可用。如果闭包内尝试修改或消费 `greeting`，就需要 `FnMut` 或 `FnOnce` 了。
</details>

### 练习 10-08: FnMut——可变捕获

> 难度：⭐⭐
> 类似 C++ 的 `[&]` 捕获 mutable 变量，但 Rust 必须显式声明 mut

闭包可以修改其捕获的变量，前提是变量本身是 `mut` 的，且闭包被声明为 `mut`。

```rust
fn main() {
    let mut counter = 0;

    // TODO: 定义一个可变闭包 increment，每次调用使 counter 加 1
    // let mut increment = ...;

    increment();
    increment();
    increment();

    println!("counter = {}", counter); // 3

    // TODO: 定义一个闭包 get_counter，返回当前 counter 值（不可变借用）
    // let get_counter = ...;
    // println!("get_counter = {}", get_counter()); // 3
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
    };

    increment();
    increment();
    increment();

    println!("counter = {}", counter);

    let get_counter = || counter;
    // 注意：这里不能同时使用 increment 和 get_counter，因为可变借用和不可变借用冲突
    // println!("get_counter = {}", get_counter());
}
```

**说明：** 当闭包需要修改捕获的变量时，它实现的是 `FnMut` trait，而不是 `Fn`。调用可变闭包时，必须将闭包变量声明为 `mut`。注意：`increment` 可变借用了 `counter`，之后不能再创建 `get_counter`（不可变借用），因为 Rust 的借用规则不允许可变借用与不可变借用同时存在。
</details>

### 练习 10-09: FnMut——累加器

> 难度：⭐⭐
> 类似 C++ 的 `[&]` 捕获 mutable 容器

使用 `FnMut` 闭包实现一个累加器函数，统计传入的历史值。

```rust
// TODO: 实现 make_accumulator 函数，返回一个闭包
// 该闭包每次接受一个 i32 参数，返回所有传入参数的总和
fn make_accumulator() -> impl FnMut(i32) -> i32 {
    // TODO: 使用可变捕获来累计总和
    unimplemented!()
}

fn main() {
    let mut acc = make_accumulator();
    println!("{}", acc(10)); // 10
    println!("{}", acc(20)); // 30
    println!("{}", acc(30)); // 60
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn make_accumulator() -> impl FnMut(i32) -> i32 {
    let mut sum = 0;
    move |x| {
        sum += x;
        sum
    }
}

fn main() {
    let mut acc = make_accumulator();
    println!("{}", acc(10));
    println!("{}", acc(20));
    println!("{}", acc(30));
}
```

**说明：** `make_accumulator` 内部定义了一个可变变量 `sum`，闭包捕获它并在每次调用时累加。这里使用了 `move` 关键字将 `sum` 的所有权移入闭包。返回类型 `impl FnMut(i32) -> i32` 表明返回的是一个可变闭包。调用时闭包变量本身也必须是 `mut` 的。
</details>

### 练习 10-10: FnOnce——消费捕获

> 难度：⭐⭐⭐
> 类似 C++ 的 `[x = std::move(captured)]`，Rust 用 FnOnce 表示消费语义

当一个闭包消费（获取所有权并销毁）捕获的变量时，它只能被调用一次，实现 `FnOnce` trait。

```rust
fn main() {
    let name = String::from("Rust");

    // TODO: 定义一个闭包 consume_name，消费 name 的所有权
    // 提示：在闭包体中直接将 name 作为返回值
    // let consume_name = ...;

    // println!("{}", consume_name()); // "Rust"
    // println!("{}", consume_name()); // 编译错误：闭包已被消费

    // TODO: 验证 name 的所有权已被转移
    // println!("name = {}", name); // 编译错误：name 已被 move
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let name = String::from("Rust");

    let consume_name = || name;

    println!("{}", consume_name());
    // println!("{}", consume_name()); // 编译错误
    // println!("name = {}", name); // 编译错误
}
```

**说明：** 当闭包体内将捕获的变量直接返回（或移出）时，该闭包只实现了 `FnOnce`。`FnOnce` 意味着闭包只能被调用一次，因为第一次调用已经将捕获的值移出了闭包。Rust 编译器会根据闭包体中的操作自动判断实现哪个 trait：只读借用 → `Fn`，可变借用 → `FnMut`，消费所有权 → `FnOnce`。
</details>

### 练习 10-11: move 关键字——转移所有权

> 难度：⭐
> 类似 C++ 的 `[captured = std::move(var)]`，Rust 的 move 关键字强制所有权转移

当一个闭包需要在闭包创建时就获得变量的所有权时，可以使用 `move` 关键字。

```rust
fn main() {
    let data = vec![1, 2, 3];

    // TODO: 使用 move 关键字创建闭包，将 data 的所有权移入闭包
    // let closure = ...;

    println!("闭包内: {:?}", closure()); // [1, 2, 3]

    // 下面的代码能否编译？为什么？
    // println!("data = {:?}", data);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let data = vec![1, 2, 3];

    let closure = move || data;

    println!("闭包内: {:?}", closure());

    // println!("data = {:?}", data); // 编译错误：data 所有权已移入闭包
}
```

**说明：** `move` 关键字强制闭包获取捕获变量的**所有权**，而不是借用。这在闭包创建时就将所有权转移，无论闭包体中实际如何使用。即使闭包只是读取变量，`move` 也会转移所有权。`move` 的必要性在线程场景中最为明显——当闭包需要跨线程传递时，必须独占所有权以避免悬垂引用。
</details>

### 练习 10-12: move 闭包复制语义

> 难度：⭐
> 类似 C++ 的 `[=]` 按值捕获，Rust 的 move 对 Copy 类型也有相同效果

对于实现了 `Copy` trait 的类型（如整数、布尔等），`move` 闭包的效果是复制而非转移。

```rust
fn main() {
    let x = 42;
    let y = true;

    // TODO: 使用 move 关键字创建闭包，捕获 x 和 y
    // let closure = ...;

    println!("闭包内: {:?}", closure()); // (42, true)

    // 对于 Copy 类型，即使 move 了，原变量仍然可用
    println!("x = {}, y = {}", x, y); // 42, true
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x = 42;
    let y = true;

    let closure = move || (x, y);

    println!("闭包内: {:?}", closure());

    println!("x = {}, y = {}", x, y);
}
```

**说明：** 对于实现了 `Copy` 的类型（如整数、布尔、浮点数等），`move` 关键字的效果实际上是复制而不是转移所有权。因此 `x` 和 `y` 在闭包创建后仍然可用。对于非 `Copy` 类型（如 `String`、`Vec`），`move` 会真正转移所有权，原变量失效。
</details>

### 练习 10-13: move + 线程

> 难度：⭐⭐
> 类似 C++ 中 lambda 在线程中需要按值捕获，Rust 的 std::thread::spawn 要求闭包是 'static

`std::thread::spawn` 要求传入的闭包满足 `'static` 生命周期，这意味着它必须拥有所有捕获变量的所有权。`move` 关键字是满足这一要求的标准方式。

```rust
use std::thread;

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    // TODO: 创建一个线程，计算 numbers 中所有元素的和
    // 使用 move 关键字将 numbers 的所有权移入闭包
    // let handle = thread::spawn(move || { ... });

    // println!("和: {}", handle.join().unwrap()); // 150

    // 下面的代码能否编译？为什么？
    // println!("numbers = {:?}", numbers);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let handle = thread::spawn(move || {
        numbers.iter().sum::<i32>()
    });

    println!("和: {}", handle.join().unwrap());

    // println!("numbers = {:?}", numbers); // 编译错误
}
```

**说明：** `thread::spawn` 要求传入的闭包是 `'static`（不包含任何借用）。`move` 关键字将 `numbers` 的所有权从主线程转移到新线程的闭包中，因此主线程不再拥有 `numbers` 的所有权。这是 Rust 中并发编程的典型模式——通过所有权转移保证线程安全。
</details>

### 练习 10-14: move + 多线程任务

> 难度：⭐⭐
> 类似 C++ 中使用 std::thread 传递复杂 lambda

创建多个线程，每个线程处理不同的数据片段。

```rust
use std::thread;

fn main() {
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    // TODO: 为 data 中的每个子向量创建一个线程，计算其元素之和
    // 收集所有线程的句柄，然后等待所有线程完成并汇总总和
    // let handles: Vec<_> = data.into_iter().enumerate().map(|(i, chunk)| { ... }).collect();

    // let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    // println!("总和: {}", total); // 45
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;

fn main() {
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let handles: Vec<_> = data.into_iter().enumerate().map(|(i, chunk)| {
        thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            println!("线程 {} 结果: {}", i, sum);
            sum
        })
    }).collect();

    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("总和: {}", total);
}
```

**说明：** `data.into_iter()` 消费 `data`，将每个子 `Vec<i32>` 的所有权交给迭代器。`move` 闭包捕获每个子向量并在线程中独立处理。`enumerate` 为每个块添加索引以便追踪。这种模式是 Rust 中"数据并行"的典型写法——将数据分割后通过 `move` 闭包发送到不同线程。
</details>

### 练习 10-15: move 语义综合

> 难度：⭐⭐⭐
> 综合运用 move 闭包、所有权转移、Copy 类型与非 Copy 类型的区别

实现一个函数 `create_counter`，返回一个能生成自增 ID 的闭包。每次调用返回一个新的、唯一的 ID（从 1 开始递增）。要求使用 `move` 语义。

```rust
// TODO: 实现 create_counter 函数
// 返回一个闭包，每次调用返回下一个自增 ID
// 使用 move 将计数器变量移入闭包
fn create_counter() -> impl FnMut() -> u32 {
    // TODO
    unimplemented!()
}

fn main() {
    let mut counter = create_counter();
    println!("ID: {}", counter()); // 1
    println!("ID: {}", counter()); // 2
    println!("ID: {}", counter()); // 3

    let mut counter2 = create_counter();
    println!("counter2 ID: {}", counter2()); // 1（独立的计数器）

    // 验证 create_counter 返回的闭包是 FnMut，需要 mut 才能调用多次
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn create_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn main() {
    let mut counter = create_counter();
    println!("ID: {}", counter());
    println!("ID: {}", counter());
    println!("ID: {}", counter());

    let mut counter2 = create_counter();
    println!("counter2 ID: {}", counter2());
}
```

**说明：** `create_counter` 内部声明了一个局部变量 `count`，通过 `move` 关键字将其所有权移入闭包。每次闭包被调用时修改并返回 `count`。不同闭包拥有独立的 `count` 副本，互不干扰。注意调用方必须将闭包声明为 `mut`，因为 `FnMut` 需要可变引用。
</details>

### 练习 10-16: 闭包作为函数参数（Fn 约束）

> 难度：⭐
> 类似 C++ 的模板参数接受 lambda，Rust 用泛型 + Fn trait 约束

编写一个函数，接受一个闭包作为参数，并对某个值应用该闭包。

```rust
// TODO: 实现 apply_twice 函数
// 接受一个闭包 f 和一个值 x，返回 f(f(x))
fn apply_twice<T>(f: ???, x: T) -> T {
    // TODO
}

fn main() {
    let double = |x| x * 2;
    println!("{}", apply_twice(double, 5)); // 20 (5 → 10 → 20)

    let square = |x| x * x;
    println!("{}", apply_twice(square, 3)); // 81 (3 → 9 → 81)
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn apply_twice<T>(f: impl Fn(T) -> T, x: T) -> T {
    f(f(x))
}

fn main() {
    let double = |x| x * 2;
    println!("{}", apply_twice(double, 5));

    let square = |x| x * x;
    println!("{}", apply_twice(square, 3));
}
```

**说明：** `impl Fn(T) -> T` 是泛型参数 `T` 上的 `Fn` trait 约束，表示接受一个从 `T` 到 `T` 的闭包。`Fn` 是"可以多次调用且不修改环境"的闭包 trait。使用 `impl Trait` 语法可以简洁地表达"接受实现了某 trait 的类型"。
</details>

### 练习 10-17: 闭包参数——Fn、FnMut、FnOnce 的区别

> 难度：⭐
> 类似 C++ 中区分 mutable lambda 和普通 lambda，Rust 用三种 trait 精确控制

编写三个函数，分别接受 `Fn`、`FnMut`、`FnOnce` 约束的闭包，展示三者的区别。

```rust
// TODO: 实现 call_fn，接受 Fn 闭包
fn call_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    // TODO: 调用两次 f
}

// TODO: 实现 call_fn_mut，接受 FnMut 闭包
fn call_fn_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    // TODO: 调用两次 f
}

// TODO: 实现 call_fn_once，接受 FnOnce 闭包
fn call_fn_once(f: impl FnOnce(i32) -> i32, x: i32) -> i32 {
    // TODO: 调用一次 f
}

fn main() {
    let a = 10;
    let fn_closure = |x| x + a; // Fn
    println!("call_fn: {}", call_fn(fn_closure, 5));

    let mut sum = 0;
    let mut fn_mut_closure = |x| { sum += x; sum }; // FnMut
    println!("call_fn_mut: {}", call_fn_mut(fn_mut_closure, 5));

    let name = String::from("Rust");
    let fn_once_closure = || name.len() as i32; // FnOnce
    println!("call_fn_once: {}", call_fn_once(fn_once_closure, 0));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn call_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

fn call_fn_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

fn call_fn_once(f: impl FnOnce(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let a = 10;
    let fn_closure = |x| x + a;
    println!("call_fn: {}", call_fn(fn_closure, 5));

    let mut sum = 0;
    let mut fn_mut_closure = |x| { sum += x; sum };
    println!("call_fn_mut: {}", call_fn_mut(fn_mut_closure, 5));

    let name = String::from("Rust");
    let fn_once_closure = |x| name.len() as i32 + x;
    println!("call_fn_once: {}", call_fn_once(fn_once_closure, 0));
}
```

**说明：** 三种 trait 的关系：`Fn` 是 `FnMut` 的子 trait，`FnMut` 是 `FnOnce` 的子 trait。
- `Fn`：不可变捕获，可多次调用，不修改环境。
- `FnMut`：可变捕获，可多次调用，可修改环境。
- `FnOnce`：消费捕获，只能调用一次。
接受 `FnOnce` 的函数最通用（可以接受任何闭包），但只能调用一次。接受 `Fn` 的函数约束最强（只接受不修改环境的闭包）。
</details>

### 练习 10-18: 闭包作为返回值——返回闭包

> 难度：⭐⭐
> 类似 C++ 中返回 lambda 需要 `auto` 或 `std::function`，Rust 用 `impl Fn`

实现一个函数，根据参数返回不同的闭包。

```rust
// TODO: 实现 create_multiplier 函数
// 接受一个 i32 因子，返回一个闭包，该闭包将输入乘以此因子
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    // TODO: 返回捕获 factor 的闭包
    unimplemented!()
}

fn main() {
    let double = create_multiplier(2);
    let triple = create_multiplier(3);

    println!("5 * 2 = {}", double(5)); // 10
    println!("5 * 3 = {}", triple(5)); // 15
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let double = create_multiplier(2);
    let triple = create_multiplier(3);

    println!("5 * 2 = {}", double(5));
    println!("5 * 3 = {}", triple(5));
}
```

**说明：** `impl Fn(i32) -> i32` 作为返回类型表示"返回一个实现了 `Fn(i32) -> i32` 的类型"。注意 `move` 关键字的使用：将 `factor` 的所有权移入闭包，避免悬垂引用（因为 `factor` 是函数局部变量，函数返回后将被销毁）。`impl Trait` 在返回值位置允许返回不透明类型，且编译器在编译期知道具体类型。
</details>

### 练习 10-19: 返回 FnMut 和 FnOnce 闭包

> 难度：⭐⭐
> 返回带状态的闭包需要 FnMut，返回消费闭包需要 FnOnce

实现两个函数，一个返回 `FnMut` 闭包（带状态），一个返回 `FnOnce` 闭包（消费捕获的值）。

```rust
// TODO: 实现一个函数 make_prefixer，返回 FnMut 闭包
// 每次调用时，在原前缀后追加 "!" 并返回 "前缀: 输入"
// 例如：第一次调用 prefixer("A") -> "!A"，第二次 -> "!!A"
fn make_prefixer() -> impl FnMut(&str) -> String {
    // TODO
    unimplemented!()
}

// TODO: 实现函数 make_greeter，返回 FnOnce 闭包
// 该闭包接受名字并拼接问候语，但消费掉捕获的 greeting
fn make_greeter(greeting: String) -> impl FnOnce(&str) -> String {
    // TODO
    unimplemented!()
}

fn main() {
    let mut prefixer = make_prefixer();
    println!("{}", prefixer("A"));  // !A
    println!("{}", prefixer("B"));  // !!B

    let greeter = make_greeter(String::from("Hello"));
    println!("{}", greeter("World")); // Hello, World!
    // println!("{}", greeter("Rust")); // 编译错误：FnOnce只能调用一次
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn make_prefixer() -> impl FnMut(&str) -> String {
    let mut prefix = String::new();
    move |s| {
        prefix.push('!');
        format!("{}{}", prefix, s)
    }
}

fn make_greeter(greeting: String) -> impl FnOnce(&str) -> String {
    move |name| format!("{}, {}!", greeting, name)
}

fn main() {
    let mut prefixer = make_prefixer();
    println!("{}", prefixer("A"));
    println!("{}", prefixer("B"));

    let greeter = make_greeter(String::from("Hello"));
    println!("{}", greeter("World"));
    // println!("{}", greeter("Rust")); // 编译错误
}
```

**说明：** `make_prefixer` 返回 `FnMut` 闭包，因为它在内部修改了捕获的 `prefix` 字符串。每次调用在 `prefix` 后追加 `!`，记录了状态变化。`make_greeter` 返回 `FnOnce` 闭包，因为闭包体内的 `format!` 宏消费了 `greeting`（将其 move 进 `format!`）。
</details>

### 练习 10-20: 高阶函数综合

> 难度：⭐⭐⭐
> 综合运用闭包、迭代器、高阶函数——类似 C++ 中 std::transform 配合 lambda 的复杂场景

实现一个数据处理管道，使用高阶函数对整数序列进行多次变换。

```rust
// TODO: 实现 compose 函数，组合两个闭包
// compose(f, g) 返回一个闭包，等价于 f(g(x))
fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    // TODO
    unimplemented!()
}

// TODO: 实现 pipeline 函数，接受一个初始值和一系列变换闭包
// 依次应用所有变换，返回最终结果
fn pipeline<T>(init: T, transforms: Vec<impl Fn(T) -> T>) -> T {
    // TODO
    unimplemented!()
}

fn main() {
    // 使用 compose 组合 "加 1" 和 "乘 2"
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let add_then_double = compose(double, add_one); // (x + 1) * 2
    println!("compose: {}", add_then_double(5)); // 12

    // 使用 pipeline 执行链式变换
    let transforms = vec![
        |x| x + 10,
        |x| x * 2,
        |x| x - 5,
    ];
    let result = pipeline(100, transforms);
    println!("pipeline: {}", result); // (100 + 10) * 2 - 5 = 215

    // 综合：结合迭代器、闭包和函数组合
    let numbers = 1..=10;
    let processed: Vec<i32> = numbers
        .filter(|x| x % 2 == 0)      // 偶数
        .map(|x| x * 3)              // 乘 3
        .map(|x| x + 1)              // 加 1
        .collect();
    println!("综合: {:?}", processed); // [7, 13, 19, 25, 31]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

fn pipeline<T>(init: T, transforms: Vec<impl Fn(T) -> T>) -> T {
    let mut result = init;
    for f in transforms {
        result = f(result);
    }
    result
}

fn main() {
    let add_one = |x| x + 1;
    let double = |x| x * 2;
    let add_then_double = compose(double, add_one);
    println!("compose: {}", add_then_double(5));

    let transforms = vec![
        |x| x + 10,
        |x| x * 2,
        |x| x - 5,
    ];
    let result = pipeline(100, transforms);
    println!("pipeline: {}", result);

    let numbers = 1..=10;
    let processed: Vec<i32> = numbers
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .map(|x| x + 1)
        .collect();
    println!("综合: {:?}", processed);
}
```

**说明：** 这是闭包与函数式编程的综合练习。
- `compose` 实现数学上的函数组合 `f ∘ g`，返回的闭包需要 `move` 捕获 `f` 和 `g` 的所有权（因为它们是栈上的局部变量）。
- `pipeline` 通过循环依次应用每个变换，展示了"处理管道"模式。
- 最后一部分展示了 Rust 迭代器链式调用的函数式编程风格——`filter`、`map` 等适配器都接受闭包作为参数，形成声明式的数据处理流程。
- 注意 `Vec<impl Fn(T) -> T>` 的写法：`Vec` 中每个元素都是相同的闭包类型（同一个具体类型），而不是 `Box<dyn Fn>`。实际应用中类型擦除版本更常见：`Vec<Box<dyn Fn(T) -> T>>`。
</details>
