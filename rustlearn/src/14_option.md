# 14 Option

`Option<T>` 是 Rust 中最常用的枚举之一，表示一个值可能存在（`Some(T)`）也可能不存在（`None`）。它类似 Java 的 `Optional<T>` 或 C++17 的 `std::optional<T>`，但 Rust 通过模式匹配和组合子方法提供了更安全、更灵活的使用方式。本章练习涵盖 unwrap/expect、模式匹配、组合子（map/and_then）、take/replace/as_ref、Option 与 Result 互转等核心操作。

### 练习 14-01: unwrap 基础

> 难度：⭐
> 类似 Java 的 `Optional.get()` / C++ 的 `optional::value()`

补全代码，使用 `unwrap()` 从 `Option` 中取出值。

```rust
fn main() {
    let x: Option<i32> = Some(42);
    // TODO: 使用 unwrap() 取出 x 中的值并打印
    let value = // TODO
    println!("value = {}", value);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x: Option<i32> = Some(42);
    let value = x.unwrap();
    println!("value = {}", value);
}
```

**说明：** `unwrap()` 是取出 `Some` 内部值最简单的方法。如果 `Option` 是 `None`，`unwrap()` 会 panic。在生产代码中应优先使用模式匹配或组合子方法，`unwrap` 通常仅用于原型和测试。
</details>

### 练习 14-02: expect 基础

> 难度：⭐
> 类似 Java 的 `Optional.orElseThrow()` / C++ 的 `optional::value()`

补全代码，使用 `expect()` 从 `Option` 中取出值，并提供有意义的错误信息。

```rust
fn main() {
    let y: Option<i32> = None;
    // TODO: 使用 expect() 取出 y 中的值，错误信息为 "y 应该是 Some"
    let value = // TODO
    println!("value = {}", value);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let y: Option<i32> = None;
    let value = y.expect("y 应该是 Some");
    println!("value = {}", value);
}
```

**说明：** `expect()` 与 `unwrap()` 功能相同，但允许自定义 panic 信息。当 `Option` 为 `None` 时，panic 消息会包含你提供的字符串，便于定位问题。两者在 `None` 时都会 panic，应谨慎使用。
</details>

### 练习 14-03: match 匹配 Option

> 难度：⭐⭐
> 类似 Java 的 `if (opt.isPresent())` / C++ 的 `if (opt.has_value())`

补全代码，使用 `match` 表达式处理 `Option`，分别处理 `Some` 和 `None` 的情况。

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    // TODO: 使用 match 处理 result
    // Some(v) 时打印 "结果: {v}"
    // None 时打印 "除数不能为零"
    match result {
        // TODO: 补全分支
    }
    
    let invalid = divide(5.0, 0.0);
    // TODO: 使用 match 处理 invalid
    match invalid {
        // TODO: 补全分支
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    match result {
        Some(v) => println!("结果: {}", v),
        None => println!("除数不能为零"),
    }
    
    let invalid = divide(5.0, 0.0);
    match invalid {
        Some(v) => println!("结果: {}", v),
        None => println!("除数不能为零"),
    }
}
```

**说明：** `match` 是处理 `Option` 最安全的方式，编译器强制你同时处理 `Some` 和 `None` 两个分支，不会遗漏。这是 Rust 中"让错误不可能发生"的设计理念的体现。
</details>

### 练习 14-04: if let 匹配 Option

> 难度：⭐⭐
> 类似 Java 的 `if (opt.isPresent())` / C++17 的 `if (auto x = opt)`

补全代码，使用 `if let` 简化只关心 `Some` 分支的匹配。

```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

fn main() {
    // TODO: 使用 if let 处理 find_user(1) 的结果
    // 如果找到用户，打印 "用户: {name}"
    if let Some(name) = find_user(1) {
        // TODO
    }
    
    // TODO: 使用 if let + else 处理 find_user(3) 的结果
    // 如果找到用户，打印 "用户: {name}"
    // 否则打印 "用户未找到"
    if let Some(name) = find_user(3) {
        // TODO
    } else {
        // TODO
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

fn main() {
    if let Some(name) = find_user(1) {
        println!("用户: {}", name);
    }
    
    if let Some(name) = find_user(3) {
        println!("用户: {}", name);
    } else {
        println!("用户未找到");
    }
}
```

**说明：** `if let` 是 `match` 的语法糖，当你只关心一个分支时更简洁。`if let Some(x) = option` 类似 C++17 的 `if (auto x = opt)` 但更安全——它强制解构，不会意外访问空值。
</details>

### 练习 14-05: 挑战——Option 基础综合

> 难度：⭐⭐⭐
> 综合运用 unwrap、expect、match 和 if let 处理 Option

实现一个简单的配置查询系统。定义 `Config` 结构体，包含可选的数据库主机、端口和超时时间。实现 `build_connection_string` 方法，将所有配置组合成连接字符串。

要求：
- 如果 `host` 是 `None`，直接返回 `None`
- 如果 `port` 是 `None`，使用默认端口 `5432`
- 如果 `timeout` 是 `None`，使用默认超时 `30`
- 返回格式: `"postgresql://{host}:{port}?timeout={timeout}"`

```rust
struct Config {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

// TODO: 为 Config 实现 build_connection_string 方法
// 返回 Option<String>
impl Config {
    fn build_connection_string(&self) -> Option<String> {
        // 使用 match 或 if let 提取值
        // 提示：let host = self.host.as_ref()?; 或使用 match
        // TODO
    }
}

fn main() {
    let cfg = Config {
        host: Some("db.example.com".to_string()),
        port: None,     // 应使用默认 5432
        timeout: Some(15),
    };
    
    match cfg.build_connection_string() {
        Some(s) => println!("连接字符串: {}", s),
        None => println!("配置无效：缺少主机名"),
    }
    
    let invalid = Config {
        host: None,
        port: Some(8080),
        timeout: None,
    };
    
    match invalid.build_connection_string() {
        Some(s) => println!("连接字符串: {}", s),
        None => println!("配置无效：缺少主机名"),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Config {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl Config {
    fn build_connection_string(&self) -> Option<String> {
        let host = self.host.as_ref()?;
        let port = self.port.unwrap_or(5432);
        let timeout = self.timeout.unwrap_or(30);
        Some(format!("postgresql://{}:{}?timeout={}", host, port, timeout))
    }
}

fn main() {
    let cfg = Config {
        host: Some("db.example.com".to_string()),
        port: None,
        timeout: Some(15),
    };
    
    match cfg.build_connection_string() {
        Some(s) => println!("连接字符串: {}", s),
        None => println!("配置无效：缺少主机名"),
    }
    
    let invalid = Config {
        host: None,
        port: Some(8080),
        timeout: None,
    };
    
    match invalid.build_connection_string() {
        Some(s) => println!("连接字符串: {}", s),
        None => println!("配置无效：缺少主机名"),
    }
}
```

**说明：** `?` 运算符在 `Option` 上使用时，如果值是 `None` 则立即返回 `None`，否则解包为内部值。`unwrap_or` 提供默认值，当 `Option` 为 `None` 时使用备选值。`as_ref()` 将 `Option<String>` 转为 `Option<&String>`，避免所有权转移。
</details>

### 练习 14-06: map 组合子

> 难度：⭐
> 类似 Java 的 `Optional.map()` / C++ 的 `std::optional::transform()`

补全代码，使用 `map` 对 `Option` 中的值进行转换。

```rust
fn main() {
    let some_num: Option<i32> = Some(5);
    // TODO: 使用 map 将 some_num 中的值乘以 2
    // 结果应为 Some(10)
    let doubled = // TODO
    println!("doubled = {:?}", doubled);
    
    let none_num: Option<i32> = None;
    // TODO: 使用 map 处理 none_num（乘以 2），应为 None
    let doubled_none = // TODO
    println!("doubled_none = {:?}", doubled_none);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let some_num: Option<i32> = Some(5);
    let doubled = some_num.map(|x| x * 2);
    println!("doubled = {:?}", doubled);
    
    let none_num: Option<i32> = None;
    let doubled_none = none_num.map(|x| x * 2);
    println!("doubled_none = {:?}", doubled_none);
}
```

**说明：** `map` 是 Option 最常用的组合子之一：如果值是 `Some(x)`，则对 `x` 应用闭包并返回 `Some(结果)`；如果值是 `None`，则直接返回 `None`。这比手动 `match` 更简洁，且天然支持链式调用。
</details>

### 练习 14-07: map_or 组合子

> 难度：⭐
> 类似 Java 的 `Optional.map().orElse()` / C++ 的 `std::optional::value_or()`

补全代码，使用 `map_or` 在转换的同时提供默认值。

```rust
fn to_prefixed_string(s: Option<&str>) -> String {
    // TODO: 使用 map_or
    // 如果 s 是 Some(v)，返回 "值: {v}"
    // 如果 s 是 None，返回 "值: (空)"
    s.map_or(/* TODO */, /* TODO */)
}

fn main() {
    println!("{}", to_prefixed_string(Some("hello")));
    println!("{}", to_prefixed_string(None));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn to_prefixed_string(s: Option<&str>) -> String {
    s.map_or("值: (空)".to_string(), |v| format!("值: {}", v))
}

fn main() {
    println!("{}", to_prefixed_string(Some("hello")));
    println!("{}", to_prefixed_string(None));
}
```

**说明：** `map_or(default, f)` 等价于 `map(f).unwrap_or(default)`。第一个参数是默认值，第二个参数是转换闭包。当 `Option` 为 `None` 时使用默认值，为 `Some(x)` 时对 `x` 应用闭包。注意两个参数的类型必须匹配——这里都是 `String`。
</details>

### 练习 14-08: and_then 链式调用

> 难度：⭐⭐
> 类似 Java 的 `Optional.flatMap()` / C++23 的 `std::optional::and_then()`

补全代码，使用 `and_then` 进行链式调用。编写一个解析整数并计算平方根的函数链。

```rust
fn parse_number(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn sqrt(n: i32) -> Option<f64> {
    if n >= 0 {
        Some((n as f64).sqrt())
    } else {
        None
    }
}

fn main() {
    // TODO: 使用 and_then 将 parse_number 和 sqrt 链式组合
    // 从字符串 "16" 开始，解析后计算平方根
    let result: Option<f64> = // TODO
    println!("sqrt(16) = {:?}", result); // 应输出 Some(4.0)
    
    // 从字符串 "-4" 开始
    let negative: Option<f64> = // TODO
    println!("sqrt(-4) = {:?}", negative); // 应输出 None
    
    // 从字符串 "abc" 开始
    let invalid: Option<f64> = // TODO
    println!("sqrt(abc) = {:?}", invalid); // 应输出 None
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn parse_number(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn sqrt(n: i32) -> Option<f64> {
    if n >= 0 {
        Some((n as f64).sqrt())
    } else {
        None
    }
}

fn main() {
    let result: Option<f64> = parse_number("16").and_then(sqrt);
    println!("sqrt(16) = {:?}", result);
    
    let negative: Option<f64> = parse_number("-4").and_then(sqrt);
    println!("sqrt(-4) = {:?}", negative);
    
    let invalid: Option<f64> = parse_number("abc").and_then(sqrt);
    println!("sqrt(abc) = {:?}", invalid);
}
```

**说明：** `and_then` 类似 `map`，但闭包返回的是 `Option<U>` 而非 `U`。这在链式调用中非常有用——每一步都可能失败（返回 `None`），而 `and_then` 会自动传播 `None`。这类似 Java 的 `flatMap`，避免了嵌套的 `Option<Option<...>>`。
</details>

### 练习 14-09: and_then 链式调用进阶

> 难度：⭐⭐
> 类似 Java 的 `Optional.flatMap()` 链式调用

补全代码，使用 `and_then` 构建多步处理管道。给定一个可能为空的字符串，依次执行：去除前后空格 → 提取前两个字符作为前缀 → 转为大写。

```rust
fn trim_option(s: Option<&str>) -> Option<&str> {
    s.map(|s| s.trim())
}

fn prefix(s: &str) -> Option<&str> {
    if s.len() >= 2 {
        Some(&s[..2])
    } else {
        None
    }
}

fn upper(s: &str) -> Option<String> {
    if s.chars().all(|c| c.is_ascii()) {
        Some(s.to_uppercase())
    } else {
        None
    }
}

fn process(input: Option<&str>) -> Option<String> {
    // TODO: 使用 and_then 链式调用 trim_option -> prefix -> upper
    // 提示：需要先用 map 处理 trim_option 返回的 Option<&str>
    // 然后用 and_then 接 prefix，再用 and_then 接 upper
    // TODO
}

fn main() {
    println!("{:?}", process(Some("  hello  "))); // Some("HE")
    println!("{:?}", process(Some("a")));         // None（长度不足）
    println!("{:?}", process(Some("  你好 ")));   // None（非 ASCII）
    println!("{:?}", process(None));              // None
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn trim_option(s: Option<&str>) -> Option<&str> {
    s.map(|s| s.trim())
}

fn prefix(s: &str) -> Option<&str> {
    if s.len() >= 2 {
        Some(&s[..2])
    } else {
        None
    }
}

fn upper(s: &str) -> Option<String> {
    if s.chars().all(|c| c.is_ascii()) {
        Some(s.to_uppercase())
    } else {
        None
    }
}

fn process(input: Option<&str>) -> Option<String> {
    trim_option(input)
        .and_then(prefix)
        .and_then(upper)
}

fn main() {
    println!("{:?}", process(Some("  hello  ")));
    println!("{:?}", process(Some("a")));
    println!("{:?}", process(Some("  你好 ")));
    println!("{:?}", process(None));
}
```

**说明：** `and_then` 链式调用让多步可能失败的处理变得清晰流畅。每一步的输出类型都是 `Option<T>`，自动在 `None` 时短路。注意 `trim_option` 返回 `Option<&str>`，所以后面用 `and_then` 而非 `map`——如果错用 `map` 会产生 `Option<Option<&str>>` 的嵌套。
</details>

### 练习 14-10: 挑战——组合子链式调用综合

> 难度：⭐⭐⭐
> 综合运用 map、and_then、map_or 等组合子

实现一个用户积分系统。给定一个用户 ID，查询用户信息，然后根据用户等级计算折扣后的积分。

规则：
1. `find_user(id)` 返回 `Option<User>`（用户可能不存在）
2. 每个 `User` 有 `name`、`level`（1-5）和 `points`（积分）
3. 如果 `points >= 1000`，标记为 VIP（用 `Some(true)`），否则 `Some(false)`；如果用户不存在则整条链返回 `None`
4. 根据 level 计算折扣倍率：1→1.0，2→0.9，3→0.8，4→0.7，5→0.6
5. 最终返回 `(name, is_vip, discounted_points)` 的 `Option`

```rust
struct User {
    name: String,
    level: u32,
    points: u32,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            name: "Alice".to_string(),
            level: 3,
            points: 1500,
        }),
        2 => Some(User {
            name: "Bob".to_string(),
            level: 5,
            points: 800,
        }),
        3 => Some(User {
            name: "Charlie".to_string(),
            level: 1,
            points: 200,
        }),
        _ => None,
    }
}

fn discount_rate(level: u32) -> f64 {
    match level {
        1 => 1.0,
        2 => 0.9,
        3 => 0.8,
        4 => 0.7,
        5 => 0.6,
        _ => 1.0,
    }
}

// TODO: 实现 process_user 函数，使用组合子链式调用
// 返回 Option<(String, bool, f64)>
fn process_user(id: u32) -> Option<(String, bool, f64)> {
    // 提示: find_user(id)
    //       .map(|user| { ... })  // 在这里处理 name, is_vip, 计算折扣积分
    // TODO
}

fn main() {
    // Alice: level 3, 1500 分 → VIP, 折扣后 1500 * 0.8 = 1200
    println!("Alice: {:?}", process_user(1));
    // Bob: level 5, 800 分 → 非 VIP, 折扣后 800 * 0.6 = 480
    println!("Bob: {:?}", process_user(2));
    // Charlie: level 1, 200 分 → 非 VIP, 折扣后 200 * 1.0 = 200
    println!("Charlie: {:?}", process_user(3));
    // 不存在的用户
    println!("Unknown: {:?}", process_user(99));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct User {
    name: String,
    level: u32,
    points: u32,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            name: "Alice".to_string(),
            level: 3,
            points: 1500,
        }),
        2 => Some(User {
            name: "Bob".to_string(),
            level: 5,
            points: 800,
        }),
        3 => Some(User {
            name: "Charlie".to_string(),
            level: 1,
            points: 200,
        }),
        _ => None,
    }
}

fn discount_rate(level: u32) -> f64 {
    match level {
        1 => 1.0,
        2 => 0.9,
        3 => 0.8,
        4 => 0.7,
        5 => 0.6,
        _ => 1.0,
    }
}

fn process_user(id: u32) -> Option<(String, bool, f64)> {
    find_user(id).map(|user| {
        let is_vip = user.points >= 1000;
        let rate = discount_rate(user.level);
        let discounted = user.points as f64 * rate;
        (user.name, is_vip, discounted)
    })
}

fn main() {
    println!("Alice: {:?}", process_user(1));
    println!("Bob: {:?}", process_user(2));
    println!("Charlie: {:?}", process_user(3));
    println!("Unknown: {:?}", process_user(99));
}
```

**说明：** 使用 `map` 可以在 `Option` 内部对值进行转换。这里 `find_user(id)` 返回 `Option<User>`，`map` 中的闭包接收 `User` 并返回 `(String, bool, f64)` 三元组。整个表达式的结果是 `Option<(String, bool, f64)>`。`map` 在遇到 `None` 时直接返回 `None`，无需手动检查。
</details>

### 练习 14-11: take 和 replace

> 难度：⭐
> 类似 Java 的 `Optional.orElse(null)` + 置空 / C++ 的 `std::exchange`

补全代码，使用 `take` 将 `Option` 置为 `None` 并取出原值，使用 `replace` 替换 `Option` 的值。

```rust
fn main() {
    let mut x = Some(42);
    
    // TODO: 使用 take 将 x 置为 None 并取出原值赋给 y
    let y = // TODO
    println!("x = {:?}, y = {:?}", x, y); // x = None, y = Some(42)
    
    let mut z = Some(10);
    // TODO: 使用 replace 将 z 替换为 Some(20)，并取出旧值赋给 old
    let old = // TODO
    println!("old = {:?}, z = {:?}", old, z); // old = Some(10), z = Some(20)
    
    let mut none_val: Option<i32> = None;
    // TODO: 对 None 使用 take，结果应为 None
    let taken = // TODO
    println!("taken = {:?}", taken); // None
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut x = Some(42);
    
    let y = x.take();
    println!("x = {:?}, y = {:?}", x, y);
    
    let mut z = Some(10);
    let old = z.replace(20);
    println!("old = {:?}, z = {:?}", old, z);
    
    let mut none_val: Option<i32> = None;
    let taken = none_val.take();
    println!("taken = {:?}", taken);
}
```

**说明：** `take()` 将 `Option` 置为 `None` 并返回原值（所有权的安全转移）。`replace(value)` 将 `Option` 替换为新值并返回旧值。两者都无需 `&mut self` 之外的其他限制。`take` 常用于从结构中移出字段而不违反借用规则。
</details>

### 练习 14-12: as_ref 基础

> 难度：⭐
> 类似 C++ 中 `optional::value()` 的 const 重载

补全代码，使用 `as_ref` 在持有引用时获取 `Option` 内部值的引用。

```rust
fn print_if_some(val: &Option<String>) {
    // TODO: 使用 as_ref() 配合 if let 打印内部值
    // 提示：val.as_ref() 将 &Option<String> 转为 Option<&String>
    if let Some(s) = val.as_ref() {
        // TODO: 打印 s
    }
}

fn main() {
    let x = Some("hello".to_string());
    let y: Option<String> = None;
    
    print_if_some(&x); // 应打印 "hello"
    print_if_some(&y); // 什么都不打印
    
    // 也可以配合 map 使用
    // TODO: 使用 as_ref() + map 获取 x 中字符串的长度
    let len: Option<usize> = // TODO
    println!("len = {:?}", len); // Some(5)
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn print_if_some(val: &Option<String>) {
    if let Some(s) = val.as_ref() {
        println!("{}", s);
    }
}

fn main() {
    let x = Some("hello".to_string());
    let y: Option<String> = None;
    
    print_if_some(&x);
    print_if_some(&y);
    
    let len: Option<usize> = x.as_ref().map(|s| s.len());
    println!("len = {:?}", len);
}
```

**说明：** `as_ref()` 将 `Option<T>` 转换为 `Option<&T>`，在不获取所有权的情况下访问内部值。这在只需要借用内部值（如打印、计算长度）时非常有用。类似地还有 `as_mut()` 获取可变引用。
</details>

### 练习 14-13: Option 转 Result——ok_or

> 难度：⭐⭐
> 类似 Java 的 `Optional.orElseThrow()` / C++ 的 `optional::value_or()`

补全代码，使用 `ok_or` 将 `Option` 转换为 `Result`，将 `None` 映射为错误信息。

```rust
fn get_env_var(name: &str) -> Option<String> {
    match name {
        "HOME" => Some("/home/user".to_string()),
        "PATH" => Some("/usr/bin:/bin".to_string()),
        _ => None,
    }
}

// TODO: 实现 read_env_var，返回 Result<String, String>
// 使用 ok_or 将 None 转换为错误信息 "环境变量 {name} 未设置"
fn read_env_var(name: &str) -> Result<String, String> {
    // TODO
}

fn main() {
    match read_env_var("HOME") {
        Ok(val) => println!("HOME = {}", val),
        Err(e) => println!("错误: {}", e),
    }
    
    match read_env_var("UNDEFINED") {
        Ok(val) => println!("UNDEFINED = {}", val),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn get_env_var(name: &str) -> Option<String> {
    match name {
        "HOME" => Some("/home/user".to_string()),
        "PATH" => Some("/usr/bin:/bin".to_string()),
        _ => None,
    }
}

fn read_env_var(name: &str) -> Result<String, String> {
    get_env_var(name).ok_or_else(|| format!("环境变量 {} 未设置", name))
}

fn main() {
    match read_env_var("HOME") {
        Ok(val) => println!("HOME = {}", val),
        Err(e) => println!("错误: {}", e),
    }
    
    match read_env_var("UNDEFINED") {
        Ok(val) => println!("UNDEFINED = {}", val),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** `ok_or(error)` 将 `Option<T>` 转换为 `Result<T, E>`：`Some(v)` → `Ok(v)`，`None` → `Err(error)`。`ok_or_else(f)` 接受闭包惰性求值，避免在 `Some` 时创建错误值。反过来，`Result` 的 `ok()` 方法将其转换为 `Option`。
</details>

### 练习 14-14: transpos——`Result<Option<T>>` 的翻转

> 难度：⭐⭐
> Rust 独有的实用工具，用于翻转嵌套的 Result 和 Option

补全代码，使用 `transpose` 交换 `Result<Option<T>>` 的层级顺序。

```rust
fn parse_score(input: &str) -> Result<Option<i32>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None); // 空字符串视为成绩缺失
    }
    match trimmed.parse::<i32>() {
        Ok(n) if (0..=100).contains(&n) => Ok(Some(n)),
        Ok(_) => Err(format!("成绩 {} 不在 0-100 范围内", trimmed)),
        Err(_) => Err(format!("'{}' 不是有效数字", trimmed)),
    }
}

fn main() {
    // 现有数据: Vec<Result<Option<i32>, String>>
    let scores = vec![
        parse_score("85"),
        parse_score(""),
        parse_score("-1"),
        parse_score("92"),
        parse_score("abc"),
    ];
    
    // 当前类型: Vec<Result<Option<i32>, String>>
    // TODO: 使用 transpose() 将每个元素翻转
    // 目标: Vec<Option<Result<i32, String>>>
    // 规则: Ok(Some(v)) → Some(Ok(v)), Ok(None) → None, Err(e) → Some(Err(e))
    let transposed: Vec<Option<Result<i32, String>>> = scores
        .into_iter()
        .map(|item| /* TODO: 使用 transpose */)
        .collect();
    
    for item in transposed {
        match item {
            Some(Ok(v)) => println!("有效成绩: {}", v),
            None => println!("成绩缺失（跳过）"),
            Some(Err(e)) => println!("错误: {}", e),
        }
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn parse_score(input: &str) -> Result<Option<i32>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    match trimmed.parse::<i32>() {
        Ok(n) if (0..=100).contains(&n) => Ok(Some(n)),
        Ok(_) => Err(format!("成绩 {} 不在 0-100 范围内", trimmed)),
        Err(_) => Err(format!("'{}' 不是有效数字", trimmed)),
    }
}

fn main() {
    let scores = vec![
        parse_score("85"),
        parse_score(""),
        parse_score("-1"),
        parse_score("92"),
        parse_score("abc"),
    ];
    
    let transposed: Vec<Option<Result<i32, String>>> = scores
        .into_iter()
        .map(|item| item.transpose())
        .collect();
    
    for item in transposed {
        match item {
            Some(Ok(v)) => println!("有效成绩: {}", v),
            None => println!("成绩缺失（跳过）"),
            Some(Err(e)) => println!("错误: {}", e),
        }
    }
}
```

**说明：** `transpose()` 将 `Result<Option<T>, E>` 翻转为 `Option<Result<T, E>>`。这在处理"可能出错的可能缺失的数据"时非常有用——翻转后，`None` 表示数据缺失（跳过），`Some(Err(e))` 表示解析出错，`Some(Ok(v))` 表示有效数据。这种分层让错误处理和缺失处理更清晰。
</details>

### 练习 14-15: 挑战——多层 Option 数据结构处理

> 难度：⭐⭐⭐
> 综合运用 map、and_then、as_ref、ok_or 处理多层嵌套的 Option

实现一个公司部门员工查询系统。给定部门和员工 ID，返回员工的薪资信息。

数据结构：
- `Company` 包含多个 `Department`（用 `HashMap` 表示）
- `Department` 包含多个 `Employee`
- `Employee` 有 `name`、`salary`、`position`

要求实现以下函数：
1. `get_employee_salary(company, dept_name, emp_id)` → `Option<f64>` — 使用组合子链式调用
2. `get_employee_summary(company, dept_name, emp_id)` → `Result<String, String>` — 使用 `ok_or` 将 `None` 转为错误

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    salary: f64,
    position: String,
}

#[derive(Debug, Clone)]
struct Department {
    name: String,
    employees: HashMap<u32, Employee>,
}

#[derive(Debug, Clone)]
struct Company {
    departments: HashMap<String, Department>,
}

fn get_employee_salary(company: &Company, dept_name: &str, emp_id: u32) -> Option<f64> {
    // TODO: 使用组合子链式调用
    // company.departments.get(dept_name) -> Option<&Department>
    //   .and_then(|dept| dept.employees.get(&emp_id)) -> Option<&Employee>
    //   .map(|emp| emp.salary)
    // TODO
}

fn get_employee_summary(company: &Company, dept_name: &str, emp_id: u32) -> Result<String, String> {
    // TODO: 使用 ok_or 将链式调用的结果转为 Result
    // 如果部门不存在，错误信息 "部门 {dept_name} 不存在"
    // 如果员工不存在，错误信息 "员工 {emp_id} 不在 {dept_name} 部门"
    // 成功时返回 "{name}（{position}）: 薪资 {salary}"
    // TODO
}

fn main() {
    let mut company = Company {
        departments: HashMap::new(),
    };
    
    let mut eng = Department {
        name: "工程部".to_string(),
        employees: HashMap::new(),
    };
    eng.employees.insert(
        1001,
        Employee {
            name: "张三".to_string(),
            salary: 25000.0,
            position: "高级工程师".to_string(),
        },
    );
    eng.employees.insert(
        1002,
        Employee {
            name: "李四".to_string(),
            salary: 18000.0,
            position: "工程师".to_string(),
        },
    );
    company.departments.insert("工程部".to_string(), eng);
    
    let mut hr = Department {
        name: "人事部".to_string(),
        employees: HashMap::new(),
    };
    hr.employees.insert(
        2001,
        Employee {
            name: "王五".to_string(),
            salary: 15000.0,
            position: "人事经理".to_string(),
        },
    );
    company.departments.insert("人事部".to_string(), hr);
    
    // 测试 get_employee_salary
    println!("张三薪资: {:?}", get_employee_salary(&company, "工程部", 1001));
    println!("未知员工: {:?}", get_employee_salary(&company, "工程部", 9999));
    println!("未知部门: {:?}", get_employee_salary(&company, "市场部", 1001));
    
    // 测试 get_employee_summary
    println!("---");
    match get_employee_summary(&company, "工程部", 1001) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("错误: {}", e),
    }
    match get_employee_summary(&company, "工程部", 9999) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("错误: {}", e),
    }
    match get_employee_summary(&company, "市场部", 1001) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("错误: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    salary: f64,
    position: String,
}

#[derive(Debug, Clone)]
struct Department {
    name: String,
    employees: HashMap<u32, Employee>,
}

#[derive(Debug, Clone)]
struct Company {
    departments: HashMap<String, Department>,
}

fn get_employee_salary(company: &Company, dept_name: &str, emp_id: u32) -> Option<f64> {
    company.departments
        .get(dept_name)
        .and_then(|dept| dept.employees.get(&emp_id))
        .map(|emp| emp.salary)
}

fn get_employee_summary(company: &Company, dept_name: &str, emp_id: u32) -> Result<String, String> {
    let dept = company.departments
        .get(dept_name)
        .ok_or_else(|| format!("部门 {} 不存在", dept_name))?;
    
    let emp = dept.employees
        .get(&emp_id)
        .ok_or_else(|| format!("员工 {} 不在 {} 部门", emp_id, dept_name))?;
    
    Ok(format!("{}（{}）: 薪资 {}", emp.name, emp.position, emp.salary))
}

fn main() {
    let mut company = Company {
        departments: HashMap::new(),
    };
    
    let mut eng = Department {
        name: "工程部".to_string(),
        employees: HashMap::new(),
    };
    eng.employees.insert(
        1001,
        Employee {
            name: "张三".to_string(),
            salary: 25000.0,
            position: "高级工程师".to_string(),
        },
    );
    eng.employees.insert(
        1002,
        Employee {
            name: "李四".to_string(),
            salary: 18000.0,
            position: "工程师".to_string(),
        },
    );
    company.departments.insert("工程部".to_string(), eng);
    
    let mut hr = Department {
        name: "人事部".to_string(),
        employees: HashMap::new(),
    };
    hr.employees.insert(
        2001,
        Employee {
            name: "王五".to_string(),
            salary: 15000.0,
            position: "人事经理".to_string(),
        },
    );
    company.departments.insert("人事部".to_string(), hr);
    
    println!("张三薪资: {:?}", get_employee_salary(&company, "工程部", 1001));
    println!("未知员工: {:?}", get_employee_salary(&company, "工程部", 9999));
    println!("未知部门: {:?}", get_employee_salary(&company, "市场部", 1001));
    
    println!("---");
    match get_employee_summary(&company, "工程部", 1001) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("错误: {}", e),
    }
    match get_employee_summary(&company, "工程部", 9999) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("错误: {}", e),
    }
    match get_employee_summary(&company, "市场部", 1001) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("错误: {}", e),
    }
}
```

**说明：** 这个综合练习展示了多层 `Option` 的处理方式。`HashMap::get` 返回 `Option<&V>`，通过 `and_then` 链式调用逐层深入数据结构。`ok_or_else` 在需要将缺失信息转换为具体错误时非常有用，配合 `?` 运算符可以让代码保持线性可读。注意这里 `HashMap::get(&emp_id)` 传入的是 `&u32` 而非 `u32`，因为 `get` 接受键的引用。
</details>
