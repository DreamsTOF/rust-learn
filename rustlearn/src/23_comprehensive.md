# 23 综合实战

本章是本书的最终章，将前面 22 章所学的所有权、借用、生命周期、结构体、枚举、模式匹配、Trait、泛型、迭代器、闭包、错误处理、智能指针、多线程、异步编程等核心概念融合到五个完整项目中。通过完成这 25 道综合实战题，你将真正具备用 Rust 解决实际问题的能力。

---

## 项目一：迷你 JSON 解析器

> 本系列 5 道题从零构建一个简化版 JSON 解析器，覆盖枚举建模、递归下降解析、错误处理、迭代器适配、格式化输出等核心技能。

### 练习 23-01: 用枚举表示 JSON 值

> 难度：⭐⭐
> 本练习融合：枚举 + 泛型 + Box（递归类型）+ 模式匹配

Rust 的枚举非常适合表示 JSON 这种异构数据结构。一个 JSON 值可以是 null、布尔、数字、字符串、数组或对象（键值对）。

```rust
// 补全 JsonValue 枚举，使其能够表示完整的 JSON 数据类型
// 提示：数组用 Vec<JsonValue>，对象用 Vec<(String, JsonValue)> 或 HashMap
// 注意递归类型需要 Box 包裹

// TODO: 定义 JsonValue 枚举，包含 Null、Bool(bool)、Number(f64)、
//       String(String)、Array(Vec<JsonValue>)、Object(Vec<(String, JsonValue)>) 变体

fn main() {
    // 创建一个 JSON 对象: {"name": "Rust", "year": 2015, "features": ["安全", "并发", "高性能"]}
    // TODO: 使用 JsonValue::Object 构建上述 JSON 结构
    
    // TODO: 打印对象的 "name" 字段值
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, Clone)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn main() {
    let json = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::String("Rust".to_string())),
        ("year".to_string(), JsonValue::Number(2015.0)),
        ("features".to_string(), JsonValue::Array(vec![
            JsonValue::String("安全".to_string()),
            JsonValue::String("并发".to_string()),
            JsonValue::String("高性能".to_string()),
        ])),
    ]);

    if let JsonValue::Object(ref fields) = json {
        for (key, value) in fields {
            if key == "name" {
                if let JsonValue::String(ref name) = value {
                    println!("name: {}", name);
                }
            }
        }
    }
}
```

**说明：** 递归类型（如 `Object` 包含 `JsonValue`）必须用 `Box` 或 `Vec` 等堆分配容器打破递归，否则编译时大小无法确定。这里 `Vec` 和 `String` 本身已在堆上分配，因此不需要额外 `Box`。
</details>

---

### 练习 23-02: 解析 JSON null 和布尔值

> 难度：⭐⭐
> 本练习融合：模式匹配 + 字符迭代器 + Option + 切片

实现一个极简的 JSON 分词/解析器，从字符串切片逐步解析。本练习先解析简单的 `null` 和布尔值。

```rust
// 实现两个解析函数 parse_null 和 parse_bool
// 输入是 &str，返回值是 Option<(JsonValue, &str)> — 解析结果和剩余未解析部分

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// TODO: 实现 parse_null，输入 "null..." 返回 Some((JsonValue::Null, 剩余部分))
fn parse_null(input: &str) -> Option<(JsonValue, &str)> {
    // 检查 input 是否以 "null" 开头
    todo!()
}

// TODO: 实现 parse_bool，输入 "true..." 或 "false..." 返回对应的 JsonValue
fn parse_bool(input: &str) -> Option<(JsonValue, &str)> {
    todo!()
}

fn main() {
    // 测试解析
    assert_eq!(parse_null("null"), Some((JsonValue::Null, "")));
    assert_eq!(parse_null("null,abc"), Some((JsonValue::Null, ",abc")));
    assert_eq!(parse_bool("true"), Some((JsonValue::Bool(true), "")));
    assert_eq!(parse_bool("false,xyz"), Some((JsonValue::Bool(false), ",xyz")));
    assert_eq!(parse_bool("tru"), None);
    println!("JSON 解析基础测试通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn parse_null(input: &str) -> Option<(JsonValue, &str)> {
    if input.starts_with("null") {
        Some((JsonValue::Null, &input[4..]))
    } else {
        None
    }
}

fn parse_bool(input: &str) -> Option<(JsonValue, &str)> {
    if input.starts_with("true") {
        Some((JsonValue::Bool(true), &input[4..]))
    } else if input.starts_with("false") {
        Some((JsonValue::Bool(false), &input[5..]))
    } else {
        None
    }
}

fn main() {
    assert_eq!(parse_null("null"), Some((JsonValue::Null, "")));
    assert_eq!(parse_null("null,abc"), Some((JsonValue::Null, ",abc")));
    assert_eq!(parse_bool("true"), Some((JsonValue::Bool(true), "")));
    assert_eq!(parse_bool("false,xyz"), Some((JsonValue::Bool(false), ",xyz")));
    assert_eq!(parse_bool("tru"), None);
    println!("JSON 解析基础测试通过！");
}
```

**说明：** 采用"解析器组合子"风格——每个函数接收 `&str` 输入，返回解析结果和剩余输入。`&input[4..]` 利用了字符串切片操作来跳过已解析部分。
</details>

---

### 练习 23-03: 解析 JSON 数字和字符串

> 难度：⭐⭐
> 本练习融合：字符迭代器 + char 方法 + 字符串处理 + Result 转换

解析 JSON 数字（整数和浮点数）和字符串（带转义支持）。

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// TODO: 实现 parse_number，解析可选负号 + 整数部分 + 可选小数部分
// 如 "42", "-3.14", "0.5" 等
fn parse_number(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    // 收集数字字符
    let end = input.len();
    let mut i = 0;
    // 处理负号
    if input.as_bytes().get(i)? == &b'-' {
        i += 1;
    }
    // TODO: 继续读取数字字符（0-9）和小数点（仅允许一个）
    // 使用 input[i..].chars() 遍历
    
    todo!()
}

// TODO: 实现 parse_string，解析双引号包裹的字符串
// 需要处理转义字符如 \" \\ \n \t
fn parse_string(input: &str) -> Option<(JsonValue, &str)> {
    todo!()
}

fn main() {
    // 数字测试
    let (val, rest) = parse_number("42").unwrap();
    assert_eq!(val, JsonValue::Number(42.0));
    let (val, rest) = parse_number("-3.14abc").unwrap();
    assert_eq!(val, JsonValue::Number(-3.14));
    
    // 字符串测试
    let (val, rest) = parse_string("\"hello\"").unwrap();
    assert_eq!(val, JsonValue::String("hello".to_string()));
    let (val, rest) = parse_string("\"hello\\nworld\"").unwrap();
    assert_eq!(val, JsonValue::String("hello\nworld".to_string()));
    
    println!("JSON 数字和字符串解析测试通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn parse_number(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    if input.is_empty() {
        return None;
    }
    let mut i = 0;
    let bytes = input.as_bytes();
    if i < bytes.len() && bytes[i] == b'-' {
        i += 1;
    }
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return None;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    let num_str = &input[..i];
    let value: f64 = num_str.parse().ok()?;
    Some((JsonValue::Number(value), &input[i..]))
}

fn parse_string(input: &str) -> Option<(JsonValue, &str)> {
    let input = input.trim_start();
    if !input.starts_with('"') {
        return None;
    }
    let mut s = String::new();
    let mut chars = input[1..].chars();
    loop {
        match chars.next() {
            None => return None, // 未闭合
            Some('"') => break,
            Some('\\') => {
                match chars.next() {
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('/') => s.push('/'),
                    Some(c) => return None, // 未知转义
                    None => return None,
                }
            }
            Some(c) => s.push(c),
        }
    }
    let consumed = input.len() - chars.as_str().len() - 1;
    Some((JsonValue::String(s), &input[consumed..]))
}

fn main() {
    let (val, _) = parse_number("42").unwrap();
    assert_eq!(val, JsonValue::Number(42.0));
    let (val, _) = parse_number("-3.14abc").unwrap();
    assert_eq!(val, JsonValue::Number(-3.14));
    
    let (val, _) = parse_string("\"hello\"").unwrap();
    assert_eq!(val, JsonValue::String("hello".to_string()));
    let (val, _) = parse_string("\"hello\\nworld\"").unwrap();
    assert_eq!(val, JsonValue::String("hello\nworld".to_string()));
    
    println!("JSON 数字和字符串解析测试通过！");
}
```

**说明：** 数字解析逐字符扫描收集合法的数字字符，最后用 `f64::parse` 转换。字符串解析需要处理反斜杠转义序列——遇到 `\` 时读取下一个字符决定实际字符。`chars.as_str().len()` 是一个巧妙技巧，用来计算已消耗的字符数。
</details>

---

### 练习 23-04: 解析 JSON 数组和对象

> 难度：⭐⭐
> 本练习融合：递归调用 + 跳过空白 + 错误传播 + Vec 操作

在前三题解析器的基础上，实现数组 `[...]` 和对象 `{...}` 的解析。数组元素和对象键值对之间用逗号分隔。

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// 假定已有 parse_value（分派函数）、parse_null、parse_bool、parse_number、parse_string
// TODO: 实现 parse_value，根据第一个字符分派到具体的解析函数

// TODO: 实现 parse_array，解析 [...]，内部元素递归调用 parse_value

// TODO: 实现 parse_object，解析 {...}，键值对用 "key": value 格式

// ===== 以下为已实现的辅助函数（可直接复用）=====
fn parse_null(input: &str) -> Option<(JsonValue, &str)> {
    let s = input.trim_start();
    if s.starts_with("null") {
        Some((JsonValue::Null, &s[4..]))
    } else { None }
}

fn parse_bool(input: &str) -> Option<(JsonValue, &str)> {
    let s = input.trim_start();
    if s.starts_with("true") {
        Some((JsonValue::Bool(true), &s[4..]))
    } else if s.starts_with("false") {
        Some((JsonValue::Bool(false), &s[5..]))
    } else { None }
}

fn parse_number(input: &str) -> Option<(JsonValue, &str)> {
    let s = input.trim_start();
    if s.is_empty() { return None; }
    let mut i = 0;
    let bytes = s.as_bytes();
    if bytes[i] == b'-' { i += 1; }
    if i >= bytes.len() || !bytes[i].is_ascii_digit() { return None; }
    while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    if i < bytes.len() && bytes[i] == b'.' { i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    }
    let num: f64 = s[..i].parse().ok()?;
    Some((JsonValue::Number(num), &s[i..]))
}

fn parse_string(input: &str) -> Option<(JsonValue, &str)> {
    let s = input.trim_start();
    if !s.starts_with('"') { return None; }
    let mut result = String::new();
    let mut chars = s[1..].chars();
    loop {
        match chars.next() {
            None => return None,
            Some('"') => break,
            Some('\\') => match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                _ => return None,
            },
            Some(c) => result.push(c),
        }
    }
    let consumed = s.len() - chars.as_str().len() - 1;
    Some((JsonValue::String(result), &s[consumed..]))
}

fn main() {
    // 测试解析数组: [1, 2, 3]
    let json_str = r#"[1, 2, 3]"#;
    let (val, rest) = parse_value(json_str).unwrap();
    println!("解析数组: {:?}", val);
    
    // 测试解析对象: {"name": "Rust", "year": 2015}
    let json_str = r#"{"name": "Rust", "year": 2015}"#;
    let (val, rest) = parse_value(json_str).unwrap();
    println!("解析对象: {:?}", val);
    
    // 测试嵌套: {"arr": [1, {"key": "val"}], "flag": true}
    let json_str = r#"{"arr": [1, {"key": "val"}], "flag": true}"#;
    let (val, rest) = parse_value(json_str).unwrap();
    println!("解析嵌套: {:?}", val);
    
    println!("JSON 数组和对象解析测试通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn skip_whitespace(input: &str) -> &str {
    input.trim_start()
}

fn parse_value(input: &str) -> Option<(JsonValue, &str)> {
    let input = skip_whitespace(input);
    if input.is_empty() { return None; }
    match input.as_bytes()[0] {
        b'n' => parse_null(input),
        b't' | b'f' => parse_bool(input),
        b'"' => parse_string(input),
        b'-' | b'0'..=b'9' => parse_number(input),
        b'[' => parse_array(input),
        b'{' => parse_object(input),
        _ => None,
    }
}

fn parse_array(input: &str) -> Option<(JsonValue, &str)> {
    let input = skip_whitespace(input);
    let input = input.strip_prefix('[')?;
    let input = skip_whitespace(input);
    let mut values = Vec::new();
    let mut rest = input;
    if rest.starts_with(']') {
        return Some((JsonValue::Array(values), &rest[1..]));
    }
    loop {
        let (val, remaining) = parse_value(rest)?;
        values.push(val);
        rest = skip_whitespace(remaining);
        if rest.starts_with(']') {
            return Some((JsonValue::Array(values), &rest[1..]));
        }
        rest = rest.strip_prefix(',').or(Some(rest))?; // 期望逗号
        rest = skip_whitespace(rest);
    }
}

fn parse_object(input: &str) -> Option<(JsonValue, &str)> {
    let input = skip_whitespace(input);
    let input = input.strip_prefix('{')?;
    let input = skip_whitespace(input);
    let mut fields = Vec::new();
    let mut rest = input;
    if rest.starts_with('}') {
        return Some((JsonValue::Object(fields), &rest[1..]));
    }
    loop {
        let (key, remaining) = parse_string(rest)?;
        let key = if let JsonValue::String(s) = key { s } else { unreachable!() };
        rest = skip_whitespace(remaining);
        rest = rest.strip_prefix(':')?; // 期望冒号
        rest = skip_whitespace(rest);
        let (val, remaining) = parse_value(rest)?;
        fields.push((key, val));
        rest = skip_whitespace(remaining);
        if rest.starts_with('}') {
            return Some((JsonValue::Object(fields), &rest[1..]));
        }
        rest = rest.strip_prefix(',')?; // 期望逗号
        rest = skip_whitespace(rest);
    }
}

// 辅助函数（同前，略去重复）
fn parse_null(input: &str) -> Option<(JsonValue, &str)> {
    let s = skip_whitespace(input);
    if s.starts_with("null") { Some((JsonValue::Null, &s[4..])) } else { None }
}
fn parse_bool(input: &str) -> Option<(JsonValue, &str)> {
    let s = skip_whitespace(input);
    if s.starts_with("true") { Some((JsonValue::Bool(true), &s[4..])) }
    else if s.starts_with("false") { Some((JsonValue::Bool(false), &s[5..])) }
    else { None }
}
fn parse_number(input: &str) -> Option<(JsonValue, &str)> {
    let s = skip_whitespace(input);
    if s.is_empty() { return None; }
    let mut i = 0;
    let bytes = s.as_bytes();
    if bytes[i] == b'-' { i += 1; }
    if i >= bytes.len() || !bytes[i].is_ascii_digit() { return None; }
    while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    if i < bytes.len() && bytes[i] == b'.' { i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
    }
    let num: f64 = s[..i].parse().ok()?;
    Some((JsonValue::Number(num), &s[i..]))
}
fn parse_string(input: &str) -> Option<(JsonValue, &str)> {
    let s = skip_whitespace(input);
    if !s.starts_with('"') { return None; }
    let mut result = String::new();
    let mut chars = s[1..].chars();
    loop {
        match chars.next() {
            None => return None,
            Some('"') => break,
            Some('\\') => match chars.next() {
                Some('"') | Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                _ => return None,
            },
            Some(c) => result.push(c),
        }
    }
    let consumed = s.len() - chars.as_str().len() - 1;
    Some((JsonValue::String(result), &s[consumed..]))
}

fn main() {
    let (val, _) = parse_value(r#"[1, 2, 3]"#).unwrap();
    println!("解析数组: {:?}", val);
    
    let (val, _) = parse_value(r#"{"name": "Rust", "year": 2015}"#).unwrap();
    println!("解析对象: {:?}", val);
    
    let (val, _) = parse_value(r#"{"arr": [1, {"key": "val"}], "flag": true}"#).unwrap();
    println!("解析嵌套: {:?}", val);
    
    println!("JSON 数组和对象解析测试通过！");
}
```

**说明：** `parse_value` 作为分派函数，根据首字节决定调用哪个具体解析器。数组和对象的解析是递归的——内部元素通过 `parse_value` 递归解析。`skip_whitespace` 确保可以处理格式化 JSON 中的空白字符。这种递归下降解析器结构清晰，每种 JSON 值类型对应一个函数。
</details>

---

### 练习 23-05: JSON 序列化（格式化输出）

> 难度：⭐⭐
> 本练习融合：Display trait + 递归 + 缩进管理 + 迭代器

解析的反向操作——将 `JsonValue` 枚举序列化为格式化的 JSON 字符串。实现 `Display` trait，支持带缩进的美化输出。

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// TODO: 为 JsonValue 实现 fmt::Display
// 要求美化输出（带缩进），每层缩进 2 个空格
// 字符串值用双引号包裹并转义特殊字符
// 数字正常输出（整数不要小数点）

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 实现序列化，这里先写无缩进的版本
        // 提示：使用辅助方法 fn fmt_inner(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result
        todo!()
    }
}

fn main() {
    let json = JsonValue::Object(vec![
        ("name".into(), JsonValue::String("Rust".into())),
        ("year".into(), JsonValue::Number(2015.0)),
        ("features".into(), JsonValue::Array(vec![
            JsonValue::String("安全".into()),
            JsonValue::String("并发".into()),
            JsonValue::String("高性能".into()),
        ])),
        ("details".into(), JsonValue::Object(vec![
            ("stable".into(), JsonValue::Bool(true)),
            ("version".into(), JsonValue::Number(2024.0)),
        ])),
    ]);
    
    println!("{}", json);
    // 输出应类似：
    // {
    //   "name": "Rust",
    //   "year": 2015,
    //   "features": [
    //     "安全",
    //     "并发",
    //     "高性能"
    //   ],
    //   "details": {
    //     "stable": true,
    //     "version": 2024
    //   }
    // }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    fn fmt_inner(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let pad = " ".repeat(indent);
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::String(s) => {
                let escaped: String = s.chars().flat_map(|c| match c {
                    '"' => "\\\"".chars().collect(),
                    '\\' => "\\\\".chars().collect(),
                    '\n' => "\\n".chars().collect(),
                    '\t' => "\\t".chars().collect(),
                    '\r' => "\\r".chars().collect(),
                    c => vec![c].into_iter(),
                }).collect();
                write!(f, "\"{}\"", escaped)
            }
            JsonValue::Array(arr) => {
                if arr.is_empty() { return write!(f, "[]"); }
                writeln!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    write!(f, "  {}", pad)?;
                    val.fmt_inner(f, indent + 2)?;
                    if i < arr.len() - 1 { write!(f, ",")?; }
                    writeln!(f)?;
                }
                write!(f, "{}]", pad.trim_end_matches(' ')) // 保持缩进对齐
            }
            JsonValue::Object(obj) => {
                if obj.is_empty() { return write!(f, "{{}}"); }
                writeln!(f, "{{")?;
                for (i, (key, val)) in obj.iter().enumerate() {
                    write!(f, "  {}\"{}\": ", pad, key)?;
                    val.fmt_inner(f, indent + 2)?;
                    if i < obj.len() - 1 { write!(f, ",")?; }
                    writeln!(f)?;
                }
                write!(f, "{}}}", pad)
            }
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_inner(f, 0)
    }
}

fn main() {
    let json = JsonValue::Object(vec![
        ("name".into(), JsonValue::String("Rust".into())),
        ("year".into(), JsonValue::Number(2015.0)),
        ("features".into(), JsonValue::Array(vec![
            JsonValue::String("安全".into()),
            JsonValue::String("并发".into()),
            JsonValue::String("高性能".into()),
        ])),
        ("details".into(), JsonValue::Object(vec![
            ("stable".into(), JsonValue::Bool(true)),
            ("version".into(), JsonValue::Number(2024.0)),
        ])),
    ]);
    
    println!("{}", json);
}
```

**说明：** `fmt_inner` 递归方法接收 `indent` 参数管理缩进级别。对数字用 `fract() == 0.0` 判断是否为整数，避免输出 `2015.0`。字符串转义处理 `"`, `\`, 换行等特殊字符。数组和对象的逗号处理需要注意——最后一项后面不加逗号。
</details>

---

## 项目二：简易 HTTP 客户端

> 本系列 5 道题使用 `std::net::TcpStream` 手动实现 HTTP/1.0 客户端，不依赖第三方 HTTP 库。覆盖 TcpStream 连接、请求构建、响应解析、Header 处理、超时重试等底层网络知识。

### 练习 23-06: 建立 TCP 连接并发送 HTTP GET 请求

> 难度：⭐⭐⭐
> 本练习融合：TcpStream + 网络编程 + 字符串格式化 + 错误处理

使用 `std::net::TcpStream` 连接到 HTTP 服务器（如 `httpbin.org` 的 80 端口），发送一个原始的 HTTP GET 请求并读取响应。

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn http_get(host: &str, port: u16, path: &str) -> Result<String, String> {
    // TODO: 
    // 1. 使用 TcpStream::connect 连接 host:port
    // 2. 构建 HTTP GET 请求字符串
    //    GET {path} HTTP/1.0\r\n
    //    Host: {host}\r\n
    //    Connection: close\r\n
    //    \r\n
    // 3. 发送请求
    // 4. 读取响应并返回响应体字符串
    // 提示：使用 Vec<u8> 作为缓冲区读取，再转为 String
    todo!()
}

fn main() -> Result<(), String> {
    let response = http_get("httpbin.org", 80, "/get")?;
    // 只打印响应前 500 个字符
    println!("{}", &response[..response.len().min(500)]);
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn http_get(host: &str, port: u16, path: &str) -> Result<String, String> {
    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(&addr)
        .map_err(|e| format!("无法连接 {}: {}", addr, e))?;
    
    let request = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    
    stream.write_all(request.as_bytes())
        .map_err(|e| format!("发送请求失败: {}", e))?;
    
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)
        .map_err(|e| format!("读取响应失败: {}", e))?;
    
    String::from_utf8(buf).map_err(|e| format!("响应不是合法 UTF-8: {}", e))
}

fn main() -> Result<(), String> {
    let response = http_get("httpbin.org", 80, "/get")?;
    println!("{}", &response[..response.len().min(500)]);
    Ok(())
}
```

**说明：** HTTP/1.0 不需要 keep-alive 和 chunked encoding，适合学习。`Connection: close` 告诉服务器响应结束后关闭连接，这样 `read_to_end` 会在读取完响应后自然结束。核心步骤：connect → write_all → read_to_end → 转 String。
</details>

---

### 练习 23-07: 解析 HTTP 响应状态行和 Header

> 难度：⭐⭐⭐
> 本练习融合：字符串解析 + 模式匹配 + 结构体建模

将 HTTP 响应解析为结构化数据：状态码、状态消息、Headers（键值对）、响应体。

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_message: String,
    headers: HashMap<String, String>,
    body: String,
}

// TODO: 实现 parse_http_response，解析原始 HTTP 响应字符串
// 响应格式：
// HTTP/1.1 200 OK\r\n
// Content-Type: application/json\r\n
// Content-Length: 42\r\n
// \r\n
// {body content}
fn parse_http_response(raw: &str) -> Result<HttpResponse, String> {
    // 提示：使用 "\r\n" 分割行
    // 第一行是状态行 "HTTP/1.1 {code} {message}"
    // 后续行是 headers，直到空行
    // 空行之后是 body
    todo!()
}

fn main() -> Result<(), String> {
    let raw_response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 27\r\nServer: test\r\n\r\n{\"message\": \"Hello, Rust!\"}";
    
    let response = parse_http_response(raw_response)?;
    println!("状态码: {}", response.status_code);
    println!("Content-Type: {:?}", response.headers.get("Content-Type"));
    println!("响应体: {}", response.body);
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_message: String,
    headers: HashMap<String, String>,
    body: String,
}

fn parse_http_response(raw: &str) -> Result<HttpResponse, String> {
    let mut parts = raw.splitn(2, "\r\n\r\n");
    let header_section = parts.next().ok_or("缺少 Header 部分")?;
    let body = parts.next().unwrap_or("").to_string();
    
    let mut lines: Vec<&str> = header_section.split("\r\n").collect();
    if lines.is_empty() {
        return Err("空响应".into());
    }
    
    // 解析状态行: HTTP/1.1 200 OK
    let status_line = lines.remove(0);
    let mut status_parts = status_line.splitn(3, ' ');
    let _http_version = status_parts.next().ok_or("缺少 HTTP 版本")?;
    let status_code: u16 = status_parts.next()
        .ok_or("缺少状态码")?
        .parse()
        .map_err(|_| "状态码解析失败")?;
    let status_message = status_parts.next().unwrap_or("").to_string();
    
    // 解析 Headers
    let mut headers = HashMap::new();
    for line in lines {
        if let Some(idx) = line.find(':') {
            let key = line[..idx].trim().to_string();
            let value = line[idx+1..].trim().to_string();
            headers.insert(key, value);
        }
    }
    
    Ok(HttpResponse { status_code, status_message, headers, body })
}

fn main() -> Result<(), String> {
    let raw_response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 27\r\nServer: test\r\n\r\n{\"message\": \"Hello, Rust!\"}";
    
    let response = parse_http_response(raw_response)?;
    println!("状态码: {}", response.status_code);
    println!("Content-Type: {:?}", response.headers.get("Content-Type"));
    println!("响应体: {}", response.body);
    Ok(())
}
```

**说明：** `splitn(2, "\r\n\r\n")` 巧妙地将响应划分为 headers 和 body 两部分。`split(':')` 可能因 value 中包含 `:` 而出错，改用 `find(':')` 更安全。`HashMap` 用于存储键值不敏感的头字段。
</details>

---

### 练习 23-08: 改进 HTTP 客户端—支持 HTTPS（使用 rustls）

> 难度：⭐⭐⭐
> 本练习融合：外部 crate + rustls 原生 TLS + 泛型抽象

在 23-06 基础上添加 HTTPS 支持。使用 `rustls` 和 `webpki` 和 `webpki-roots`（或更简单的组合）建立 TLS 连接。本练习假设已添加依赖。

```rust
// Cargo.toml 中需要添加:
// [dependencies]
// rustls = "0.23"
// webpki-roots = "0.26"
// rustls-pki-types = "1"

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

// TODO: 实现一个通用的 fetch_url 函数
// 如果 url 以 "https://" 开头，使用 TLS 连接（443 端口）
// 如果以 "http://" 开头，使用普通 TCP 连接（80 端口）
// 返回响应体字符串

fn fetch_url(url: &str) -> Result<String, String> {
    // 1. 解析 URL，提取 host 和 path
    // 2. 判断协议
    // 3. 建立连接并发送 HTTP GET 请求
    // 4. 读取并返回响应
    
    // HTTPS 连接示例（rustls）：
    // let mut root_store = rustls::RootCertStore::empty();
    // root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    // let config = rustls::ClientConfig::builder()
    //     .with_root_certificates(root_store)
    //     .with_no_client_auth();
    // let connector = rustls::Stream::new(Arc::new(config), stream);
    
    todo!()
}

fn main() -> Result<(), String> {
    let response = fetch_url("https://httpbin.org/get")?;
    println!("{}", &response[..response.len().min(500)]);
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

fn fetch_url(url: &str) -> Result<String, String> {
    let (protocol, rest) = url.split_once("://").ok_or("无效 URL 格式")?;
    let (host, path) = match rest.split_once('/') {
        Some((h, p)) => (h, format!("/{}", p)),
        None => (rest, "/".to_string()),
    };
    
    let request = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    
    match protocol {
        "http" => {
            let mut stream = TcpStream::connect((host, 80))
                .map_err(|e| format!("连接失败: {}", e))?;
            stream.write_all(request.as_bytes())
                .map_err(|e| format!("发送失败: {}", e))?;
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf)
                .map_err(|e| format!("读取失败: {}", e))?;
            String::from_utf8(buf).map_err(|e| format!("编码错误: {}", e))
        }
        "https" => {
            let stream = TcpStream::connect((host, 443))
                .map_err(|e| format!("TLS 连接失败: {}", e))?;
            
            let mut root_store = rustls::RootCertStore::empty();
            root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            
            let config = rustls::ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();
            
            let server_name = rustls::pki_types::ServerName::try_from(host)
                .map_err(|_| "无效域名")?;
            
            let mut tls = rustls::Stream::new(
                Arc::new(config),
                stream,
                server_name,
            ).map_err(|e| format!("TLS 握手失败: {}", e))?;
            
            tls.write_all(request.as_bytes())
                .map_err(|e| format!("发送失败: {}", e))?;
            
            let mut buf = Vec::new();
            tls.read_to_end(&mut buf)
                .map_err(|e| format!("读取失败: {}", e))?;
            String::from_utf8(buf).map_err(|e| format!("编码错误: {}", e))
        }
        _ => Err(format!("不支持的协议: {}", protocol)),
    }
}

fn main() -> Result<(), String> {
    let response = fetch_url("https://httpbin.org/get")?;
    println!("{}", &response[..response.len().min(500)]);
    Ok(())
}
```

**说明：** 通过 `split_once("://")` 分离协议和主机路径。HTTPS 在 TCP 连接之上建立 TLS 加密层，使用 `rustls`（纯 Rust TLS 库）。`webpki_roots` 提供 Mozilla 根证书库用于验证服务器证书。
</details>

---

### 练习 23-09: 连接超时与重试机制

> 难度：⭐⭐⭐
> 本练习融合：Duration + TcpStream 超时设置 + 重试策略 + 错误处理最佳实践

为 HTTP 客户端添加连接超时和自动重试功能（遇到网络错误时最多重试 3 次）。

```rust
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

// TODO: 实现带有超时和重试的 http_get_with_retry 函数
// 参数：
//   host: &str — 目标主机
//   port: u16 — 端口
//   path: &str — 请求路径
//   timeout_secs: u64 — 超时秒数
//   max_retries: u32 — 最大重试次数
// 返回值：Result<String, String>

fn http_get_with_retry(
    host: &str,
    port: u16,
    path: &str,
    timeout_secs: u64,
    max_retries: u32,
) -> Result<String, String> {
    let request = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    
    let addr = format!("{}:{}", host, port);
    
    // TODO: 实现重试循环
    // 每次尝试：
    //   1. 创建 TcpStream::connect_timeout 设置超时
    //   2. 设置 stream 的读写超时
    //   3. 发送请求并读取响应
    //   4. 如果失败且剩余重试次数 > 0，等待 1 秒后重试
    //   5. 最后返回成功结果或最后一次错误
    todo!()
}

fn main() -> Result<(), String> {
    // 测试超时和重试——连接一个不存在的端口应该失败但不会 panic
    let result = http_get_with_retry("localhost", 9999, "/", 2, 3);
    match result {
        Ok(body) => println!("成功: {}", &body[..200.min(body.len())]),
        Err(e) => println!("预期中的失败（已重试 3 次）: {}", e),
    }
    
    // 真实的请求
    let result = http_get_with_retry("httpbin.org", 80, "/get", 5, 2)?;
    println!("成功响应: {}", &result[..200.min(result.len())]);
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn http_get_with_retry(
    host: &str,
    port: u16,
    path: &str,
    timeout_secs: u64,
    max_retries: u32,
) -> Result<String, String> {
    let request = format!(
        "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    let addr = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(timeout_secs);
    
    let mut last_error = String::new();
    for attempt in 0..=max_retries {
        if attempt > 0 {
            println!("重试第 {} 次...", attempt);
            std::thread::sleep(Duration::from_secs(1));
        }
        
        match TcpStream::connect_timeout(&addr.parse().map_err(|e| format!("地址无效: {}", e))?, timeout) {
            Ok(mut stream) => {
                stream.set_read_timeout(Some(timeout)).ok();
                stream.set_write_timeout(Some(timeout)).ok();
                
                if let Err(e) = stream.write_all(request.as_bytes()) {
                    last_error = format!("写入失败: {}", e);
                    continue;
                }
                
                let mut buf = Vec::new();
                match stream.read_to_end(&mut buf) {
                    Ok(_) => {
                        return String::from_utf8(buf)
                            .map_err(|e| format!("编码错误: {}", e));
                    }
                    Err(e) => {
                        last_error = format!("读取失败: {}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                last_error = format!("连接失败: {}", e);
                continue;
            }
        }
    }
    Err(format!("所有 {} 次重试均失败: {}", max_retries, last_error))
}

fn main() -> Result<(), String> {
    let result = http_get_with_retry("localhost", 9999, "/", 2, 3);
    match result {
        Ok(body) => println!("成功: {}", &body[..200.min(body.len())]),
        Err(e) => println!("预期中的失败（已重试 3 次）: {}", e),
    }
    
    let result = http_get_with_retry("httpbin.org", 80, "/get", 5, 2)?;
    println!("成功响应: {}", &result[..200.min(result.len())]);
    Ok(())
}
```

**说明：** `connect_timeout` 替代 `connect` 实现连接超时。`set_read_timeout` / `set_write_timeout` 控制 I/O 操作的超时。重试循环用 `for attempt in 0..=max_retries` 控制，失败时 `continue` 进入下一次尝试。每次重试前 `sleep(1s)` 避免频繁重试。
</details>

---

### 练习 23-10: 完整的 HTTP 客户端封装

> 难度：⭐⭐⭐
> 本练习融合：结构体 + 方法 + Option 默认值 + 建造者模式 + Trait

将前几题的功能封装为一个 `HttpClient` 结构体，支持可配置的超时、默认 Headers、User-Agent 等特性。

```rust
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
struct HttpClient {
    timeout_secs: u64,
    default_headers: HashMap<String, String>,
    user_agent: String,
}

impl HttpClient {
    // TODO: 创建一个新的 HttpClient，设置默认值
    // 默认超时 10 秒，User-Agent: "Rust-HttpClient/1.0"
    fn new() -> Self {
        todo!()
    }
    
    // TODO: 设置超时（建造者模式）
    fn with_timeout(mut self, timeout_secs: u64) -> Self {
        todo!()
    }
    
    // TODO: 添加默认 Header
    fn with_header(mut self, key: &str, value: &str) -> Self {
        todo!()
    }
    
    // TODO: 设置 User-Agent
    fn with_user_agent(mut self, ua: &str) -> Self {
        todo!()
    }
    
    // TODO: 发送 GET 请求，返回响应体字符串
    fn get(&self, url: &str) -> Result<String, String> {
        // 解析 URL，支持 http:// 和 https://
        // 使用 self 的配置（超时、默认 headers）
        todo!()
    }
}

fn main() -> Result<(), String> {
    let client = HttpClient::new()
        .with_timeout(5)
        .with_header("Accept", "application/json")
        .with_user_agent("MyBot/1.0");
    
    let response = client.get("http://httpbin.org/get")?;
    println!("{}", &response[..response.len().min(300)]);
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
struct HttpClient {
    timeout_secs: u64,
    default_headers: HashMap<String, String>,
    user_agent: String,
}

impl HttpClient {
    fn new() -> Self {
        HttpClient {
            timeout_secs: 10,
            default_headers: HashMap::new(),
            user_agent: "Rust-HttpClient/1.0".to_string(),
        }
    }
    
    fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }
    
    fn with_header(mut self, key: &str, value: &str) -> Self {
        self.default_headers.insert(key.to_string(), value.to_string());
        self
    }
    
    fn with_user_agent(mut self, ua: &str) -> Self {
        self.user_agent = ua.to_string();
        self
    }
    
    fn get(&self, url: &str) -> Result<String, String> {
        let (protocol, rest) = url.split_once("://").ok_or("无效 URL")?;
        let (host, path) = match rest.split_once('/') {
            Some((h, p)) => (h, format!("/{}", p)),
            None => (rest, "/".to_string()),
        };
        
        let port = match protocol {
            "http" => 80u16,
            "https" => 443u16,
            _ => return Err(format!("不支持的协议: {}", protocol)),
        };
        
        // 构建请求
        let mut request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n", path, host);
        request += &format!("User-Agent: {}\r\n", self.user_agent);
        for (key, value) in &self.default_headers {
            request += &format!("{}: {}\r\n", key, value);
        }
        request += "Connection: close\r\n\r\n";
        
        let timeout = Duration::from_secs(self.timeout_secs);
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect_timeout(
            &addr.parse().map_err(|e| format!("地址无效: {}", e))?,
            timeout,
        ).map_err(|e| format!("连接失败: {}", e))?;
        
        stream.set_read_timeout(Some(timeout)).ok();
        stream.write_all(request.as_bytes())
            .map_err(|e| format!("发送失败: {}", e))?;
        
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf)
            .map_err(|e| format!("读取失败: {}", e))?;
        
        String::from_utf8(buf).map_err(|e| format!("编码错误: {}", e))
    }
}

fn main() -> Result<(), String> {
    let client = HttpClient::new()
        .with_timeout(5)
        .with_header("Accept", "application/json")
        .with_user_agent("MyBot/1.0");
    
    let response = client.get("http://httpbin.org/get")?;
    println!("{}", &response[..response.len().min(300)]);
    Ok(())
}
```

**说明：** 建造者模式（Builder Pattern）通过 `with_*` 方法消费并返回 `Self`，支持链式调用。`HttpClient` 封装了所有配置项，`get` 方法利用这些配置构建 HTTP 请求。默认 headers 用 `HashMap` 存储，在构建请求时逐条写入。
</details>

---

## 项目三：日志系统设计

> 本系列 5 道题设计一个可扩展的日志系统，支持多种日志级别、输出目标（控制台/文件）以及灵活的格式化。覆盖枚举、Trait 对象、动态分发、文件 I/O 和模板方法模式。

### 练习 23-11: 日志级别枚举与 Logger Trait

> 难度：⭐⭐⭐
> 本练习融合：枚举 + 派生 trait + trait 定义 + 默认实现

定义日志级别枚举和 Log trait，以及一个基础的控制台 Logger。

```rust
use std::fmt;

// TODO: 定义 LogLevel 枚举：Debug, Info, Warn, Error
// 要求实现 Display（输出 "DEBUG", "INFO", "WARN", "ERROR"）
#[derive(Debug, Clone, PartialEq, Eq)]
enum LogLevel {
    // TODO
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        todo!()
    }
}

// TODO: 定义 Logger trait
// 包含方法：
//   fn log(&self, level: LogLevel, message: &str)
//   fn info(&self, message: &str) — 默认实现，调用 log(LogLevel::Info, message)
//   fn warn(&self, message: &str) — 默认实现
//   fn error(&self, message: &str) — 默认实现
trait Logger {
    // TODO
}

// TODO: 实现 ConsoleLogger — 将日志输出到控制台
// 格式: [级别] 消息
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    // TODO
}

fn main() {
    let logger = ConsoleLogger;
    logger.info("系统启动完成");
    logger.warn("内存使用率 85%");
    logger.error("数据库连接超时");
    // 输出:
    // [INFO] 系统启动完成
    // [WARN] 内存使用率 85%
    // [ERROR] 数据库连接超时
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

fn main() {
    let logger = ConsoleLogger;
    logger.info("系统启动完成");
    logger.warn("内存使用率 85%");
    logger.error("数据库连接超时");
}
```

**说明：** `LogLevel` 枚举实现了 `Display` trait，使 `level` 可直接用于 `println!`。`Logger trait` 为 `info` / `warn` / `error` 提供了默认实现（委托给 `log`），使用者只需实现 `log` 方法即可获得完整功能。
</details>

---

### 练习 23-12: 文件日志输出 (FileLogger)

> 难度：⭐⭐⭐
> 本练习融合：文件 I/O + BufWriter + Trait 实现 + 生命周期 + 错误处理

实现将日志写入文件的 `FileLogger`。注意文件资源的正确管理（打开、写入、刷盘、关闭）。

```rust
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
enum LogLevel {
    Debug, Info, Warn, Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

// TODO: 实现 FileLogger，将日志写入指定文件
// 使用 BufWriter 提高写入性能
// 每条日志格式："[级别] 消息\n"
// 需要 flush 确保数据落盘
struct FileLogger {
    // TODO: 需要的字段
}

impl FileLogger {
    // TODO: 创建 FileLogger，filename 是文件路径
    // 使用 OpenOptions 以追加模式打开文件，不存在则创建
    fn new(filename: &str) -> Result<Self, String> {
        todo!()
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        // TODO: 写入文件
        todo!()
    }
}

impl Drop for FileLogger {
    fn drop(&mut self) {
        // TODO: 关闭前刷盘
    }
}

fn main() -> Result<(), String> {
    let logger = FileLogger::new("app.log")?;
    logger.info("应用启动");
    logger.warn("配置未找到，使用默认值");
    logger.error("无法打开数据库连接");
    
    // 程序退出时自动调用 drop，刷盘并关闭文件
    println!("日志已写入 app.log");
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
enum LogLevel {
    Debug, Info, Warn, Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

struct FileLogger {
    writer: std::sync::Mutex<BufWriter<File>>,
}

impl FileLogger {
    fn new(filename: &str) -> Result<Self, String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .map_err(|e| format!("无法打开日志文件 {}: {}", filename, e))?;
        Ok(FileLogger {
            writer: std::sync::Mutex::new(BufWriter::new(file)),
        })
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        if let Ok(mut w) = self.writer.lock() {
            let _ = writeln!(w, "[{}] {}", level, message);
            let _ = w.flush();
        }
    }
}

fn main() -> Result<(), String> {
    let logger = FileLogger::new("app.log")?;
    logger.info("应用启动");
    logger.warn("配置未找到，使用默认值");
    logger.error("无法打开数据库连接");
    println!("日志已写入 app.log");
    Ok(())
}
```

**说明：** `FileLogger` 使用 `BufWriter` 包装 `File` 减少系统调用。`OpenOptions::new().create(true).append(true)` 确保文件不存在时创建，存在时追加。`Mutex` 用于内部可变性，使 `log` 方法可以接受 `&self` 而非 `&mut self`（满足 `Logger` trait 签名）。每次写入后 `flush` 确保日志不丢失。
</details>

---

### 练习 23-13: 日志级别过滤

> 难度：⭐⭐⭐
> 本练习融合：过滤策略 + trait 对象 + `Box<dyn Trait>` + 装饰器模式

实现一个 `LevelFilter` 包装器，只输出不低于指定级别的日志（Error > Warn > Info > Debug）。

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

// 控制台日志器（略去细节，可复用 23-11 代码）
struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

// TODO: 实现 LevelFilter — 装饰器模式
// 只输出级别 >= min_level 的日志
struct LevelFilter {
    inner: Box<dyn Logger>,
    min_level: LogLevel,
}

impl LevelFilter {
    fn new(inner: Box<dyn Logger>, min_level: LogLevel) -> Self {
        LevelFilter { inner, min_level }
    }
}

impl Logger for LevelFilter {
    fn log(&self, level: LogLevel, message: &str) {
        // TODO: 如果 level >= min_level 才传递给 inner.log()
        todo!()
    }
}

fn main() {
    // 创建控制台日志器，但只显示 Warn 及以上级别的日志
    let logger = LevelFilter::new(Box::new(ConsoleLogger), LogLevel::Warn);
    
    logger.info("这条信息不会显示");     // 被过滤
    logger.warn("这条警告会显示");
    logger.error("这条错误会显示");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

struct LevelFilter {
    inner: Box<dyn Logger>,
    min_level: LogLevel,
}

impl LevelFilter {
    fn new(inner: Box<dyn Logger>, min_level: LogLevel) -> Self {
        LevelFilter { inner, min_level }
    }
}

impl Logger for LevelFilter {
    fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            self.inner.log(level, message);
        }
    }
}

fn main() {
    let logger = LevelFilter::new(Box::new(ConsoleLogger), LogLevel::Warn);
    
    logger.info("这条信息不会显示");
    logger.warn("这条警告会显示");
    logger.error("这条错误会显示");
}
```

**说明：** `LevelFilter` 是**装饰器模式**的体现——它包裹另一个 `Logger` 并添加过滤功能。`LogLevel` 通过 `#[derive(PartialOrd, Ord)]` 使级别的比较变得简单（`Error > Warn > Info > Debug` 与枚举定义顺序一致）。`Box<dyn Logger>` 实现 trait 对象，支持动态分发。
</details>

---

### 练习 23-14: 多输出日志器 (MultiLogger)

> 难度：⭐⭐⭐
> 本练习融合：Vec<Box<dyn Trait>> + 迭代器 + trait 对象 + 组合模式

实现 `MultiLogger` 同时将日志输出到多个目标（例如控制台 + 文件）。

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug = 0, Info = 1, Warn = 2, Error = 3,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

// TODO: 实现 MultiLogger，内部持有多个 Box<dyn Logger>
// log 方法遍历所有 logger 并调用它们的 log
struct MultiLogger {
    loggers: Vec<Box<dyn Logger>>,
}

impl MultiLogger {
    // TODO: 创建空的 MultiLogger
    fn new() -> Self {
        todo!()
    }
    
    // TODO: 添加一个 logger
    fn add(&mut self, logger: Box<dyn Logger>) {
        todo!()
    }
}

impl Logger for MultiLogger {
    fn log(&self, level: LogLevel, message: &str) {
        // TODO: 遍历所有 loggers 并调用 log
        todo!()
    }
}

fn main() {
    // 创建一个同时输出到控制台和文件的日志系统
    let mut multi = MultiLogger::new();
    multi.add(Box::new(ConsoleLogger));
    // 假设已有 FileLogger，这里演示只用控制台
    multi.add(Box::new(ConsoleLogger)); // 故意加两个控制台，模拟多输出
    
    multi.info("同时输出到多个目标");
    multi.error("所有 logger 都会收到这条消息");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug = 0, Info = 1, Warn = 2, Error = 3,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

struct MultiLogger {
    loggers: Vec<Box<dyn Logger>>,
}

impl MultiLogger {
    fn new() -> Self {
        MultiLogger { loggers: Vec::new() }
    }
    
    fn add(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }
}

impl Logger for MultiLogger {
    fn log(&self, level: LogLevel, message: &str) {
        for logger in &self.loggers {
            logger.log(level, message);
        }
    }
}

fn main() {
    let mut multi = MultiLogger::new();
    multi.add(Box::new(ConsoleLogger));
    multi.add(Box::new(ConsoleLogger));
    
    multi.info("同时输出到多个目标");
    multi.error("所有 logger 都会收到这条消息");
}
```

**说明：** `MultiLogger` 是**组合模式**的体现，它将多个 `Logger` 组合成一个统一的接口。`Vec<Box<dyn Logger>>` 可以持有不同类型的日志器（ConsoleLogger、FileLogger、LevelFilter 等），得益于 trait 对象的动态分发。遍历调用实现了"广播"语义。
</details>

---

### 练习 23-15: 带时间戳的格式化日志

> 难度：⭐⭐⭐
> 本练习融合：格式化 + 装饰器 + 系统时间 + 自定义 Display

实现一个 `TimestampDecorator`，在每条日志前添加时间戳（格式：`[2024-01-15 10:30:45]`）。

```rust
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug = 0, Info = 1, Warn = 2, Error = 3,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

// TODO: 实现 TimestampDecorator
// 格式: [2024-01-15 10:30:45] [级别] 消息
// 提示：SystemTime::now().duration_since(UNIX_EPOCH) 获取时间戳
// 用 chrono crate 更方便，但这里手动计算时间

struct TimestampDecorator {
    inner: Box<dyn Logger>,
}

impl TimestampDecorator {
    fn new(inner: Box<dyn Logger>) -> Self {
        TimestampDecorator { inner }
    }
    
    // TODO: 辅助函数，生成形如 "2024-01-15 10:30:45" 的时间字符串
    fn current_timestamp() -> String {
        // 从 SystemTime 计算年月日时分秒
        todo!()
    }
}

impl Logger for TimestampDecorator {
    fn log(&self, level: LogLevel, message: &str) {
        // TODO: 在消息前添加时间戳
        todo!()
    }
}

fn main() {
    let logger = TimestampDecorator::new(Box::new(ConsoleLogger));
    logger.info("系统启动完成");
    logger.error("发生致命错误");
    // 输出示例:
    // [2024-01-15 10:30:45] [INFO] 系统启动完成
    // [2024-01-15 10:30:45] [ERROR] 发生致命错误
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug = 0, Info = 1, Warn = 2, Error = 3,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, message: &str) {
        println!("[{}] {}", level, message);
    }
}

struct TimestampDecorator {
    inner: Box<dyn Logger>,
}

impl TimestampDecorator {
    fn new(inner: Box<dyn Logger>) -> Self {
        TimestampDecorator { inner }
    }
    
    fn current_timestamp() -> String {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let secs = dur.as_secs();
        
        // 粗略计算（忽略闰秒，不考虑时区，简化为 UTC+8）
        let days = secs / 86400;
        let time_secs = secs % 86400;
        let hours = (time_secs / 3600 + 8) % 24; // UTC+8
        let minutes = (time_secs % 3600) / 60;
        let seconds = time_secs % 60;
        
        // 从 1970-01-01 计算年月日（简化算法）
        let mut y = 1970i64;
        let mut remaining_days = days as i64;
        loop {
            let days_in_year = if is_leap(y) { 366 } else { 365 };
            if remaining_days < days_in_year { break; }
            remaining_days -= days_in_year;
            y += 1;
        }
        let month_days = if is_leap(y) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        let mut m = 1usize;
        for &md in month_days.iter() {
            if remaining_days < md { break; }
            remaining_days -= md;
            m += 1;
        }
        let d = remaining_days + 1;
        
        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, hours, minutes, seconds)
    }
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

impl Logger for TimestampDecorator {
    fn log(&self, level: LogLevel, message: &str) {
        let ts = Self::current_timestamp();
        self.inner.log(level, &format!("[{}] {}", ts, message));
    }
}

fn main() {
    let logger = TimestampDecorator::new(Box::new(ConsoleLogger));
    logger.info("系统启动完成");
    logger.error("发生致命错误");
}
```

**说明：** `TimestampDecorator` 是装饰器模式的再次应用。时间计算从 `SystemTime` 获取 Unix 时间戳（秒），手动推算年月日（处理闰年）。实际项目中应该使用 `chrono` crate，这里手动实现是为了加深对时间处理的理解。注意 UTC+8 时区的偏移处理。
</details>

---

## 项目四：配置管理器

> 本系列 5 道题构建一个多层配置管理器，支持从环境变量、配置文件、命令行参数和默认值中读取配置，按优先级合并。覆盖 HashMap 操作、文件解析、环境变量读取、serde 序列化、默认值合并。

### 练习 23-16: 配置存储与默认值

> 难度：⭐⭐⭐
> 本练习融合：HashMap + 结构体 + 默认值 + Builder 模式

设计 `Config` 结构体存储键值对配置，支持设置默认值。

```rust
use std::collections::HashMap;

// TODO: 定义 Config 结构体，内部使用 HashMap<String, String> 存储配置项
struct Config {
    // TODO
}

impl Config {
    // 创建一个空的配置
    fn new() -> Self {
        todo!()
    }
    
    // TODO: 从已有 HashMap 创建 Config
    fn from_map(map: HashMap<String, String>) -> Self {
        todo!()
    }
    
    // TODO: 获取配置值，返回 Option<&str>
    fn get(&self, key: &str) -> Option<&str> {
        todo!()
    }
    
    // TODO: 设置配置值
    fn set(&mut self, key: &str, value: &str) {
        todo!()
    }
    
    // TODO: 合并另一个 Config 到当前（相同 key 用新值覆盖旧值）
    fn merge(&mut self, other: Config) {
        todo!()
    }
    
    // TODO: 获取所有配置项数量
    fn len(&self) -> usize {
        todo!()
    }
    
    // TODO: 检查是否包含某个 key
    fn contains(&self, key: &str) -> bool {
        todo!()
    }
    
    // 获取所有 key
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

fn main() {
    let mut config = Config::new();
    config.set("host", "localhost");
    config.set("port", "8080");
    
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("port"), Some("8080"));
    assert_eq!(config.get("missing"), None);
    
    // 合并默认配置
    let mut defaults = Config::new();
    defaults.set("host", "127.0.0.1");
    defaults.set("timeout", "30");
    
    config.merge(defaults);
    assert_eq!(config.get("host"), Some("localhost")); // 未覆盖
    assert_eq!(config.get("timeout"), Some("30"));     // 新增
    
    println!("配置管理器基础功能测试通过！共 {} 项配置", config.len());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self {
        Config(HashMap::new())
    }
    
    fn from_map(map: HashMap<String, String>) -> Self {
        Config(map)
    }
    
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
    
    fn len(&self) -> usize {
        self.0.len()
    }
    
    fn contains(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
    
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

fn main() {
    let mut config = Config::new();
    config.set("host", "localhost");
    config.set("port", "8080");
    
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("port"), Some("8080"));
    assert_eq!(config.get("missing"), None);
    
    let mut defaults = Config::new();
    defaults.set("host", "127.0.0.1");
    defaults.set("timeout", "30");
    
    config.merge(defaults);
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("timeout"), Some("30"));
    
    println!("配置管理器基础功能测试通过！共 {} 项配置", config.len());
}
```

**说明：** `Config` 是 `HashMap<String, String>` 的简单封装。`merge` 使用 `entry().or_insert()` 确保已有值不被覆盖（源配置优先级更高），这是"默认值 + 覆盖"的典型实现。
</details>

---

### 练习 23-17: 从环境变量读取配置

> 难度：⭐⭐⭐
> 本练习融合：环境变量 + 命名约定 + 字符串转换 + HashMap

从环境变量读取配置，约定环境变量以 `APP_` 为前缀，如 `APP_HOST=localhost` 对应配置项 `host`。

```rust
use std::collections::HashMap;
use std::env;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    
    fn from_map(map: HashMap<String, String>) -> Self { Config(map) }
    
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
}

// TODO: 实现从环境变量加载配置
// 1. 遍历所有环境变量
// 2. 过滤出 APP_ 前缀的变量
// 3. 去除前缀并将剩余部分转小写作为 key
//    例如: APP_DB_HOST -> db_host
// 4. 返回 Config
fn load_from_env() -> Config {
    todo!()
}

// 辅助函数：将分隔符风格转换
// 环境变量中使用大写 + 下划线，配置项中使用小写 + 下划线
// 将 "DB_HOST" 转为 "db_host"
fn normalize_key(key: &str) -> String {
    key.to_lowercase()
}

fn main() {
    // 设置测试环境变量
    env::set_var("APP_HOST", "production.example.com");
    env::set_var("APP_PORT", "5432");
    env::set_var("APP_DB_NAME", "myapp");
    env::set_var("UNRELATED", "should_not_appear");
    
    let config = load_from_env();
    
    assert_eq!(config.get("host"), Some("production.example.com"));
    assert_eq!(config.get("port"), Some("5432"));
    assert_eq!(config.get("db_name"), Some("myapp"));
    assert_eq!(config.get("unrelated"), None); // 无 APP_ 前缀
    
    println!("环境变量加载测试通过！共 {} 项配置", config.len());
    
    // 清理
    env::remove_var("APP_HOST");
    env::remove_var("APP_PORT");
    env::remove_var("APP_DB_NAME");
    env::remove_var("UNRELATED");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;
use std::env;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn from_map(map: HashMap<String, String>) -> Self { Config(map) }
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
}

fn load_from_env() -> Config {
    let mut config = Config::new();
    let prefix = "APP_";
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix(prefix) {
            let normalized = normalize_key(rest);
            config.set(&normalized, &value);
        }
    }
    config
}

fn normalize_key(key: &str) -> String {
    key.to_lowercase()
}

fn main() {
    env::set_var("APP_HOST", "production.example.com");
    env::set_var("APP_PORT", "5432");
    env::set_var("APP_DB_NAME", "myapp");
    env::set_var("UNRELATED", "should_not_appear");
    
    let config = load_from_env();
    
    assert_eq!(config.get("host"), Some("production.example.com"));
    assert_eq!(config.get("port"), Some("5432"));
    assert_eq!(config.get("db_name"), Some("myapp"));
    assert_eq!(config.get("unrelated"), None);
    
    println!("环境变量加载测试通过！共 {} 项配置", config.len());
    
    env::remove_var("APP_HOST");
    env::remove_var("APP_PORT");
    env::remove_var("APP_DB_NAME");
    env::remove_var("UNRELATED");
}
```

**说明：** `env::vars()` 返回所有环境变量的迭代器。`strip_prefix` 过滤出 `APP_` 前缀的变量。`normalize_key` 将大写转小写。这种"前缀 + 转换"的约定在 12-Factor App 中广泛使用（如 Kubernetes 的 ConfigMap 转环境变量）。
</details>

---

### 练习 23-18: 从配置文件读取（简化版 .env 格式）

> 难度：⭐⭐⭐
> 本练习融合：文件读取 + 行解析 + 字符串分割 + trim + 过滤注释

解析 `.env` 格式的配置文件（每行 `KEY=VALUE`，`#` 开头为注释，忽略空行）。

```rust
use std::collections::HashMap;
use std::fs;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn from_map(map: HashMap<String, String>) -> Self { Config(map) }
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
}

// TODO: 从 .env 格式文件加载配置
// 规则：
//   1. 空行跳过
//   2. # 开头的行为注释，跳过
//   3. 每行格式 KEY=VALUE，前后空白被去除
//   4. VALUE 可以包含 = 号（只用第一个 = 分隔）
//   5. VALUE 可以用双引号包裹（去除引号）
fn load_from_file(filename: &str) -> Result<Config, String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("无法读取文件 {}: {}", filename, e))?;
    
    let mut config = Config::new();
    
    for line in content.lines() {
        let line = line.trim();
        // TODO: 跳过空行和注释
        
        // TODO: 按第一个 = 分割 KEY 和 VALUE
        
        // TODO: VALUE 去除首尾双引号
        
        // TODO: 设置配置项
    }
    
    Ok(config)
}

fn main() -> Result<(), String> {
    // 创建测试配置文件
    let test_content = r#"
# 数据库配置
DB_HOST=localhost
DB_PORT=5432

# 应用配置
APP_NAME="My Rust App"
APP_DEBUG=true
GREETING=Hello=World
"#;
    std::fs::write("test.env", test_content)
        .map_err(|e| format!("无法写入测试文件: {}", e))?;
    
    let config = load_from_file("test.env")?;
    
    assert_eq!(config.get("db_host"), Some("localhost"));
    assert_eq!(config.get("db_port"), Some("5432"));
    assert_eq!(config.get("app_name"), Some("My Rust App"));
    assert_eq!(config.get("app_debug"), Some("true"));
    assert_eq!(config.get("greeting"), Some("Hello=World")); // 包含 = 的值
    
    println!("配置文件解析测试通过！共 {} 项配置", config.len());
    
    std::fs::remove_file("test.env").ok();
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;
use std::fs;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn from_map(map: HashMap<String, String>) -> Self { Config(map) }
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
}

fn load_from_file(filename: &str) -> Result<Config, String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("无法读取文件 {}: {}", filename, e))?;
    
    let mut config = Config::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim().to_lowercase();
            let mut value = line[idx+1..].trim().to_string();
            // 去除首尾双引号
            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                value = value[1..value.len()-1].to_string();
            }
            config.set(&key, &value);
        }
    }
    
    Ok(config)
}

fn main() -> Result<(), String> {
    let test_content = r#"
# 数据库配置
DB_HOST=localhost
DB_PORT=5432

# 应用配置
APP_NAME="My Rust App"
APP_DEBUG=true
GREETING=Hello=World
"#;
    std::fs::write("test.env", test_content)
        .map_err(|e| format!("无法写入测试文件: {}", e))?;
    
    let config = load_from_file("test.env")?;
    
    assert_eq!(config.get("db_host"), Some("localhost"));
    assert_eq!(config.get("db_port"), Some("5432"));
    assert_eq!(config.get("app_name"), Some("My Rust App"));
    assert_eq!(config.get("app_debug"), Some("true"));
    assert_eq!(config.get("greeting"), Some("Hello=World"))?;
    
    println!("配置文件解析测试通过！共 {} 项配置", config.len());
    
    std::fs::remove_file("test.env").ok();
    Ok(())
}
```

**说明：** 用 `line.find('=')` 而非 `splitn(2, '=')` 同样只分割第一个 `=`，但当 value 包含 `=` 时两种方式都能正确处理，这里保留 `find` 方式。key 转为小写保持一致性。双引号仅去除最外层一对，不做转义处理（简化）。
</details>

---

### 练习 23-19: 配置优先级合并

> 难度：⭐⭐⭐
> 本练习融合：优先级策略 + 链式调用 + 多层覆盖

实现配置的优先级合并：**命令行参数（最高）> 环境变量 > 配置文件 > 默认值**。

```rust
use std::collections::HashMap;
use std::env;
use std::fs;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn from_map(map: HashMap<String, String>) -> Self { Config(map) }
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
    fn merge_override(&mut self, other: Config) {
        // 与 merge 相反——新值覆盖旧值
        for (k, v) in other.0 {
            self.0.insert(k, v);
        }
    }
}

// 辅助：从环境变量加载（同 23-17）
fn load_from_env() -> Config {
    let mut config = Config::new();
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("APP_") {
            config.set(&rest.to_lowercase(), &value);
        }
    }
    config
}

// 辅助：从文件加载（同 23-18）
fn load_from_file(filename: &str) -> Result<Config, String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("读取失败: {}", e))?;
    let mut config = Config::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim().to_lowercase();
            let mut value = line[idx+1..].trim().to_string();
            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                value = value[1..value.len()-1].to_string();
            }
            config.set(&key, &value);
        }
    }
    Ok(config)
}

// TODO: 实现 build_config 函数
// 优先级（从低到高）：默认值 -> 配置文件 -> 环境变量 -> 命令行参数（--key=value 格式）
// 先加载低优先级，再 merge_override 高优先级
fn build_config(
    defaults: Config,
    config_file: Option<&str>,
    cli_args: &[String],
) -> Config {
    // 1. 从默认值开始
    // 2. 如果有配置文件，从文件加载并 merge（文件优先级高于默认值）
    // 3. 从环境变量加载并 merge（环境变量优先级高于文件）
    // 4. 解析命令行参数（--key=value 格式）并 merge_override（最高优先级）
    todo!()
}

fn main() -> Result<(), String> {
    // 设置测试环境变量
    env::set_var("APP_PORT", "9000");
    env::set_var("APP_DEBUG", "false");
    
    // 创建测试配置文件
    std::fs::write("test_config.env", "HOST=from_file\nPORT=8080\nTIMEOUT=60")
        .map_err(|e| format!("写入失败: {}", e))?;
    
    // 默认值
    let mut defaults = Config::new();
    defaults.set("host", "127.0.0.1");
    defaults.set("port", "3000");
    defaults.set("debug", "true");
    defaults.set("timeout", "30");
    
    // 模拟命令行参数：--host=cli_host --debug=true
    let cli_args = vec!["--host=cli_host".to_string(), "--debug=true".to_string()];
    
    let config = build_config(defaults, Some("test_config.env"), &cli_args);
    
    assert_eq!(config.get("host"), Some("cli_host"));    // 命令行最高
    assert_eq!(config.get("port"), Some("9000"));         // 环境变量覆盖文件
    assert_eq!(config.get("debug"), Some("true"));        // 命令行覆盖环境变量
    assert_eq!(config.get("timeout"), Some("60"));        // 来自文件（默认值被覆盖）
    
    println!("配置优先级合并测试通过！");
    
    env::remove_var("APP_PORT");
    env::remove_var("APP_DEBUG");
    std::fs::remove_file("test_config.env").ok();
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;
use std::env;
use std::fs;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn from_map(map: HashMap<String, String>) -> Self { Config(map) }
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    fn merge(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.entry(k).or_insert(v);
        }
    }
    fn merge_override(&mut self, other: Config) {
        for (k, v) in other.0 {
            self.0.insert(k, v);
        }
    }
}

fn load_from_env() -> Config {
    let mut config = Config::new();
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("APP_") {
            config.set(&rest.to_lowercase(), &value);
        }
    }
    config
}

fn load_from_file(filename: &str) -> Result<Config, String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("读取失败: {}", e))?;
    let mut config = Config::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some(idx) = line.find('=') {
            let key = line[..idx].trim().to_lowercase();
            let mut value = line[idx+1..].trim().to_string();
            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                value = value[1..value.len()-1].to_string();
            }
            config.set(&key, &value);
        }
    }
    Ok(config)
}

fn build_config(
    defaults: Config,
    config_file: Option<&str>,
    cli_args: &[String],
) -> Config {
    let mut config = defaults;
    
    // 配置文件（第二优先级）
    if let Some(filename) = config_file {
        if let Ok(file_config) = load_from_file(filename) {
            config.merge(file_config);
        }
    }
    
    // 环境变量（第三优先级）
    let env_config = load_from_env();
    config.merge(env_config);
    
    // 命令行参数（最高优先级）
    let mut cli_config = Config::new();
    for arg in cli_args {
        if let Some(rest) = arg.strip_prefix("--") {
            if let Some(idx) = rest.find('=') {
                let key = rest[..idx].to_string();
                let value = rest[idx+1..].to_string();
                cli_config.set(&key, &value);
            }
        }
    }
    config.merge_override(cli_config);
    
    config
}

fn main() -> Result<(), String> {
    env::set_var("APP_PORT", "9000");
    env::set_var("APP_DEBUG", "false");
    
    std::fs::write("test_config.env", "HOST=from_file\nPORT=8080\nTIMEOUT=60")
        .map_err(|e| format!("写入失败: {}", e))?;
    
    let mut defaults = Config::new();
    defaults.set("host", "127.0.0.1");
    defaults.set("port", "3000");
    defaults.set("debug", "true");
    defaults.set("timeout", "30");
    
    let cli_args = vec!["--host=cli_host".to_string(), "--debug=true".to_string()];
    
    let config = build_config(defaults, Some("test_config.env"), &cli_args);
    
    assert_eq!(config.get("host"), Some("cli_host"));
    assert_eq!(config.get("port"), Some("9000"));
    assert_eq!(config.get("debug"), Some("true"));
    assert_eq!(config.get("timeout"), Some("60"));
    
    println!("配置优先级合并测试通过！");
    
    env::remove_var("APP_PORT");
    env::remove_var("APP_DEBUG");
    std::fs::remove_file("test_config.env").ok();
    Ok(())
}
```

**说明：** 优先级策略通过 `merge`（保留原值）和 `merge_override`（覆盖原值）的组合实现。加载顺序：默认值 → 文件 `merge` → 环境变量 `merge` → 命令行 `merge_override`。这样后加载的高优先级源可以覆盖低优先级源的配置。
</details>

---

### 练习 23-20: 类型安全的配置访问

> 难度：⭐⭐⭐
> 本练习融合：泛型 + FromStr trait + 类型转换 + 自定义错误

为 `Config` 添加类型安全的泛型获取方法，支持将字符串值自动转换为 `u16`、`bool` 等目标类型。

```rust
use std::collections::HashMap;
use std::str::FromStr;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    
    // 获取原始字符串值
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    
    // TODO: 实现泛型方法 get_as<T: FromStr>
    // 从 HashMap 获取值并尝试解析为目标类型 T
    // 返回值: Result<Option<T>, String>
    //  - Ok(Some(val)) — 成功解析
    //  - Ok(None) — key 不存在
    //  - Err(msg) — 存在但解析失败
    fn get_as<T: FromStr>(&self, key: &str) -> Result<Option<T>, String> {
        todo!()
    }
    
    // TODO: 实现 get_as_or — 获取并解析，如果不存在或解析失败则返回默认值
    fn get_as_or<T: FromStr>(&self, key: &str, default: T) -> T {
        todo!()
    }
}

fn main() {
    let mut config = Config::new();
    config.set("port", "8080");
    config.set("debug", "true");
    config.set("pi", "3.14");
    
    // 类型安全的读取
    let port: u16 = config.get_as::<u16>("port")
        .unwrap()  // Result<Option<u16>, String>
        .expect("port 必须配置");
    assert_eq!(port, 8080);
    
    let debug: bool = config.get_as::<bool>("debug")
        .unwrap()
        .unwrap_or(false);
    assert!(debug);
    
    let pi: f64 = config.get_as::<f64>("pi")
        .unwrap()
        .unwrap();
    assert!((pi - 3.14).abs() < 0.001);
    
    // 不存在的 key
    let missing: Option<u16> = config.get_as::<u16>("missing").unwrap();
    assert_eq!(missing, None);
    
    // 使用默认值
    let timeout: u16 = config.get_as_or("timeout", 30);
    assert_eq!(timeout, 30);
    
    println!("类型安全配置访问测试通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;
use std::str::FromStr;

struct Config(HashMap<String, String>);

impl Config {
    fn new() -> Self { Config(HashMap::new()) }
    fn set(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
    
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
    
    fn get_as<T: FromStr>(&self, key: &str) -> Result<Option<T>, String> {
        match self.get(key) {
            None => Ok(None),
            Some(value) => {
                value.parse::<T>()
                    .map(Some)
                    .map_err(|_| format!("无法将 '{}' 解析为目标类型", value))
            }
        }
    }
    
    fn get_as_or<T: FromStr>(&self, key: &str, default: T) -> T {
        self.get_as::<T>(key)
            .ok()
            .flatten()
            .unwrap_or(default)
    }
}

fn main() {
    let mut config = Config::new();
    config.set("port", "8080");
    config.set("debug", "true");
    config.set("pi", "3.14");
    
    let port: u16 = config.get_as::<u16>("port")
        .unwrap()
        .expect("port 必须配置");
    assert_eq!(port, 8080);
    
    let debug: bool = config.get_as::<bool>("debug")
        .unwrap()
        .unwrap_or(false);
    assert!(debug);
    
    let pi: f64 = config.get_as::<f64>("pi")
        .unwrap()
        .unwrap();
    assert!((pi - 3.14).abs() < 0.001);
    
    let missing: Option<u16> = config.get_as::<u16>("missing").unwrap();
    assert_eq!(missing, None);
    
    let timeout: u16 = config.get_as_or("timeout", 30);
    assert_eq!(timeout, 30);
    
    println!("类型安全配置访问测试通过！");
}
```

**说明：** `get_as<T: FromStr>` 利用 `FromStr` trait 实现泛型类型转换。标准库中所有常见类型（`u16`、`f64`、`bool`、`String` 等）都实现了 `FromStr`。`Result<Option<T>, String>` 的嵌套在语义上区分了"key 不存在"和"值无法解析"两种不同的错误场景。
</details>

---

## 项目五：完整 CLI 应用

> 本系列 5 道题构建一个完整的命令行应用，结合手动参数解析、文件操作、错误处理、配置读取和用户交互。最终成品是一个简易的"文件搜索与统计"工具。

### 练习 23-21: 命令行参数解析

> 难度：⭐⭐⭐
> 本练习融合：env::args + 模式匹配 + 错误处理 + Vec 操作

手动解析命令行参数，支持子命令模式：`search`（搜索文件）和 `stats`（统计信息）。

```rust
use std::env;

// TODO: 定义 Command 枚举，表示两种子命令
// Search { pattern: String, path: String }
// Stats { path: String }
enum Command {
    // TODO
}

// TODO: 实现 parse_args 函数，从 env::args() 解析出 Command
// 用法：
//   app search <pattern> <path>
//   app stats <path>
// 如果参数不足或无法识别，返回错误信息
fn parse_args(args: &[String]) -> Result<Command, String> {
    todo!()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args)?;
    
    match command {
        Command::Search { pattern, path } => {
            println!("搜索模式 '{}' 在路径 '{}' 中", pattern, path);
            // 后续实现搜索逻辑
        }
        Command::Stats { path } => {
            println!("统计路径 '{}' 的信息", path);
            // 后续实现统计逻辑
        }
    }
    
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::env;

enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("用法: app <子命令> [参数...]\n子命令: search <模式> <路径>, stats <路径>".into());
    }
    
    let subcommand = args[1].as_str();
    match subcommand {
        "search" => {
            if args.len() < 4 {
                return Err("用法: app search <模式> <路径>".into());
            }
            Ok(Command::Search {
                pattern: args[2].clone(),
                path: args[3].clone(),
            })
        }
        "stats" => {
            if args.len() < 3 {
                return Err("用法: app stats <路径>".into());
            }
            Ok(Command::Stats {
                path: args[2].clone(),
            })
        }
        _ => Err(format!("未知子命令: '{}'\n支持: search, stats", subcommand)),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args)?;
    
    match command {
        Command::Search { pattern, path } => {
            println!("搜索模式 '{}' 在路径 '{}' 中", pattern, path);
        }
        Command::Stats { path } => {
            println!("统计路径 '{}' 的信息", path);
        }
    }
    
    Ok(())
}
```

**说明：** 使用枚举 `Command` 表示不同子命令及其参数。`parse_args` 根据位置参数分派：`args[1]` 是子命令名，后续元素是参数。错误消息提示用户正确的用法格式。这种方式虽然手动，但完全可控，不依赖外部 crate。
</details>

---

### 练习 23-22: 文件搜索实现

> 难度：⭐⭐⭐
> 本练习融合：目录递归 + WalkDir 替代（标准库 fs）+ 模式匹配 + 字符串包含

实现 `search` 子命令——在指定目录中递归搜索文件名包含指定模式的文件。

```rust
use std::env;
use std::fs;
use std::path::Path;

enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("用法: app <子命令> [参数...]".into());
    }
    match args[1].as_str() {
        "search" => {
            if args.len() < 4 {
                return Err("用法: app search <模式> <路径>".into());
            }
            Ok(Command::Search {
                pattern: args[2].clone(),
                path: args[3].clone(),
            })
        }
        "stats" => {
            if args.len() < 3 {
                return Err("用法: app stats <路径>".into());
            }
            Ok(Command::Stats { path: args[2].clone() })
        }
        _ => Err(format!("未知子命令: '{}'", args[1])),
    }
}

// TODO: 实现 search_files 函数
// 递归遍历 path 目录，找出文件名包含 pattern 的所有文件
// 打印每个匹配文件的完整路径
// 返回匹配的文件数量
fn search_files(path: &str, pattern: &str) -> Result<usize, String> {
    // 提示：使用 fs::read_dir 递归遍历
    // 对每个条目检查是否是文件，文件名是否包含 pattern
    todo!()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args)?;
    
    match command {
        Command::Search { pattern, path } => {
            let count = search_files(&path, &pattern)?;
            println!("搜索完成，找到 {} 个匹配文件", count);
        }
        Command::Stats { path } => {
            println!("统计路径 '{}' 的信息（待实现）", path);
        }
    }
    
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::env;
use std::fs;
use std::path::Path;

enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("用法: app <子命令> [参数...]".into());
    }
    match args[1].as_str() {
        "search" => {
            if args.len() < 4 {
                return Err("用法: app search <模式> <路径>".into());
            }
            Ok(Command::Search {
                pattern: args[2].clone(),
                path: args[3].clone(),
            })
        }
        "stats" => {
            if args.len() < 3 {
                return Err("用法: app stats <路径>".into());
            }
            Ok(Command::Stats { path: args[2].clone() })
        }
        _ => Err(format!("未知子命令: '{}'", args[1])),
    }
}

fn search_files(path: &str, pattern: &str) -> Result<usize, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("路径不存在: {}", path.display()));
    }
    
    let mut count = 0;
    
    if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                // 递归子目录
                count += search_files(&entry_path.to_string_lossy(), pattern)?;
            } else if let Some(filename) = entry_path.file_name() {
                let filename = filename.to_string_lossy();
                if filename.contains(pattern) {
                    println!("{}", entry_path.display());
                    count += 1;
                }
            }
        }
    } else if let Some(filename) = path.file_name() {
        if filename.to_string_lossy().contains(pattern) {
            println!("{}", path.display());
            count = 1;
        }
    }
    
    Ok(count)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args)?;
    
    match command {
        Command::Search { pattern, path } => {
            let count = search_files(&path, &pattern)?;
            println!("搜索完成，找到 {} 个匹配文件", count);
        }
        Command::Stats { path } => {
            println!("统计路径 '{}' 的信息（待实现）", path);
        }
    }
    
    Ok(())
}
```

**说明：** 递归遍历用 `fs::read_dir` 实现，遇到子目录递归调用自身。`Path::file_name` 获取文件名部分，`to_string_lossy` 处理非 UTF-8 文件名。匹配使用 `contains`（简单子串匹配），也可扩展为正则表达式。
</details>

---

### 练习 23-23: 文件统计信息

> 难度：⭐⭐⭐
> 本练习融合：元组结构体 + 文件元数据 + 单位转换 + 格式化输出

实现 `stats` 子命令——统计指定路径的信息：总文件数、总大小、最大/最小文件。

```rust
use std::env;
use std::fs;
use std::path::Path;

enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("用法: app <子命令> [参数...]".into());
    }
    match args[1].as_str() {
        "search" => {
            if args.len() < 4 { return Err("用法: app search <模式> <路径>".into()); }
            Ok(Command::Search { pattern: args[2].clone(), path: args[3].clone() })
        }
        "stats" => {
            if args.len() < 3 { return Err("用法: app stats <路径>".into()); }
            Ok(Command::Stats { path: args[2].clone() })
        }
        _ => Err(format!("未知子命令: '{}'", args[1])),
    }
}

// TODO: 定义 FileStats 结构体
struct FileStats {
    total_files: usize,
    total_size: u64,
    largest_file: Option<(String, u64)>,
    smallest_file: Option<(String, u64)>,
}

// TODO: 实现 collect_stats 函数
// 递归统计指定路径下的文件信息
fn collect_stats(path: &str) -> Result<FileStats, String> {
    todo!()
}

// 辅助函数：人性化显示文件大小
fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.2} {}", size, UNITS[unit_idx])
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args)?;
    
    match command {
        Command::Search { pattern, path } => {
            println!("搜索功能（略）");
        }
        Command::Stats { path } => {
            let stats = collect_stats(&path)?;
            println!("===== 文件统计 =====");
            println!("路径: {}", path);
            println!("文件总数: {}", stats.total_files);
            println!("总大小: {}", human_readable_size(stats.total_size));
            if let Some((name, size)) = &stats.largest_file {
                println!("最大文件: {} ({})", name, human_readable_size(*size));
            }
            if let Some((name, size)) = &stats.smallest_file {
                println!("最小文件: {} ({})", name, human_readable_size(*size));
            }
        }
    }
    
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::env;
use std::fs;
use std::path::Path;

enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("用法: app <子命令> [参数...]".into());
    }
    match args[1].as_str() {
        "search" => {
            if args.len() < 4 { return Err("用法: app search <模式> <路径>".into()); }
            Ok(Command::Search { pattern: args[2].clone(), path: args[3].clone() })
        }
        "stats" => {
            if args.len() < 3 { return Err("用法: app stats <路径>".into()); }
            Ok(Command::Stats { path: args[2].clone() })
        }
        _ => Err(format!("未知子命令: '{}'", args[1])),
    }
}

struct FileStats {
    total_files: usize,
    total_size: u64,
    largest_file: Option<(String, u64)>,
    smallest_file: Option<(String, u64)>,
}

fn collect_stats(path: &str) -> Result<FileStats, String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("路径不存在: {}", path.display()));
    }
    
    let mut stats = FileStats {
        total_files: 0,
        total_size: 0,
        largest_file: None,
        smallest_file: None,
    };
    
    if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                let sub_stats = collect_stats(&entry_path.to_string_lossy())?;
                stats.total_files += sub_stats.total_files;
                stats.total_size += sub_stats.total_size;
                // 合并最大/最小文件
                if let Some((name, size)) = sub_stats.largest_file {
                    let should_replace = match &stats.largest_file {
                        Some((_, current_max)) => size > *current_max,
                        None => true,
                    };
                    if should_replace { stats.largest_file = Some((name, size)); }
                }
                if let Some((name, size)) = sub_stats.smallest_file {
                    let should_replace = match &stats.smallest_file {
                        Some((_, current_min)) => size < *current_min,
                        None => true,
                    };
                    if should_replace { stats.smallest_file = Some((name, size)); }
                }
            } else if entry_path.is_file() {
                let metadata = fs::metadata(&entry_path)
                    .map_err(|e| format!("读取元数据失败: {}", e))?;
                let size = metadata.len();
                stats.total_files += 1;
                stats.total_size += size;
                
                let filename = entry_path.to_string_lossy().to_string();
                match &stats.largest_file {
                    Some((_, max_size)) if size > *max_size => {
                        stats.largest_file = Some((filename.clone(), size));
                    }
                    None => { stats.largest_file = Some((filename.clone(), size)); }
                    _ => {}
                }
                match &stats.smallest_file {
                    Some((_, min_size)) if size < *min_size => {
                        stats.smallest_file = Some((filename.clone(), size));
                    }
                    None => { stats.smallest_file = Some((filename.clone(), size)); }
                    _ => {}
                }
            }
        }
    } else if path.is_file() {
        let metadata = fs::metadata(path)
            .map_err(|e| format!("读取元数据失败: {}", e))?;
        let size = metadata.len();
        stats.total_files = 1;
        stats.total_size = size;
        stats.largest_file = Some((path.to_string_lossy().to_string(), size));
        stats.smallest_file = Some((path.to_string_lossy().to_string(), size));
    }
    
    Ok(stats)
}

fn human_readable_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.2} {}", size, UNITS[unit_idx])
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let command = parse_args(&args)?;
    
    match command {
        Command::Stats { path } => {
            let stats = collect_stats(&path)?;
            println!("===== 文件统计 =====");
            println!("路径: {}", path);
            println!("文件总数: {}", stats.total_files);
            println!("总大小: {}", human_readable_size(stats.total_size));
            if let Some((name, size)) = &stats.largest_file {
                println!("最大文件: {} ({})", name, human_readable_size(*size));
            }
            if let Some((name, size)) = &stats.smallest_file {
                println!("最小文件: {} ({})", name, human_readable_size(*size));
            }
        }
        _ => {}
    }
    
    Ok(())
}
```

**说明：** 递归统计时，对于目录先递归收集子目录的 `FileStats`，再合并到当前结果。合并时比较当前最大/最小值，决定是否替换。`human_readable_size` 将字节数转换为带单位的易读格式（KB/MB/GB）。
</details>

---

### 练习 23-24: 添加配置文件支持

> 难度：⭐⭐⭐
> 本练习融合：配置集成 + Option + 默认路径 + 环境变量

为 CLI 应用添加配置文件支持。默认读取当前目录下的 `.filestool.toml`（简化版，使用 `=` 格式），配置文件可设置默认搜索路径和排除模式。

```rust
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

// 配置结构
struct AppConfig {
    default_path: String,
    exclude_patterns: Vec<String>,
    max_depth: Option<usize>,
}

impl AppConfig {
    // TODO: 加载配置，按优先级：命令行指定 > 配置文件 > 默认值
    // 默认 default_path = ".", exclude_patterns = [], max_depth = None
    fn load(config_file: Option<&str>) -> Self {
        // 1. 尝试从配置文件读取
        // 2. 环境变量覆盖
        // 3. 命令行参数覆盖（在 main 中处理）
        todo!()
    }
}

// ===== 命令行解析（复用 23-21） =====
enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    match args.get(1).map(|s| s.as_str()) {
        Some("search") => {
            let pattern = args.get(2).cloned().unwrap_or_default();
            let path = args.get(3).cloned().unwrap_or_else(|| ".".to_string());
            Ok(Command::Search { pattern, path })
        }
        Some("stats") => {
            let path = args.get(2).cloned().unwrap_or_else(|| ".".to_string());
            Ok(Command::Stats { path })
        }
        Some(other) => Err(format!("未知子命令: {}", other)),
        None => Err("请指定子命令: search|stats".into()),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    
    // 加载配置（命令行可以指定 --config 参数）
    let config_file = args.iter()
        .position(|a| a == "--config")
        .and_then(|i| args.get(i + 1).map(|s| s.as_str()));
    
    let config = AppConfig::load(config_file);
    
    let command = parse_args(&args)?;
    match command {
        Command::Search { pattern, path } => {
            println!("搜索 '{}' (路径: {}, 最大深度: {:?})",
                     pattern, path, config.max_depth);
        }
        Command::Stats { path } => {
            println!("统计 (路径: {})", path);
        }
    }
    
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

struct AppConfig {
    default_path: String,
    exclude_patterns: Vec<String>,
    max_depth: Option<usize>,
}

impl AppConfig {
    fn load(config_file: Option<&str>) -> Self {
        let mut config = AppConfig {
            default_path: ".".to_string(),
            exclude_patterns: Vec::new(),
            max_depth: None,
        };
        
        // 1. 尝试加载配置文件
        let config_path = config_file.unwrap_or(".filestool.toml");
        if Path::new(config_path).exists() {
            if let Ok(content) = fs::read_to_string(config_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                        continue; // 跳过注释和 section 头
                    }
                    if let Some(idx) = line.find('=') {
                        let key = line[..idx].trim();
                        let value = line[idx+1..].trim().trim_matches('"');
                        match key {
                            "default_path" => config.default_path = value.to_string(),
                            "max_depth" => config.max_depth = value.parse().ok(),
                            "exclude" => config.exclude_patterns.push(value.to_string()),
                            _ => {}
                        }
                    }
                }
            }
        }
        
        // 2. 环境变量覆盖
        if let Ok(val) = env::var("FILETOOL_PATH") {
            config.default_path = val;
        }
        if let Ok(val) = env::var("FILETOOL_MAX_DEPTH") {
            config.max_depth = val.parse().ok();
        }
        
        config
    }
}

enum Command {
    Search { pattern: String, path: String },
    Stats { path: String },
}

fn parse_args(args: &[String]) -> Result<Command, String> {
    match args.get(1).map(|s| s.as_str()) {
        Some("search") => {
            let pattern = args.get(2).cloned().unwrap_or_default();
            let path = args.get(3).cloned().unwrap_or_else(|| ".".to_string());
            Ok(Command::Search { pattern, path })
        }
        Some("stats") => {
            let path = args.get(2).cloned().unwrap_or_else(|| ".".to_string());
            Ok(Command::Stats { path })
        }
        Some(other) => Err(format!("未知子命令: {}", other)),
        None => Err("请指定子命令: search|stats".into()),
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    
    let config_file = args.iter()
        .position(|a| a == "--config")
        .and_then(|i| args.get(i + 1).map(|s| s.as_str()));
    
    let config = AppConfig::load(config_file);
    
    let command = parse_args(&args)?;
    match command {
        Command::Search { pattern, path } => {
            println!("搜索 '{}' (路径: {}, 最大深度: {:?})",
                     pattern, path, config.max_depth);
        }
        Command::Stats { path } => {
            println!("统计 (路径: {})", path);
        }
    }
    
    Ok(())
}
```

**说明：** `AppConfig::load` 实现多层配置加载：文件 → 环境变量 → 命令行。配置文件解析跳过注释（`#`）和 section 头（`[toml]` 风格的 `[...]`）。`env::var("FILETOOL_PATH")` 允许通过环境变量覆盖默认路径。
</details>

---

### 练习 23-25: 完善 CLI — 错误处理与用户交互

> 难度：⭐⭐⭐
> 本练习融合：anyhow 风格（手动）+ 用户输入 + 退出码 + 完整流程

整合最终 CLI 应用：完善的错误处理（有意义的错误消息 + 建议）、用户确认删除功能、退出码设置、彩色输出（备选）。

```rust
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

// ===== 错误处理 =====
// TODO: 定义 AppError 枚举，包含不同错误类型
//  - IoError(String) — 文件 I/O 错误
//  - InvalidPath(String) — 无效路径
//  - UserAbort — 用户取消操作
//  - ArgumentError(String) — 参数错误
// 实现 Display，给出友好的错误消息

use std::fmt;
#[derive(Debug)]
enum AppError {
    // TODO
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 输出友好的错误消息，包含修复建议
        todo!()
    }
}

// ===== 主逻辑 =====

// TODO: 实现 confirm 函数，提示用户输入 y/n 确认
fn confirm(prompt: &str) -> Result<bool, AppError> {
    todo!()
}

// TODO: 实现 delete_file 函数（安全删除——先确认）
fn delete_file(path: &str) -> Result<(), AppError> {
    todo!()
}

// 搜索（简化版）
fn search_files(path: &str, pattern: &str) -> Result<Vec<String>, AppError> {
    let mut results = Vec::new();
    let dir = Path::new(path);
    if !dir.is_dir() {
        return Err(AppError::InvalidPath(format!("'{}' 不是有效目录", path)));
    }
    let entries = fs::read_dir(dir)
        .map_err(|e| AppError::IoError(format!("读取目录失败: {}", e)))?;
    for entry in entries {
        let entry = entry.map_err(|e| AppError::IoError(format!("读取条目失败: {}", e)))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.contains(pattern) {
            results.push(entry.path().to_string_lossy().to_string());
        }
    }
    Ok(results)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let result = match args.get(1).map(|s| s.as_str()) {
        Some("search") => {
            let pattern = args.get(2).map(|s| s.as_str()).unwrap_or("");
            let path = args.get(3).map(|s| s.as_str()).unwrap_or(".");
            match search_files(path, pattern) {
                Ok(files) => {
                    if files.is_empty() {
                        println!("未找到匹配文件");
                    } else {
                        for f in &files {
                            println!("{}", f);
                        }
                        println!("共找到 {} 个文件", files.len());
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Some("delete") => {
            // TODO: 接收文件路径作为参数，安全删除（需要确认）
            // 用法: app delete <文件路径>
            todo!()
        }
        Some("interactive") => {
            // TODO: 交互模式——循环读取用户输入并执行命令
            // 输入 "search <pattern>" 执行搜索
            // 输入 "delete <path>" 执行删除
            // 输入 "quit" 退出
            todo!()
        }
        Some(other) => {
            Err(AppError::ArgumentError(format!("未知命令: '{}'。\n可用命令: search, delete, interactive", other)))
        }
        None => {
            Err(AppError::ArgumentError("请指定命令。\n用法: app <命令> [参数]\n命令: search <模式> [路径], delete <路径>, interactive".into()))
        }
    };
    
    if let Err(e) = result {
        eprintln!("错误: {}", e);
        process::exit(1);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

#[derive(Debug)]
enum AppError {
    IoError(String),
    InvalidPath(String),
    UserAbort,
    ArgumentError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(msg) => {
                write!(f, "I/O 错误: {}\n  → 请检查文件权限和磁盘空间", msg)
            }
            AppError::InvalidPath(msg) => {
                write!(f, "路径无效: {}\n  → 请确认路径存在且可访问", msg)
            }
            AppError::UserAbort => {
                write!(f, "操作已取消")
            }
            AppError::ArgumentError(msg) => {
                write!(f, "参数错误: {}", msg)
            }
        }
    }
}

fn confirm(prompt: &str) -> Result<bool, AppError> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().map_err(|e| AppError::IoError(e.to_string()))?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| AppError::IoError(format!("读取输入失败: {}", e)))?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn delete_file(path: &str) -> Result<(), AppError> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(AppError::InvalidPath(format!("文件不存在: {}", path)));
    }
    if !p.is_file() {
        return Err(AppError::InvalidPath(format!("'{}' 不是文件", path)));
    }
    
    // 显示文件信息并确认
    let metadata = fs::metadata(p)
        .map_err(|e| AppError::IoError(format!("读取文件信息失败: {}", e)))?;
    println!("即将删除: {}", path);
    println!("文件大小: {} 字节", metadata.len());
    
    if !confirm("确定要删除吗?")? {
        return Err(AppError::UserAbort);
    }
    
    fs::remove_file(p)
        .map_err(|e| AppError::IoError(format!("删除失败: {}", e)))?;
    println!("已删除: {}", path);
    Ok(())
}

fn search_files(path: &str, pattern: &str) -> Result<Vec<String>, AppError> {
    let mut results = Vec::new();
    let dir = Path::new(path);
    if !dir.is_dir() {
        return Err(AppError::InvalidPath(format!("'{}' 不是有效目录", path)));
    }
    let entries = fs::read_dir(dir)
        .map_err(|e| AppError::IoError(format!("读取目录失败: {}", e)))?;
    for entry in entries {
        let entry = entry.map_err(|e| AppError::IoError(format!("读取条目失败: {}", e)))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.contains(pattern) {
            results.push(entry.path().to_string_lossy().to_string());
        }
    }
    Ok(results)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let result = match args.get(1).map(|s| s.as_str()) {
        Some("search") => {
            let pattern = args.get(2).map(|s| s.as_str()).unwrap_or("");
            let path = args.get(3).map(|s| s.as_str()).unwrap_or(".");
            match search_files(path, pattern) {
                Ok(files) => {
                    if files.is_empty() {
                        println!("未找到匹配文件");
                    } else {
                        for f in &files {
                            println!("{}", f);
                        }
                        println!("共找到 {} 个文件", files.len());
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Some("delete") => {
            let path = match args.get(2) {
                Some(p) => p.as_str(),
                None => {
                    eprintln!("错误: 请指定文件路径\n用法: app delete <文件路径>");
                    process::exit(1);
                }
            };
            delete_file(path)
        }
        Some("interactive") => {
            println!("交互模式已启动。输入命令: search <模式>, delete <路径>, quit");
            loop {
                print!("> ");
                io::stdout().flush().ok();
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    break;
                }
                let input = input.trim();
                if input.eq_ignore_ascii_case("quit") || input.is_empty() {
                    break;
                }
                if let Some(pattern) = input.strip_prefix("search ") {
                    match search_files(".", pattern) {
                        Ok(files) => {
                            for f in &files { println!("{}", f); }
                            println!("共找到 {} 个文件", files.len());
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                } else if let Some(path) = input.strip_prefix("delete ") {
                    if let Err(e) = delete_file(path) {
                        eprintln!("{}", e);
                    }
                } else {
                    println!("未知命令。支持: search <模式>, delete <路径>, quit");
                }
            }
            Ok(())
        }
        Some(other) => {
            Err(AppError::ArgumentError(format!(
                "未知命令: '{}'。\n可用命令: search, delete, interactive", other
            )))
        }
        None => {
            Err(AppError::ArgumentError(
                "请指定命令。\n用法: app <命令> [参数]\n命令: search <模式> [路径], delete <路径>, interactive".into()
            ))
        }
    };
    
    if let Err(e) = result {
        eprintln!("错误: {}", e);
        process::exit(1);
    }
}
```

**说明：** 这是本书的最终综合练习，融合了以下 Rust 核心概念：
- **自定义错误类型**（`AppError` 枚举）——为不同类型错误提供有意义的提示
- **用户交互**（`io::stdin()` / `stdout()`）——安全操作前需要确认
- **递归与遍历**——文件搜索
- **`Result` 错误传播**——整个 `main` 使用统一的错误处理
- **退出码设置**——`process::exit(1)` 表示失败
- **交互式 REPL**——循环读取命令并执行
- **所有权的隐式使用**——字符串传递、文件句柄管理

</details>

---

> 🎉 恭喜你完成了全部 25 道综合实战题！你已经深入体验了 Rust 的核心概念在实际项目中的运用。从 JSON 解析到 HTTP 客户端，从日志系统到配置管理，再到完整的 CLI 应用——你现在已经具备了用 Rust 构建真实项目的能力。
