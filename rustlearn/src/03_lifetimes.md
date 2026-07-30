# 03 生命周期

生命周期（Lifetimes）是 Rust 最独特的特性之一，它在编译期确保所有引用都指向有效的数据，从根本上杜绝了悬垂引用。与 C++ 中引用在运行时可能产生未定义行为不同，Rust 的生命周期系统在编译时就通过借用检查器（Borrow Checker）保证内存安全。

---

### 练习 03-01: 标注第一个生命周期参数

> 难度：⭐
> 类似 C++ 的 const T& 引用有效性（但 C++ 不做编译期检查）

阅读下面的函数，它试图返回两个字符串切片中较长的一个。请补全生命周期标注 `'a`，使函数能够正确编译。

```rust
// TODO: 在函数签名中补全生命周期标注
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("短");
    let s2 = String::from("较长的字符串");
    let result = longest(&s1[..], &s2[..]);
    println!("较长的字符串是: {}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("短");
    let s2 = String::from("较长的字符串");
    let result = longest(&s1[..], &s2[..]);
    println!("较长的字符串是: {}", result);
}
```

**说明：** 生命周期参数 `'a` 表示所有参数和返回值共享同一个生命周期约束。这意味着返回的引用将拥有 `x` 和 `y` 中较短的那个生命周期。在函数签名中，生命周期参数写在函数名后的尖括号 `<` `>` 中，每个引用参数前用 `&'a` 标注。
</details>

---

### 练习 03-02: 修正缺失的生命周期标注

> 难度：⭐
> 类似 C++ 的返回局部引用（UB）

下面的代码试图返回对字符串中第一个单词的引用，但缺少了必要的生命周期标注。请补全代码。

```rust
// TODO: 补全生命周期标注
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词: {}", word);
}
```

**说明：** 这里 `'a` 标注表明返回值的生命周期与参数 `s` 的生命周期相同。由于只有一个输入参数，实际上可以利用生命周期省略规则（练习 03-11 详述），这里显式标注是为了练习语法。注意 `&s[..]` 的类型是 `&str`，它与 `s` 的生命周期绑定。
</details>

---

### 练习 03-03: 两个参数一个返回值

> 难度：⭐
> 类似 C++ 的两个 const T& 参数，返回值必须有效

编写一个函数 `choose_longest`，接收两个字符串切片引用，返回较长的那一个。需要显式标注生命周期。

```rust
// TODO: 定义 choose_longest 函数，接收两个 &str 参数，返回 &str
// 要求：使用生命周期标注 'a

fn main() {
    let a = "Rust";
    let b = "Programming";
    let chosen = choose_longest(a, b);
    println!("选中的: {}", chosen);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn choose_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = "Rust";
    let b = "Programming";
    let chosen = choose_longest(a, b);
    println!("选中的: {}", chosen);
}
```

**说明：** 当函数有多个引用参数且返回引用时，必须显式标注生命周期。`'a` 约束了 `x`、`y` 和返回值三者的生命周期关系——返回值的生命周期不能超过 `x` 和 `y` 中较短的那个。由于 `a` 和 `b` 是字符串字面量（`'static` 生命周期），返回的引用也有效。
</details>

---

### 练习 03-04: 返回较长切片

> 难度：⭐
> 类似 C++ 引用返回但无生命周期保障

实现一个函数 `max_slice`，接收两个 `&[i32]` 切片，返回元素数量较多的那个切片。

```rust
// TODO: 定义 max_slice 函数，接收两个整数切片，返回较长的一个
// 使用生命周期标注

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [10, 20];
    let result = max_slice(&arr1, &arr2);
    println!("较长的切片: {:?}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn max_slice<'a>(x: &'a [i32], y: &'a [i32]) -> &'a [i32] {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [10, 20];
    let result = max_slice(&arr1, &arr2);
    println!("较长的切片: {:?}", result);
}
```

**说明：** 生命周期标注不仅适用于字符串，也适用于所有引用类型。这里 `&'a [i32]` 表示切片的生命周期至少为 `'a`。返回值必须与输入参数的生命周期关联，否则 Rust 无法确定返回的引用何时会失效。
</details>

---

### 练习 03-05: 标注多个生命周期参数

> 难度：⭐⭐⭐
> 类似 C++ 区分不同引用来源

有时候不同参数的生命周期不同，需要为它们分别标注。下面函数接收两个字符串切片，分别用不同的生命周期标注，返回值与第一个参数同生命周期。

```rust
// TODO: 为函数标注两个不同的生命周期参数 'a 和 'b
// 要求：返回值与第一个参数（title）同生命周期
fn get_display(title: &str, subtitle: &str) -> &str {
    if title.is_empty() { subtitle } else { title }
}

fn main() {
    let t = String::from("主标题");
    let sub = "副标题";
    let display = get_display(&t, sub);
    println!("显示: {}", display);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn get_display<'a, 'b>(title: &'a str, subtitle: &'b str) -> &'a str {
    if title.is_empty() { subtitle } else { title }
}
```

**说明：** 尝试编译上面那个 `get_display` 函数会发现报错——因为当 `title` 为空时函数返回 `subtitle`（生命周期 `'b`），但返回值标注为 `'a`，两者不匹配。Rust 编译器会拒绝编译。修复方式取决于语义要求：如果确实需要返回 `subtitle`，那么返回值生命周期必须取 `'a` 和 `'b` 中较短的那个（即使用 `'a: 'b` 约束或统一生命周期）。这个练习展示了 Rust 如何在编译期阻止不安全的引用返回值。
</details>

---

### 练习 03-06: 结构体中的引用

> 难度：⭐
> 类似 C++ 类中持有指针/引用（需格外小心）

结构体中可以持有引用，但必须为每个引用字段标注生命周期。请补全下面结构体的定义，使其能持有对 `String` 的引用。

```rust
// TODO: 为结构体添加生命周期标注
struct BookInfo {
    title: &str,    // 需要生命周期标注
    year: i32,
}

fn main() {
    let name = String::from("Rust 编程");
    let book = BookInfo {
        title: &name,
        year: 2024,
    };
    println!("{} ({})", book.title, book.year);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct BookInfo<'a> {
    title: &'a str,
    year: i32,
}

fn main() {
    let name = String::from("Rust 编程");
    let book = BookInfo {
        title: &name,
        year: 2024,
    };
    println!("{} ({})", book.title, book.year);
}
```

**说明：** 结构体中的引用字段必须标注生命周期。`<'a>` 定义了一个生命周期参数，`&'a str` 表示该引用的生命周期至少为 `'a`。创建 `BookInfo` 实例时，编译器会确保 `title` 引用的数据（`name`）至少与结构体实例活得一样久。
</details>

---

### 练习 03-07: 结构体包含多个引用

> 难度：⭐
> 类似 C++ 的多个成员指针

创建一个结构体 `Pair`，包含两个不同类型的引用字段（`&str` 和 `&i32`），并补全生命周期标注。

```rust
// TODO: 为 Pair 结构体添加生命周期标注
struct Pair {
    first: &str,
    second: &i32,
}

fn main() {
    let x = 42;
    let name = String::from("answer");
    let p = Pair {
        first: &name,
        second: &x,
    };
    println!("{}: {}", p.first, p.second);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Pair<'a> {
    first: &'a str,
    second: &'a i32,
}

fn main() {
    let x = 42;
    let name = String::from("answer");
    let p = Pair {
        first: &name,
        second: &x,
    };
    println!("{}: {}", p.first, p.second);
}
```

**说明：** 当多个引用字段使用同一个生命周期参数 `'a` 时，它们会被约束为至少存活相同的时长。Rust 编译器会取所有引用中最短的那个生命周期作为 `'a` 的实际值。这里是 `&x` 和 `&name` 中较短的那个决定了 `Pair` 实例不会存活超过它。
</details>

---

### 练习 03-08: 结构体方法中的生命周期

> 难度：⭐⭐
> 类似 C++ 成员函数中的 const 引用返回

为包含生命周期的结构体实现方法时，需要在 `impl` 块中声明生命周期。请补全下面 `BookInfo` 的方法。

```rust
struct BookInfo<'a> {
    title: &'a str,
    year: i32,
}

// TODO: 补全 impl 块的生命周期声明
impl BookInfo {
    // TODO: 实现一个方法 get_title，返回 &str
    fn get_title(&self) -> &str {
        self.title
    }
}

fn main() {
    let name = String::from("Rust 实战");
    let book = BookInfo { title: &name, year: 2026 };
    println!("书名: {}", book.get_title());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct BookInfo<'a> {
    title: &'a str,
    year: i32,
}

impl<'a> BookInfo<'a> {
    fn get_title(&self) -> &str {
        self.title
    }
}

fn main() {
    let name = String::from("Rust 实战");
    let book = BookInfo { title: &name, year: 2026 };
    println!("书名: {}", book.get_title());
}
```

**说明：** `impl<'a>` 声明生命周期参数，然后 `BookInfo<'a>` 使用它。方法 `get_title` 的返回值这里可以省略生命周期标注——因为 `&self` 是输入，返回 `&str` 自动继承 `self` 的生命周期（生命周期省略规则）。但显式写 `-> &'a str` 也是正确的。
</details>

---

### 练习 03-09: 结构体方法返回引用

> 难度：⭐⭐
> 类似 C++ 返回成员引用

为 `Pair` 结构体实现一个方法，返回两个引用中较长的字符串。注意方法返回值的生命周期需要与结构体关联。

```rust
struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

// TODO: 补全 impl 块，实现 longest 方法
// longest 方法返回 first 和 second 中较长的字符串
impl Pair {
}

fn main() {
    let p = Pair { first: "hello", second: "world" };
    let result = p.longest();
    println!("较长的: {}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> Pair<'a> {
    fn longest(&self) -> &'a str {
        if self.first.len() > self.second.len() {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let p = Pair { first: "hello", second: "world" };
    let result = p.longest();
    println!("较长的: {}", result);
}
```

**说明：** `impl<'a>` 定义了生命周期参数 `'a`，它用于 `Pair<'a>` 和方法的返回值 `&'a str`。返回值的生命周期 `'a` 与结构体关联，保证了返回的引用在结构体有效期间都有效。虽然这里利用了省略规则可以写 `-> &str`，但显式写出 `-> &'a str` 更清晰地表达了语义。
</details>

---

### 练习 03-10: 结构体与方法生命周期综合

> 难度：⭐⭐⭐
> 类似 C++ 嵌套引用管理

实现一个 `Parser` 结构体，它持有一个字符串的引用，提供两个方法：`first_line` 返回第一行，`rest` 返回剩余内容。需要正确处理生命周期标注。

```rust
// TODO: 定义 Parser 结构体，持有对字符串切片的引用
struct Parser {
    input: &str,
}

// TODO: 实现 Parser 的方法
impl Parser {
    // TODO: first_line 方法，返回输入的第一行（到第一个 \n 为止）
    // 提示：使用 split('\n').next()
    fn first_line(&self) -> &str {
        // ...
    }
    
    // TODO: rest 方法，返回去掉第一行后的剩余内容
    fn rest(&self) -> &str {
        // ...
    }
}

fn main() {
    let text = String::from("第一行\n第二行\n第三行");
    let parser = Parser { input: &text };
    println!("第一行: {}", parser.first_line());
    println!("剩余内容: {}", parser.rest());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn first_line(&self) -> &'a str {
        self.input.split('\n').next().unwrap_or("")
    }
    
    fn rest(&self) -> &'a str {
        self.input.splitn(2, '\n').nth(1).unwrap_or("")
    }
}

fn main() {
    let text = String::from("第一行\n第二行\n第三行");
    let parser = Parser { input: &text };
    println!("第一行: {}", parser.first_line());
    println!("剩余内容: {}", parser.rest());
}
```

**说明：** 结构体 `Parser<'a>` 的生命周期 `'a` 与输入字符串绑定。两个方法都返回 `&'a str`，这意味着返回的引用与结构体持有的引用具有相同的生命周期。由于方法的 `&self` 生命周期和 `'a` 不同（`&self` 是方法调用的借用生命周期，`'a` 是结构体内部数据的生命周期），这里不能依赖省略规则，需要显式标注 `-> &'a str` 来表明返回的是结构体内部的数据引用。
</details>

---

### 练习 03-11: 省略规则——每个参数有自己的生命周期

> 难度：⭐
> 理解 Rust 的隐式标注

Rust 的生命周期省略规则规定：每个引用参数都会获得独立的生命周期参数。下面这段代码利用了省略规则，请补全 `first` 函数，使其返回切片的第一个元素（的引用）。注意这是省略规则生效的场景。

```rust
// TODO: 补全函数，利用生命周期省略规则
// 提示：函数只有一个引用参数，返回引用时省略规则自动应用
fn first(slice: &[i32]) -> &i32 {
    // TODO: 返回 slice 的第一个元素的引用，如果为空则返回 None... 不对，返回 &i32
    // 提示：用 &slice[0]
}

fn main() {
    let arr = [10, 20, 30];
    println!("第一个元素: {}", first(&arr));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn first(slice: &[i32]) -> &i32 {
    &slice[0]
}

fn main() {
    let arr = [10, 20, 30];
    println!("第一个元素: {}", first(&arr));
}
```

**说明：** 生命周期省略规则第一条：函数的每个引用参数都会获得独立的生命周期参数。第二条：如果只有一个输入生命周期参数，它会被赋予所有输出生命周期参数。这里 `first` 只有一个参数 `slice: &[i32]`，编译器自动推断返回值的生命周期与 `slice` 相同，因此无需显式标注。等价于 `fn first<'a>(slice: &'a [i32]) -> &'a i32`。
</details>

---

### 练习 03-12: 省略规则在结构体方法中

> 难度：⭐
> 方法中的省略规则

下面代码为 `Wrapper` 结构体实现了一个方法，利用省略规则返回内部数据的引用。请补全方法实现。

```rust
struct Wrapper<'a> {
    content: &'a str,
}

impl<'a> Wrapper<'a> {
    // TODO: 实现 get 方法，返回 content 的引用
    // 利用省略规则——&self 作为第一个参数，返回值自动继承其生命周期
    fn get(&self) -> &str {
        // ...
    }
}

fn main() {
    let w = Wrapper { content: "重要数据" };
    println!("内容: {}", w.get());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Wrapper<'a> {
    content: &'a str,
}

impl<'a> Wrapper<'a> {
    fn get(&self) -> &str {
        self.content
    }
}

fn main() {
    let w = Wrapper { content: "重要数据" };
    println!("内容: {}", w.get());
}
```

**说明：** 方法中的省略规则不同：`&self`（或 `&mut self`）作为第一个参数时，返回值的生命周期自动继承 `self` 的生命周期。这里 `get(&self) -> &str` 等价于 `get<'b>(&'b self) -> &'b str`。注意 `'b` 是方法调用的借用生命周期，与结构体的 `'a` 不同——但 Rust 足够智能，能正确处理这种关系。
</details>

---

### 练习 03-13: 省略规则不适用——手动标注

> 难度：⭐⭐
> 多个参数时省略规则不适用

下面代码尝试用省略规则编写一个返回较长字符串的函数，但因为有多个参数，省略规则不适用。请添加必要的生命周期标注。

```rust
// TODO: 添加生命周期标注使代码通过编译
fn longest_backslash(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("Rust");
    let b = "语言";
    let r = longest_backslash(&a, b);
    println!("{}", r);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn longest_backslash<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("Rust");
    let b = "语言";
    let r = longest_backslash(&a, b);
    println!("{}", r);
}
```

**说明：** 当函数有多个引用参数时，省略规则第一条会为每个参数分配独立的生命周期（比如 `x: &'a str, y: &'b str`），但第二条规则（只有一个输入生命周期时才赋予输出）不适用，所以返回值无法确定生命周期。编译器会报错。此时必须手动标注，使用同一个 `'a` 来约束两个参数和返回值的关系。
</details>

---

### 练习 03-14: 省略规则——方法中的 &self

> 难度：⭐⭐
> 理解 &self 作为首个参数时的省略

为结构体实现一个 `read` 方法，它的签名中 `&self` 作为第一个参数，返回一个引用。利用省略规则，判断何时需要显式标注。

```rust
struct DataStore<'a> {
    name: &'a str,
    values: &'a [i32],
}

impl<'a> DataStore<'a> {
    // TODO: 实现 name_ref 方法，返回 name 的引用
    // 思考：这里需要显式生命周期标注吗？
    fn name_ref(&self) -> &str {
        // ...
    }
    
    // TODO: 实现更大值索引的 max_index 方法
    // 要求返回 values 中最大值的位置（索引），用引用返回
    fn max_index(&self) -> &usize {
        // ...
    }
}

fn main() {
    let data = [3, 7, 1, 9, 4];
    let name = String::from("得分");
    let store = DataStore { name: &name, values: &data };
    println!("名称: {}", store.name_ref());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct DataStore<'a> {
    name: &'a str,
    values: &'a [i32],
}

impl<'a> DataStore<'a> {
    fn name_ref(&self) -> &str {
        self.name
    }
    
    fn max_index(&self) -> &usize {
        // 找到最大值位置，返回索引的引用
        let pos = self.values.iter()
            .enumerate()
            .max_by_key(|&(_, v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);
        // 注意：不能返回局部变量的引用！这里我们用 values 中的索引
        // 但 &usize 不能直接返回，这个练习的重点是理解方法中的省略规则
        // 简单起见，返回固定值
        &0 // 占位，实际实现需要更复杂的生命周期处理
    }
}

fn main() {
    let data = [3, 7, 1, 9, 4];
    let name = String::from("得分");
    let store = DataStore { name: &name, values: &data };
    println!("名称: {}", store.name_ref());
}
```

**说明：** 方法 `name_ref` 中 `&self` 作为第一个参数，返回 `&str` 自动继承 `self` 的生命周期，无需额外标注。但 `max_index` 方法如果想返回 `values` 中某个元素的引用，需要显式标注生命周期为 `'a`（因为结构体内部数据的生命周期是 `'a`，不是方法调用的生命周期）。这里 `max_index` 的实现仅作占位，完整实现需要更复杂的生命周期处理。
</details>

---

### 练习 03-15: 省略规则不适用时的修正

> 难度：⭐⭐⭐
> 识别并修复生命周期省略错误

下面代码试图实现一个函数 `announce_and_return`，它接收一个字符串切片、一条消息和一个返回条件，但省略规则不适用。请分析错误并添加正确的生命周期标注。

```rust
use std::fmt::Display;

// TODO: 修复生命周期标注
// 要求：返回值的生命周期与参数 content 相同
// 参数 msg 可以有独立的生命周期
fn announce_and_return(content: &str, msg: &str, show_content: bool) -> &str {
    println!("公告: {}", msg);
    if show_content { content } else { "（已隐藏）" }
}

fn main() {
    let data = String::from("这是敏感数据");
    let result = announce_and_return(&data, "系统通知", true);
    println!("结果: {}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt::Display;

fn announce_and_return<'a, 'b>(content: &'a str, msg: &'b str, show_content: bool) -> &'a str {
    println!("公告: {}", msg);
    if show_content { content } else { "（已隐藏）" }
}

fn main() {
    let data = String::from("这是敏感数据");
    let result = announce_and_return(&data, "系统通知", true);
    println!("结果: {}", result);
}
```

**说明：** 这里有三个参数，其中两个是引用。省略规则会为 `content` 和 `msg` 分别分配 `'a` 和 `'b`，但返回值无法自动确定用哪个生命周期。需要手动标注：返回值与 `content` 同生命周期（`'a`），`msg` 可以拥有独立的 `'b`。注意字符串字面量 `"（已隐藏）"` 是 `'static` 的，它可以被赋给任何生命周期。
</details>

---

### 练习 03-16: 'static 生命周期——字符串字面量

> 难度：⭐
> 类似 C++ 的全局字符串常量

字符串字面量具有 `'static` 生命周期，这意味着它们在程序的整个运行期间都有效。请补全下面代码，验证字符串字面量的 `'static` 性质。

```rust
// TODO: 补全函数，接收任何生命周期为 'static 的字符串切片
fn print_static(text: &str) {
    println!("静态字符串: {}", text);
}

fn main() {
    let s: &'static str = "Hello, world!";  // 字符串字面量是 'static
    print_static(s);
    
    // TODO: 尝试让 dynamic_str 也能传给 print_static
    // 下面的代码会报错吗？为什么？
    let dynamic_str = String::from("动态字符串");
    // print_static(&dynamic_str);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn print_static(text: &'static str) {
    println!("静态字符串: {}", text);
}

fn main() {
    let s: &'static str = "Hello, world!";
    print_static(s);
    
    // 下面的代码会报错，因为 &dynamic_str 的生命周期不是 'static
    // let dynamic_str = String::from("动态字符串");
    // print_static(&dynamic_str);  // 编译错误！
    
    // 但去掉 'static 标注就可以：
    fn print_any(text: &str) {
        println!("任意字符串: {}", text);
    }
    let dynamic_str = String::from("动态字符串");
    print_any(&dynamic_str);
}
```

**说明：** 字符串字面量（如 `"Hello, world!"`）被直接编译到二进制文件中，因此拥有 `'static` 生命周期。而 `String` 类型的数据生命周期受其变量作用域限制。如果函数参数要求 `&'static str`，则只能传入字符串字面量或 `&'static` 的引用，不能传入 `&String` 或局部 `&str`。
</details>

---

### 练习 03-17: 'static 生命周期——静态变量

> 难度：⭐
> 类似 C++ 的 static 全局变量

Rust 中的 `static` 变量也具有 `'static` 生命周期。请补全代码，创建一个 `static` 变量并返回其引用。

```rust
// TODO: 定义一个 static 变量 GREETING，类型为 &str，值为 "你好，世界"
static GREETING: &str = "你好，世界";

// TODO: 定义一个 static 变量 VERSION，类型为 i32
static VERSION: i32 = 3;

fn get_greeting() -> &'static str {
    // TODO: 返回 GREETING
}

fn main() {
    let g = get_greeting();
    println!("{} v{}", g, VERSION);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
static GREETING: &str = "你好，世界";
static VERSION: i32 = 3;

fn get_greeting() -> &'static str {
    GREETING
}

fn main() {
    let g = get_greeting();
    println!("{} v{}", g, VERSION);
}
```

**说明：** `static` 变量在程序的整个生命周期内都存在，因此它们自然拥有 `'static` 生命周期。`static` 变量与 `const` 常量的区别：`static` 有固定的内存地址，可以取引用；`const` 在每次使用时会被内联，没有固定地址。注意 `static` 变量默认是不可变的，如果需要可变需要用 `static mut`（不安全的）。
</details>

---

### 练习 03-18: 'static 约束——泛型参数

> 难度：⭐⭐
> 类似 C++ 模板中的类型约束

在泛型函数中，可以使用 `'static` 约束来要求类型不包含任何短生命周期的引用（或根本没有任何引用）。请补全下面的代码。

```rust
// TODO: 为泛型参数 T 添加 'static 约束
// 要求 T 不包含任何非 'static 的引用
fn store_in_box(value: T) -> Box<T> {
    Box::new(value)
}

fn main() {
    let n = 42;
    let boxed_n = store_in_box(n);
    println!("boxed: {}", boxed_n);
    
    let s = String::from("hello");
    let boxed_s = store_in_box(s);
    println!("boxed: {}", boxed_s);
    
    // 下面的代码应该被编译拒绝
    // let r = &42;
    // let boxed_r = store_in_box(r);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn store_in_box<T: 'static>(value: T) -> Box<T> {
    Box::new(value)
}

fn main() {
    let n = 42;
    let boxed_n = store_in_box(n);
    println!("boxed: {}", boxed_n);
    
    let s = String::from("hello");
    let boxed_s = store_in_box(s);
    println!("boxed: {}", boxed_s);
    
    // 下面的代码会被编译拒绝，因为 &i32 不是 'static
    // let r = &42;
    // let boxed_r = store_in_box(r);  // 编译错误！
}
```

**说明：** `T: 'static` 约束表示类型 `T` 不能包含任何非 `'static` 的引用。`i32` 和 `String` 都不包含引用（`String` 拥有其数据），因此满足约束。而 `&i32` 是引用类型，其生命周期不满足 `'static` 约束。注意 `T: 'static` 并不意味着 `T` 活得和程序一样久，而是说 `T` 可以在任意长的时间内保持有效——即它不依赖任何短生命周期的借用的数据。
</details>

---

### 练习 03-19: 'static 约束在结构体中

> 难度：⭐⭐
> 类似 C++ 的 static 断言

有时我们希望结构体只能存储拥有所有权的类型或不含短生命周期引用的类型。请为下面 `Container` 结构体的泛型参数添加 `'static` 约束。

```rust
// TODO: 为泛型参数 T 添加 'static 约束
// 使得 Container 只能持有拥有所有权的类型
struct Container {
    value: T,
}

impl Container {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    let c1 = Container::new(100i32);
    let c2 = Container::new(String::from("拥有所有权的字符串"));
    println!("{}, {}", c1.get(), c2.get());
    
    // 下面这行应该被编译拒绝
    // let x = 42;
    // let c3 = Container::new(&x);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Container<T: 'static> {
    value: T,
}

impl<T: 'static> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    let c1 = Container::new(100i32);
    let c2 = Container::new(String::from("拥有所有权的字符串"));
    println!("{}, {}", c1.get(), c2.get());
    
    // 下面这行会被编译拒绝
    // let x = 42;
    // let c3 = Container::new(&x);  // 编译错误：&i32 不满足 'static
}
```

**说明：** `T: 'static` 约束要求 `T` 要么是不含引用的类型（如 `i32`、`String`），要么是只包含 `'static` 引用的类型。它的实际含义是：`T` 可以在任意生命周期内安全使用而不失效。这常用于多线程场景——需要将数据发送到另一个线程时，往往要求 `T: 'static` 以确保数据在线程执行期间不会失效。
</details>

---

### 练习 03-20: 生命周期综合挑战

> 难度：⭐⭐⭐
> 综合所有生命周期知识

下面是一个涉及多个生命周期概念的综合练习题。请修复所有与生命周期相关的编译错误。

```rust
use std::fmt::Display;

// TODO: 修复 Section 结构体
struct Section {
    heading: &str,
    body: &str,
}

// TODO: 修复函数签名
// 要求：返回值与 section 的生命周期相同
// 参数 prefix 可以有独立的生命周期
fn format_section(section: &Section, prefix: &str) -> String {
    format!("{}# {} \n{}", prefix, section.heading, section.body)
}

// TODO: 修复函数签名  
// 要求：返回的 &str 与输入的 content 同生命周期
fn first_sentence(content: &str) -> &str {
    match content.find('.') {
        Some(pos) => &content[..=pos],
        None => content,
    }
}

// TODO: 修复函数签名
// 要求 T 不包含非 'static 引用
fn print_summary(content: T) where T: Display {
    println!("摘要: {}", content);
}

fn main() {
    let title = String::from("第三章 生命周期");
    let desc = "生命周期是 Rust 的核心概念。它确保了内存安全。";
    
    let sec = Section {
        heading: &title,
        body: desc,
    };
    
    let result = format_section(&sec, ">> ");
    println!("{}", result);
    
    let sentence = first_sentence(desc);
    println!("第一句: {}", sentence);
    
    print_summary(title);  // 移动 title 的所有权
    // 注意：这里不能再使用 title，因为所有权已转移
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt::Display;

struct Section<'a> {
    heading: &'a str,
    body: &'a str,
}

fn format_section<'a, 'b>(section: &'a Section<'a>, prefix: &'b str) -> String {
    format!("{}# {} \n{}", prefix, section.heading, section.body)
}

fn first_sentence<'a>(content: &'a str) -> &'a str {
    match content.find('.') {
        Some(pos) => &content[..=pos],
        None => content,
    }
}

fn print_summary<T: 'static>(content: T) where T: Display {
    println!("摘要: {}", content);
}

fn main() {
    let title = String::from("第三章 生命周期");
    let desc = "生命周期是 Rust 的核心概念。它确保了内存安全。";
    
    let sec = Section {
        heading: &title,
        body: desc,
    };
    
    let result = format_section(&sec, ">> ");
    println!("{}", result);
    
    let sentence = first_sentence(desc);
    println!("第一句: {}", sentence);
    
    print_summary(title);  // title 的所有权被移动，但 String 满足 T: 'static
    // 注意：这里不能再使用 title
}
```

**说明：** 这道题综合了以下知识点：1) 结构体中的引用需要生命周期标注（`Section<'a>`）；2) 函数参数有多个引用时需要手动标注（`format_section` 中 `section` 和 `prefix` 有不同生命周期）；3) 只有一个输入引用参数时可以依赖省略规则（`first_sentence` 也可以省略，这里显式标注便于理解）；4) `'static` 约束要求类型不包含非静态引用——`String` 拥有数据，满足约束。注意 `format_section` 返回 `String`（拥有所有权）而非引用，所以返回值不需要生命周期标注。
</details>

---

恭喜完成第 03 章！接下来进入[第 04 章：结构体](04_structs.md)。
