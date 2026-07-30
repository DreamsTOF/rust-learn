# 15 Result 与错误处理

Rust 没有传统的异常机制（如 C++ 的 `try-catch` 或 Java 的异常），而是通过 `Result<T, E>` 枚举类型进行显式的错误处理。`Result` 是 Rust 标准库中最核心的类型之一，强制开发者处理每一种可能的错误，从而编写更健壮的代码。本章练习将帮助你掌握 `Result` 的基本使用、`?` 操作符传播错误、自定义错误类型以及错误包装与转换。

### 练习 15-01: 使用 match 处理 Result

> 难度：⭐
> 类似 Java 的 try-catch / 类似 C++ 的异常机制

补全代码，使用 `match` 表达式分别处理 `Ok` 和 `Err` 两种情况。

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    // TODO: 使用 match 处理 result，Ok 时打印 "结果: {值}"，Err 时打印 "错误: {信息}"
    
    let result2 = divide(10.0, 0.0);
    // TODO: 再次使用 match 处理 result2
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("结果: {}", value),
        Err(msg) => println!("错误: {}", msg),
    }
    
    let result2 = divide(10.0, 0.0);
    match result2 {
        Ok(value) => println!("结果: {}", value),
        Err(msg) => println!("错误: {}", msg),
    }
}
```

**说明：** `Result` 是一个枚举，定义了两个变体：`Ok(T)` 表示成功，携带值；`Err(E)` 表示失败，携带错误信息。使用 `match` 对 `Result` 进行模式匹配是最基础的处理方式，Rust 编译器强制要求覆盖所有可能的分支。
</details>

### 练习 15-02: 从字符串解析数字

> 难度：⭐
> 类似 Java 的 Integer.parseInt 但返回 Result

补全代码，使用 `match` 处理 `"42"` 和 `"abc"` 两个字符串的解析结果。

```rust
fn main() {
    let valid = "42";
    let invalid = "abc";
    
    // TODO: 使用 str::parse::<i32>() 解析 valid，用 match 处理结果
    
    // TODO: 使用 str::parse::<i32>() 解析 invalid，用 match 处理结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let valid = "42";
    let invalid = "abc";
    
    match valid.parse::<i32>() {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
    
    match invalid.parse::<i32>() {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }
}
```

**说明：** `str::parse::<T>()` 返回 `Result<T, ParseIntError>`，是 Rust 中将字符串转换为其他类型的标准方式。与 Java 的 `Integer.parseInt` 抛出异常不同，Rust 将错误作为值返回，调用者必须显式处理。
</details>

### 练习 15-03: unwrap 与 expect 的使用

> 难度：⭐⭐
> 类似 C++ 的 assert / Java 的 Objects.requireNonNull

补全代码，使用 `unwrap` 和 `expect` 从 `Result` 中取出值，其中 `get_env` 函数模拟读取环境变量。

```rust
fn get_env(key: &str) -> Result<String, String> {
    match key {
        "HOME" => Ok(String::from("/home/user")),
        "PATH" => Ok(String::from("/usr/bin:/bin")),
        _ => Err(format!("环境变量 {} 未设置", key)),
    }
}

fn main() {
    // TODO: 使用 unwrap() 获取 "HOME" 的值
    
    // TODO: 使用 expect() 获取 "PATH" 的值，错误信息为 "PATH 必须设置"
    
    // TODO: 使用 unwrap() 获取 "MY_ENV" 的值——此处会 panic
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn get_env(key: &str) -> Result<String, String> {
    match key {
        "HOME" => Ok(String::from("/home/user")),
        "PATH" => Ok(String::from("/usr/bin:/bin")),
        _ => Err(format!("环境变量 {} 未设置", key)),
    }
}

fn main() {
    let home = get_env("HOME").unwrap();
    println!("HOME: {}", home);
    
    let path = get_env("PATH").expect("PATH 必须设置");
    println!("PATH: {}", path);
    
    // 这行会 panic，因为 "MY_ENV" 不存在
    let my_env = get_env("MY_ENV").unwrap();
    println!("MY_ENV: {}", my_env);
}
```

**说明：** `unwrap()` 在 `Ok` 时返回内部值，在 `Err` 时调用 `panic!`。`expect()` 类似但允许自定义 panic 信息。它们适合在原型开发、测试或确信不会出错的情况下使用，生产代码中应谨慎使用。
</details>

### 练习 15-04: unwrap_or 与 unwrap_or_else

> 难度：⭐⭐
> 类似 Java 的 Optional.orElse / orElseGet

补全代码，使用 `unwrap_or` 和 `unwrap_or_else` 为失败情况提供默认值。

```rust
fn get_cached_value(key: &str) -> Result<i32, String> {
    match key {
        "count" => Ok(42),
        "max" => Ok(100),
        _ => Err(format!("缓存键 {} 不存在", key)),
    }
}

fn main() {
    // TODO: 使用 unwrap_or 获取 "count"，失败时默认 0
    
    // TODO: 使用 unwrap_or_else 获取 "unknown"，失败时返回计算默认值  -1
    
    // TODO: 使用 unwrap_or 获取 "max"，失败时默认 0
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn get_cached_value(key: &str) -> Result<i32, String> {
    match key {
        "count" => Ok(42),
        "max" => Ok(100),
        _ => Err(format!("缓存键 {} 不存在", key)),
    }
}

fn main() {
    let count = get_cached_value("count").unwrap_or(0);
    println!("count: {}", count);
    
    let unknown = get_cached_value("unknown").unwrap_or_else(|_| -1);
    println!("unknown: {}", unknown);
    
    let max = get_cached_value("max").unwrap_or(0);
    println!("max: {}", max);
}
```

**说明：** `unwrap_or(default)` 在 `Err` 时返回指定的默认值，`unwrap_or_else(fn)` 则接受一个闭包，惰性计算默认值。两者都避免了 panic，是更安全的取值方式。
</details>

### 练习 15-05: Result 基础综合挑战

> 难度：⭐⭐⭐
> 类似 Java 的 try-catch 综合

实现一个简单的成绩管理系统。要求：

1. 解析三个字符串为 `f64` 分数
2. 计算平均分
3. 将平均分转换为等级（A/B/C/D/F）
4. 每一步都可能失败，使用 `match` 逐层处理

```rust
fn parse_score(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|_| format!("无法解析分数: {}", s))
}

fn calculate_average(a: f64, b: f64, c: f64) -> Result<f64, String> {
    if a < 0.0 || b < 0.0 || c < 0.0 {
        Err(String::from("分数不能为负数"))
    } else {
        Ok((a + b + c) / 3.0)
    }
}

fn score_to_grade(avg: f64) -> Result<&'static str, String> {
    match avg {
        _ if avg > 100.0 => Err(String::from("平均分不能超过 100")),
        _ if avg >= 90.0 => Ok("A"),
        _ if avg >= 80.0 => Ok("B"),
        _ if avg >= 70.0 => Ok("C"),
        _ if avg >= 60.0 => Ok("D"),
        _ if avg >= 0.0 => Ok("F"),
        _ => Err(String::from("平均分不能为负数")),
    }
}

fn main() {
    let scores = ["85.5", "92.0", "78.5"];
    
    // TODO: 使用 match 链式处理：
    // 1. 解析三个分数
    // 2. 计算平均分
    // 3. 转换为等级
    // 最终打印 "成绩等级: {等级}" 或错误信息
    
    let invalid_scores = ["85.5", "abc", "78.5"];
    // TODO: 同样处理 invalid_scores，观察错误处理流程
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn parse_score(s: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|_| format!("无法解析分数: {}", s))
}

fn calculate_average(a: f64, b: f64, c: f64) -> Result<f64, String> {
    if a < 0.0 || b < 0.0 || c < 0.0 {
        Err(String::from("分数不能为负数"))
    } else {
        Ok((a + b + c) / 3.0)
    }
}

fn score_to_grade(avg: f64) -> Result<&'static str, String> {
    match avg {
        _ if avg > 100.0 => Err(String::from("平均分不能超过 100")),
        _ if avg >= 90.0 => Ok("A"),
        _ if avg >= 80.0 => Ok("B"),
        _ if avg >= 70.0 => Ok("C"),
        _ if avg >= 60.0 => Ok("D"),
        _ if avg >= 0.0 => Ok("F"),
        _ => Err(String::from("平均分不能为负数")),
    }
}

fn main() {
    let scores = ["85.5", "92.0", "78.5"];
    
    let result = parse_score(scores[0])
        .and_then(|a| parse_score(scores[1]).map(|b| (a, b)))
        .and_then(|(a, b)| parse_score(scores[2]).map(|c| (a, b, c)))
        .and_then(|(a, b, c)| calculate_average(a, b, c))
        .and_then(|avg| score_to_grade(avg));
    
    match result {
        Ok(grade) => println!("成绩等级: {}", grade),
        Err(e) => println!("错误: {}", e),
    }
    
    let invalid_scores = ["85.5", "abc", "78.5"];
    
    let result2 = parse_score(invalid_scores[0])
        .and_then(|a| parse_score(invalid_scores[1]).map(|b| (a, b)))
        .and_then(|(a, b)| parse_score(invalid_scores[2]).map(|c| (a, b, c)))
        .and_then(|(a, b, c)| calculate_average(a, b, c))
        .and_then(|avg| score_to_grade(avg));
    
    match result2 {
        Ok(grade) => println!("成绩等级: {}", grade),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** `and_then` 是 `Result` 的链式组合方法，类似 `flat_map`：如果当前是 `Ok`，则执行闭包并返回新的 `Result`；如果是 `Err`，则直接传递错误。这种方式可以在不使用 `?` 操作符的情况下实现多层错误传播。
</details>

### 练习 15-06: ? 操作符基础

> 难度：⭐
> 类似 C++ 的异常自动传播 / Java 的 throws

补全代码，使用 `?` 操作符传播错误，注意 `?` 只能在返回 `Result`（或 `Option`）的函数中使用。

```rust
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    // TODO: 使用 ? 操作符解析字符串并乘以 2
    // 如果解析失败，? 会自动返回 Err
}

fn main() {
    match parse_and_double("21") {
        Ok(n) => println!("21 的两倍是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_double("abc") {
        Ok(n) => println!("abc 的两倍是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    match parse_and_double("21") {
        Ok(n) => println!("21 的两倍是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_double("abc") {
        Ok(n) => println!("abc 的两倍是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** `?` 操作符是 Rust 错误处理的精髓。它会自动展开 `Result`：如果是 `Ok`，取出内部值；如果是 `Err`，则立即从当前函数返回该错误。这大大简化了错误传播代码，是 Rust 中处理错误的推荐方式。
</details>

### 练习 15-07: ? 操作符在多个步骤中的应用

> 难度：⭐
> 类似 C++ 的异常自动传播

补全代码，在链式操作中使用 `?` 操作符逐步处理数据。

```rust
fn first_char(s: &str) -> Result<char, String> {
    s.chars().next().ok_or_else(|| String::from("字符串为空"))
}

fn to_uppercase(c: char) -> Result<char, String> {
    if c.is_ascii_lowercase() {
        Ok(c.to_ascii_uppercase())
    } else if c.is_ascii_uppercase() {
        Ok(c)
    } else {
        Err(format!("字符 '{}' 不是 ASCII 字母", c))
    }
}

fn process(s: &str) -> Result<char, String> {
    // TODO: 使用 ? 先获取首字符，再转换为大写
}

fn main() {
    match process("hello") {
        Ok(c) => println!("结果: {}", c),
        Err(e) => println!("错误: {}", e),
    }
    
    match process("") {
        Ok(c) => println!("结果: {}", c),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn first_char(s: &str) -> Result<char, String> {
    s.chars().next().ok_or_else(|| String::from("字符串为空"))
}

fn to_uppercase(c: char) -> Result<char, String> {
    if c.is_ascii_lowercase() {
        Ok(c.to_ascii_uppercase())
    } else if c.is_ascii_uppercase() {
        Ok(c)
    } else {
        Err(format!("字符 '{}' 不是 ASCII 字母", c))
    }
}

fn process(s: &str) -> Result<char, String> {
    let c = first_char(s)?;
    to_uppercase(c)
}

fn main() {
    match process("hello") {
        Ok(c) => println!("结果: {}", c),
        Err(e) => println!("错误: {}", e),
    }
    
    match process("") {
        Ok(c) => println!("结果: {}", c),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** `?` 操作符可以连续使用，每个 `?` 都在对应步骤失败时提前返回。这使代码像"直线流水线"一样直观，避免了嵌套的 `match`。
</details>

### 练习 15-08: ? 操作符链式调用

> 难度：⭐⭐
> 类似 Java 的方法链 + throws 声明

补全代码，编写一个函数使用 `?` 进行链式操作，从两数相除到取整。

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn square_root(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("不能对负数开平方"))
    } else {
        Ok(x.sqrt())
    }
}

fn round_to_int(x: f64) -> Result<i32, String> {
    if x.is_finite() {
        Ok(x.round() as i32)
    } else {
        Err(String::from("无法对无限值取整"))
    }
}

// TODO: 实现 compute 函数，接收 a, b
// 使用 ? 执行: 除法 → 开平方 → 取整
fn compute(a: f64, b: f64) -> Result<i32, String> {
    // 在这里填写
}

fn main() {
    match compute(16.0, 4.0) {
        Ok(n) => println!("结果: {}", n),  // 16/4=4, sqrt=2, round=2
        Err(e) => println!("错误: {}", e),
    }
    
    match compute(16.0, 0.0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn square_root(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("不能对负数开平方"))
    } else {
        Ok(x.sqrt())
    }
}

fn round_to_int(x: f64) -> Result<i32, String> {
    if x.is_finite() {
        Ok(x.round() as i32)
    } else {
        Err(String::from("无法对无限值取整"))
    }
}

fn compute(a: f64, b: f64) -> Result<i32, String> {
    let quotient = divide(a, b)?;
    let root = square_root(quotient)?;
    round_to_int(root)
}

fn main() {
    match compute(16.0, 4.0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match compute(16.0, 0.0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** 使用 `?` 操作符可以将多个可能失败的步骤串联起来，代码形式上如同操作普通值一样流畅。任何一步失败都会立即返回，不会继续执行后续步骤。
</details>

### 练习 15-09: ? 操作符与多种错误类型

> 难度：⭐⭐
> 类似 Java 的多 catch 块

Rust 的 `?` 操作符要求函数返回的错误类型与 `?` 产生的错误类型一致。当错误类型不匹配时，需要使用 `map_err` 进行转换。补全代码，将不同类型的错误转换为统一的 `String` 类型。

```rust
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

fn divide(n: i32, d: i32) -> Result<i32, &'static str> {
    if d == 0 {
        Err("除数不能为零")
    } else {
        Ok(n / d)
    }
}

fn compute(s: &str, d: i32) -> Result<i32, String> {
    // TODO: 使用 ? 操作符
    // 先 parse_int(s)，再 divide(结果, d)
    // 提示: 由于错误类型不同，需要用 map_err 转换
}

fn main() {
    match compute("42", 2) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match compute("42", 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match compute("abc", 2) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

fn divide(n: i32, d: i32) -> Result<i32, &'static str> {
    if d == 0 {
        Err("除数不能为零")
    } else {
        Ok(n / d)
    }
}

fn compute(s: &str, d: i32) -> Result<i32, String> {
    let n = parse_int(s).map_err(|e| e.to_string())?;
    divide(n, d).map_err(|e| e.to_string())
}

fn main() {
    match compute("42", 2) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match compute("42", 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match compute("abc", 2) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** 当多个可能失败的调用返回不同错误类型时，可以使用 `map_err` 将它们统一转换为相同的错误类型（这里统一为 `String`），从而让 `?` 操作符能够正常工作。
</details>

### 练习 15-10: ? 操作符综合挑战

> 难度：⭐⭐⭐
> 类似 C++ 的多层嵌套异常传播 / Java 的 checked exception 链

实现一个用户注册验证系统，包含多个验证步骤，每个步骤都可能失败。使用 `?` 操作符串联整个过程。

```rust
fn validate_username(username: &str) -> Result<&str, String> {
    if username.len() < 3 {
        Err(String::from("用户名至少需要 3 个字符"))
    } else if username.len() > 20 {
        Err(String::from("用户名不能超过 20 个字符"))
    } else {
        Ok(username)
    }
}

fn validate_email(email: &str) -> Result<&str, String> {
    if email.contains('@') && email.contains('.') {
        Ok(email)
    } else {
        Err(String::from("邮箱格式不正确"))
    }
}

fn validate_age(age_str: &str) -> Result<u32, String> {
    let age: u32 = age_str.parse().map_err(|_| String::from("年龄必须是数字"))?;
    if age < 18 {
        Err(String::from("年龄必须至少 18 岁"))
    } else {
        Ok(age)
    }
}

fn register(username: &str, email: &str, age_str: &str) -> Result<String, String> {
    // TODO: 使用 ? 依次验证 username、email、age
    // 返回 "用户 {用户名} 注册成功" 或错误
}

fn main() {
    match register("alice", "alice@example.com", "25") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
    
    match register("ab", "alice@example.com", "25") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
    
    match register("alice", "bademail", "25") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
    
    match register("alice", "alice@example.com", "16") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn validate_username(username: &str) -> Result<&str, String> {
    if username.len() < 3 {
        Err(String::from("用户名至少需要 3 个字符"))
    } else if username.len() > 20 {
        Err(String::from("用户名不能超过 20 个字符"))
    } else {
        Ok(username)
    }
}

fn validate_email(email: &str) -> Result<&str, String> {
    if email.contains('@') && email.contains('.') {
        Ok(email)
    } else {
        Err(String::from("邮箱格式不正确"))
    }
}

fn validate_age(age_str: &str) -> Result<u32, String> {
    let age: u32 = age_str.parse().map_err(|_| String::from("年龄必须是数字"))?;
    if age < 18 {
        Err(String::from("年龄必须至少 18 岁"))
    } else {
        Ok(age)
    }
}

fn register(username: &str, email: &str, age_str: &str) -> Result<String, String> {
    let valid_username = validate_username(username)?;
    validate_email(email)?;
    validate_age(age_str)?;
    Ok(format!("用户 {} 注册成功", valid_username))
}

fn main() {
    match register("alice", "alice@example.com", "25") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
    
    match register("ab", "alice@example.com", "25") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
    
    match register("alice", "bademail", "25") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
    
    match register("alice", "alice@example.com", "16") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("注册失败: {}", e),
    }
}
```

**说明：** `?` 操作符让验证流程的代码极其简洁，每个验证函数专注于单一职责，`register` 函数像"流水线"一样串联各个步骤。任何一步失败，整个注册过程立即中止并返回错误信息。
</details>

### 练习 15-11: 自定义错误类型 —— struct 错误

> 难度：⭐
> 类似 Java 的自定义异常类

补全代码，定义一个 `DivisionError` 结构体，实现 `Display` 和 `Debug` trait。

```rust
use std::fmt;

// TODO: 定义 DivisionError 结构体，包含一个字段 msg: String

// TODO: 为 DivisionError 实现 Display trait
// 格式化输出 "除法错误: {msg}"

// TODO: 为 DivisionError 实现 Debug trait（可以使用 derive 宏）

fn safe_divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError { msg: String::from("除数不能为零") })
    } else {
        Ok(a / b)
    }
}

fn main() {
    match safe_divide(10, 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("{}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug)]
struct DivisionError {
    msg: String,
}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "除法错误: {}", self.msg)
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError { msg: String::from("除数不能为零") })
    } else {
        Ok(a / b)
    }
}

fn main() {
    match safe_divide(10, 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("{}", e),
    }
}
```

**说明：** 自定义错误类型通常是一个结构体（或枚举），需要实现 `Display`（供用户查看）和 `Debug`（供开发者调试）。通常使用 `#[derive(Debug)]` 自动实现 `Debug`。
</details>

### 练习 15-12: 自定义错误类型 —— enum 错误

> 难度：⭐
> 类似 Java 的枚举异常

补全代码，定义一个枚举错误类型来表示除法的不同错误情况。

```rust
use std::fmt;

// TODO: 定义 DivisionError 枚举，包含两个变体：
// - DivideByZero
// - NegativeDividend(i32)  // 记录被除数的值

// TODO: 为 DivisionError 实现 Display

fn safe_divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a < 0 {
        Err(DivisionError::NegativeDividend(a))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match safe_divide(10, 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("{}", e),
    }
    
    match safe_divide(-5, 2) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("{}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

enum DivisionError {
    DivideByZero,
    NegativeDividend(i32),
}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DivisionError::DivideByZero => write!(f, "除数不能为零"),
            DivisionError::NegativeDividend(n) => write!(f, "被除数不能为负数: {}", n),
        }
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a < 0 {
        Err(DivisionError::NegativeDividend(a))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match safe_divide(10, 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("{}", e),
    }
    
    match safe_divide(-5, 2) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("{}", e),
    }
}
```

**说明：** 使用枚举作为错误类型可以清晰地表达多种错误情况，每种变体还可以携带额外的上下文信息。这是 Rust 中定义错误类型的推荐方式。
</details>

### 练习 15-13: 实现 Error trait

> 难度：⭐⭐
> 类似 Java 的 Exception 类层次

`std::error::Error` trait 是 Rust 错误类型的标准接口。为自定义错误类型实现 `Error` trait（以及 `Display` 和 `Debug`）后，它可以与 Rust 的错误处理生态无缝集成。

```rust
use std::fmt;
use std::error::Error;

// TODO: 定义 ParseError 结构体，包含字段 field: String 和 message: String

// TODO: 实现 Display trait

// TODO: 实现 Error trait（只需实现，无需额外方法）

fn parse_field(input: &str) -> Result<String, ParseError> {
    if input.is_empty() {
        Err(ParseError {
            field: String::from("input"),
            message: String::from("输入不能为空"),
        })
    } else {
        Ok(input.to_string())
    }
}

fn main() {
    match parse_field("") {
        Ok(val) => println!("值: {}", val),
        Err(e) => {
            println!("错误: {}", e);
            println!("调试: {:?}", e);
            // Error trait 使这个错误可以在?操作符中与其它错误类型互操作
        }
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct ParseError {
    field: String,
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "解析字段 '{}' 失败: {}", self.field, self.message)
    }
}

impl Error for ParseError {}

fn parse_field(input: &str) -> Result<String, ParseError> {
    if input.is_empty() {
        Err(ParseError {
            field: String::from("input"),
            message: String::from("输入不能为空"),
        })
    } else {
        Ok(input.to_string())
    }
}

fn main() {
    match parse_field("") {
        Ok(val) => println!("值: {}", val),
        Err(e) => {
            println!("错误: {}", e);
            println!("调试: {:?}", e);
        }
    }
}
```

**说明：** `std::error::Error` trait 是 Rust 错误类型的标准接口。它通常不需要额外实现方法（默认实现已够用），但标记类型实现了 `Error`，使其能用于 `Box<dyn Error>`、`anyhow` 等更上层的错误处理工具。
</details>

### 练习 15-14: 为枚举实现 Error trait

> 难度：⭐⭐
> 类似 Java 的多异常类型

补全代码，为一个枚举错误类型实现 `Display`、`Debug` 和 `Error` trait。

```rust
use std::fmt;
use std::error::Error;

// TODO: 定义 DataError 枚举，包含两个变体：
// - NotFound(String)   // 记录未找到的键
// - PermissionDenied   // 权限拒绝

// TODO: 为 DataError 派生 Debug

// TODO: 实现 Display

// TODO: 实现 Error

fn lookup_data(key: &str, admin: bool) -> Result<String, DataError> {
    match key {
        "secret" if !admin => Err(DataError::PermissionDenied),
        "secret" => Ok(String::from("top_secret_value")),
        "name" => Ok(String::from("Alice")),
        _ => Err(DataError::NotFound(key.to_string())),
    }
}

fn main() {
    match lookup_data("name", false) {
        Ok(v) => println!("找到: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    
    match lookup_data("secret", false) {
        Ok(v) => println!("找到: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    
    match lookup_data("secret", true) {
        Ok(v) => println!("找到: {}", v),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum DataError {
    NotFound(String),
    PermissionDenied,
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::NotFound(key) => write!(f, "未找到数据: {}", key),
            DataError::PermissionDenied => write!(f, "权限不足，无法访问"),
        }
    }
}

impl Error for DataError {}

fn lookup_data(key: &str, admin: bool) -> Result<String, DataError> {
    match key {
        "secret" if !admin => Err(DataError::PermissionDenied),
        "secret" => Ok(String::from("top_secret_value")),
        "name" => Ok(String::from("Alice")),
        _ => Err(DataError::NotFound(key.to_string())),
    }
}

fn main() {
    match lookup_data("name", false) {
        Ok(v) => println!("找到: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    
    match lookup_data("secret", false) {
        Ok(v) => println!("找到: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    
    match lookup_data("secret", true) {
        Ok(v) => println!("找到: {}", v),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** 为枚举实现 `Error` trait 后，这个枚举类型就可以作为完整的 Rust 错误类型使用，可以在 `Box<dyn Error>` 中传递，也可以与 `thiserror` 等第三方库配合。
</details>

### 练习 15-15: 自定义错误综合挑战

> 难度：⭐⭐⭐
> 类似 Java 的自定义异常层次结构

实现一个银行账户系统，定义全面的错误枚举，包含多种错误变体并携带丰富信息。

```rust
use std::fmt;
use std::error::Error;

// TODO: 定义 BankError 枚举，包含以下变体：
// - InsufficientFunds { balance: f64, amount: f64 }
// - AccountNotFound(String)
// - InvalidAmount(f64)
// - FrozenAccount

// TODO: 为 BankError 派生 Debug，实现 Display 和 Error

struct BankAccount {
    owner: String,
    balance: f64,
    frozen: bool,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance,
            frozen: false,
        }
    }
    
    fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        // TODO: 如果 amount <= 0，返回 InvalidAmount
        // 如果账户冻结，返回 FrozenAccount
        // 否则增加余额并返回 Ok(())
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        // TODO: 如果 amount <= 0，返回 InvalidAmount
        // 如果账户冻结，返回 FrozenAccount
        // 如果余额不足，返回 InsufficientFunds
        // 否则减少余额
    }
}

fn main() {
    let mut account = BankAccount::new("Alice", 100.0);
    
    match account.deposit(50.0) {
        Ok(()) => println!("存款成功，余额: {}", account.balance),
        Err(e) => println!("存款失败: {}", e),
    }
    
    match account.withdraw(200.0) {
        Ok(()) => println!("取款成功，余额: {}", account.balance),
        Err(e) => println!("取款失败: {}", e),
    }
    
    match account.withdraw(30.0) {
        Ok(()) => println!("取款成功，余额: {}", account.balance),
        Err(e) => println!("取款失败: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum BankError {
    InsufficientFunds { balance: f64, amount: f64 },
    AccountNotFound(String),
    InvalidAmount(f64),
    FrozenAccount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { balance, amount } => {
                write!(f, "余额不足: 当前余额 {:.2}，需要 {:.2}", balance, amount)
            }
            BankError::AccountNotFound(id) => write!(f, "账户不存在: {}", id),
            BankError::InvalidAmount(amt) => write!(f, "无效金额: {}", amt),
            BankError::FrozenAccount => write!(f, "账户已冻结"),
        }
    }
}

impl Error for BankError {}

struct BankAccount {
    owner: String,
    balance: f64,
    frozen: bool,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance,
            frozen: false,
        }
    }
    
    fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }
        if self.frozen {
            return Err(BankError::FrozenAccount);
        }
        self.balance += amount;
        Ok(())
    }
    
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }
        if self.frozen {
            return Err(BankError::FrozenAccount);
        }
        if self.balance < amount {
            return Err(BankError::InsufficientFunds {
                balance: self.balance,
                amount,
            });
        }
        self.balance -= amount;
        Ok(())
    }
}

fn main() {
    let mut account = BankAccount::new("Alice", 100.0);
    
    match account.deposit(50.0) {
        Ok(()) => println!("存款成功，余额: {}", account.balance),
        Err(e) => println!("存款失败: {}", e),
    }
    
    match account.withdraw(200.0) {
        Ok(()) => println!("取款成功，余额: {}", account.balance),
        Err(e) => println!("取款失败: {}", e),
    }
    
    match account.withdraw(30.0) {
        Ok(()) => println!("取款成功，余额: {}", account.balance),
        Err(e) => println!("取款失败: {}", e),
    }
}
```

**说明：** 设计良好的自定义错误类型应该：1) 使用枚举表达不同的错误种类；2) 每个变体携带足够的上下文信息；3) 实现 `Display` 提供友好的错误消息；4) 实现 `Error` trait 以便与 Rust 错误处理生态集成。
</details>

### 练习 15-16: 使用 map_err 进行错误转换

> 难度：⭐
> 类似 Java 的异常包装转换

当内部函数的错误类型与外部函数不匹配时，使用 `map_err` 进行转换。补全代码。

```rust
use std::num::ParseIntError;

fn parse_and_square(s: &str) -> Result<i32, String> {
    // TODO: 使用 map_err 将 ParseIntError 转换为 String
    // 提示: 使用 ? 操作符 + map_err
}

fn main() {
    match parse_and_square("5") {
        Ok(n) => println!("5 的平方是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_square("abc") {
        Ok(n) => println!("abc 的平方是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::num::ParseIntError;

fn parse_and_square(s: &str) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|e: ParseIntError| format!("解析失败: {}", e))?;
    Ok(n * n)
}

fn main() {
    match parse_and_square("5") {
        Ok(n) => println!("5 的平方是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_square("abc") {
        Ok(n) => println!("abc 的平方是 {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** `map_err` 将 `Result<T, E>` 转换为 `Result<T, F>`，通过对错误值应用闭包来转换错误类型。这在需要统一多种错误类型时非常有用。
</details>

### 练习 15-17: 使用 From trait 实现自动错误转换

> 难度：⭐
> 类似 C++ 的异常转换 / Java 的异常链

Rust 的 `?` 操作符会自动使用 `From` trait 将底层错误转换为上层错误类型。补全代码，为自定义错误类型实现 `From` trait。

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.kind, self.message)
    }
}

// TODO: 为 AppError 实现 Error trait

// TODO: 实现 From<ParseIntError> for AppError
// 将 ParseIntError 转换为 AppError，kind 设为 "ParseError"

fn process_input(s: &str) -> Result<i32, AppError> {
    // TODO: 使用 ? 操作符，由于实现了 From，? 会自动转换 ParseIntError 为 AppError
}

fn main() {
    match process_input("42") {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match process_input("abc") {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.kind, self.message)
    }
}

impl Error for AppError {}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError {
            kind: String::from("ParseError"),
            message: e.to_string(),
        }
    }
}

fn process_input(s: &str) -> Result<i32, AppError> {
    let n = s.parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    match process_input("42") {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match process_input("abc") {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** 当为错误类型实现了 `From<T>` trait 后，`?` 操作符会自动将 `T` 类型的错误转换为该错误类型。这是 Rust 错误处理中非常强大的特性，让错误传播代码更加简洁。
</details>

### 练习 15-18: 多种错误类型的统一处理（1）

> 难度：⭐⭐
> 类似 Java 的多 catch 块统一处理

使用 `Box<dyn Error>` 统一处理多种错误类型。补全代码。

```rust
use std::error::Error;

fn parse_number(s: &str) -> Result<i32, Box<dyn Error>> {
    // TODO: 使用 ? 操作符解析字符串为 i32
    // Box<dyn Error> 可以容纳任何实现了 Error 的类型
}

fn parse_and_divide(s: &str, divisor: i32) -> Result<i32, Box<dyn Error>> {
    let n = parse_number(s)?;
    if divisor == 0 {
        Err("除数不能为零".into())  // &str 自动转换为 Box<dyn Error>
    } else {
        Ok(n / divisor)
    }
}

fn main() {
    match parse_and_divide("100", 5) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_divide("abc", 5) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_divide("100", 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::error::Error;

fn parse_number(s: &str) -> Result<i32, Box<dyn Error>> {
    let n = s.parse::<i32>()?;
    Ok(n)
}

fn parse_and_divide(s: &str, divisor: i32) -> Result<i32, Box<dyn Error>> {
    let n = parse_number(s)?;
    if divisor == 0 {
        Err("除数不能为零".into())
    } else {
        Ok(n / divisor)
    }
}

fn main() {
    match parse_and_divide("100", 5) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_divide("abc", 5) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
    
    match parse_and_divide("100", 0) {
        Ok(n) => println!("结果: {}", n),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** `Box<dyn Error>` 是一种简单的"橡皮擦"类型，可以容纳任何实现了 `Error` trait 的错误类型。`?` 操作符会自动将具体错误类型装箱（`into()` 或 `from()`）。适合快速原型开发，但在大型项目中推荐使用更精确的错误类型。
</details>

### 练习 15-19: 多种错误类型的统一处理（2）

> 难度：⭐⭐
> 类似 Java 的异常层次结构

结合使用自定义错误枚举和 `From` trait，实现优雅的多种错误统一处理。

```rust
use std::fmt;
use std::num::ParseIntError;
use std::error::Error;

// 自定义错误枚举，覆盖多种错误来源
#[derive(Debug)]
enum ProcessingError {
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            ProcessingError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

impl Error for ProcessingError {}

// TODO: 实现 From<ParseIntError> for ProcessingError
// 将 ParseIntError 转换为 ProcessingError::ParseError

fn parse_age(s: &str) -> Result<u32, ProcessingError> {
    // TODO: 使用 ? 操作符解析年龄（利用 From 自动转换）
}

fn validate_age(age: u32) -> Result<u32, ProcessingError> {
    if age > 150 {
        Err(ProcessingError::ValidationError(String::from("年龄不能超过 150")))
    } else if age < 0 {
        Err(ProcessingError::ValidationError(String::from("年龄不能为负数")))
    } else {
        Ok(age)
    }
}

fn process(s: &str) -> Result<u32, ProcessingError> {
    let age = parse_age(s)?;
    validate_age(age)
}

fn main() {
    match process("25") {
        Ok(age) => println!("年龄: {}", age),
        Err(e) => println!("错误: {}", e),
    }
    
    match process("abc") {
        Ok(age) => println!("年龄: {}", age),
        Err(e) => println!("错误: {}", e),
    }
    
    match process("200") {
        Ok(age) => println!("年龄: {}", age),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::num::ParseIntError;
use std::error::Error;

#[derive(Debug)]
enum ProcessingError {
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            ProcessingError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
        }
    }
}

impl Error for ProcessingError {}

impl From<ParseIntError> for ProcessingError {
    fn from(e: ParseIntError) -> Self {
        ProcessingError::ParseError(e.to_string())
    }
}

fn parse_age(s: &str) -> Result<u32, ProcessingError> {
    let age = s.parse::<u32>()?;  // 自动使用 From<ParseIntError> 转换
    Ok(age)
}

fn validate_age(age: u32) -> Result<u32, ProcessingError> {
    if age > 150 {
        Err(ProcessingError::ValidationError(String::from("年龄不能超过 150")))
    } else if age < 0 {
        Err(ProcessingError::ValidationError(String::from("年龄不能为负数")))
    } else {
        Ok(age)
    }
}

fn process(s: &str) -> Result<u32, ProcessingError> {
    let age = parse_age(s)?;
    validate_age(age)
}

fn main() {
    match process("25") {
        Ok(age) => println!("年龄: {}", age),
        Err(e) => println!("错误: {}", e),
    }
    
    match process("abc") {
        Ok(age) => println!("年龄: {}", age),
        Err(e) => println!("错误: {}", e),
    }
    
    match process("200") {
        Ok(age) => println!("年龄: {}", age),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** 组合使用自定义错误枚举和 `From` trait，可以将所有可能的错误统一到单一的错误类型中。`?` 操作符利用 `From` 自动进行错误转换，让调用者能够在一个 `match` 中处理所有错误情况。
</details>

### 练习 15-20: 综合挑战 —— 文件读取场景模拟

> 难度：⭐⭐⭐
> 类似 Java 的文件 I/O 异常处理

实现一个模拟文件读取与处理的程序，综合运用 `Result`、`?` 操作符、自定义错误类型、`From` trait 和错误包装。

```rust
use std::fmt;
use std::error::Error;

// 模拟文件内容
const FILE_SYSTEM: &[(&str, &str)] = &[
    ("data.txt", "42"),
    ("config.txt", "enabled"),
    ("empty.txt", ""),
];

// TODO: 定义 FileError 枚举，包含以下变体：
// - NotFound(String)      // 文件不存在
// - PermissionDenied      // 权限拒绝
// - InvalidContent(String) // 文件内容无效
// - EmptyFile             // 文件为空

// TODO: 为 FileError 派生 Debug，实现 Display 和 Error

// TODO: 定义 ParseError 结构体（包装 ParseIntError）
// 实现 Display、Debug、Error
// 实现 From<ParseIntError> 和 From<FileError>

// 模拟读取文件（返回 Result<&'static str, FileError>）
fn read_file(name: &str) -> Result<&'static str, FileError> {
    for &(fname, content) in FILE_SYSTEM {
        if fname == name {
            if content.is_empty() {
                return Err(FileError::EmptyFile);
            }
            return Ok(content);
        }
    }
    Err(FileError::NotFound(name.to_string()))
}

// 模拟检查权限
fn check_permission(name: &str) -> Result<(), FileError> {
    // "secret.txt" 需要特殊权限
    if name == "secret.txt" {
        Err(FileError::PermissionDenied)
    } else {
        Ok(())
    }
}

// TODO: 实现 parse_number 函数
// 1. check_permission(name)?
// 2. read_file(name)? 获取内容
// 3. 将内容解析为 i32
// 返回 Result<i32, ParseError>（利用 From 自动转换）

fn main() {
    // 测试数据
    let files = ["data.txt", "config.txt", "empty.txt", "missing.txt", "secret.txt"];
    
    for &file in &files {
        match parse_number(file) {
            Ok(n) => println!("{}: {}", file, n),
            Err(e) => println!("{}: 错误 - {}", file, e),
        }
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

const FILE_SYSTEM: &[(&str, &str)] = &[
    ("data.txt", "42"),
    ("config.txt", "enabled"),
    ("empty.txt", ""),
];

#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied,
    InvalidContent(String),
    EmptyFile,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::NotFound(name) => write!(f, "文件不存在: {}", name),
            FileError::PermissionDenied => write!(f, "权限不足"),
            FileError::InvalidContent(msg) => write!(f, "文件内容无效: {}", msg),
            FileError::EmptyFile => write!(f, "文件为空"),
        }
    }
}

impl Error for FileError {}

#[derive(Debug)]
struct ParseError {
    kind: String,
    source: Box<dyn Error>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.kind, self.source)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError {
            kind: String::from("整数解析失败"),
            source: Box::new(e),
        }
    }
}

impl From<FileError> for ParseError {
    fn from(e: FileError) -> Self {
        ParseError {
            kind: String::from("文件操作失败"),
            source: Box::new(e),
        }
    }
}

fn read_file(name: &str) -> Result<&'static str, FileError> {
    for &(fname, content) in FILE_SYSTEM {
        if fname == name {
            if content.is_empty() {
                return Err(FileError::EmptyFile);
            }
            return Ok(content);
        }
    }
    Err(FileError::NotFound(name.to_string()))
}

fn check_permission(name: &str) -> Result<(), FileError> {
    if name == "secret.txt" {
        Err(FileError::PermissionDenied)
    } else {
        Ok(())
    }
}

fn parse_number(name: &str) -> Result<i32, ParseError> {
    check_permission(name)?;
    let content = read_file(name)?;
    let n = content.parse::<i32>()?;
    Ok(n)
}

fn main() {
    let files = ["data.txt", "config.txt", "empty.txt", "missing.txt", "secret.txt"];
    
    for &file in &files {
        match parse_number(file) {
            Ok(n) => println!("{}: {}", file, n),
            Err(e) => println!("{}: 错误 - {}", file, e),
        }
    }
}
```

**说明：** 这个综合练习展示了 Rust 错误处理的最佳实践：1) 使用枚举清晰表达不同错误类型（`FileError`）；2) 通过 `From` trait 实现自动错误转换；3) 使用包装错误（`ParseError`）保留原始错误信息；4) 使用 `?` 操作符简化错误传播。输出结果：
- `data.txt: 42`
- `config.txt: 错误 - [整数解析失败] invalid digit found in string`
- `empty.txt: 错误 - [文件操作失败] 文件为空`
- `missing.txt: 错误 - [文件操作失败] 文件不存在`
- `secret.txt: 错误 - [文件操作失败] 权限不足`
</details>
