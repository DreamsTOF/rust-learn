# 20 宏

宏（Macros）是 Rust 强大的元编程工具，允许你编写生成代码的代码。`macro_rules!` 声明宏通过模式匹配在 AST（抽象语法树）层面工作，比 C/C++ 的 `#define` 文本替换更安全、更强大。本章练习涵盖 `macro_rules!` 基础语法、模式匹配、重复（repetition）、`derive` 宏的使用以及编写实用宏。

---

### 练习 20-01: 定义第一个宏

> 难度：⭐⭐
> 类似 C 的 `#define`，但 Rust 宏在 AST 层面工作

补全 `greet!` 宏的定义，使其在调用时打印 `"Hello, Rust!"`。

```rust
// TODO: 补全宏定义
macro_rules! greet {
    // TODO: 当宏被调用时，打印 "Hello, Rust!"
}

fn main() {
    greet!();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! greet {
    () => {
        println!("Hello, Rust!");
    };
}

fn main() {
    greet!();
}
```

**说明：** `macro_rules!` 用于定义声明宏。`()` 是匹配模式——这里匹配空的调用。`=>` 后是要生成的代码。宏以 `!` 结尾调用。

</details>

---

### 练习 20-02: 带参数的宏

> 难度：⭐⭐

补全 `greet!` 宏，使其接收一个名字参数并打印 `"Hello, {name}!"`。

```rust
macro_rules! greet {
    ($name:expr) => {
        // TODO: 补全宏体，打印 "Hello, {name}!"
    };
}

fn main() {
    greet!("World");
    greet!("Rust");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!("World");
    greet!("Rust");
}
```

**说明：** `$name:expr` 中的 `$` 表示宏变量，`expr` 是片段类型说明符（fragment specifier），表示匹配一个表达式。在宏体中通过 `$name` 使用该变量。

</details>

---

### 练习 20-03: 模式匹配宏

> 难度：⭐⭐

`math_op!` 宏支持通过不同关键词进行不同运算。补全 `sub` 和 `mul` 两个匹配臂。

```rust
macro_rules! math_op {
    (add $a:tt, $b:tt) => {
        $a + $b
    };
    // TODO: 实现 sub（减法）
    // TODO: 实现 mul（乘法）
}

fn main() {
    let sum = math_op!(add 5, 3);
    let diff = math_op!(sub 10, 4);
    let prod = math_op!(mul 6, 7);
    println!("{} {} {}", sum, diff, prod);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! math_op {
    (add $a:tt, $b:tt) => {
        $a + $b
    };
    (sub $a:tt, $b:tt) => {
        $a - $b
    };
    (mul $a:tt, $b:tt) => {
        $a * $b
    };
}

fn main() {
    let sum = math_op!(add 5, 3);
    let diff = math_op!(sub 10, 4);
    let prod = math_op!(mul 6, 7);
    println!("{} {} {}", sum, diff, prod);
}
```

**说明：** `macro_rules!` 支持多个匹配臂，每个臂由模式 + 展开代码组成。`tt`（token tree）片段类型匹配单个 token 或括号分组，适合作为运算数。

</details>

---

### 练习 20-04: 多规则匹配宏

> 难度：⭐⭐

补全 `describe!` 宏，根据不同前缀处理不同类型的参数。

```rust
macro_rules! describe {
    (num $n:expr) => {
        println!("数字: {}", $n);
    };
    // TODO: 添加 str 规则，接收字符串并打印
    // TODO: 添加 vec 规则，接收 Vec 并打印
}

fn main() {
    describe!(num 42);
    describe!(str "hello");
    describe!(vec vec![1, 2, 3]);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! describe {
    (num $n:expr) => {
        println!("数字: {}", $n);
    };
    (str $s:expr) => {
        println!("字符串: {}", $s);
    };
    (vec $v:expr) => {
        println!("向量: {:?}", $v);
    };
}

fn main() {
    describe!(num 42);
    describe!(str "hello");
    describe!(vec vec![1, 2, 3]);
}
```

**说明：** 宏的模式可以包含字面量 token（如 `num`、`str`、`vec`），用于区分不同的匹配分支。这类似于函数重载的效果。

</details>

---

### 练习 20-05: 挑战 — 四则运算宏

> 难度：⭐⭐

实现 `calc!` 宏，使其支持 `+`、`-`、`*`、`/` 四种运算。提示：使用 `tt` 片段类型。

```rust
// TODO: 实现 calc! 宏
// calc!(1 + 2) => 3
// calc!(3 * 4) => 12
// calc!(10 - 3) => 7
// calc!(20 / 5) => 4

fn main() {
    println!("{}", calc!(1 + 2));
    println!("{}", calc!(3 * 4));
    println!("{}", calc!(10 - 3));
    println!("{}", calc!(20 / 5));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! calc {
    ($a:tt + $b:tt) => {
        $a + $b
    };
    ($a:tt - $b:tt) => {
        $a - $b
    };
    ($a:tt * $b:tt) => {
        $a * $b
    };
    ($a:tt / $b:tt) => {
        $a / $b
    };
}

fn main() {
    println!("{}", calc!(1 + 2));
    println!("{}", calc!(3 * 4));
    println!("{}", calc!(10 - 3));
    println!("{}", calc!(20 / 5));
}
```

**说明：** 使用 `tt`（token tree）可以匹配单个 token（如数字、变量名），配合运算符字面量实现简单的 DSL。注意 `expr` 会贪婪匹配，而 `tt` 每次只匹配一个 token。

</details>

---

### 练习 20-06: 重复 — 构建数组

> 难度：⭐⭐

补全 `make_array!` 宏，使用 `$()*` 重复语法将多个值放入数组。

```rust
macro_rules! make_array {
    // TODO: 使用 $()* 将多个值放入数组，用逗号分隔
}

fn main() {
    let arr = make_array![1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);
    assert_eq!(arr[2], 3);
    println!("{:?}", arr);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! make_array {
    ($($x:expr),*) => {
        [$($x),*]
    };
}

fn main() {
    let arr = make_array![1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);
    assert_eq!(arr[2], 3);
    println!("{:?}", arr);
}
```

**说明：** `$($x:expr),*` 中的 `$()*` 表示零次或多次重复，`,` 是重复之间的分隔符。展开时 `[$($x),*]` 也会重复生成对应的元素。

</details>

---

### 练习 20-07: 重复 — 逐个打印

> 难度：⭐⭐

补全 `print_all!` 宏，使用 `$()*` 语法逐个打印传入的所有参数，每个参数占一行。

```rust
macro_rules! print_all {
    // TODO: 使用 $()* 匹配多个表达式参数
    ($($x:expr),*) => {
        // TODO: 逐个打印每个参数
    };
}

fn main() {
    print_all!("A", "B", "C");
    // 期望输出三行: A  B  C
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! print_all {
    ($($x:expr),*) => {
        $(println!("{}", $x);)*
    };
}

fn main() {
    print_all!("A", "B", "C");
}
```

**说明：** `$(println!("{}", $x);)*` 在展开时会对每个匹配的 `$x` 生成一条 `println!` 语句。`$()*` 在展开侧同样使用重复语法，每个重复单元独立生成代码。

</details>

---

### 练习 20-08: 补全 — 求和宏

> 难度：⭐⭐

实现 `sum!` 宏，接收多个整数并返回它们的和。提示：可以用 `0 $(+ $x)*` 的技巧。

```rust
// TODO: 实现 sum! 宏
// 例如 sum!(1, 2, 3) 应返回 6

fn main() {
    let total = sum!(1, 2, 3, 4, 5);
    assert_eq!(total, 15);
    println!("Sum = {}", total);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}

fn main() {
    let total = sum!(1, 2, 3, 4, 5);
    assert_eq!(total, 15);
    println!("Sum = {}", total);
}
```

**说明：** `0 $(+ $x)*` 展开为 `0 + 1 + 2 + 3 + 4 + 5`。这种"初始值 + 重复运算"的模式是宏重复的常见用法。

</details>

---

### 练习 20-09: 补全 — 最大值宏

> 难度：⭐⭐

补全 `max!` 宏，接收多个整数并返回其中的最大值。

```rust
// TODO: 实现 max! 宏
// 提示：用第一个值作为初始最大值，然后依次比较

fn main() {
    let m = max!(3, 7, 2, 9, 5);
    assert_eq!(m, 9);
    println!("Max = {}", m);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! max {
    ($x:expr $(, $rest:expr)*) => {
        {
            let mut m = $x;
            $(m = std::cmp::max(m, $rest);)*
            m
        }
    };
}

fn main() {
    let m = max!(3, 7, 2, 9, 5);
    assert_eq!(m, 9);
    println!("Max = {}", m);
}
```

**说明：** 模式 `$x:expr $(, $rest:expr)*` 将第一个参数单独匹配，剩余参数通过重复匹配。展开时用 `std::cmp::max` 逐一比较更新最大值。

</details>

---

### 练习 20-10: 挑战 — 嵌套重复构建矩阵

> 难度：⭐⭐

实现 `matrix!` 宏，使用 `;` 分隔行，`,` 分隔列，创建 `Vec<Vec<i32>>`。

```rust
// TODO: 实现 matrix! 宏
// matrix![1, 2, 3; 4, 5, 6] 应创建 vec![vec![1,2,3], vec![4,5,6]]

fn main() {
    let m = matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    println!("{:?}", m);
    assert_eq!(m[0][1], 2);
    assert_eq!(m[2][0], 7);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! matrix {
    ($($($x:expr),*);*) => {
        vec![$(vec![$($x),*]),*]
    };
}

fn main() {
    let m = matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    println!("{:?}", m);
    assert_eq!(m[0][1], 2);
    assert_eq!(m[2][0], 7);
}
```

**说明：** 嵌套 `$()*` 实现二维重复：外层 `$()*` 匹配以 `;` 分隔的行，内层 `$()*` 匹配行内以 `,` 分隔的元素。展开时对应生成嵌套的 `vec!`。

</details>

---

### 练习 20-11: 使用 Derive — Debug

> 难度：⭐⭐
> 类似其他语言的注解/属性，但 derive 会自动生成 trait 实现

为 `Point` 添加适当的 derive 使其支持 `{:?}` 格式化打印。

```rust
// TODO: 为 Point 添加 derive，使其支持 {:?} 打印

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

**说明：** `#[derive(Debug)]` 自动为类型实现 `std::fmt::Debug` trait，使其支持 `{:?}` 格式输出，是调试时最常用的 derive。

</details>

---

### 练习 20-12: 使用 Derive — Clone

> 难度：⭐⭐

为 `Data` 添加适当的 derive 使其支持 `.clone()` 方法。

```rust
// TODO: 为 Data 添加 derive，使其支持 .clone()

struct Data {
    value: String,
}

fn main() {
    let d1 = Data { value: "hello".into() };
    let d2 = d1.clone();
    println!("{}", d2.value);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Clone)]
struct Data {
    value: String,
}

fn main() {
    let d1 = Data { value: "hello".into() };
    let d2 = d1.clone();
    println!("{}", d2.value);
}
```

**说明：** `#[derive(Clone)]` 自动生成 `clone` 方法实现对所有字段的逐一遍制。注意 `Clone` 是显式复制（需要调用 `.clone()`），区别于 `Copy`（自动按位复制）。

</details>

---

### 练习 20-13: 使用 Derive — PartialEq

> 难度：⭐⭐

添加适当的 derive 使 `Point` 支持 `==` 比较。

```rust
// TODO: 添加 derive，使 Point 支持 == 比较

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    assert!(a == b);
    println!("相等！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    assert!(a == b);
    println!("相等！");
}
```

**说明：** `#[derive(PartialEq)]` 自动实现 `PartialEq` trait，允许使用 `==` 和 `!=` 运算符。比较逻辑为逐字段比较——所有字段相等则实例相等。

</details>

---

### 练习 20-14: 使用 Derive — 组合多个 Derive

> 难度：⭐⭐

添加适当的 derive，使 `Point` 同时支持 `Debug`、`Clone`、`PartialEq`。

```rust
// TODO: 添加 derive，使 Point 同时支持 Debug、Clone、PartialEq

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("{:?}", p2);
    assert!(p1 == p2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("{:?}", p2);
    assert!(p1 == p2);
}
```

**说明：** `#[derive(Debug, Clone, PartialEq)]` 可以同时派生多个 trait，用逗号分隔。这是 Rust 中非常常见的组合，尤其在定义数据模型时几乎成为标配。

</details>

---

### 练习 20-15: 挑战 — 为枚举添加 Derive

> 难度：⭐⭐

为 `Color` 枚举添加适当的 derive，使其支持 `==` 比较、`.clone()` 复制和 `{:?}` 打印。

```rust
// TODO: 为 Color 枚举添加适当的 derive

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c1 = Color::Red;
    let c2 = Color::Red;
    assert!(c1 == c2);
    let c3 = c1.clone();
    println!("{:?}", c3);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c1 = Color::Red;
    let c2 = Color::Red;
    assert!(c1 == c2);
    let c3 = c1.clone();
    println!("{:?}", c3);
}
```

**说明：** `derive` 宏对枚举同样有效。`Debug` 生成枚举变体名称和关联值的输出格式，`Clone` 生成逐字段复制，`PartialEq` 生成逐个变体比较。

</details>

---

### 练习 20-16: 编写简化版 vec!

> 难度：⭐⭐⭐
> 这是 Rust 元编程的核心能力——用宏减少样板代码

实现 `my_vec!` 宏，接收任意数量的表达式并创建 `Vec`。

```rust
// TODO: 实现 my_vec! 宏
// my_vec![1, 2, 3] => vec![1, 2, 3]

fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    assert_eq!(v.len(), 5);
    assert_eq!(v[2], 3);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    assert_eq!(v.len(), 5);
    assert_eq!(v[2], 3);
}
```

**说明：** 这是 `vec!` 宏的简化实现。`$()*` 重复生成多条 `v.push($x)` 语句。注意外层使用块表达式 `{ ... }` 来包含多条语句并返回 `v`。

</details>

---

### 练习 20-17: 编写 hashmap! 宏

> 难度：⭐⭐⭐

实现 `hashmap!` 宏，使用 `key => value` 语法快速创建 `HashMap`。

```rust
use std::collections::HashMap;

// TODO: 实现 hashmap! 宏
// hashmap!("one" => 1, "two" => 2)

fn main() {
    let map = hashmap!("one" => 1, "two" => 2, "three" => 3);
    println!("{:?}", map);
    assert_eq!(map["one"], 1);
    assert_eq!(map["three"], 3);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

fn main() {
    let map = hashmap!("one" => 1, "two" => 2, "three" => 3);
    println!("{:?}", map);
    assert_eq!(map["one"], 1);
    assert_eq!(map["three"], 3);
}
```

**说明：** 使用 `=>` 作为键值对分隔符是 Rust 宏中常见的 DSL 风格。重复模式 `$($key:expr => $value:expr),*` 配对匹配键和值。

</details>

---

### 练习 20-18: 编写 set! 宏

> 难度：⭐⭐⭐

实现 `set!` 宏，接收多个值创建 `HashSet`（自动去重）。

```rust
use std::collections::HashSet;

// TODO: 实现 set! 宏
// set![1, 2, 3, 2, 1] => 包含 {1, 2, 3} 的 HashSet

fn main() {
    let s = set![1, 2, 3, 2, 1];
    println!("{:?}", s);
    assert_eq!(s.len(), 3);
    assert!(s.contains(&1));
    assert!(s.contains(&2));
    assert!(s.contains(&3));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashSet;

macro_rules! set {
    ($($x:expr),*) => {
        {
            let mut set = HashSet::new();
            $(set.insert($x);)*
            set
        }
    };
}

fn main() {
    let s = set![1, 2, 3, 2, 1];
    println!("{:?}", s);
    assert_eq!(s.len(), 3);
    assert!(s.contains(&1));
    assert!(s.contains(&2));
    assert!(s.contains(&3));
}
```

**说明：** `HashSet` 自动去重，插入重复元素不会增加集合大小。宏的重复插入模式与 `my_vec!` 类似，但 `HashSet` 的 `insert` 会自动处理重复值。

</details>

---

### 练习 20-19: 编写生成函数的宏

> 难度：⭐⭐⭐

实现 `create_func!` 宏，根据传入的名称生成一个函数，该函数打印 `"Hello from {name}!"`。

```rust
// TODO: 实现 create_func! 宏
// create_func!(greet) 应生成 fn greet() { println!("Hello from greet!"); }

fn main() {
    create_func!(greet);
    greet();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! create_func {
    ($name:ident) => {
        fn $name() {
            println!("Hello from {}!", stringify!($name));
        }
    };
}

fn main() {
    create_func!(greet);
    greet();
}
```

**说明：** `ident` 片段类型匹配标识符。`stringify!` 是 Rust 内置宏，将标识符或表达式转换为字符串字面量。宏可以生成函数定义——这是 Rust 宏元编程的强大体现。Rust 允许在函数体内嵌套定义函数（即 inner function），因此宏展开的 `fn` 定义在 `main` 内是合法的。

</details>

---

### 练习 20-20: 编写计时宏

> 难度：⭐⭐⭐

实现 `timed!` 宏，接收一个代码块，测量并打印其执行时间（使用 `std::time::Instant`）。

```rust
// TODO: 实现 timed! 宏，接收代码块并测量执行时间
// 提示：用 $block:block 匹配代码块

fn main() {
    timed!({
        let mut sum = 0u64;
        for i in 0..1000000 {
            sum += i;
        }
        println!("Sum = {}", sum);
    });
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
macro_rules! timed {
    ($block:block) => {
        {
            let start = std::time::Instant::now();
            let result = $block;
            let duration = start.elapsed();
            println!("执行时间: {:?}", duration);
            result
        }
    };
}

fn main() {
    timed!({
        let mut sum = 0u64;
        for i in 0..1000000 {
            sum += i;
        }
        println!("Sum = {}", sum);
    });
}
```

**说明：** `block` 片段类型匹配一个块表达式（即 `{ ... }`）。宏在代码块执行前后记录时间并计算差值。这种"包裹"模式是实现日志、计时、重试等切面功能的常用手法。

</details>
