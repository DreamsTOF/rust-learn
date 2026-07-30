# 07 Trait

Trait 是 Rust 中定义共享行为的方式，类似于其他语言中的接口（interface）或抽象类。本章练习涵盖 trait 的定义与实现、默认方法、trait 继承（super trait）、关联类型、trait 对象（dyn Trait）以及常用标准库 trait。

### 练习 07-01: 定义并实现一个简单的 Trait

> 难度：⭐
> 类似 Java 的 interface，但 trait 可以包含默认实现

定义一个 `Describe` trait，包含一个 `describe` 方法。然后为 `Person` 结构体实现该 trait。

```rust
// TODO: 定义 Describe trait，包含 describe(&self) -> String 方法

struct Person {
    name: String,
    age: u32,
}

// TODO: 为 Person 实现 Describe

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{}", person.describe());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u32,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{}", person.describe());
}
```

**说明：** trait 使用 `trait` 关键字定义，只声明方法签名。使用 `impl Trait for Type` 语法为特定类型实现 trait。方法中通过 `&self` 访问实例数据。
</details>

### 练习 07-02: 为多种类型实现同一个 Trait

> 难度：⭐
> 类似 Java 中多个类实现同一个接口，每个类有自己的实现

补全代码，分别为 `Circle` 和 `Triangle` 实现 `Area` trait，计算各自的面积。

```rust
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

// TODO: 为 Circle 实现 Area（面积 = π * r²）

struct Triangle {
    base: f64,
    height: f64,
}

// TODO: 为 Triangle 实现 Area（面积 = 底 × 高 / 2）

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 10.0, height: 4.0 };
    println!("Circle area: {:.2}", circle.area());
    println!("Triangle area: {:.2}", triangle.area());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 10.0, height: 4.0 };
    println!("Circle area: {:.2}", circle.area());
    println!("Triangle area: {:.2}", triangle.area());
}
```

**说明：** 同一个 trait 可以为任意多个不同类型实现，每个类型有自己的实现逻辑。调用时通过实例的 `.` 语法调用对应方法。
</details>

### 练习 07-03: Trait 作为参数

> 难度：⭐⭐
> 类似 Java 的接口类型作为方法参数，Rust 使用 impl Trait 或泛型约束

编写一个函数 `print_info`，接受任何实现了 `Describe` trait 的参数，并调用其 `describe` 方法。

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
    author: String,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("《{}》 by {}", self.title, self.author)
    }
}

struct Movie {
    name: String,
    year: u32,
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("{} ({})", self.name, self.year)
    }
}

// TODO: 使用 impl Trait 语法编写 print_info 函数

fn main() {
    let book = Book {
        title: String::from("Rust 程序设计"),
        author: String::from("张三"),
    };
    let movie = Movie {
        name: String::from("星际穿越"),
        year: 2014,
    };
    print_info(&book);
    print_info(&movie);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
    author: String,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("《{}》 by {}", self.title, self.author)
    }
}

struct Movie {
    name: String,
    year: u32,
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("{} ({})", self.name, self.year)
    }
}

fn print_info(item: &impl Describe) {
    println!("{}", item.describe());
}

fn main() {
    let book = Book {
        title: String::from("Rust 程序设计"),
        author: String::from("张三"),
    };
    let movie = Movie {
        name: String::from("星际穿越"),
        year: 2014,
    };
    print_info(&book);
    print_info(&movie);
}
```

**说明：** `&impl Describe` 是 trait 作为参数的语法糖，等价于泛型约束 `fn print_info<T: Describe>(item: &T)`。它表示函数接受任何实现了 `Describe` 的类型的引用。
</details>

### 练习 07-04: 泛型函数与 Trait 约束

> 难度：⭐⭐
> 类似 Java 的 `T extends Interface` 泛型边界，Rust 使用冒号语法

补全泛型函数 `largest`，使其能返回两个值中较大的一个。要求使用 `T: PartialOrd` 约束。

```rust
// TODO: 编写泛型函数 largest，返回两个参数中较大的值

fn main() {
    let a = 10;
    let b = 20;
    println!("largest({}, {}) = {}", a, b, largest(&a, &b));

    let x = 3.14;
    let y = 2.72;
    println!("largest({:.2}, {:.2}) = {:.2}", x, y, largest(&x, &y));

    // 字符串比较
    let s1 = String::from("apple");
    let s2 = String::from("orange");
    println!("largest({}, {}) = {}", s1, s2, largest(&s1, &s2));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn largest<T: PartialOrd>(a: &T, b: &T) -> &T {
    if a >= b { a } else { b }
}

fn main() {
    let a = 10;
    let b = 20;
    println!("largest({}, {}) = {}", a, b, largest(&a, &b));

    let x = 3.14;
    let y = 2.72;
    println!("largest({:.2}, {:.2}) = {:.2}", x, y, largest(&x, &y));

    let s1 = String::from("apple");
    let s2 = String::from("orange");
    println!("largest({}, {}) = {}", s1, s2, largest(&s1, &s2));
}
```

**说明：** `T: PartialEq` 是对类型参数 T 的 trait 约束，要求 T 必须实现 `PartialOrd` trait（即可比较大小）。`PartialOrd` 提供了 `>=`、`>` 等比较运算符。
</details>

### 练习 07-05: 多重 Trait 约束

> 难度：⭐⭐⭐
> 类似 Java 的 `<T extends Interface1 & Interface2>`，Rust 使用 `+` 连接多个约束

编写一个函数 `summarize`，要求参数同时实现 `Display` 和 `Summarizable` trait（自定），并返回格式化的字符串。

```rust
use std::fmt::Display;

trait Summarizable {
    fn summary(&self) -> String;
}

struct Weather {
    temperature: f64,
    condition: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C, {}", self.temperature, self.condition)
    }
}

impl Summarizable for Weather {
    fn summary(&self) -> String {
        format!("Weather: {}", self.condition)
    }
}

// TODO: 编写函数 summarize，要求 T 同时实现 Display 和 Summarizable
// 返回格式: "Summary: [summary] | Details: [display]"
// fn summarize(item: /* ??? */) -> String {

fn main() {
    let weather = Weather {
        temperature: 25.5,
        condition: String::from("晴"),
    };
    println!("{}", summarize(&weather));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt::Display;

trait Summarizable {
    fn summary(&self) -> String;
}

struct Weather {
    temperature: f64,
    condition: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C, {}", self.temperature, self.condition)
    }
}

impl Summarizable for Weather {
    fn summary(&self) -> String {
        format!("Weather: {}", self.condition)
    }
}

fn summarize(item: &(impl Display + Summarizable)) -> String {
    format!("Summary: {} | Details: {}", item.summary(), item)
}

fn main() {
    let weather = Weather {
        temperature: 25.5,
        condition: String::from("晴"),
    };
    println!("{}", summarize(&weather));
}
```

**说明：** `impl Display + Summarizable` 表示参数必须同时实现 `Display` 和 `Summarizable` 两个 trait。多重约束的泛型版本写作 `fn summarize<T: Display + Summarizable>(item: &T)`，也可以使用 `where` 子句。
</details>

### 练习 07-06: 带有默认实现的 Trait

> 难度：⭐
> 类似 Java 的 default 方法，Rust trait 可以直接提供方法体

定义一个 `Greeter` trait，包含一个默认实现的 `greet` 方法（输出 "Hello!"）和一个需要手动实现的 `name` 方法。让 `Person` 只需实现 `name` 即可。

```rust
trait Greeter {
    // TODO: 定义 name(&self) -> &str（无默认实现）
    // TODO: 定义 greet(&self)（默认实现：打印 "Hello, {name}!"）
}

struct Person {
    name: String,
}

// TODO: 为 Person 实现 Greeter（只需实现 name 方法）

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Greeter {
    fn name(&self) -> &str;

    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}

struct Person {
    name: String,
}

impl Greeter for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.greet();
}
```

**说明：** trait 中的方法可以有默认实现。实现 trait 时可以选择覆盖默认方法，也可以直接使用默认实现。默认实现中可以调用没有默认实现的方法（如 `name`），这要求实现者必须提供这些方法。
</details>

### 练习 07-07: 覆盖默认实现

> 难度：⭐
> 类似 Java 的子类重写父类方法，Rust 中实现者可以覆盖 trait 的默认方法

`Warning` trait 定义了默认的 `warn` 方法。分别为 `LowBattery` 和 `Overheat` 实现该 trait，其中 `Overheat` 需要覆盖默认实现以发出更紧急的警告。

```rust
trait Warning {
    fn warn(&self) -> String {
        String::from("Warning: 发生了异常情况")
    }
}

struct LowBattery {
    percentage: u8,
}

// TODO: 为 LowBattery 实现 Warning，使用默认实现

struct Overheat {
    temperature: f64,
}

// TODO: 为 Overheat 实现 Warning，覆盖默认实现
// 返回格式: "⚠️ 紧急警告：温度 {temperature}°C 过高！"

fn main() {
    let battery = LowBattery { percentage: 15 };
    let heat = Overheat { temperature: 95.0 };
    println!("{}", battery.warn());
    println!("{}", heat.warn());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Warning {
    fn warn(&self) -> String {
        String::from("Warning: 发生了异常情况")
    }
}

struct LowBattery {
    percentage: u8,
}

impl Warning for LowBattery {}

struct Overheat {
    temperature: f64,
}

impl Warning for Overheat {
    fn warn(&self) -> String {
        format!("⚠️ 紧急警告：温度 {}°C 过高！", self.temperature)
    }
}

fn main() {
    let battery = LowBattery { percentage: 15 };
    let heat = Overheat { temperature: 95.0 };
    println!("{}", battery.warn());
    println!("{}", heat.warn());
}
```

**说明：** 使用默认实现时，`impl Warning for Type` 块可以为空。需要自定义行为时，在 `impl` 块中重新实现该方法即可覆盖默认版本。
</details>

### 练习 07-08: Trait 继承（Super Trait）

> 难度：⭐⭐
> 类似 Java 的 interface extends interface，Rust 使用 `trait B: A` 表示 B 继承 A

定义一个 `Printable` trait（包含 `print` 方法），再定义一个继承自 `Printable` 的 `Renderable` trait（增加 `render` 方法）。为 `Document` 类型实现 `Renderable`。

```rust
// TODO: 定义 Printable trait，包含 print(&self) 方法

// TODO: 定义 Renderable trait，继承 Printable，增加 render(&self) 方法

struct Document {
    title: String,
    content: String,
}

// TODO: 为 Document 实现 Renderable（需要同时满足 Printable 和 Renderable）

fn main() {
    let doc = Document {
        title: String::from("Rust 入门"),
        content: String::from("Rust 是一门系统编程语言……"),
    };
    doc.print();
    doc.render();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Printable {
    fn print(&self);
}

trait Renderable: Printable {
    fn render(&self);
}

struct Document {
    title: String,
    content: String,
}

impl Renderable for Document {
    fn print(&self) {
        println!("[{}] {}", self.title, self.content);
    }

    fn render(&self) {
        println!("<h1>{}</h1><p>{}</p>", self.title, self.content);
    }
}

fn main() {
    let doc = Document {
        title: String::from("Rust 入门"),
        content: String::from("Rust 是一门系统编程语言……"),
    };
    doc.print();
    doc.render();
}
```

**说明：** `trait Renderable: Printable` 表示 `Renderable` 是 `Printable` 的 super trait。实现 `Renderable` 时必须同时实现 `Printable` 的所有方法。任何实现了 `Renderable` 的类型也一定实现了 `Printable`。
</details>

### 练习 07-09: Super Trait 约束

> 难度：⭐⭐
> 类似 Java 中某个接口要求实现者必须实现另一个接口

编写一个函数 `render_and_print`，接受一个实现了 `Renderable` 的参数（`Renderable` 继承自 `Printable`），依次调用 `render` 和 `print`。

```rust
trait Printable {
    fn print(&self);
}

trait Renderable: Printable {
    fn render(&self);
}

struct Article {
    headline: String,
    body: String,
}

impl Printable for Article {
    fn print(&self) {
        println!("头条: {}", self.headline);
    }
}

impl Renderable for Article {
    fn render(&self) {
        println!("<article><h1>{}</h1><p>{}</p></article>", self.headline, self.body);
    }
}

// TODO: 编写函数 render_and_print，接受任何实现了 Renderable 的类型
// 先调用 render()，再调用 print()

fn main() {
    let article = Article {
        headline: String::from("Rust 2026 发布"),
        body: String::from("新版本引入了多项改进……"),
    };
    render_and_print(&article);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Printable {
    fn print(&self);
}

trait Renderable: Printable {
    fn render(&self);
}

struct Article {
    headline: String,
    body: String,
}

impl Printable for Article {
    fn print(&self) {
        println!("头条: {}", self.headline);
    }
}

impl Renderable for Article {
    fn render(&self) {
        println!("<article><h1>{}</h1><p>{}</p></article>", self.headline, self.body);
    }
}

fn render_and_print(item: &impl Renderable) {
    item.render();
    item.print();
}

fn main() {
    let article = Article {
        headline: String::from("Rust 2026 发布"),
        body: String::from("新版本引入了多项改进……"),
    };
    render_and_print(&article);
}
```

**说明：** 由于 `Renderable: Printable`，函数参数 `&impl Renderable` 隐含了 `Printable` 约束，因此可以直接调用 `print` 和 `render` 方法，无需额外指定 `Printable` 约束。
</details>

### 练习 07-10: 标准库 Trait 作为 Super Trait

> 难度：⭐⭐⭐
> Rust 中 trait 可以继承标准库 trait，如 Display + Clone 组合

定义一个 `Loggable` trait，要求继承 `std::fmt::Display`。实现一个 `log` 方法，输出格式为 `[LOG] <display内容>`。为 `Event` 结构体实现 `Loggable`。

```rust
use std::fmt::Display;

// TODO: 定义 Loggable trait，继承 Display，添加 log(&self) 方法
// 默认实现：打印 "[LOG] {}"

struct Event {
    name: String,
    timestamp: u64,
}

// TODO: 为 Event 实现 Display（显示为 "Event: {name} @ {timestamp}"）
// TODO: 为 Event 实现 Loggable（log 使用默认实现）

fn main() {
    let event = Event {
        name: String::from("登录成功"),
        timestamp: 1700000000,
    };
    println!("{}", event);    // 使用 Display
    event.log();              // 使用 Loggable
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt::Display;

trait Loggable: Display {
    fn log(&self) {
        println!("[LOG] {}", self);
    }
}

struct Event {
    name: String,
    timestamp: u64,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Event: {} @ {}", self.name, self.timestamp)
    }
}

impl Loggable for Event {}

fn main() {
    let event = Event {
        name: String::from("登录成功"),
        timestamp: 1700000000,
    };
    println!("{}", event);
    event.log();
}
```

**说明：** `trait Loggable: Display` 意味着只有实现了 `Display` 的类型才能实现 `Loggable`。在 `Loggable` 的默认实现中可以直接调用 `self` 的 `Display` 格式化（`{}`），因为编译器知道 `Self: Display`。
</details>

### 练习 07-11: 关联类型基础

> 难度：⭐
> 类似 Java 的泛型接口，但关联类型在实现时只需指定一次类型

定义一个 `Container` trait，包含一个关联类型 `Item` 和两个方法 `get` 与 `set`。为 `Box` 结构体实现该 trait。

```rust
// TODO: 定义 Container trait，包含关联类型 Item
// 方法: get(&self) -> Option<&Item> 和 set(&mut self, item: Item)

struct Box<T> {
    value: Option<T>,
}

// TODO: 为 Box<String> 实现 Container，Item = String

fn main() {
    let mut my_box = Box { value: None };
    my_box.set(String::from("hello"));
    if let Some(item) = my_box.get() {
        println!("Box 内容: {}", item);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
    fn set(&mut self, item: Self::Item);
}

struct Box<T> {
    value: Option<T>,
}

impl Container for Box<String> {
    type Item = String;

    fn get(&self) -> Option<&Self::Item> {
        self.value.as_ref()
    }

    fn set(&mut self, item: Self::Item) {
        self.value = Some(item);
    }
}

fn main() {
    let mut my_box = Box { value: None };
    my_box.set(String::from("hello"));
    if let Some(item) = my_box.get() {
        println!("Box 内容: {}", item);
    }
}
```

**说明：** 关联类型使用 `type` 关键字在 trait 中声明。实现 trait 时用 `type Item = ...` 指定具体类型。关联类型与泛型参数的区别在于：一个类型只能为 trait 的关联类型指定一个具体类型（一对一关系）。
</details>

### 练习 07-12: 使用关联类型

> 难度：⭐
> 关联类型常用于迭代器、集合等场景，类似 Java 集合框架的 Iterator

定义一个 `Collection` trait，包含关联类型 `Item` 和方法 `len`、`get`。为 `Vec<i32>` 包装器实现该 trait（或使用自定义 `NumberList` 结构体）。

```rust
trait Collection {
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct NumberList {
    numbers: Vec<i32>,
}

// TODO: 为 NumberList 实现 Collection trait
// Item = i32, len 返回 numbers 长度, get 返回索引处的元素引用

fn main() {
    let list = NumberList {
        numbers: vec![10, 20, 30, 40, 50],
    };
    println!("长度: {}", list.len());
    println!("索引 2: {:?}", list.get(2));
    println!("索引 10: {:?}", list.get(10));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Collection {
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct NumberList {
    numbers: Vec<i32>,
}

impl Collection for NumberList {
    type Item = i32;

    fn len(&self) -> usize {
        self.numbers.len()
    }

    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.numbers.get(index)
    }
}

fn main() {
    let list = NumberList {
        numbers: vec![10, 20, 30, 40, 50],
    };
    println!("长度: {}", list.len());
    println!("索引 2: {:?}", list.get(2));
    println!("索引 10: {:?}", list.get(10));
}
```

**说明：** `Collection` trait 通过关联类型 `Item` 定义了集合中元素的类型。实现者指定 `type Item = i32` 后，方法签名中的 `Self::Item` 就被替换为 `i32`。`Option<&Self::Item>` 返回引用以避免所有权转移。
</details>

### 练习 07-13: 关联类型与泛型对比

> 难度：⭐⭐
> 关联类型 vs 泛型：关联类型确保每个类型只有一种实现，泛型允许多种

补全代码，使用关联类型定义一个 `Converter` trait，将一种类型转换为另一种。对比泛型版本和关联类型版本的区别。

```rust
// 关联类型版本：一个类型只能有一种 Output
trait Converter {
    type Output;
    fn convert(&self) -> Self::Output;
}

struct ToString;

// TODO: 为 ToString 实现 Converter，关联类型 Output = String
// convert 返回 String::from("converted")

struct ToBytes;

// TODO: 为 ToBytes 实现 Converter，关联类型 Output = Vec<u8>
// convert 返回 b"converted".to_vec()

fn main() {
    let s = ToString;
    let b = ToBytes;
    let s_result: String = s.convert();
    let b_result: Vec<u8> = b.convert();
    println!("String: {}", s_result);
    println!("Bytes: {:?}", b_result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Converter {
    type Output;
    fn convert(&self) -> Self::Output;
}

struct ToString;

impl Converter for ToString {
    type Output = String;
    fn convert(&self) -> Self::Output {
        String::from("converted")
    }
}

struct ToBytes;

impl Converter for ToBytes {
    type Output = Vec<u8>;
    fn convert(&self) -> Self::Output {
        b"converted".to_vec()
    }
}

fn main() {
    let s = ToString;
    let b = ToBytes;
    let s_result: String = s.convert();
    let b_result: Vec<u8> = b.convert();
    println!("String: {}", s_result);
    println!("Bytes: {:?}", b_result);
}
```

**说明：** 关联类型将输出类型与实现绑定——每个实现只能有一个 `Output`。相比之下，泛型 trait（如 `trait Converter<Output>`）允许同一类型实现多次（每次指定不同的 `Output`）。关联类型更简洁，适用于"一个类型对应一种实现"的场景。
</details>

### 练习 07-14: 带关联类型的 Trait 作为约束

> 难度：⭐⭐
> 使用关联类型时，函数约束需要指定关联类型的具体类型

编写一个函数 `process_items`，接受任何实现了 `Collection`（关联类型 `Item = i32`）的类型，并计算所有元素的和。

```rust
trait Collection {
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct NumberList {
    numbers: Vec<i32>,
}

impl Collection for NumberList {
    type Item = i32;
    fn len(&self) -> usize { self.numbers.len() }
    fn get(&self, index: usize) -> Option<&Self::Item> { self.numbers.get(index) }
}

// TODO: 编写函数 sum_items，约束 Collection<Item = i32>
// 遍历所有元素并求和

fn main() {
    let list = NumberList {
        numbers: vec![10, 20, 30, 40, 50],
    };
    println!("总和: {}", sum_items(&list));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Collection {
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct NumberList {
    numbers: Vec<i32>,
}

impl Collection for NumberList {
    type Item = i32;
    fn len(&self) -> usize { self.numbers.len() }
    fn get(&self, index: usize) -> Option<&Self::Item> { self.numbers.get(index) }
}

fn sum_items(c: &impl Collection<Item = i32>) -> i32 {
    let mut sum = 0i32;
    for i in 0..c.len() {
        if let Some(val) = c.get(i) {
            sum += val;
        }
    }
    sum
}

fn main() {
    let list = NumberList {
        numbers: vec![10, 20, 30, 40, 50],
    };
    println!("总和: {}", sum_items(&list));
}
```

**说明：** `impl Collection<Item = i32>` 约束参数必须实现 `Collection` 且其关联类型 `Item` 必须等于 `i32`。这样函数内部就可以安全地使用 `i32` 的算术操作。
</details>

### 练习 07-15: 迭代器模式（关联类型实战）

> 难度：⭐⭐⭐
> 类似 Java 的 Iterator<T> 接口，Rust 的 Iterator trait 使用关联类型

为 `Counter` 结构体实现 `Iterator` trait（标准库中的 `std::iter::Iterator`），使其生成从 1 到 `max` 的连续整数。

```rust
// 标准库 Iterator trait 的定义（参考）：
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { current: 0, max }
    }
}

// TODO: 为 Counter 实现 Iterator
// Item = u32
// next 每次返回 current+1，直到达到 max

fn main() {
    let mut counter = Counter::new(5);
    let mut sum = 0;
    while let Some(val) = counter.next() {
        sum += val;
    }
    println!("1 到 5 的和: {}", sum); // 应该输出 15

    // 也可以用 for 循环
    let mut counter2 = Counter::new(3);
    for val in counter2 {
        print!("{} ", val);
    }
    println!(); // 输出: 1 2 3
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            None
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}

fn main() {
    let mut counter = Counter::new(5);
    let mut sum = 0;
    while let Some(val) = counter.next() {
        sum += val;
    }
    println!("1 到 5 的和: {}", sum);

    let mut counter2 = Counter::new(3);
    for val in counter2 {
        print!("{} ", val);
    }
    println!();
}
```

**说明：** 标准库 `Iterator` trait 使用关联类型 `Item` 定义迭代产生的元素类型。实现 `Iterator` 后，类型会自动获得大量适配器方法（如 `map`、`filter`、`fold` 等）。`for` 循环实际是 `into_iter().next()` 的语法糖。
</details>

### 练习 07-16: Trait 对象基础

> 难度：⭐
> 类似 Java 的接口引用或 C++ 的虚函数调用，Rust 使用 dyn Trait 实现动态分发

创建一个 `Drawable` trait，包含一个 `draw` 方法。创建 `Circle` 和 `Square` 结构体，使用 trait 对象 `Vec<Box<dyn Drawable>>` 统一存储并调用 `draw`。

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

// TODO: 为 Circle 实现 Drawable（打印 "绘制圆形，半径: {radius}"）

struct Square {
    side: f64,
}

// TODO: 为 Square 实现 Drawable（打印 "绘制正方形，边长: {side}"）

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    for shape in &shapes {
        shape.draw();
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制圆形，半径: {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("绘制正方形，边长: {}", self.side);
    }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    for shape in &shapes {
        shape.draw();
    }
}
```

**说明：** `Box<dyn Drawable>` 是一个 trait 对象，它可以存储任何实现了 `Drawable` 的类型。Trait 对象通过虚表（vtable）实现动态分发，在运行时确定实际类型并调用对应方法。使用 trait 对象可以实现类似 Java 接口引用的多态效果。
</details>

### 练习 07-17: Trait 对象作为参数

> 难度：⭐
> 类似 Java 中接受接口类型作为参数，Rust 也可以使用 &dyn Trait

补全 `render_scene` 函数，使其接受一个 `&dyn Drawable` 并调用其 `draw` 方法。然后分别传入 `Circle` 和 `Square` 的引用。

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Drawable for Circle {
    fn draw(&self) {
        println!("⚪ 圆形 (r={})", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("⬜ 方形 (s={})", self.side);
    }
}

// TODO: 编写 render_scene 函数，接受 &dyn Drawable 并调用 draw

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 3.0 };

    render_scene(&circle);
    render_scene(&square);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Drawable for Circle {
    fn draw(&self) {
        println!("⚪ 圆形 (r={})", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("⬜ 方形 (s={})", self.side);
    }
}

fn render_scene(item: &dyn Drawable) {
    item.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 3.0 };

    render_scene(&circle);
    render_scene(&square);
}
```

**说明：** `&dyn Drawable` 是 trait 对象的引用形式，可以指向任何实现了 `Drawable` 的类型的引用。这类似于 `&impl Drawable`（静态分发），但 `&dyn Drawable` 使用动态分发——方法调用在运行时通过 vtable 决定。
</details>

### 练习 07-18: 对象安全（Object Safety）

> 难度：⭐⭐
> Rust 的 trait 对象有安全限制：只有满足对象安全的 trait 才能用于 dyn Trait

判断以下 trait 哪些可以用于 trait 对象，并解释原因。补全代码使其能通过编译。

```rust
// 以下 trait 哪些是"对象安全"的？哪些不是？为什么？

trait SafeTrait {
    fn do_something(&self);
}

trait UnsafeTrait {
    fn new() -> Self;  // 返回 Self 类型
}

trait MaybeSafe {
    fn process(&self) -> Self;  // 返回 Self
}

// TODO: 选择一个可以用于 trait 对象的 trait，创建相应的结构体并实现它
// 然后在 main 中使用 Box<dyn YourTrait>

struct MyStruct;

// TODO: 为 MyStruct 实现合适的 trait

fn main() {
    // TODO: 创建 trait 对象并调用方法
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
// SafeTrait: ✅ 对象安全——所有方法都接收 &self，不返回 Self
// UnsafeTrait: ❌ 不安全——new() 返回 Self，无法确定具体类型大小
// MaybeSafe: ❌ 不安全——process 返回 Self，vtable 无法确定返回类型的大小

trait SafeTrait {
    fn do_something(&self);
}

struct MyStruct;

impl SafeTrait for MyStruct {
    fn do_something(&self) {
        println!("我是对象安全的！");
    }
}

fn main() {
    let obj: Box<dyn SafeTrait> = Box::new(MyStruct);
    obj.do_something();
}
```

**说明：** 对象安全的 trait 必须满足：
1. 方法返回类型不是 `Self`（不能返回自身类型）
2. 方法中没有泛型参数（泛型会在编译期展开，与动态分发冲突）
3. 方法必须接收 `&self`、`&mut self` 或 `self` 参数（不能是静态方法）

满足这些条件的 trait 才能用于 `dyn Trait` 语法。
</details>

### 练习 07-19: 通过 Trait 对象实现工厂模式

> 难度：⭐⭐
> 类似 Java 的工厂方法返回接口类型，Rust 中 trait 对象常用于工厂模式

定义一个 `Animal` trait（包含 `speak` 和 `name` 方法），然后创建不同的动物类型。实现一个 `animal_factory` 函数，根据字符串返回对应的动物 trait 对象。

```rust
trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
}

// TODO: 定义 Dog 和 Cat 结构体
// TODO: 为两者实现 Animal trait
// Dog: name "小狗", speak "汪汪!"
// Cat: name "小猫", speak "喵喵~"

// TODO: 编写 animal_factory 函数，接收 &str，返回 Box<dyn Animal>
// "dog" -> Dog, "cat" -> Cat, _ -> panic

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        animal_factory("dog"),
        animal_factory("cat"),
        animal_factory("dog"),
    ];

    for animal in &animals {
        print!("{} 说: ", animal.name());
        animal.speak();
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Animal {
    fn speak(&self);
    fn name(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("汪汪!"); }
    fn name(&self) -> &str { "小狗" }
}

impl Animal for Cat {
    fn speak(&self) { println!("喵喵~"); }
    fn name(&self) -> &str { "小猫" }
}

fn animal_factory(animal_type: &str) -> Box<dyn Animal> {
    match animal_type {
        "dog" => Box::new(Dog),
        "cat" => Box::new(Cat),
        _ => panic!("未知动物类型: {}", animal_type),
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        animal_factory("dog"),
        animal_factory("cat"),
        animal_factory("dog"),
    ];

    for animal in &animals {
        print!("{} 说: ", animal.name());
        animal.speak();
    }
}
```

**说明：** 工厂函数返回 `Box<dyn Animal>`，调用者无需关心具体类型。Trait 对象的大小在编译期未知，所以必须放在堆上（`Box`）或使用引用（`&dyn`）。这是 Rust 中实现运行时多态的典型方式。
</details>

### 练习 07-20: Trait 对象与泛型的取舍

> 难度：⭐⭐⭐
> 对比静态分发（泛型）和动态分发（dyn Trait）的语法和性能特征

分别用泛型函数和 trait 对象实现一个 `process` 函数，调用 `Drawable` 的 `draw` 方法。观察两种写法的区别。

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("圆形 (r={})", self.radius); }
}

impl Drawable for Square {
    fn draw(&self) { println!("方形 (s={})", self.side); }
}

// TODO: 使用泛型（静态分发）编写 process_static
// fn process_static(item: &impl Drawable)

// TODO: 使用 trait 对象（动态分发）编写 process_dyn
// fn process_dyn(item: &dyn Drawable)

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 3.0 };

    // 静态分发：编译期为不同类型生成不同版本的代码
    process_static(&circle);
    process_static(&square);

    // 动态分发：同一份代码通过 vtable 调用
    process_dyn(&circle);
    process_dyn(&square);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("圆形 (r={})", self.radius); }
}

impl Drawable for Square {
    fn draw(&self) { println!("方形 (s={})", self.side); }
}

fn process_static(item: &impl Drawable) {
    item.draw();
}

fn process_dyn(item: &dyn Drawable) {
    item.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 3.0 };

    process_static(&circle);
    process_static(&square);

    process_dyn(&circle);
    process_dyn(&square);
}
```

**说明：** 
- **静态分发**（泛型/`impl Trait`）：编译期为每个具体类型生成单独的函数副本，无运行时开销，但增加二进制体积。
- **动态分发**（`dyn Trait`）：通过 vtable 在运行时查找方法，函数只有一份代码，但有轻微间接调用开销。
- 选择原则：性能关键路径用静态分发；需要异构集合（如 `Vec<Box<dyn Trait>>`）时用动态分发。
</details>

### 练习 07-21: 实现 Display

> 难度：⭐
> 类似 Java 的 toString()，Rust 中 Display 用于用户可读的输出（{} 格式化）

为 `Point` 结构体实现 `Display` trait，使其能够通过 `{}` 格式化为 `(x, y)` 形式。

```rust
use std::fmt::Display;

struct Point {
    x: i32,
    y: i32,
}

// TODO: 为 Point 实现 Display
// 格式化输出为 "(x, y)" 格式

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("坐标: {}", p);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt::Display;

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("坐标: {}", p);
}
```

**说明：** `Display` trait 需要实现 `fmt` 方法，接收一个 `Formatter` 并返回 `std::fmt::Result`。使用 `write!` 宏写入格式化内容。`Display` 用于用户可见的输出，对应 `{}` 占位符。
</details>

### 练习 07-22: 实现 Debug

> 难度：⭐
> Rust 的 Debug trait 用于调试输出 ({:?})，可以通过 #[derive(Debug)] 自动派生

分别为 `Person` 结构体手动实现 `Debug` 和自动派生 `Debug`，观察两种方式的区别。

```rust
// TODO: 为 Person1 使用 #[derive(Debug)]
struct Person1 {
    name: String,
    age: u8,
}

// TODO: 为 Person2 手动实现 Debug trait
// 输出格式: "Person2 {{ name: {name}, age: {age} }}"
struct Person2 {
    name: String,
    age: u8,
}

fn main() {
    let p1 = Person1 {
        name: String::from("Alice"),
        age: 30,
    };
    let p2 = Person2 {
        name: String::from("Bob"),
        age: 25,
    };
    println!("自动 Debug: {:?}", p1);
    println!("手动 Debug: {:?}", p2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug)]
struct Person1 {
    name: String,
    age: u8,
}

struct Person2 {
    name: String,
    age: u8,
}

impl std::fmt::Debug for Person2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Person2")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

fn main() {
    let p1 = Person1 {
        name: String::from("Alice"),
        age: 30,
    };
    let p2 = Person2 {
        name: String::from("Bob"),
        age: 25,
    };
    println!("自动 Debug: {:?}", p1);
    println!("手动 Debug: {:?}", p2);
}
```

**说明：** `Debug` 可以通过 `#[derive(Debug)]` 自动派生，适用于大多数简单结构体。手动实现时可以使用 `Formatter` 的 `debug_struct`、`debug_tuple` 等方法辅助格式化。`Debug` 使用 `{:?}` 占位符输出。
</details>

### 练习 07-23: 实现 PartialEq

> 难度：⭐⭐
> 类似 Java 的 equals()，Rust 的 PartialEq 用于 == 和 != 运算

为 `Book` 结构体手动实现 `PartialEq`，要求两本书的 `isbn` 相同即视为相等（忽略书名和作者）。

```rust
struct Book {
    isbn: String,
    title: String,
    author: String,
}

// TODO: 为 Book 手动实现 PartialEq
// 只比较 isbn 字段

fn main() {
    let book1 = Book {
        isbn: String::from("978-3-16-148410-0"),
        title: String::from("Rust 入门"),
        author: String::from("张三"),
    };
    let book2 = Book {
        isbn: String::from("978-3-16-148410-0"),
        title: String::from("Rust 进阶"),
        author: String::from("李四"),
    };
    let book3 = Book {
        isbn: String::from("978-0-12-345678-9"),
        title: String::from("Rust 入门"),
        author: String::from("张三"),
    };

    println!("book1 == book2: {}", book1 == book2); // true（相同 ISBN）
    println!("book1 == book3: {}", book1 == book3); // false（不同 ISBN）
    println!("book1 != book3: {}", book1 != book3); // true
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Book {
    isbn: String,
    title: String,
    author: String,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

fn main() {
    let book1 = Book {
        isbn: String::from("978-3-16-148410-0"),
        title: String::from("Rust 入门"),
        author: String::from("张三"),
    };
    let book2 = Book {
        isbn: String::from("978-3-16-148410-0"),
        title: String::from("Rust 进阶"),
        author: String::from("李四"),
    };
    let book3 = Book {
        isbn: String::from("978-0-12-345678-9"),
        title: String::from("Rust 入门"),
        author: String::from("张三"),
    };

    println!("book1 == book2: {}", book1 == book2);
    println!("book1 == book3: {}", book1 == book3);
    println!("book1 != book3: {}", book1 != book3);
}
```

**说明：** `PartialEq` trait 定义 `eq` 方法（必须实现）和 `ne` 方法（可选，默认取反）。实现后类型支持 `==` 和 `!=` 运算符。不实现 `Eq`（`PartialEq` 的子 trait）表示该类型上的相等关系可能不是等价关系（如浮点数的 NaN != NaN）。
</details>

### 练习 07-24: 实现 Clone

> 难度：⭐⭐
> 类似 C++ 的拷贝构造函数，Rust 的 Clone 用于显式克隆（.clone()）

为 `Profile` 结构体手动实现 `Clone` trait，实现深拷贝。注意 `id` 字段需要生成新的唯一值。

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

struct Profile {
    id: u64,
    name: String,
    email: String,
}

impl Profile {
    fn new(name: &str, email: &str) -> Self {
        Profile {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: String::from(name),
            email: String::from(email),
        }
    }
}

// TODO: 为 Profile 手动实现 Clone
// 注意：克隆时应生成新的 id，而不是复制原来的 id

fn main() {
    let original = Profile::new("Alice", "alice@example.com");
    let cloned = original.clone();

    println!("原始: id={}, name={}", original.id, original.name);
    println!("克隆: id={}, name={}", cloned.id, cloned.name);
    println!("id 不同: {}", original.id != cloned.id);
    println!("name 相同: {}", original.name == cloned.name);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

struct Profile {
    id: u64,
    name: String,
    email: String,
}

impl Profile {
    fn new(name: &str, email: &str) -> Self {
        Profile {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: String::from(name),
            email: String::from(email),
        }
    }
}

impl Clone for Profile {
    fn clone(&self) -> Self {
        Profile {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

fn main() {
    let original = Profile::new("Alice", "alice@example.com");
    let cloned = original.clone();

    println!("原始: id={}, name={}", original.id, original.name);
    println!("克隆: id={}, name={}", cloned.id, cloned.name);
    println!("id 不同: {}", original.id != cloned.id);
    println!("name 相同: {}", original.name == cloned.name);
}
```

**说明：** `Clone` trait 定义 `clone(&self) -> Self` 方法。手动实现时需要对每个字段进行克隆——`String` 的 `.clone()` 做深拷贝。本例中 `id` 字段生成新值而非复制，演示了"克隆但保持唯一性"的模式。也可以用 `#[derive(Clone)]` 自动派生标准克隆行为。
</details>

### 练习 07-25: 组合使用标准库 Trait

> 难度：⭐⭐⭐
> Rust 中经常需要对同一类型实现多个标准库 trait，实现全面的类型支持

为 `Product` 结构体同时实现 `Debug`、`PartialEq`、`Clone` 和 `Display`，使其支持多种操作。

```rust
use std::fmt;

struct Product {
    id: u32,
    name: String,
    price: f64,
}

// TODO: 使用 #[derive] 自动派生 Debug、PartialEq、Clone

// TODO: 手动实现 Display，格式为 "Product#{id}: {name} (¥{price:.2})"

fn main() {
    let p1 = Product {
        id: 1,
        name: String::from("Rust 编程书"),
        price: 79.99,
    };
    let p2 = Product {
        id: 1,
        name: String::from("Rust 编程书"),
        price: 79.99,
    };
    let p3 = p1.clone();

    println!("Display: {}", p1);
    println!("Debug: {:?}", p1);
    println!("p1 == p2: {}", p1 == p2);
    println!("p3.id == p1.id: {}", p3.id == p1.id);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Product#{}: {} (¥{:.2})", self.id, self.name, self.price)
    }
}

fn main() {
    let p1 = Product {
        id: 1,
        name: String::from("Rust 编程书"),
        price: 79.99,
    };
    let p2 = Product {
        id: 1,
        name: String::from("Rust 编程书"),
        price: 79.99,
    };
    let p3 = p1.clone();

    println!("Display: {}", p1);
    println!("Debug: {:?}", p1);
    println!("p1 == p2: {}", p1 == p2);
    println!("p3.id == p1.id: {}", p3.id == p1.id);
}
```

**说明：** Rust 中常用标准库 trait 各有用途：
- `Debug`（`{:?}`）——调试输出，通常用 `#[derive(Debug)]`
- `Display`（`{}`）——用户可读输出，需手动实现 `fmt` 方法
- `PartialEq`（`==`/`!=`）——相等比较，可派生或手动实现
- `Clone`（`.clone()`）——显式复制，可派生或手动实现
- 组合使用这些 trait 使类型更加完善、易用。
</details>
