# 13 String 与字符串

Rust 中的字符串处理与其他语言有显著不同：Rust 有 `String`（可变、拥有者类型）和 `&str`（不可变、借用类型）两种核心字符串类型，且天然支持 UTF-8。本章 20 道练习题将帮助你掌握两者的区别、互转、拼接、切割、遍历及编码相关知识。

### 练习 13-01: String 从字面量创建

> 难度：⭐
> 对比：Java 的 `"hello"` 是 String 对象；C++ 的 `"hello"` 是 `const char*`

用两种方式从字符串字面量 `"hello"` 创建 `String`。

```rust
fn main() {
    // TODO: 使用 String::from 创建 String
    let s1: String = /* ??? */;

    // TODO: 使用 .to_string() 方法创建 String
    let s2: String = /* ??? */;

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s1: String = String::from("hello");
    let s2: String = "hello".to_string();
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

**说明：** `String::from` 和 `to_string()` 都能从 `&str` 创建 `String`。`String::from` 更强调"从...创建"的语义，`to_string()` 则通用性更广（任何实现了 `ToString` 的类型均可调用）。
</details>

### 练习 13-02: &str 从 String 借用

> 难度：⭐
> 对比：Java 无法从 String 获取"借用"，C++ 可通过 `.c_str()` 获得 `const char*`

通过取引用将 `String` 转为 `&str`。

```rust
fn main() {
    let s = String::from("world");
    
    // TODO: 通过取引用获得 &str
    let s1: &str = /* ??? */;

    // TODO: 通过显式类型标注（as_str）获得 &str
    let s2: &str = /* ??? */;

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s = String::from("world");
    let s1: &str = &s;
    let s2: &str = s.as_str();
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

**说明：** `&s` 利用了 `Deref` 解引用强制转换，`s.as_str()` 则显式获取 `&str`。两者底层相同，都是获取 `String` 内部缓冲区的一个切片引用。
</details>

### 练习 13-03: 使用 to_string() 转换多种类型

> 难度：⭐⭐
> 对比：Java 的 `String.valueOf()`，C++ 的 `std::to_string()`

调用 `to_string()` 将数字和布尔值转为字符串。

```rust
fn main() {
    // TODO: 使用 to_string() 将 42 转为字符串
    let a: String = /* ??? */;
    
    // TODO: 将 true 转为字符串
    let b: String = /* ??? */;
    
    // TODO: 将 3.14 转为字符串
    let c: String = /* ??? */;
    
    println!("{}, {}, {}", a, b, c);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let a: String = 42.to_string();
    let b: String = true.to_string();
    let c: String = 3.14.to_string();
    println!("{}, {}, {}", a, b, c);
}
```

**说明：** `to_string()` 来自 `ToString` trait，任何实现了 `Display` 的类型都会自动实现 `ToString`，因此数字、布尔值等均可直接调用。
</details>

### 练习 13-04: 使用 to_owned() 克隆字符串数据

> 难度：⭐⭐
> 对比：类似 Java 的 `new String(original)`，C++ 的拷贝构造函数

`to_owned()` 从借用的字符串数据创建一个拥有所有权的副本。

```rust
fn main() {
    let s: &str = "hello rust";
    
    // TODO: 使用 to_owned() 将 &str 转为 String
    let owned: String = /* ??? */;
    
    // TODO: 再从一个 String 的引用调用 to_owned()
    let original = String::from("copyme");
    let r: &String = &original;
    let cloned: String = /* ??? */;
    
    println!("owned = {}, cloned = {}", owned, cloned);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s: &str = "hello rust";
    let owned: String = s.to_owned();
    let original = String::from("copyme");
    let r: &String = &original;
    let cloned: String = r.to_owned();
    println!("owned = {}, cloned = {}", owned, cloned);
}
```

**说明：** `to_owned()` 定义在 `ToOwned` trait 中，能从任何借用的数据创建拥有所有权的副本。对 `&str` 和 `&String` 都能工作，结果都是 `String`。
</details>

### 练习 13-05: String 与 &str 互转综合挑战

> 难度：⭐⭐⭐
> 对比：Java 没有明显的所有权区分，C++ 中 `std::string` 和 `std::string_view` 类似

完成一个函数，接收 `&str` 返回 `String`，并验证两者之间的关系。

```rust
fn greet(name: &str) -> String {
    // TODO: 返回 "Hello, " + name + "!" 的 String
    // 提示：可以使用 format! 或 push_str
    format!(/* ??? */)
}

fn main() {
    let name = "Alice";
    let greeting = greet(name);
    println!("{}", greeting);
    
    // TODO: 证明 greeting 是 String，不是 &str
    // 调用某个 String 独有的方法（&str 没有的方法）
    let len = /* ??? */;  // 获取 String 的容量
    println!("Capacity: {}", len);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = "Alice";
    let greeting = greet(name);
    println!("{}", greeting);
    
    let len = greeting.capacity();
    println!("Capacity: {}", len);
}
```

**说明：** `format!` 宏返回 `String`。`capacity()` 是 `String` 独有的方法（`&str` 没有），可以借此证明变量类型。`String` 拥有堆上分配的缓冲区，`&str` 只是指向某处字符串数据的引用。
</details>

### 练习 13-06: 使用 + 号拼接字符串

> 难度：⭐
> 对比：Java 的 `+` 可以拼接任意对象；C++ 的 `operator+` 也支持拼接

使用 `+` 操作符拼接两个字符串。

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    
    // TODO: 使用 + 拼接 s1 和 s2，中间加空格和感叹号
    // 注意：+ 会取得左侧 String 的所有权
    let result = /* ??? */;
    
    println!("{}", result);
    // println!("{}", s1); // 取消注释看是否会报错
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let result = s1 + " " + &s2 + "!";
    println!("{}", result);
    // println!("{}", s1); // 编译错误！s1 的所有权已被 + 移动
}
```

**说明：** `+` 操作符实际调用 `add(self, s: &str)` 方法，它**取得**左侧 `String` 的所有权（`self`），右侧接受 `&str`。因此需要 `&s2`（自动解引用为 `&str`），而 `s1` 在拼接后不能再使用。
</details>

### 练习 13-07: 使用 push_str 和 format! 拼接

> 难度：⭐
> 对比：Java 的 `StringBuilder.append()`，C++ 的 `std::string::append()` / `operator+=`

使用 `push_str` 和 `format!` 两种方式拼接字符串。

```rust
fn main() {
    // 方式一：push_str
    let mut s = String::from("Rust");
    // TODO: 使用 push_str 追加 " is awesome"
    /* ??? */

    println!("push_str: {}", s);

    // 方式二：format!
    let lang = "Rust";
    let desc = "is awesome";
    // TODO: 使用 format! 创建新字符串
    let result = /* ??? */;
    
    println!("format!: {}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    // 方式一：push_str
    let mut s = String::from("Rust");
    s.push_str(" is awesome");
    println!("push_str: {}", s);

    // 方式二：format!
    let lang = "Rust";
    let desc = "is awesome";
    let result = format!("{} {}", lang, desc);
    println!("format!: {}", result);
}
```

**说明：** `push_str` 会追加到原字符串（需要 `mut`），不涉及所有权转移。`format!` 返回新的 `String`，不修改原变量，也不转移所有权——是最灵活安全的拼接方式。
</details>

### 练习 13-08: format! 格式化输出

> 难度：⭐⭐
> 对比：Java 的 `String.format()`，C++20 的 `std::format()`

使用 `format!` 对数字进行格式化填充。

```rust
fn main() {
    let name = "Bob";
    let age = 25;
    let score = 92.5;
    
    // TODO: 格式化为 "Name: Bob | Age: 25 | Score: 92.5"
    let info = format!(/* ??? */);
    println!("{}", info);
    
    // TODO: 将数字 42 格式化为 5 位宽度、右对齐（"   42"）
    let padded = format!(/* ??? */);
    println!("'{}'", padded);
    
    // TODO: 将整数 255 格式化为十六进制（"ff"）
    let hex = format!(/* ??? */);
    println!("{}", hex);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let name = "Bob";
    let age = 25;
    let score = 92.5;
    
    let info = format!("Name: {} | Age: {} | Score: {}", name, age, score);
    println!("{}", info);
    
    let padded = format!("{:>5}", 42);
    println!("'{}'", padded);
    
    let hex = format!("{:x}", 255);
    println!("{}", hex);
}
```

**说明：** `format!` 支持丰富的格式化选项：`{}` 默认显示，`{:>5}` 右对齐宽度 5，`{:x}` 十六进制小写，`{:X}` 十六进制大写，`{:.2}` 指定小数位数等。
</details>

### 练习 13-09: format! 进阶 — 位置参数与命名参数

> 难度：⭐⭐
> 对比：Java 的 `%1$s` 位置引用，C++ 的 `std::format` 也支持位置参数

使用 `format!` 的位置参数和命名参数功能。

```rust
fn main() {
    let a = 1;
    let b = 2;
    
    // TODO: 使用位置参数输出 "1 + 2 = 3"（第 3 个参数计算 a + b）
    let result = format!(/* ??? */, a, b, a + b);
    println!("{}", result);
    
    // TODO: 使用命名参数输出 "Hello, Rust!"
    let greeting = format!(/* ??? */, /* ??? */);
    println!("{}", greeting);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let a = 1;
    let b = 2;
    
    let result = format!("{0} + {1} = {2}", a, b, a + b);
    println!("{}", result);
    
    let greeting = format!("Hello, {lang}!", lang = "Rust");
    println!("{}", greeting);
}
```

**说明：** `{0}`、`{1}` 引用位置参数，可在多个位置重复使用同一参数。命名参数如 `{lang}` 让格式化字符串更具可读性，适合模板场景。
</details>

### 练习 13-10: 字符串拼接综合挑战

> 难度：⭐⭐⭐
> 对比：Java/C++ 中连续 `+` 都会产生中间对象，Rust 中的 `+` 会转移所有权

实现一个 `build_sentence` 函数，接收多个单词并拼接成一句话。

```rust
/// 将单词列表拼接成一句话：在单词间加空格，句尾加句号。
/// 如果列表为空，返回空字符串。
fn build_sentence(words: &[&str]) -> String {
    // TODO: 实现拼接逻辑
    // 提示：可以用 let mut s = String::new(); 然后循环 push_str
    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push_str(" ");
        }
        result.push_str(word);
    }
    if !result.is_empty() {
        result.push_str(".");
    }
    result
}

fn main() {
    let words = vec!["Rust", "is", "fun"];
    let sentence = build_sentence(&words);
    println!("{}", sentence);
    assert_eq!(sentence, "Rust is fun.");
    
    let empty: Vec<&str> = vec![];
    assert_eq!(build_sentence(&empty), "");
    println!("综合挑战通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn build_sentence(words: &[&str]) -> String {
    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push_str(" ");
        }
        result.push_str(word);
    }
    if !result.is_empty() {
        result.push_str(".");
    }
    result
}

fn main() {
    let words = vec!["Rust", "is", "fun"];
    let sentence = build_sentence(&words);
    println!("{}", sentence);
    assert_eq!(sentence, "Rust is fun.");
    
    let empty: Vec<&str> = vec![];
    assert_eq!(build_sentence(&empty), "");
    println!("综合挑战通过！");
}
```

**说明：** 循环拼接时需要注意：两个单词之间才需要空格（通过 `enumerate` 的索引判断），句尾统一加句号。空列表需要返回空字符串，判断 `is_empty` 避免多余句号。这里也体现了 `String` 的可变增长能力。
</details>

### 练习 13-11: 使用 split 切割字符串

> 难度：⭐
> 对比：Java 的 `String.split()`，C++ 常用 `stringstream` 或 `find`

使用 `split` 方法按指定分隔符切割字符串。

```rust
fn main() {
    let data = "apple,banana,cherry,date";
    
    // TODO: 用 ',' 分割 data，并收集每个水果到 Vec<&str>
    let fruits: Vec<&str> = /* ??? */;
    
    println!("{:?}", fruits);
    assert_eq!(fruits, vec!["apple", "banana", "cherry", "date"]);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let data = "apple,banana,cherry,date";
    let fruits: Vec<&str> = data.split(',').collect();
    println!("{:?}", fruits);
    assert_eq!(fruits, vec!["apple", "banana", "cherry", "date"]);
}
```

**说明：** `split` 返回一个迭代器，调用 `.collect()` 可以收集到 `Vec<&str>`。重要的是，返回的 `&str` 仍然引用原字符串的数据，没有发生拷贝，非常高效。
</details>

### 练习 13-12: 使用 chars 获取单个字符

> 难度：⭐
> 对比：Java 的 `.charAt()`，C++ 的 `operator[]`（不适用于多字节字符）

使用 `chars` 方法获取字符串中的第 N 个字符。

```rust
fn main() {
    let text = "你好，世界！";
    
    // TODO: 获取 text 的第一个字符
    let first = text.chars()./* ??? */;
    println!("第一个字符: {:?}", first);
    assert_eq!(first, Some('你'));
    
    // TODO: 获取 text 的第三个字符
    let third = text.chars()./* ??? */;
    println!("第三个字符: {:?}", third);
    assert_eq!(third, Some('，'));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let text = "你好，世界！";
    let first = text.chars().next();
    println!("第一个字符: {:?}", first);
    assert_eq!(first, Some('你'));
    
    let third = text.chars().nth(2);
    println!("第三个字符: {:?}", third);
    assert_eq!(third, Some('，'));
}
```

**说明：** 对于 UTF-8 字符串，不能直接用索引访问字符（因为一个字符可能占 1-4 字节）。`.chars()` 返回字符迭代器，`.next()` 取第一个，`.nth(n)` 取第 n+1 个。返回 `Option<char>` 安全处理空字符串或超界情况。
</details>

### 练习 13-13: 使用 bytes 遍历字节

> 难度：⭐⭐
> 对比：C++ 可直接遍历 `std::string` 的字节数组，Java 用 `.getBytes()`

遍历字符串的底层字节序列。

```rust
fn main() {
    let s = "Rust";
    
    println!("字节序列:");
    // TODO: 使用 bytes() 遍历每个字节并打印
    for byte in s./* ??? */ {
        println!("  0x{:02X}", byte);
    }
    
    // 中文
    let cn = "中";
    println!("'中' 的字节序列:");
    // TODO: 打印出 '中' 的 UTF-8 编码（应当是 3 个字节）
    for byte in cn./* ??? */ {
        println!("  0x{:02X}", byte);
    }
    // TODO: 断言字节数是 3
    assert_eq!(cn.len(), /* ??? */);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s = "Rust";
    println!("字节序列:");
    for byte in s.bytes() {
        println!("  0x{:02X}", byte);
    }
    
    let cn = "中";
    println!("'中' 的字节序列:");
    for byte in cn.bytes() {
        println!("  0x{:02X}", byte);
    }
    assert_eq!(cn.len(), 3);
}
```

**说明：** `.bytes()` 返回字节迭代器，每次 yield 一个 `u8`。这里可以看到 `"Rust"` 每个字符 1 字节（ASCII），而 `"中"` 占 3 字节（E4 B8 AD）。`len()` 返回的是字节数，不是字符数。
</details>

### 练习 13-14: 使用 chars 遍历字符

> 难度：⭐⭐
> 对比：Java 的 `for (char c : str.toCharArray())`，C++ 需要手动处理 UTF-8

遍历字符串中的每一个 Unicode 字符。

```rust
fn main() {
    let s = "a🦀🔥";
    
    println!("字符遍历:");
    // TODO: 使用 chars() 遍历所有字符
    for c in s./* ??? */ {
        println!("  '{}'", c);
    }
    
    // TODO: 断言字符个数（不是字节数）
    assert_eq!(s.chars().count(), /* ??? */);
    
    // 提示：字符数不等于字节数
    println!("字节数: {}, 字符数: {}", s.len(), s.chars().count());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s = "a🦀🔥";
    println!("字符遍历:");
    for c in s.chars() {
        println!("  '{}'", c);
    }
    assert_eq!(s.chars().count(), 3);
    
    println!("字节数: {}, 字符数: {}", s.len(), s.chars().count());
}
```

**说明：** `a` 占 1 字节，`🦀`（螃蟹 emoji）占 4 字节，`🔥`（火焰 emoji）占 4 字节，共 9 字节但只有 3 个字符。`.chars().count()` 给出真正的字符数。这在处理用户可见的文本长度时至关重要。
</details>

### 练习 13-15: 字符串切割遍历综合挑战

> 难度：⭐⭐⭐
> 对比：Java 的 `substring` 基于 char 索引会出错，C++17 的 `std::string_view` 类似 &str

实现一个函数，将长文本按指定宽度（字符数）换行。

```rust
/// 将文本按指定字符宽度换行，返回每一行的切片
fn wrap_text(text: &str, max_width: usize) -> Vec<&str> {
    // TODO: 实现换行逻辑
    // 提示：收集所有 char，然后按 max_width 分组切片
    let chars: Vec<char> = text.chars().collect();
    let mut result = Vec::new();
    let mut start = 0;
    while start < chars.len() {
        let end = (start + max_width).min(chars.len());
        // 从原字符串中切出对应的子串
        // 需要找到第 start 个 char 和第 end 个 char 的字节偏移
        let byte_start = chars[..start].iter().map(|c| c.len_utf8()).sum();
        let byte_end = chars[..end].iter().map(|c| c.len_utf8()).sum();
        result.push(&text[byte_start..byte_end]);
        start = end;
    }
    result
}

fn main() {
    let text = "Hello世界Rust编程";
    let lines = wrap_text(text, 4);
    for (i, line) in lines.iter().enumerate() {
        println!("第{}行: '{}'", i + 1, line);
    }
    // 每个汉字和英文字母都算一个字符
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "Hello");
    assert_eq!(lines[1], "世界Ru");
    assert_eq!(lines[2], "st编程");
    println!("综合挑战通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn wrap_text(text: &str, max_width: usize) -> Vec<&str> {
    let chars: Vec<char> = text.chars().collect();
    let mut result = Vec::new();
    let mut start = 0;
    while start < chars.len() {
        let end = (start + max_width).min(chars.len());
        let byte_start = chars[..start].iter().map(|c| c.len_utf8()).sum();
        let byte_end = chars[..end].iter().map(|c| c.len_utf8()).sum();
        result.push(&text[byte_start..byte_end]);
        start = end;
    }
    result
}

fn main() {
    let text = "Hello世界Rust编程";
    let lines = wrap_text(text, 4);
    for (i, line) in lines.iter().enumerate() {
        println!("第{}行: '{}'", i + 1, line);
    }
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "Hello");
    assert_eq!(lines[1], "世界Ru");
    assert_eq!(lines[2], "st编程");
    println!("综合挑战通过！");
}
```

**说明：** 字符串切片操作 `&text[start..end]` 使用**字节偏移**，而非字符索引。直接按字符索引切割会越界或切在字符中间导致 panic。正确做法是：先将字符收集到 `Vec<char>`，计算对应字符位置的字节偏移，再执行切片。`char::len_utf8()` 返回该字符的 UTF-8 编码长度。
</details>

### 练习 13-16: UTF-8 编码基础

> 难度：⭐
> 对比：Java 的 `char` 是 UTF-16 编码（2 字节），C++ 的 `char` 通常是 1 字节 ASCII

了解 UTF-8 编码下不同字符的字节长度。

```rust
fn main() {
    // TODO: 在下方空格填入正确的字节数
    // ASCII 字符
    assert_eq!("A".len(), /* ? */);
    
    // 拉丁字母重音字符
    assert_eq!("é".len(), /* ? */);
    
    // 中文字符
    assert_eq!("中".len(), /* ? */);
    
    // Emoji
    assert_eq!("🎉".len(), /* ? */);
    
    println!("所有 UTF-8 长度判断正确！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    assert_eq!("A".len(), 1);
    assert_eq!("é".len(), 2);
    assert_eq!("中".len(), 3);
    assert_eq!("🎉".len(), 4);
    println!("所有 UTF-8 长度判断正确！");
}
```

**说明：** UTF-8 编码是变长编码：ASCII（U+0000 ~ U+007F）占 1 字节；拉丁扩展等（U+0080 ~ U+07FF）占 2 字节；常用汉字（U+0800 ~ U+FFFF）占 3 字节；Emoji 和生僻字（U+10000 以上）占 4 字节。`len()` 返回字节数而非字符数。
</details>

### 练习 13-17: 字符与字节数的换算

> 难度：⭐
> 对比：C++ 中 `std::string::size()` 同样是字节数，容易混淆

计算字符串的字符数（不直接使用 `.chars().count()`，而是手动遍历计数）。

```rust
fn char_count(s: &str) -> usize {
    // TODO: 手动遍历 chars 计数
    let mut count = 0;
    for /* ??? */ in s.chars() {
        count += 1;
    }
    count
}

fn main() {
    let s = "Rust🦀编程🔥";
    println!("字符串: '{}'", s);
    println!("字节数: {}", s.len());
    println!("字符数: {}", char_count(s));
    
    assert_eq!(s.len(), 15);   // R(1) u(1) s(1) t(1) 🦀(4) 编(3) 程(3) 🔥(4)
    assert_eq!(char_count(s), 8);
    println!("换算正确！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn char_count(s: &str) -> usize {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}

fn main() {
    let s = "Rust🦀编程🔥";
    println!("字符串: '{}'", s);
    println!("字节数: {}", s.len());
    println!("字符数: {}", char_count(s));
    
    assert_eq!(s.len(), 15);
    assert_eq!(char_count(s), 8);
    println!("换算正确！");
}
```

**说明：** 手动遍历 `chars` 演示了字符计数的原理。`s.len()` 返回字节数，`s.chars().count()` 返回字符数。对于包含非 ASCII 的文本，两者差异明显，初学者经常混淆。
</details>

### 练习 13-18: char 的基本操作

> 难度：⭐⭐
> 对比：Java 的 `Character.isDigit()`、`Character.isUpperCase()`，C++ 的 `std::isalpha`

使用 `char` 的方法判断字符类别。

```rust
fn main() {
    let text = "Rust 3.0 🚀";
    
    let mut digit_count = 0;
    let mut alpha_count = 0;
    let mut whitespace_count = 0;
    
    for c in text.chars() {
        // TODO: 使用 char 方法判断并分类
        if c./* ??? */ {
            digit_count += 1;
        } else if c./* ??? */ {
            alpha_count += 1;
        } else if c./* ??? */ {
            whitespace_count += 1;
        }
    }
    
    println!("数字: {}", digit_count);
    println!("字母: {}", alpha_count);
    println!("空白: {}", whitespace_count);
    
    assert_eq!(digit_count, 2);   // 3 和 0
    assert_eq!(alpha_count, 4);   // R, u, s, t
    assert_eq!(whitespace_count, 2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let text = "Rust 3.0 🚀";
    
    let mut digit_count = 0;
    let mut alpha_count = 0;
    let mut whitespace_count = 0;
    
    for c in text.chars() {
        if c.is_ascii_digit() {
            digit_count += 1;
        } else if c.is_ascii_alphabetic() {
            alpha_count += 1;
        } else if c.is_ascii_whitespace() {
            whitespace_count += 1;
        }
    }
    
    println!("数字: {}", digit_count);
    println!("字母: {}", alpha_count);
    println!("空白: {}", whitespace_count);
    
    assert_eq!(digit_count, 2);
    assert_eq!(alpha_count, 4);
    assert_eq!(whitespace_count, 2);
}
```

**说明：** `char` 类型提供了丰富的方法：`is_ascii_digit()` 判断是否为 ASCII 数字，`is_ascii_alphabetic()` 判断字母，`is_ascii_whitespace()` 判断空白。还有更通用的 `is_uppercase()`、`is_lowercase()`、`is_alphanumeric()` 等 Unicode 感知方法。
</details>

### 练习 13-19: char 大小写转换

> 难度：⭐⭐
> 对比：Java 的 `Character.toUpperCase()`，C++ 的 `std::toupper`

实现字符串的大小写转换（不借助 `to_uppercase()` / `to_lowercase()` 全串方法，手动逐字符转换）。

```rust
fn to_upper_first(s: &str) -> String {
    // TODO: 将首字母转为大写，其余保持不变
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            // 转为大写
            for upper in c./* ??? */ {
                result.push(upper);
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn main() {
    let text = "hello 世界";
    let result = to_upper_first(text);
    println!("'{}' -> '{}'", text, result);
    assert_eq!(result, "Hello 世界");
    
    // 注意：有些字符大写后可能变为多个字符（如德语 ß → SS）
    let german = "ß";
    let upper_german: String = german.chars().flat_map(|c| c.to_uppercase()).collect();
    println!("'{}' 大写 -> '{}'", german, upper_german);
    assert_eq!(upper_german, "SS");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn to_upper_first(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            for upper in c.to_uppercase() {
                result.push(upper);
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn main() {
    let text = "hello 世界";
    let result = to_upper_first(text);
    println!("'{}' -> '{}'", text, result);
    assert_eq!(result, "Hello 世界");
    
    let german = "ß";
    let upper_german: String = german.chars().flat_map(|c| c.to_uppercase()).collect();
    println!("'{}' 大写 -> '{}'", german, upper_german);
    assert_eq!(upper_german, "SS");
}
```

**说明：** `char::to_uppercase()` 返回迭代器而非单个字符，因为某些 Unicode 字符（如德语 `ß`）大写后变为 `SS`（2 个字符）。这在处理国际化文本时非常重要——简单的 `c as u8 - 32` 只适用于 ASCII。
</details>

### 练习 13-20: 简易模板引擎

> 难度：⭐⭐⭐
> 对比：Java 的 `String.replace()` / 模板引擎，C++ 需要手动查找替换

实现一个简易模板引擎，将 `{name}` 形式的占位符替换为实际值。

```rust
use std::collections::HashMap;

/// 简易模板引擎：将 template 中的 {key} 替换为 values 中对应的值
/// 如果占位符在 values 中不存在，保持原样
fn render(template: &str, values: &HashMap<&str, &str>) -> String {
    // TODO: 实现模板替换逻辑
    // 提示：可以遍历 template，检测 '{' 和 '}' 之间的内容作为 key
    let mut result = String::new();
    let mut key_buf = String::new();
    let mut in_key = false;

    for c in template.chars() {
        match c {
            '{' => {
                in_key = true;
                key_buf.clear();
            }
            '}' => {
                in_key = false;
                match values.get(key_buf.as_str()) {
                    Some(val) => result.push_str(val),
                    None => {
                        result.push('{');
                        result.push_str(&key_buf);
                        result.push('}');
                    }
                }
                key_buf.clear();
            }
            _ if in_key => {
                key_buf.push(c);
            }
            _ => {
                result.push(c);
            }
        }
    }

    // 如果模板结束时还没有闭合的花括号
    if in_key {
        result.push('{');
        result.push_str(&key_buf);
    }
    
    result
}

fn main() {
    let mut values = HashMap::new();
    values.insert("name", "Alice");
    values.insert("city", "Beijing");
    
    let template = "Hello {name}, welcome to {city}!";
    let output = render(template, &values);
    println!("模板: '{}'", template);
    println!("输出: '{}'", output);
    assert_eq!(output, "Hello Alice, welcome to Beijing!");
    
    // 测试不存在的占位符
    let template2 = "Hi {name}, your score is {score}";
    let output2 = render(template2, &values);
    println!("输出2: '{}'", output2);
    assert_eq!(output2, "Hi Alice, your score is {score}");
    
    println!("简易模板引擎通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn render(template: &str, values: &HashMap<&str, &str>) -> String {
    let mut result = String::new();
    let mut key_buf = String::new();
    let mut in_key = false;

    for c in template.chars() {
        match c {
            '{' => {
                in_key = true;
                key_buf.clear();
            }
            '}' => {
                in_key = false;
                match values.get(key_buf.as_str()) {
                    Some(val) => result.push_str(val),
                    None => {
                        result.push('{');
                        result.push_str(&key_buf);
                        result.push('}');
                    }
                }
                key_buf.clear();
            }
            _ if in_key => {
                key_buf.push(c);
            }
            _ => {
                result.push(c);
            }
        }
    }

    if in_key {
        result.push('{');
        result.push_str(&key_buf);
    }

    result
}

fn main() {
    let mut values = HashMap::new();
    values.insert("name", "Alice");
    values.insert("city", "Beijing");
    
    let template = "Hello {name}, welcome to {city}!";
    let output = render(template, &values);
    println!("模板: '{}'", template);
    println!("输出: '{}'", output);
    assert_eq!(output, "Hello Alice, welcome to Beijing!");
    
    let template2 = "Hi {name}, your score is {score}";
    let output2 = render(template2, &values);
    println!("输出2: '{}'", output2);
    assert_eq!(output2, "Hi Alice, your score is {score}");
    
    println!("简易模板引擎通过！");
}
```

**说明：** 这是一个完整的简易模板引擎实现。核心逻辑是：逐个遍历字符，遇到 `{` 进入 key 收集模式，遇到 `}` 结束并在 `HashMap` 中查找替换。未闭合的花括号和未找到的 key 都会保留原样。这个练习综合运用了字符串遍历、字符判断、字符串拼接和集合查找等知识。
</details>
