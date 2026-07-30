# 06 模式匹配

模式匹配（Pattern Matching）是 Rust 最强大的特性之一。与 C++ 的 `switch` 或 Java 的 `switch` 不同，Rust 的 `match` 表达式支持解构、守卫、绑定、穷尽性检查等丰富功能。本章练习涵盖 `match`、`if let`、`while let`、解构、守卫、`@` 绑定等模式匹配相关的核心概念。

### 练习 06-01: match 基础

> 难度：⭐
> 类似 C++ 的 switch，但 match 必须穷尽所有可能

补全代码，使用 `match` 将数字 1-3 转换为对应的中文数字。

```rust
fn number_to_chinese(n: u32) -> &'static str {
    // TODO: 使用 match 匹配 n
    // 1 -> "一", 2 -> "二", 3 -> "三"
    // 其他数字 -> "未知"
    match n {
        // TODO: 补全分支
    }
}

fn main() {
    println!("{}", number_to_chinese(1));
    println!("{}", number_to_chinese(3));
    println!("{}", number_to_chinese(5));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn number_to_chinese(n: u32) -> &'static str {
    match n {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "未知",
    }
}

fn main() {
    println!("{}", number_to_chinese(1));
    println!("{}", number_to_chinese(3));
    println!("{}", number_to_chinese(5));
}
```

**说明：** `match` 表达式依次匹配每个分支，`_` 是通配模式，匹配所有剩余情况。编译器要求 match 必须是穷尽的（exhaustive），`_` 确保所有未列出的值都被覆盖。
</details>

### 练习 06-02: 穷尽性匹配

> 难度：⭐
> 类似 C++ 的 switch，但如果没有 default 且遗漏了枚举变体，Rust 编译器会报错

补全代码，使用 `match` 处理 `Weekday` 枚举的所有变体。

```rust
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn is_weekend(day: Weekday) -> bool {
    // TODO: 使用 match 匹配所有变体
    // Saturday 和 Sunday 返回 true，其余返回 false
    match day {
        // TODO: 补全分支（不要使用 _ 通配符，显式列出所有变体）
    }
}

fn main() {
    println!("周一是否周末: {}", is_weekend(Weekday::Monday));
    println!("周六是否周末: {}", is_weekend(Weekday::Saturday));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn is_weekend(day: Weekday) -> bool {
    match day {
        Weekday::Saturday | Weekday::Sunday => true,
        Weekday::Monday
        | Weekday::Tuesday
        | Weekday::Wednesday
        | Weekday::Thursday
        | Weekday::Friday => false,
    }
}

fn main() {
    println!("周一是否周末: {}", is_weekend(Weekday::Monday));
    println!("周六是否周末: {}", is_weekend(Weekday::Saturday));
}
```

**说明：** `match` 必须穷尽枚举的所有变体。可以使用 `|` 运算符在一个分支中匹配多个模式。如果遗漏了某个变体，编译器会给出明确的错误提示，这是 Rust 安全性的重要体现。
</details>

### 练习 06-03: match 分支与通配符 _

> 难度：⭐⭐
> 类似 C++ 的 switch default，但 Rust 的 `_` 不绑定变量

补全代码，编写一个评分函数，根据分数返回等级。使用 `_` 处理分数不在 0-100 的情况。

```rust
fn grade(score: i32) -> &'static str {
    // TODO: 使用 match 匹配分数范围
    // 90-100 -> "优秀"
    // 80-89  -> "良好"
    // 70-79  -> "中等"
    // 60-69  -> "及格"
    // 0-59   -> "不及格"
    // 其他分数 -> "无效分数"
    match score {
        // TODO: 补全分支
    }
}

fn main() {
    println!("95 分: {}", grade(95));
    println!("83 分: {}", grade(83));
    println!("45 分: {}", grade(45));
    println!("-10 分: {}", grade(-10));
    println!("150 分: {}", grade(150));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn grade(score: i32) -> &'static str {
    match score {
        90..=100 => "优秀",
        80..=89 => "良好",
        70..=79 => "中等",
        60..=69 => "及格",
        0..=59 => "不及格",
        _ => "无效分数",
    }
}

fn main() {
    println!("95 分: {}", grade(95));
    println!("83 分: {}", grade(83));
    println!("45 分: {}", grade(45));
    println!("-10 分: {}", grade(-10));
    println!("150 分: {}", grade(150));
}
```

**说明：** Rust 的 `match` 支持范围模式 `a..=b`（包含两端）。`_` 通配符匹配所有未被前面的分支捕获的值。分支顺序很重要，范围小的分支应放在前面。
</details>

### 练习 06-04: match 多模式组合

> 难度：⭐⭐
> 类似 C++ 的 switch fallthrough，但 Rust 用 `|` 显式组合

编写一个函数，判断一个字符是元音字母、辅音字母还是其他字符。

```rust
fn classify_char(c: char) -> &'static str {
    // TODO: 使用 match + | 组合模式
    // 元音（大小写）：a, e, i, o, u, A, E, I, O, U
    // 辅音：其他 ASCII 字母
    // 其他字符：非字母
    match c {
        // TODO: 补全分支
    }
}

fn main() {
    println!("'a': {}", classify_char('a'));
    println!("'Z': {}", classify_char('Z'));
    println!("'5': {}", classify_char('5'));
    println!("'!' : {}", classify_char('!'));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn classify_char(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => "元音",
        'a'..='z' | 'A'..='Z' => "辅音",
        _ => "其他字符",
    }
}

fn main() {
    println!("'a': {}", classify_char('a'));
    println!("'Z': {}", classify_char('Z'));
    println!("'5': {}", classify_char('5'));
    println!("'!' : {}", classify_char('!'));
}
```

**说明：** `|` 运算符可以在一个 match 分支中组合多个模式。注意 `a'..='z' | 'A'..='Z'` 的范围模式组合——由于元音分支在前面，这里的范围只会匹配到辅音字母。
</details>

### 练习 06-05: 挑战——match 综合

> 难度：⭐⭐⭐
> 类似 C++ 的 switch，但 Rust match 支持范围、多模式和值绑定

编写一个函数 `analyze_number`，根据输入的整数返回描述字符串。要求：
- 负数（且是偶数）-> "负偶数"
- 负数（且是奇数）-> "负奇数"
- 0 -> "零"
- 正数 1-10 且是偶数 -> "小偶数"
- 正数 1-10 且是奇数 -> "小奇数"
- 正数 > 10 且是偶数 -> "大偶数"
- 正数 > 10 且是奇数 -> "大奇数"

```rust
// TODO: 实现 analyze_number 函数
fn analyze_number(n: i32) -> String {
    // 使用 match 和元组模式
    // 提示：可以用 (n.signum(), n.abs() % 2) 或类似方式组合判断
    // TODO
}

fn main() {
    println!("-4: {}", analyze_number(-4));
    println!("-3: {}", analyze_number(-3));
    println!("0: {}", analyze_number(0));
    println!("6: {}", analyze_number(6));
    println!("7: {}", analyze_number(7));
    println!("42: {}", analyze_number(42));
    println!("99: {}", analyze_number(99));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn analyze_number(n: i32) -> String {
    match n {
        0 => "零".to_string(),
        n if n < 0 && n % 2 == 0 => "负偶数".to_string(),
        n if n < 0 => "负奇数".to_string(),
        1..=10 if n % 2 == 0 => "小偶数".to_string(),
        1..=10 => "小奇数".to_string(),
        n if n % 2 == 0 => "大偶数".to_string(),
        _ => "大奇数".to_string(),
    }
}

fn main() {
    println!("-4: {}", analyze_number(-4));
    println!("-3: {}", analyze_number(-3));
    println!("0: {}", analyze_number(0));
    println!("6: {}", analyze_number(6));
    println!("7: {}", analyze_number(7));
    println!("42: {}", analyze_number(42));
    println!("99: {}", analyze_number(99));
}
```

**说明：** 这个练习展示了 `match` 的多种高级用法：守卫（`if`）模式可以对同一范围做进一步细分；`0` 作为特殊情况优先匹配；守卫中可以使用任意布尔表达式。注意守卫中的变量名 `n` 与绑定的值同名，不影响外层变量。
</details>

### 练习 06-06: if let 基础

> 难度：⭐
> 类似 C++17 的 `if (auto x = get_opt())`，但 Rust 的 `if let` 语法更简洁

补全代码，使用 `if let` 从 `Option<i32>` 中取出值并打印。

```rust
fn main() {
    let a = Some(42);
    let b: Option<i32> = None;
    
    // TODO: 使用 if let 匹配 Some 并打印值
    if let Some(x) = a {
        // 打印 "a 的值是: {x}"
    }
    
    // TODO: 使用 if let 匹配 b，如果无值则打印 "b 是 None"
    if let Some(x) = b {
        println!("b 的值是: {}", x);
    } else {
        // TODO: 打印 "b 是 None"
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let a = Some(42);
    let b: Option<i32> = None;
    
    if let Some(x) = a {
        println!("a 的值是: {}", x);
    }
    
    if let Some(x) = b {
        println!("b 的值是: {}", x);
    } else {
        println!("b 是 None");
    }
}
```

**说明：** `if let` 是 `match` 的语法糖，只关心一个模式而忽略其他情况。`if let Some(x) = option` 类似 C++17 的 `if (auto x = get_opt())`，但 Rust 的版本将解构和条件判断合为一句。`else` 分支对应 `match` 中的 `_ =>`。
</details>

### 练习 06-07: if let 处理 Result

> 难度：⭐
> 类似 Java 的 `if (obj instanceof String s)`，但 Rust 解构更灵活

补全代码，使用 `if let` 处理 `Result` 类型，只关心成功的情况。

```rust
fn parse_and_double(input: &str) {
    // TODO: 使用 if let 处理 input.parse::<i32>()
    // 如果解析成功，打印两倍的值
    // 如果解析失败，什么都不做
    if let Ok(n) = input.parse::<i32>() {
        // TODO: 打印 "两倍: {n * 2}"
    }
    // else 分支可以省略，因为失败时我们什么都不做
}

fn main() {
    parse_and_double("21");
    parse_and_double("abc");
    parse_and_double("-5");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn parse_and_double(input: &str) {
    if let Ok(n) = input.parse::<i32>() {
        println!("两倍: {}", n * 2);
    }
}

fn main() {
    parse_and_double("21");
    parse_and_double("abc");
    parse_and_double("-5");
}
```

**说明：** `if let` 不仅适用于 `Option`，也适用于 `Result` 等枚举。`if let Ok(n) = result` 只关心成功的情况，忽略错误。配合 `parse` 这类返回 `Result` 的函数非常实用。省略 `else` 分支意味着失败时静默忽略。
</details>

### 练习 06-08: while let 基础

> 难度：⭐⭐
> 类似 C++ 的 while 循环配合 optional，Rust 用 while let 更简洁

补全代码，使用 `while let` 从迭代器中逐一出队所有元素。

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5].into_iter();
    
    // TODO: 使用 while let 循环取出并打印 numbers 中的所有元素
    // while let Some(n) = ... 
    while let Some(n) = numbers.next() {
        // TODO: 打印 n
    }
    
    println!("循环结束");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5].into_iter();
    
    while let Some(n) = numbers.next() {
        println!("{}", n);
    }
    
    println!("循环结束");
}
```

**说明：** `while let` 在模式匹配成功时持续循环，直到匹配失败。`numbers.next()` 返回 `Option<Item>`，当迭代器耗尽时返回 `None`，循环自动结束。这比手动检查 `Option` 并跳出循环更简洁。
</details>

### 练习 06-09: while let 处理可变队列

> 难度：⭐⭐
> 类似 C++ 中 while 循环弹出队列元素

补全代码，使用 `while let` 和 `Vec::pop` 持续移除向量中的元素。

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];
    
    // TODO: 使用 while let 循环从 stack 中 pop 元素并打印
    // 直到 stack 为空
    while let Some(top) = stack.pop() {
        // TODO: 打印 top，并打印当前 stack 的长度
        println!("弹出: {}", top);
        // 提示：stack.len() 可以获取当前长度
    }
    
    println!("栈已清空，长度: {}", stack.len());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];
    
    while let Some(top) = stack.pop() {
        println!("弹出: {}，剩余 {} 个元素", top, stack.len());
    }
    
    println!("栈已清空，长度: {}", stack.len());
}
```

**说明：** `Vec::pop` 移除最后一个元素并返回 `Option<T>`。`while let` 配合 `pop` 是处理动态数据集合的常见模式，代码简洁且意图清晰。每次迭代中 `stack.len()` 反映当前剩余元素数量。
</details>

### 练习 06-10: 挑战——if let + while let 综合

> 难度：⭐⭐⭐
> 综合运用 if let 和 while let 处理嵌套 Option 和 Result

实现一个函数 `process_mixed_data`，接收一个 `Vec<Option<Result<i32, &str>>>`，按以下规则处理：
- 如果是 `Some(Ok(n))` 且 n 为正数，将其累加
- 如果是 `Some(Ok(n))` 且 n 为负数，打印 "忽略负数: {n}"
- 如果是 `Some(Err(e))`，打印 "解析错误: {e}"
- 如果是 `None`，跳过不处理

使用 `if let`（而非 `match`）完成内部判断。

```rust
// TODO: 实现 process_mixed_data 函数
fn process_mixed_data(data: Vec<Option<Result<i32, &'static str>>>) -> i32 {
    let mut sum = 0;
    // 可以用 for 循环 + if let
    for item in data {
        // TODO: 先用 if let Some(...) 解包 Option
        // 内部再用 if let Ok(n) = ... 或 if let Err(e) = ... 处理 Result
    }
    sum
}

fn main() {
    let data = vec![
        Some(Ok(10)),
        Some(Err("格式错误")),
        None,
        Some(Ok(-5)),
        Some(Ok(20)),
        Some(Err("超限")),
        None,
        Some(Ok(30)),
    ];
    
    let result = process_mixed_data(data);
    println!("正数之和: {}", result); // 应输出 60（10 + 20 + 30）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn process_mixed_data(data: Vec<Option<Result<i32, &'static str>>>) -> i32 {
    let mut sum = 0;
    for item in data {
        if let Some(result) = item {
            if let Ok(n) = result {
                if n > 0 {
                    sum += n;
                } else {
                    println!("忽略负数: {}", n);
                }
            }
            if let Err(e) = result {
                println!("解析错误: {}", e);
            }
        }
    }
    sum
}

fn main() {
    let data = vec![
        Some(Ok(10)),
        Some(Err("格式错误")),
        None,
        Some(Ok(-5)),
        Some(Ok(20)),
        Some(Err("超限")),
        None,
        Some(Ok(30)),
    ];
    
    let result = process_mixed_data(data);
    println!("正数之和: {}", result);
}
```

**说明：** 嵌套的 `if let` 可以逐层解构复合类型。先解 `Option`，再解 `Result`。注意 `result` 在第一个 `if let` 中被匹配但未消费（`Result` 实现了 `Copy`），因此第二个 `if let` 仍然可用。这种逐层处理的方式比嵌套 `match` 更扁平、更易读。
</details>

### 练习 06-11: 元组解构基础

> 难度：⭐
> 类似 C++ 的 structured binding（C++17），但 Rust 的元组解构更常用

补全代码，使用 `match` 解构元组并匹配不同形态。

```rust
fn describe_point(point: (i32, i32)) -> &'static str {
    // TODO: 使用 match 解构元组 (x, y)
    // (0, 0) -> "原点"
    // (0, _) -> "在 Y 轴上"
    // (_, 0) -> "在 X 轴上"
    // (x, y) 且 x > 0 && y > 0 -> "第一象限"
    // (x, y) 且 x < 0 && y > 0 -> "第二象限"
    // (x, y) 且 x < 0 && y < 0 -> "第三象限"
    // (x, y) 且 x > 0 && y < 0 -> "第四象限"
    match point {
        (0, 0) => "原点",
        (0, _) => "在 Y 轴上",
        (_, 0) => "在 X 轴上",
        // TODO: 补全剩余分支（使用守卫 if）
    }
}

fn main() {
    println!("(0, 0): {}", describe_point((0, 0)));
    println!("(0, 5): {}", describe_point((0, 5)));
    println!("(3, 4): {}", describe_point((3, 4)));
    println!("(-3, 4): {}", describe_point((-3, 4)));
    println!("(-3, -4): {}", describe_point((-3, -4)));
    println!("(3, -4): {}", describe_point((3, -4)));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn describe_point(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "原点",
        (0, _) => "在 Y 轴上",
        (_, 0) => "在 X 轴上",
        (x, y) if x > 0 && y > 0 => "第一象限",
        (x, y) if x < 0 && y > 0 => "第二象限",
        (x, y) if x < 0 && y < 0 => "第三象限",
        (x, y) if x > 0 && y < 0 => "第四象限",
        _ => "未知位置",
    }
}

fn main() {
    println!("(0, 0): {}", describe_point((0, 0)));
    println!("(0, 5): {}", describe_point((0, 5)));
    println!("(3, 4): {}", describe_point((3, 4)));
    println!("(-3, 4): {}", describe_point((-3, 4)));
    println!("(-3, -4): {}", describe_point((-3, -4)));
    println!("(3, -4): {}", describe_point((3, -4)));
}
```

**说明：** `match` 可以对元组进行解构，`(0, _)` 表示第一个元素为 0、第二个任意。`_` 在模式中表示"忽略这个位置的值"。守卫 `if` 在模式匹配成功后增加额外条件判断。这类似 C++17 的 structured binding + if constexpr 组合。
</details>

### 练习 06-12: 元组解构与交换

> 难度：⭐
> 类似 Python 的元组解构赋值

补全代码，使用元组解构交换两个变量的值（不借助临时变量）。

```rust
fn main() {
    let mut a = 5;
    let mut b = 10;
    
    println!("交换前: a = {}, b = {}", a, b);
    
    // TODO: 使用元组解构交换 a 和 b 的值
    // 提示: (a, b) = (b, a);
    
    println!("交换后: a = {}, b = {}", a, b);
    
    // 进阶：解构三个变量
    let mut x = 1;
    let mut y = 2;
    let mut z = 3;
    
    // TODO: 一次性将 x, y, z 循环右移（x->y, y->z, z->x）
    // 使用元组解构
    
    println!("循环右移后: x = {}, y = {}, z = {}", x, y, z);
    // 应输出: x = 3, y = 1, z = 2
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut a = 5;
    let mut b = 10;
    
    println!("交换前: a = {}, b = {}", a, b);
    
    (a, b) = (b, a);
    
    println!("交换后: a = {}, b = {}", a, b);
    
    let mut x = 1;
    let mut y = 2;
    let mut z = 3;
    
    (x, y, z) = (z, x, y);
    
    println!("循环右移后: x = {}, y = {}, z = {}", x, y, z);
}
```

**说明：** Rust 支持元组解构赋值，`(a, b) = (b, a)` 优雅地交换两个变量。类似地，`(x, y, z) = (z, x, y)` 实现循环右移。这比使用临时变量的方式更简洁、更易读。
</details>

### 练习 06-13: 结构体解构

> 难度：⭐⭐
> 类似 C++ 的 structured binding 解构结构体

补全代码，使用 `match` 解构结构体 `Point` 和 `Rectangle`。

```rust
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> i32 {
    // TODO: 解构 Rectangle，计算面积
    // 提示：可以使用嵌套模式解构
    // Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } }
    match rect {
        // TODO: 补全模式
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 5, y: 0 },
    };
    
    println!("矩形面积: {}", rect_area(rect)); // 应输出 50
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> i32 {
    match rect {
        Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } => (x2 - x1) * (y1 - y2),
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 5, y: 0 },
    };
    
    println!("矩形面积: {}", rect_area(rect));
}
```

**说明：** Rust 支持嵌套结构体解构——`Rectangle { top_left: Point { x, y }, ... }`。由于 `Rectangle` 只有一种形态，`match` 只需要一个分支（实际上也可以使用 `let` 解构）。注意面积计算中 `(y1 - y2)` 因为 y 轴方向向下。
</details>

### 练习 06-14: 枚举解构

> 难度：⭐⭐
> 类似 Java 的 pattern matching for switch（预览特性），Rust 从第一天就支持

补全代码，为 `Temperature` 枚举实现 `convert_to_celsius` 方法。

```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    // TODO: 实现 convert_to_celsius 方法
    // Celsius 直接返回值
    // Fahrenheit 转换为摄氏度: (f - 32.0) * 5.0 / 9.0
    fn to_celsius(&self) -> f64 {
        match self {
            // TODO: 解构 Celsius 和 Fahrenheit
        }
    }
}

fn main() {
    let c = Temperature::Celsius(37.0);
    let f = Temperature::Fahrenheit(98.6);
    
    println!("37°C = {:.1}°C", c.to_celsius());
    println!("98.6°F = {:.1}°C", f.to_celsius());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (*f - 32.0) * 5.0 / 9.0,
        }
    }
}

fn main() {
    let c = Temperature::Celsius(37.0);
    let f = Temperature::Fahrenheit(98.6);
    
    println!("37°C = {:.1}°C", c.to_celsius());
    println!("98.6°F = {:.1}°C", f.to_celsius());
}
```

**说明：** 枚举解构时，元组形式变体 `Celsius(f64)` 的模式为 `Temperature::Celsius(c)`，其中 `c` 绑定到内部值。由于 `&self` 是引用，解构得到的是引用，所以需要使用 `*c` 解引用为 `f64`。
</details>

### 练习 06-15: 挑战——嵌套解构

> 难度：⭐⭐⭐
> 类似 Java 中复杂的 instanceof 嵌套判断，Rust 的模式匹配可以深度解构

给定一个嵌套的 JSON-like 枚举结构，实现一个 `extract_name` 函数，从嵌套的数据中提取用户名。

```rust
#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// TODO: 实现 extract_name 函数
// 从以下嵌套结构中提取 "name" 字段的值：
// JsonValue::Object 包含键值对列表，找到键为 "name" 的条目
// 如果 "name" 对应的是 JsonValue::String(s)，返回 Some(s)
// 否则返回 None
fn extract_name(value: &JsonValue) -> Option<&str> {
    match value {
        // TODO: 匹配 JsonValue::Object，遍历其中的 Vec<(String, JsonValue)>
        // 找到 key == "name" 且对应的 value 是 String 的情况
        // 提示：可以在 match 分支中使用守卫
    }
}

fn main() {
    let data = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Number(1.0)),
        ("name".to_string(), JsonValue::String("Alice".to_string())),
        ("age".to_string(), JsonValue::Number(30.0)),
    ]);
    
    println!("name: {:?}", extract_name(&data)); // 应输出 Some("Alice")
    
    let no_name = JsonValue::Object(vec![
        ("title".to_string(), JsonValue::String("Hello".to_string())),
    ]);
    println!("no name: {:?}", extract_name(&no_name)); // 应输出 None
    
    let invalid = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::Number(42.0)),
    ]);
    println!("name is number: {:?}", extract_name(&invalid)); // 应输出 None
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn extract_name(value: &JsonValue) -> Option<&str> {
    match value {
        JsonValue::Object(pairs) => {
            for (key, val) in pairs {
                if key == "name" {
                    if let JsonValue::String(s) = val {
                        return Some(s.as_str());
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn main() {
    let data = JsonValue::Object(vec![
        ("id".to_string(), JsonValue::Number(1.0)),
        ("name".to_string(), JsonValue::String("Alice".to_string())),
        ("age".to_string(), JsonValue::Number(30.0)),
    ]);
    
    println!("name: {:?}", extract_name(&data));
    
    let no_name = JsonValue::Object(vec![
        ("title".to_string(), JsonValue::String("Hello".to_string())),
    ]);
    println!("no name: {:?}", extract_name(&no_name));
    
    let invalid = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::Number(42.0)),
    ]);
    println!("name is number: {:?}", extract_name(&invalid));
}
```

**说明：** 嵌套解构配合循环和 `if let` 可以灵活处理深层嵌套的数据结构。`JsonValue::Object` 中的 `Vec<(String, JsonValue)>` 需要通过循环遍历，在找到匹配键名后再用 `if let` 解构值。这在处理 JSON 等树形数据时非常实用。
</details>

### 练习 06-16: match 守卫基础

> 难度：⭐
> 类似 C++ 的 switch + if 组合，但 Rust 的守卫直接关联到 match 分支

补全代码，使用 match 守卫判断一个数是否为 3 的倍数、是否在指定范围内。

```rust
fn check_number(n: i32) -> &'static str {
    // TODO: 使用 match + 守卫
    // n 是 3 的倍数 -> "3 的倍数"
    // n 在 10 到 20 之间（含）且不是 3 的倍数 -> "10-20 区间"
    // 其他 -> "其他"
    match n {
        // TODO: 补全分支
    }
}

fn main() {
    for n in [3, 9, 15, 22, 12, 17] {
        println!("{}: {}", n, check_number(n));
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn check_number(n: i32) -> &'static str {
    match n {
        n if n % 3 == 0 => "3 的倍数",
        10..=20 => "10-20 区间",
        _ => "其他",
    }
}

fn main() {
    for n in [3, 9, 15, 22, 12, 17] {
        println!("{}: {}", n, check_number(n));
    }
}
```

**说明：** 守卫 `if n % 3 == 0` 在模式匹配成功后额外执行条件判断。注意这里 `10..=20` 的分支在守卫分支之后——因为守卫分支已经捕获了 10-20 区间内的 3 的倍数（如 12、15），剩余的非 3 倍数才能落到第二个分支。
</details>

### 练习 06-17: match 守卫——解构 + 条件

> 难度：⭐
> 类似 C++ 的 switch 中解构并判断

补全代码，为 `Card` 枚举实现一个方法，判断某张牌是否能击败另一张牌。

```rust
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

enum Card {
    Number(Suit, u8), // 数字牌，值 2-10
    Face(Suit, &'static str), // J, Q, K
    Ace(Suit), // A
}

// TODO: 实现 can_beat 方法
// 数字牌：点数大者胜（值越大越强）
// 人头牌：J < Q < K
// Ace：大于所有人头牌和数字牌
// 同花色没有特殊规则，只看牌面值
impl Card {
    fn can_beat(&self, other: &Card) -> bool {
        match (self, other) {
            // TODO: 补全各个分支
            // 提示：对于数字牌，可以比较值
            // 提示：Ace 可以击败任何非 Ace 的牌
            // 如果两张都是 Ace，返回 false（平局）
        }
    }
}

fn main() {
    let ace_hearts = Card::Ace(Suit::Hearts);
    let king_spades = Card::Face(Suit::Spade, "K");
    let ten_diamonds = Card::Number(Suit::Diamond, 10);
    
    println!("A♥ 能击败 K♠: {}", ace_hearts.can_beat(&king_spades)); // true
    println!("K♠ 能击败 10♦: {}", king_spades.can_beat(&ten_diamonds)); // true
    println!("10♦ 能击败 A♥: {}", ten_diamonds.can_beat(&ace_hearts)); // false
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

enum Card {
    Number(Suit, u8),
    Face(Suit, &'static str),
    Ace(Suit),
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Number(_, v) => *v,
            Card::Face(_, "J") => 11,
            Card::Face(_, "Q") => 12,
            Card::Face(_, "K") => 13,
            Card::Ace(_) => 14,
            _ => 0,
        }
    }
    
    fn can_beat(&self, other: &Card) -> bool {
        self.value() > other.value()
    }
}

fn main() {
    let ace_hearts = Card::Ace(Suit::Hearts);
    let king_spades = Card::Face(Suit::Spades, "K");
    let ten_diamonds = Card::Number(Suit::Diamonds, 10);
    
    println!("A♥ 能击败 K♠: {}", ace_hearts.can_beat(&king_spades));
    println!("K♠ 能击败 10♦: {}", king_spades.can_beat(&ten_diamonds));
    println!("10♦ 能击败 A♥: {}", ten_diamonds.can_beat(&ace_hearts));
}
```

**说明：** 这个练习展示了 match 守卫与辅助方法结合使用的常见模式。`value()` 方法将所有牌映射到数值，`can_beat` 直接比较数值大小。在 `value()` 中，`Card::Face(_, "J")` 等模式匹配了具体的字符值。这种将判断逻辑拆分为多个小方法的方式，比分枝复杂的守卫更清晰。
</details>

### 练习 06-18: @ 绑定基础

> 难度：⭐⭐
> Rust 特有功能，Java/C++ 没有直接对应的语法

补全代码，使用 `@` 绑定在匹配范围模式的同时绑定变量值。

```rust
fn describe_age(age: u32) -> String {
    // TODO: 使用 @ 绑定在匹配范围的同时捕获具体值
    match age {
        // 0 -> "新生儿"
        // 1..=12 且具体值为 n -> "儿童，{n} 岁"
        // 13..=17 且具体值为 n -> "青少年，{n} 岁"
        // 18..=59 且具体值为 n -> "成年人，{n} 岁"
        // 60..=150 且具体值为 n -> "老年人，{n} 岁"
        // 其他 -> "不合理的年龄: {age}"
        0 => "新生儿".to_string(),
        // TODO: 使用 @ 绑定剩余范围
    }
}

fn main() {
    println!("0: {}", describe_age(0));
    println!("7: {}", describe_age(7));
    println!("16: {}", describe_age(16));
    println!("30: {}", describe_age(30));
    println!("70: {}", describe_age(70));
    println!("200: {}", describe_age(200));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn describe_age(age: u32) -> String {
    match age {
        0 => "新生儿".to_string(),
        n @ 1..=12 => format!("儿童，{} 岁", n),
        n @ 13..=17 => format!("青少年，{} 岁", n),
        n @ 18..=59 => format!("成年人，{} 岁", n),
        n @ 60..=150 => format!("老年人，{} 岁", n),
        _ => format!("不合理的年龄: {}", age),
    }
}

fn main() {
    println!("0: {}", describe_age(0));
    println!("7: {}", describe_age(7));
    println!("16: {}", describe_age(16));
    println!("30: {}", describe_age(30));
    println!("70: {}", describe_age(70));
    println!("200: {}", describe_age(200));
}
```

**说明：** `n @ 1..=12` 语法将范围匹配的值绑定到变量 `n`，这样既做了范围匹配，又能获取具体值用于后续操作。`@` 绑定在需要知道"匹配到的具体值"时非常有用，避免了在守卫中重复计算。
</details>

### 练习 06-19: @ 绑定进阶

> 难度：⭐⭐
> Rust 特有功能，用于解构时同时保留外层引用

补全代码，使用 `@` 绑定在解构的同时保留整个枚举变体的引用。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn handle_message(msg: &Message) {
    // TODO: 使用 @ 绑定，在解构内部字段的同时保留对整个变体的引用
    // 对于 Move，打印完整信息 "移动到 ({x}, {y})" 
    // 对于 Write，先打印 "消息对象"（使用 @ 绑定的整个变体），再打印内容
    // 对于 Quit，打印 "退出"
    match msg {
        Message::Quit => println!("退出"),
        // TODO: 使用 m @ Message::Move { x, y } 绑定整个变体
        // TODO: 使用 m @ Message::Write(s) 绑定整个变体
    }
}

fn main() {
    handle_message(&Message::Move { x: 10, y: 20 });
    handle_message(&Message::Write("Hello".to_string()));
    handle_message(&Message::Quit);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn handle_message(msg: &Message) {
    match msg {
        Message::Quit => println!("退出"),
        m @ Message::Move { x, y } => {
            println!("移动到 ({}, {})", x, y);
            // 这里可以使用 m 引用整个 Move
        }
        m @ Message::Write(s) => {
            println!("消息对象: {:?}", m);
            println!("内容: {}", s);
        }
    }
}

fn main() {
    handle_message(&Message::Move { x: 10, y: 20 });
    handle_message(&Message::Write("Hello".to_string()));
    handle_message(&Message::Quit);
}
```

**说明：** `m @ Message::Move { x, y }` 将整个变体绑定到 `m` 的同时也解构了内部字段。这样既可以通过 `m` 访问整个变体（注意需要实现 `Debug` trait 才能打印），也可以直接使用解构出的字段。这在需要同时访问整体和部分时非常有用。
</details>

### 练习 06-20: 挑战——简单计算器

> 难度：⭐⭐⭐
> 综合运用 match、解构、守卫和 @ 绑定

实现一个简单的表达式计算器。定义 `Expr` 枚举表示算术表达式，实现 `evaluate` 方法计算结果。要求：
1. 支持 `Number(f64)`——数字
2. 支持 `Add(Box<Expr>, Box<Expr>)`——加法
3. 支持 `Sub(Box<Expr>, Box<Expr>)`——减法
4. 支持 `Mul(Box<Expr>, Box<Expr>)`——乘法
5. 支持 `Div(Box<Expr>, Box<Expr>)`——除法（除零返回 `None`）
6. 支持 `Neg(Box<Expr>)`——取负
7. `evaluate` 返回 `Option<f64>`，除零时返回 `None`

```rust
// TODO: 定义 Expr 枚举
// Number(f64), Add, Sub, Mul, Div, Neg

// TODO: 为 Expr 实现 evaluate 方法（返回 Option<f64>）
// 提示：子表达式递归调用 evaluate，然后用 match 或 ? 处理结果

fn main() {
    // 表达式: (3 + 5) * 2 - 10 / 2
    let expr = Expr::Sub(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Number(3.0)),
                Box::new(Expr::Number(5.0)),
            )),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Div(
            Box::new(Expr::Number(10.0)),
            Box::new(Expr::Number(2.0)),
        )),
    );
    
    match expr.evaluate() {
        Some(v) => println!("(3 + 5) * 2 - 10 / 2 = {}", v), // 应输出 11
        None => println!("计算错误"),
    }
    
    // 除零测试
    let div_by_zero = Expr::Div(
        Box::new(Expr::Number(5.0)),
        Box::new(Expr::Number(0.0)),
    );
    match div_by_zero.evaluate() {
        Some(v) => println!("5 / 0 = {}", v),
        None => println!("错误: 除零"), // 应输出这个
    }
    
    // 取负测试
    let neg = Expr::Neg(Box::new(Expr::Number(7.0)));
    match neg.evaluate() {
        Some(v) => println!("-7 = {}", v), // 应输出 -7
        None => println!("计算错误"),
    }
    
    // 复杂嵌套: -((10 - 3) * (5 + 1)) / 2
    let complex = Expr::Div(
        Box::new(Expr::Neg(Box::new(Expr::Mul(
            Box::new(Expr::Sub(
                Box::new(Expr::Number(10.0)),
                Box::new(Expr::Number(3.0)),
            )),
            Box::new(Expr::Add(
                Box::new(Expr::Number(5.0)),
                Box::new(Expr::Number(1.0)),
            )),
        )))),
        Box::new(Expr::Number(2.0)),
    );
    match complex.evaluate() {
        Some(v) => println!("-((10 - 3) * (5 + 1)) / 2 = {}", v), // 应输出 -21
        None => println!("计算错误"),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    fn evaluate(&self) -> Option<f64> {
        match self {
            Expr::Number(n) => Some(*n),
            Expr::Add(l, r) => {
                let l = l.evaluate()?;
                let r = r.evaluate()?;
                Some(l + r)
            }
            Expr::Sub(l, r) => {
                let l = l.evaluate()?;
                let r = r.evaluate()?;
                Some(l - r)
            }
            Expr::Mul(l, r) => {
                let l = l.evaluate()?;
                let r = r.evaluate()?;
                Some(l * r)
            }
            Expr::Div(l, r) => {
                let l = l.evaluate()?;
                let r = r.evaluate()?;
                if r == 0.0 {
                    None
                } else {
                    Some(l / r)
                }
            }
            Expr::Neg(inner) => {
                let v = inner.evaluate()?;
                Some(-v)
            }
        }
    }
}

fn main() {
    let expr = Expr::Sub(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Number(3.0)),
                Box::new(Expr::Number(5.0)),
            )),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Div(
            Box::new(Expr::Number(10.0)),
            Box::new(Expr::Number(2.0)),
        )),
    );
    
    match expr.evaluate() {
        Some(v) => println!("(3 + 5) * 2 - 10 / 2 = {}", v),
        None => println!("计算错误"),
    }
    
    let div_by_zero = Expr::Div(
        Box::new(Expr::Number(5.0)),
        Box::new(Expr::Number(0.0)),
    );
    match div_by_zero.evaluate() {
        Some(v) => println!("5 / 0 = {}", v),
        None => println!("错误: 除零"),
    }
    
    let neg = Expr::Neg(Box::new(Expr::Number(7.0)));
    match neg.evaluate() {
        Some(v) => println!("-7 = {}", v),
        None => println!("计算错误"),
    }
    
    let complex = Expr::Div(
        Box::new(Expr::Neg(Box::new(Expr::Mul(
            Box::new(Expr::Sub(
                Box::new(Expr::Number(10.0)),
                Box::new(Expr::Number(3.0)),
            )),
            Box::new(Expr::Add(
                Box::new(Expr::Number(5.0)),
                Box::new(Expr::Number(1.0)),
            )),
        )))),
        Box::new(Expr::Number(2.0)),
    );
    match complex.evaluate() {
        Some(v) => println!("-((10 - 3) * (5 + 1)) / 2 = {}", v),
        None => println!("计算错误"),
    }
}
```

**说明：** 这是一个综合练习，展示了枚举在树形数据结构（表达式树）中的强大表现力。`Box<Expr>` 用于递归类型（枚举需要确定大小，所以用 Box 堆分配）。`?` 运算符在 `Option` 上下文中的用法——如果子表达式求值为 `None`，则立即返回 `None`，否则解包继续计算。这种结构清晰地将表达式的语法和求值逻辑分离。
</details>
