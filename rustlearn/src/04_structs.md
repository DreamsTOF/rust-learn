# 04 结构体

结构体（struct）是 Rust 中自定义数据类型的主要方式，用于将多个相关联的值组合成一个整体。本章练习将帮助你掌握结构体的定义、实例化、方法实现以及元组结构体等核心概念。

### 练习 04-01: 定义结构体

> 难度：⭐
> 类似 C++ 的 struct，但 Rust 结构体不包含方法（方法在 impl 块中）

定义一个名为 `Person` 的结构体，包含 `name`（字符串）、`age`（无符号整数）和 `height`（浮点数）三个字段。

```rust
// TODO: 定义 Person 结构体

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        height: 175.5,
    };
    println!("{} is {} years old and {:.1} cm tall", person.name, person.age, person.height);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Person {
    name: String,
    age: u32,
    height: f64,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        height: 175.5,
    };
    println!("{} is {} years old and {:.1} cm tall", person.name, person.age, person.height);
}
```

**说明：** Rust 结构体使用 `struct` 关键字定义，每个字段需要声明名称和类型。创建实例时使用 `字段名: 值` 的语法，且所有字段必须全部初始化。
</details>

### 练习 04-02: 创建结构体实例

> 难度：⭐
> 类似 C++ 的 struct 初始化列表，但 Rust 要求字段名必须写全

定义一个 `Book` 结构体（字段自定），并在 `main` 中创建两个不同的实例。

```rust
// TODO: 定义 Book 结构体

fn main() {
    // TODO: 创建两个 Book 实例 book1 和 book2
    // book1: 《Rust 程序设计》(2024)
    // book2: 《算法导论》(2022)
    
    println!("《{}》由 {} 著，{} 年出版", book1.title, book1.author, book1.year);
    println!("《{}》由 {} 著，{} 年出版", book2.title, book2.author, book2.year);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Book {
    title: String,
    author: String,
    year: u32,
}

fn main() {
    let book1 = Book {
        title: String::from("Rust 程序设计"),
        author: String::from("张三个人"),
        year: 2024,
    };
    let book2 = Book {
        title: String::from("算法导论"),
        author: String::from("李四"),
        year: 2022,
    };
    
    println!("《{}》由 {} 著，{} 年出版", book1.title, book1.author, book1.year);
    println!("《{}》由 {} 著，{} 年出版", book2.title, book2.author, book2.year);
}
```

**说明：** 同一个结构体可以创建多个互不影响的实例，每个实例独立拥有字段值。Rust 要求实例化时所有字段必须显式赋值。
</details>

### 练习 04-03: 访问和修改字段

> 难度：⭐⭐
> 类似 C++ 使用 `.` 运算符访问成员，但 Rust 要求实例可变才能修改

补全代码，先打印 `point` 的坐标，然后将 `x` 改为 20 并重新打印。

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 10, y: 15 };
    // TODO: 打印当前坐标
    // TODO: 修改 x 为 20
    // TODO: 再次打印坐标
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 10, y: 15 };
    println!("当前坐标: ({}, {})", point.x, point.y);
    point.x = 20;
    println!("修改后坐标: ({}, {})", point.x, point.y);
}
```

**说明：** Rust 中结构体实例默认不可变，必须使用 `mut` 关键字声明为可变后，才能修改字段的值。访问字段使用 `.` 运算符。
</details>

### 练习 04-04: 从函数返回结构体

> 难度：⭐⭐
> 类似 C++ 中返回结构体对象，Rust 中同样使用返回值语法

补全 `build_user` 函数，它接收 `email` 和 `username` 参数，返回一个 `User` 结构体实例。

```rust
struct User {
    email: String,
    username: String,
    active: bool,
}

// TODO: 补全 build_user 函数体
fn build_user(email: String, username: String) -> User {
    // 返回一个 User 实例，active 默认为 true
}

fn main() {
    let user = build_user(
        String::from("alice@example.com"),
        String::from("alice"),
    );
    println!("用户 {} <{}>，活跃状态: {}", user.username, user.email, user.active);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct User {
    email: String,
    username: String,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
    }
}

fn main() {
    let user = build_user(
        String::from("alice@example.com"),
        String::from("alice"),
    );
    println!("用户 {} <{}>，活跃状态: {}", user.username, user.email, user.active);
}
```

**说明：** 函数可以返回结构体实例。当字段名与变量名相同时，可以简写为 `email` 而非 `email: email`，但此处显式写出以展示完整语法。
</details>

### 练习 04-05: 创建学生实例集合

> 难度：⭐⭐⭐
> 类似 C++ 的结构体数组，Rust 中使用 Vec 存放多个结构体实例

定义一个 `Student` 结构体（包含 `name`、`age`、`score`），然后创建至少 3 个不同的学生实例放入 `Vec` 中，最后遍历打印每个学生的信息。

```rust
// TODO: 定义 Student 结构体

fn main() {
    // TODO: 创建至少 3 个学生实例并放入 Vec
    // TODO: 遍历打印所有学生信息
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Student {
    name: String,
    age: u8,
    score: f32,
}

fn main() {
    let students = vec![
        Student {
            name: String::from("张三"),
            age: 20,
            score: 92.5,
        },
        Student {
            name: String::from("李四"),
            age: 21,
            score: 85.0,
        },
        Student {
            name: String::from("王五"),
            age: 19,
            score: 97.0,
        },
    ];

    for s in &students {
        println!("{}（{}岁）成绩：{:.1}", s.name, s.age, s.score);
    }
}
```

**说明：** `vec!` 宏可以创建包含结构体实例的向量。遍历时使用 `&students` 避免所有权转移，通过 `s.name` 访问字段。
</details>

### 练习 04-06: 为结构体添加方法

> 难度：⭐
> 类似 C++ 的成员函数，但 Rust 的方法定义在单独的 impl 块中

给 `Rectangle` 结构体实现一个 `area` 方法，计算矩形的面积。

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

// TODO: 实现 area 方法

fn main() {
    let rect = Rectangle { width: 30.0, height: 50.0 };
    println!("矩形面积: {:.1}", rect.area());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30.0, height: 50.0 };
    println!("矩形面积: {:.1}", rect.area());
}
```

**说明：** 方法定义在 `impl` 块中，第一个参数总是 `&self` 表示调用该方法的实例引用。方法通过 `.` 运算符调用。
</details>

### 练习 04-07: 修改结构体字段的方法

> 难度：⭐
> 类似 C++ 的非 const 成员函数，Rust 中使用 &mut self

为 `Counter` 结构体实现一个 `increment` 方法，将 `count` 字段值加 1。

```rust
struct Counter {
    count: i32,
}

// TODO: 实现 increment 方法

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    counter.increment();
    println!("计数值: {}", counter.count); // 应该输出 2
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    counter.increment();
    println!("计数值: {}", counter.count);
}
```

**说明：** 需要修改 `self` 的方法使用 `&mut self` 作为参数，此时调用者必须为可变绑定（`let mut`）。
</details>

### 练习 04-08: 关联函数——构造函数

> 难度：⭐⭐
> 类似 C++ 的静态方法或 Java 的静态工厂方法，Rust 关联函数不接收 self 参数

为 `Point` 结构体实现一个关联函数 `origin`，返回坐标在 `(0, 0)` 的 `Point` 实例。

```rust
struct Point {
    x: i32,
    y: i32,
}

// TODO: 实现 origin 关联函数

fn main() {
    let p = Point::origin();
    println!("原点坐标: ({}, {})", p.x, p.y);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

fn main() {
    let p = Point::origin();
    println!("原点坐标: ({}, {})", p.x, p.y);
}
```

**说明：** 关联函数是不以 `self` 为参数的方法，使用 `::` 语法调用（`结构体名::函数名`），常用于构造函数模式。
</details>

### 练习 04-09: new 模式

> 难度：⭐⭐
> 类似 C++ 的构造函数，Rust 惯例使用 new 作为创建实例的关联函数

为 `Temperature` 实现一个 `new` 关联函数，接收摄氏度值并返回实例。再实现一个 `to_fahrenheit` 方法进行单位转换。

```rust
struct Temperature {
    celsius: f64,
}

// TODO: 实现 new 关联函数和 to_fahrenheit 方法
// 公式: ℉ = ℃ × 1.8 + 32.0

fn main() {
    let t = Temperature::new(37.0);
    println!("{:.1}℃ = {:.1}℉", t.celsius, t.to_fahrenheit());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(celsius: f64) -> Temperature {
        Temperature { celsius }
    }

    fn to_fahrenheit(&self) -> f64 {
        self.celsius * 1.8 + 32.0
    }
}

fn main() {
    let t = Temperature::new(37.0);
    println!("{:.1}℃ = {:.1}℉", t.celsius, t.to_fahrenheit());
}
```

**说明：** `new` 是 Rust 中创建实例的惯用关联函数名。当字段名与变量名相同时可以使用简写 `Temperature { celsius }` 而非 `Temperature { celsius: celsius }`。
</details>

### 练习 04-10: 结构体方法综合

> 难度：⭐⭐⭐
> 类似 C++ 类中的多个成员函数，Rust 的 impl 块可包含多个方法

为 `Rectangle` 实现以下功能：
1. `area` 方法计算面积
2. `can_hold` 方法判断当前矩形是否能完全容纳另一个矩形

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

// TODO: 实现 area 和 can_hold 方法

fn main() {
    let rect1 = Rectangle { width: 30.0, height: 50.0 };
    let rect2 = Rectangle { width: 20.0, height: 40.0 };
    let rect3 = Rectangle { width: 40.0, height: 30.0 };

    println!("rect1 面积: {:.1}", rect1.area());
    println!("rect1 能容纳 rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3: {}", rect1.can_hold(&rect3));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30.0, height: 50.0 };
    let rect2 = Rectangle { width: 20.0, height: 40.0 };
    let rect3 = Rectangle { width: 40.0, height: 30.0 };

    println!("rect1 面积: {:.1}", rect1.area());
    println!("rect1 能容纳 rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3: {}", rect1.can_hold(&rect3));
}
```

**说明：** 一个 `impl` 块可以包含多个方法。方法可以接收另一个同类型实例的引用作为参数。这里 `can_hold` 检查两个维度的包容性。
</details>

### 练习 04-11: 定义元组结构体

> 难度：⭐
> 类似 C++ 的 typedef 或 using，但 Rust 元组结构体创建的是全新的类型

定义一个名为 `Meters` 的元组结构体，包含一个 `f64` 值。编写函数计算两个长度的和。

```rust
// TODO: 定义 Meters 元组结构体

fn add_lengths(a: Meters, b: Meters) -> Meters {
    // TODO: 返回两个长度的和
}

fn main() {
    let length1 = Meters(10.5);
    let length2 = Meters(3.5);
    let sum = add_lengths(length1, length2);
    println!("总长度: {:.1} 米", sum.0);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Meters(f64);

fn add_lengths(a: Meters, b: Meters) -> Meters {
    Meters(a.0 + b.0)
}

fn main() {
    let length1 = Meters(10.5);
    let length2 = Meters(3.5);
    let sum = add_lengths(length1, length2);
    println!("总长度: {:.1} 米", sum.0);
}
```

**说明：** 元组结构体使用 `struct 名称(类型)` 定义，字段没有名字只有索引（通过 `.0`, `.1` 等访问）。它创建的是一个独立的新类型，而非类型别名。
</details>

### 练习 04-12: 颜色元组结构体

> 难度：⭐
> 类似 C++ 的简单值包装，Rust 元组结构体常用于包装单一概念

定义一个 `Color` 元组结构体包含 RGB 三个 `u8` 值。实现一个方法将颜色格式化为 `#RRGGBB` 字符串。

```rust
// TODO: 定义 Color 元组结构体

// TODO: 实现 format_hex 方法，返回格式如 "#FF00FF"

fn main() {
    let color = Color(255, 0, 128);
    println!("颜色值: {}", color.format_hex());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Color(u8, u8, u8);

impl Color {
    fn format_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

fn main() {
    let color = Color(255, 0, 128);
    println!("颜色值: {}", color.format_hex());
}
```

**说明：** 元组结构体也可以有方法。`{:02X}` 格式化输出两位大写十六进制数，不足两位补零。通过 `self.0`、`self.1` 等访问字段。
</details>

### 练习 04-13: 结构体更新语法

> 难度：⭐⭐
> 类似 C++ 的拷贝构造或赋值，但 Rust 使用 .. 语法从另一个实例复制字段

定义一个 `Student` 结构体，创建 `alice` 实例，然后使用更新语法创建 `bob`，其中 `name` 和 `email` 不同，其余字段与 `alice` 相同。

```rust
struct Student {
    name: String,
    email: String,
    grade: u8,
    active: bool,
}

fn main() {
    let alice = Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        grade: 85,
        active: true,
    };

    // TODO: 使用更新语法创建 bob，grade 为 90，active 为 true（从 alice 继承）
    // bob 的 name 为 "Bob"，email 为 "bob@example.com"

    println!("{}, {}, 年级: {}, 活跃: {}", bob.name, bob.email, bob.grade, bob.active);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Student {
    name: String,
    email: String,
    grade: u8,
    active: bool,
}

fn main() {
    let alice = Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        grade: 85,
        active: true,
    };

    let bob = Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        grade: 90,
        ..alice
    };

    println!("{}, {}, 年级: {}, 活跃: {}", bob.name, bob.email, bob.grade, bob.active);
}
```

**说明：** `..alice` 表示从 `alice` 复制未被显式赋值的字段。注意：由于 `grade` 和 `active` 实现了 `Copy` trait（它们是基本类型），所以 `alice` 中的 `active` 字段被复制而非移动。如果字段中包含 `String` 等非 `Copy` 类型且未被重新赋值，则会移动所有权。
</details>

### 练习 04-14: 更新语法的所有权问题

> 难度：⭐⭐
> 类似 C++ 的浅拷贝 vs 深拷贝，Rust 的结构体更新语法涉及所有权转移

补全代码，观察结构体更新语法对原始实例的影响。

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1
    };

    // TODO: 下面哪一行能通过编译？取消注释并运行验证
    // println!("user1: {}", user1.username);
    // println!("user1: {}", user1.email);
    // println!("user1: {}", user1.active);
    println!("user2: {}, {}", user2.username, user2.email);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    let user2 = User {
        email: String::from("bob@example.com"),
        ..user1
    };

    // username 从 user1 移动到了 user2，所以 user1.username 不能再使用
    // email 在 user1 中也被移动了吗？不——user2.email 被显式赋值了，没有移动 user1.email
    // 所以 user1.email 仍然有效
    // active 是 bool 类型（Copy），user1.active 仍然有效
    
    // println!("user1: {}", user1.username); // ❌ user1.username 已移动
    println!("user1: {}", user1.email);    // ✅ 未移动，因为 user2.email 被显式赋新值
    println!("user1: {}", user1.active);   // ✅ bool 实现了 Copy
    
    println!("user2: {}, {}", user2.username, user2.email);
}
```

**说明：** 使用 `..user1` 时，未被显式赋值的字段会从原实例移动所有权。`username` 从 `user1` 移动到 `user2`，因此 `user1.username` 不再可用。`email` 被显式赋值，所以 `user1.email` 未被移动。`bool` 实现了 `Copy` trait，因此 `user1.active` 仍然可用。
</details>

### 练习 04-15: 银行账户综合练习

> 难度：⭐⭐⭐
> 类似 C++ 的完整类实现，Rust 使用结构体 + impl 块完成封装

设计一个 `BankAccount` 结构体，实现以下功能：
1. `new(owner: String) -> BankAccount` —— 创建新账户，余额为 0
2. `deposit(&mut self, amount: f64)` —— 存入金额（不能为负数）
3. `withdraw(&mut self, amount: f64) -> bool` —— 取款，余额不足时返回 false
4. `get_balance(&self) -> f64` —— 查看余额
5. `get_owner(&self) -> &str` —— 查看户主姓名

```rust
// TODO: 定义 BankAccount 结构体

// TODO: 实现相关方法

fn main() {
    let mut account = BankAccount::new(String::from("张三"));
    println!("{} 的账户余额: {:.2}", account.get_owner(), account.get_balance());
    
    account.deposit(1000.0);
    println!("存入 1000 后余额: {:.2}", account.get_balance());
    
    let success = account.withdraw(300.0);
    println!("取款 300 {}，余额: {:.2}", if success { "成功" } else { "失败" }, account.get_balance());
    
    let success = account.withdraw(800.0);
    println!("取款 800 {}，余额: {:.2}", if success { "成功" } else { "失败" }, account.get_balance());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String) -> BankAccount {
        BankAccount {
            owner,
            balance: 0.0,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn get_owner(&self) -> &str {
        &self.owner
    }
}

fn main() {
    let mut account = BankAccount::new(String::from("张三"));
    println!("{} 的账户余额: {:.2}", account.get_owner(), account.get_balance());
    
    account.deposit(1000.0);
    println!("存入 1000 后余额: {:.2}", account.get_balance());
    
    let success = account.withdraw(300.0);
    println!("取款 300 {}，余额: {:.2}", if success { "成功" } else { "失败" }, account.get_balance());
    
    let success = account.withdraw(800.0);
    println!("取款 800 {}，余额: {:.2}", if success { "成功" } else { "失败" }, account.get_balance());
}
```

**说明：** 这是一个综合练习，展示了结构体的完整用法：`new` 关联函数作为构造函数，`&mut self` 方法修改状态，`&self` 方法查询状态，以及方法返回值的合理设计。`get_owner` 返回字符串引用（`&str`）避免所有权转移。
</details>
