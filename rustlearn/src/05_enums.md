# 05 枚举

枚举（enum）是 Rust 中极为强大的自定义数据类型，与 C++ 的 `enum` 不同，Rust 枚举的每个变体可以携带不同类型和数量的数据，其表达能力类似于 Java 的 sealed class。本章练习将帮助你掌握 Rust 枚举的定义、数据携带、方法实现以及在实际场景中的灵活运用。

### 练习 05-01: 定义基本枚举

> 难度：⭐
> 类似 C++ 的 enum，但不带底层整数值

定义一个 `Direction` 枚举，包含 `North`、`South`、`East`、`West` 四个变体。然后编写一个函数将方向转换为中文名称。

```rust
// TODO: 定义 Direction 枚举，包含 North, South, East, West 四个变体

// TODO: 实现 direction_to_chinese 函数，接收 Direction 返回 &str
// 例如 North -> "北"

fn main() {
    let dir = Direction::East;
    println!("{} 方向是{}", "East", direction_to_chinese(dir));
    println!("{} 方向是{}", "North", direction_to_chinese(Direction::North));
    println!("{} 方向是{}", "South", direction_to_chinese(Direction::South));
    println!("{} 方向是{}", "West", direction_to_chinese(Direction::West));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn direction_to_chinese(dir: Direction) -> &'static str {
    match dir {
        Direction::North => "北",
        Direction::South => "南",
        Direction::East => "东",
        Direction::West => "西",
    }
}

fn main() {
    let dir = Direction::East;
    println!("{} 方向是{}", "East", direction_to_chinese(dir));
    println!("{} 方向是{}", "North", direction_to_chinese(Direction::North));
    println!("{} 方向是{}", "South", direction_to_chinese(Direction::South));
    println!("{} 方向是{}", "West", direction_to_chinese(Direction::West));
}
```

**说明：** Rust 枚举使用 `enum` 关键字定义，变体之间用逗号分隔。与 C++ 不同，Rust 枚举变体默认没有整数值。通常配合 `match` 表达式进行模式匹配来处理每个变体。
</details>

### 练习 05-02: 枚举与 match 表达式

> 难度：⭐
> 类似 C++ 的 switch-case，但 Rust 的 match 必须穷举所有可能性

定义一个 `TrafficLight` 枚举（Red、Yellow、Green），编写函数返回每种灯光持续的时间（秒）。

```rust
// TODO: 定义 TrafficLight 枚举

// TODO: 实现 duration 函数，接收 TrafficLight 返回 u32

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    
    println!("红灯: {} 秒", duration(red));
    println!("黄灯: {} 秒", duration(yellow));
    println!("绿灯: {} 秒", duration(green));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn duration(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 45,
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    
    println!("红灯: {} 秒", duration(red));
    println!("黄灯: {} 秒", duration(yellow));
    println!("绿灯: {} 秒", duration(green));
}
```

**说明：** Rust 的 `match` 必须穷举所有变体（exhaustive），这强制开发者处理所有情况，避免遗漏。如果不想处理所有变体，可以使用 `_ =>` 作为通配分支。
</details>

### 练习 05-03: 枚举携带整数数据

> 难度：⭐⭐
> 类似 C++ 的 enum class 带值，但每个变体可以有不同的类型

定义一个 `HttpStatus` 枚举，其中 `Ok` 携带一个 `u16` 状态码，`NotFound` 和 `ServerError` 各自携带不同的状态码。

```rust
// TODO: 定义 HttpStatus 枚举，包含 Ok(u16), NotFound(u16), ServerError(u16)

// TODO: 实现 describe 函数，返回状态码对应的文字描述
// 200 -> "OK", 404 -> "Not Found", 500 -> "Internal Server Error", 其他 -> "Unknown"

fn main() {
    let status1 = HttpStatus::Ok(200);
    let status2 = HttpStatus::NotFound(404);
    let status3 = HttpStatus::ServerError(500);
    
    println!("{}", describe(status1));
    println!("{}", describe(status2));
    println!("{}", describe(status3));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum HttpStatus {
    Ok(u16),
    NotFound(u16),
    ServerError(u16),
}

fn describe(status: HttpStatus) -> String {
    match status {
        HttpStatus::Ok(code) if code == 200 => format!("{} OK", code),
        HttpStatus::NotFound(code) if code == 404 => format!("{} Not Found", code),
        HttpStatus::ServerError(code) if code == 500 => format!("{} Internal Server Error", code),
        _ => "Unknown".to_string(),
    }
}

fn main() {
    let status1 = HttpStatus::Ok(200);
    let status2 = HttpStatus::NotFound(404);
    let status3 = HttpStatus::ServerError(500);
    
    println!("{}", describe(status1));
    println!("{}", describe(status2));
    println!("{}", describe(status3));
}
```

**说明：** Rust 枚举变体可以携带数据，且每个变体携带的数据类型可以不同。这里所有变体都携带 `u16`，但也可以携带完全不同类型的数据。`match` 中使用 `if` 守卫（guard）可以增加额外条件判断。
</details>

### 练习 05-04: 变体携带不同类型的数据

> 难度：⭐⭐
> 类似 Java 的 sealed class，每个子类可以有不同字段

定义一个 `Measurement` 枚举，包含 `Distance(f64)`、`Weight(f64, String)`、`Temperature { value: f64, unit: char }` 三个变体，展示枚举变体可以携带不同形式和数量的数据。

```rust
// TODO: 定义 Measurement 枚举
// Distance(f64): 只包含距离数值
// Weight(f64, String): 包含重量和单位名称
// Temperature { value: f64, unit: char }: 包含温度和单位字符（C/F）

// TODO: 实现 format_measurement 函数，返回格式化的测量值字符串

fn main() {
    let d = Measurement::Distance(150.0);
    let w = Measurement::Weight(75.5, String::from("kg"));
    let t = Measurement::Temperature { value: 37.0, unit: 'C' };
    
    println!("{}", format_measurement(d));
    println!("{}", format_measurement(w));
    println!("{}", format_measurement(t));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Measurement {
    Distance(f64),
    Weight(f64, String),
    Temperature { value: f64, unit: char },
}

fn format_measurement(m: Measurement) -> String {
    match m {
        Measurement::Distance(val) => format!("距离: {:.1} 米", val),
        Measurement::Weight(val, unit) => format!("重量: {:.1} {}", val, unit),
        Measurement::Temperature { value, unit } => format!("温度: {:.1}°{}", value, unit),
    }
}

fn main() {
    let d = Measurement::Distance(150.0);
    let w = Measurement::Weight(75.5, String::from("kg"));
    let t = Measurement::Temperature { value: 37.0, unit: 'C' };
    
    println!("{}", format_measurement(d));
    println!("{}", format_measurement(w));
    println!("{}", format_measurement(t));
}
```

**说明：** Rust 枚举变体可以携带数据的形式有三种：元组形式（匿名数据）、结构体形式（命名字段）和单元形式（无数据）。这使得枚举可以像 tagged union 一样灵活且安全地表达"不同类型"的变体。
</details>

### 练习 05-05: Message 枚举（多种数据类型）

> 难度：⭐⭐⭐
> 类似 Java 的 sealed class，每个子类可以有完全不同的字段和逻辑

定义一个 `Message` 枚举，包含以下变体：
- `Quit`：无数据
- `Move { x: i32, y: i32 }`：结构体形式
- `Write(String)`：字符串消息
- `ChangeColor(i32, i32, i32)`：RGB 颜色值

然后实现一个 `process` 函数，接收 `Message` 并根据不同类型执行不同逻辑。

```rust
// TODO: 定义 Message 枚举（Quit / Move / Write / ChangeColor）

// TODO: 实现 process 函数
// Quit -> 打印 "程序退出"
// Move -> 打印 "移动到 ({x}, {y})"
// Write -> 打印消息内容
// ChangeColor -> 打印 "颜色改为 RGB({r}, {g}, {b})"

fn main() {
    let msgs = vec![
        Message::Write(String::from("Hello, Rust!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 128),
        Message::Quit,
    ];
    
    for msg in msgs {
        process(msg);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("程序退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("消息: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色改为 RGB({}, {}, {})", r, g, b),
    }
}

fn main() {
    let msgs = vec![
        Message::Write(String::from("Hello, Rust!")),
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 0, 128),
        Message::Quit,
    ];
    
    for msg in msgs {
        process(msg);
    }
}
```

**说明：** `Message` 枚举是 Rust 官方文档中的经典示例，展示了枚举变体可以同时包含单元形式、结构体形式和元组形式。这种设计让一个类型能够表示多种不同形态的消息，且无需使用继承体系。模式匹配时每种形式都有对应的解构语法。
</details>

### 练习 05-06: 为枚举实现方法

> 难度：⭐
> 类似 Java 中类的方法，Rust 枚举的方法定义在 impl 块中

为 `TrafficLight` 枚举实现一个 `duration` 方法，返回每种灯光的持续秒数。

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// TODO: 为 TrafficLight 实现 duration 方法

fn main() {
    let light = TrafficLight::Red;
    println!("红灯持续 {} 秒", light.duration());
    
    let light = TrafficLight::Green;
    println!("绿灯持续 {} 秒", light.duration());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("红灯持续 {} 秒", light.duration());
    
    let light = TrafficLight::Green;
    println!("绿灯持续 {} 秒", light.duration());
}
```

**说明：** 枚举的方法定义在 `impl` 块中，与结构体完全相同。第一个参数 `&self` 表示枚举实例的引用。`match self` 可以匹配当前变体并返回对应的值。
</details>

### 练习 05-07: 为携带数据的枚举实现方法

> 难度：⭐
> 类似 Java 中不同子类有不同的行为

为 `Shape` 枚举实现一个 `area` 方法，计算不同形状的面积。`Shape` 枚举包含 `Circle(f64)`（半径）和 `Rectangle(f64, f64)`（宽，高）。

```rust
// TODO: 定义 Shape 枚举（Circle 携带半径，Rectangle 携带宽和高）

// TODO: 为 Shape 实现 area 方法，返回 f64

fn main() {
    let circle = Shape::Circle(5.0);
    let rect = Shape::Rectangle(4.0, 6.0);
    
    println!("圆面积: {:.2}", circle.area());
    println!("矩形面积: {:.2}", rect.area());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rect = Shape::Rectangle(4.0, 6.0);
    
    println!("圆面积: {:.2}", circle.area());
    println!("矩形面积: {:.2}", rect.area());
}
```

**说明：** 枚举方法中的 `match` 可以解构变体携带的数据。每个变体可以有不同的计算方法，这在面向对象语言中通常需要多态来实现，而 Rust 用枚举 + `match` 就能简洁表达。
</details>

### 练习 05-08: Option 的基本使用

> 难度：⭐
> 类似 Java 的 Optional，但 Rust 的 Option 是枚举而非包装类

Rust 的 `Option<T>` 枚举定义在标准库中：
```rust
enum Option<T> {
    None,
    Some(T),
}
```

补全代码，找出 `Vec<Option<i32>>` 中所有 `Some` 的值并求和。

```rust
// TODO: 补全代码，将 vec 中所有 Some 的值求和
fn sum_some(values: Vec<Option<i32>>) -> i32 {
    // 使用 match 或 if let 处理每个元素
    let mut total = 0;
    for val in values {
        // TODO: 如果是 Some(v)，将 v 加到 total
    }
    total
}

fn main() {
    let numbers = vec![Some(10), None, Some(20), Some(30), None, Some(40)];
    println!("求和结果: {}", sum_some(numbers)); // 应该输出 100
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn sum_some(values: Vec<Option<i32>>) -> i32 {
    let mut total = 0;
    for val in values {
        if let Some(v) = val {
            total += v;
        }
    }
    total
}

fn main() {
    let numbers = vec![Some(10), None, Some(20), Some(30), None, Some(40)];
    println!("求和结果: {}", sum_some(numbers));
}
```

**说明：** `if let Some(v) = val` 是一种简洁的模式匹配语法，只关心 `Some` 的情况而忽略 `None`。这是处理 `Option` 最常用的方式之一。也可以使用 `match` 或 `unwrap_or` 等方式处理。
</details>

### 练习 05-09: Option 链式操作

> 难度：⭐⭐
> 类似 Java Optional 的链式调用，Rust 的 Option 提供了 map、and_then 等组合方法

给定一个可能包含整数的字符串向量，尝试解析每个字符串为 `i32`，然后筛选出正数并求和。

```rust
// TODO: 补全代码，处理字符串向量中的 Option
fn sum_positive_parsed(values: Vec<&str>) -> i32 {
    // 提示：使用 str::parse::<i32>() 返回 Result，用 .ok() 转为 Option
    // 然后用 .filter() 和 .map() 等迭代器方法
    // TODO: 实现逻辑
    0 // 占位
}

fn main() {
    let inputs = vec!["10", "-5", "abc", "20", "0", "30", "def"];
    println!("正数之和: {}", sum_positive_parsed(inputs)); // 应该输出 60（10 + 20 + 30）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn sum_positive_parsed(values: Vec<&str>) -> i32 {
    values
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&n| n > 0)
        .sum()
}

fn main() {
    let inputs = vec!["10", "-5", "abc", "20", "0", "30", "def"];
    println!("正数之和: {}", sum_positive_parsed(inputs));
}
```

**说明：** `filter_map` 结合了 `filter` 和 `map` 的功能——如果闭包返回 `Some(v)` 则保留该值，返回 `None` 则丢弃。`parse::<i32>().ok()` 将 `Result` 转为 `Option`，解析失败则为 `None`。这种链式组合是 Rust 处理 `Option` 的惯用方式。
</details>

### 练习 05-10: Option 综合运用

> 难度：⭐⭐⭐
> 类似 Java 中复杂的 Optional 操作，Rust 要求类型严格匹配

实现一个函数 `safe_divide`，对两个 `Option<f64>` 进行除法运算，要求同时处理被除数为 `None`、除数为 `None`、除数为零以及正常情况。

```rust
// TODO: 实现 safe_divide 函数
// 如果 dividend 或 divisor 为 None，返回 None
// 如果 divisor 为 0.0，返回 None
// 否则返回 Some(dividend / divisor)
fn safe_divide(dividend: Option<f64>, divisor: Option<f64>) -> Option<f64> {
    // TODO
}

fn main() {
    println!("{:?}", safe_divide(Some(10.0), Some(2.0)));   // Some(5.0)
    println!("{:?}", safe_divide(Some(10.0), None));        // None
    println!("{:?}", safe_divide(None, Some(2.0)));          // None
    println!("{:?}", safe_divide(Some(10.0), Some(0.0)));   // None
    println!("{:?}", safe_divide(None, None));               // None
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn safe_divide(dividend: Option<f64>, divisor: Option<f64>) -> Option<f64> {
    match (dividend, divisor) {
        (Some(d), Some(r)) if r != 0.0 => Some(d / r),
        _ => None,
    }
}

fn main() {
    println!("{:?}", safe_divide(Some(10.0), Some(2.0)));   // Some(5.0)
    println!("{:?}", safe_divide(Some(10.0), None));        // None
    println!("{:?}", safe_divide(None, Some(2.0)));          // None
    println!("{:?}", safe_divide(Some(10.0), Some(0.0)));   // None
    println!("{:?}", safe_divide(None, None));               // None
}
```

**说明：** 元组模式匹配 `match (a, b) { ... }` 可以同时匹配两个 `Option` 的值，结合 `if` 守卫处理除零情况。这种写法比嵌套 `match` 或 `and_then` 更清晰。`_ => None` 处理了其余所有无效情况。
</details>

### 练习 05-11: 枚举作为状态机（开关状态）

> 难度：⭐
> 类似 C++ 中使用枚举表示状态，Rust 枚举天然适合做状态机建模

定义一个 `DoorState` 枚举（Open、Closed、Locked），实现一个 `transition` 函数，根据当前状态和操作返回下一个状态。操作包括：`open`、`close`、`lock`、`unlock`。

```rust
// TODO: 定义 DoorState 枚举

// TODO: 定义 Action 枚举（Open, Close, Lock, Unlock）

// TODO: 实现 transition 函数
// 规则：
// Open + Close  -> Closed
// Closed + Open  -> Open
// Closed + Lock  -> Locked
// Locked + Unlock -> Closed
// 其他组合都保持当前状态不变

fn main() {
    let state = DoorState::Open;
    let state = transition(state, Action::Close);
    println!("当前状态: {:?}", state); // Closed
    
    let state = transition(state, Action::Lock);
    println!("当前状态: {:?}", state); // Locked
    
    let state = transition(state, Action::Unlock);
    println!("当前状态: {:?}", state); // Closed
    
    // 无效操作（Locked 时尝试 Open）
    let state = transition(DoorState::Locked, Action::Open);
    println!("当前状态: {:?}", state); // Locked（不变）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug)]
enum DoorState {
    Open,
    Closed,
    Locked,
}

enum Action {
    Open,
    Close,
    Lock,
    Unlock,
}

fn transition(state: DoorState, action: Action) -> DoorState {
    match (state, action) {
        (DoorState::Open, Action::Close) => DoorState::Closed,
        (DoorState::Closed, Action::Open) => DoorState::Open,
        (DoorState::Closed, Action::Lock) => DoorState::Locked,
        (DoorState::Locked, Action::Unlock) => DoorState::Closed,
        (s, _) => s, // 其他组合不变
    }
}

fn main() {
    let state = DoorState::Open;
    let state = transition(state, Action::Close);
    println!("当前状态: {:?}", state);
    
    let state = transition(state, Action::Lock);
    println!("当前状态: {:?}", state);
    
    let state = transition(state, Action::Unlock);
    println!("当前状态: {:?}", state);
    
    let state = transition(DoorState::Locked, Action::Open);
    println!("当前状态: {:?}", state);
}
```

**说明：** 枚举非常适合建模有限状态机。使用 `match (state, action)` 的元组模式匹配可以清晰地列出所有状态转换规则，编译器会确保我们处理了所有组合。`(s, _) => s` 作为兜底分支，表示其他组合保持当前状态不变。
</details>

### 练习 05-12: 订单状态机

> 难度：⭐
> 类似 C++ 的 enum 表示状态，但 Rust 更安全

定义一个 `OrderStatus` 枚举（Pending、Shipped、Delivered、Cancelled），模拟订单的状态流转。

```rust
// TODO: 定义 OrderStatus 枚举

// TODO: 实现 can_cancel 方法，判断订单是否可以取消
// Pending -> 可以取消
// Shipped -> 不可以取消
// Delivered -> 不可以取消
// Cancelled -> 已经取消

fn main() {
    let order = OrderStatus::Pending;
    println!("待支付订单能否取消: {}", order.can_cancel());
    
    let order = OrderStatus::Shipped;
    println!("已发货订单能否取消: {}", order.can_cancel());
    
    let order = OrderStatus::Cancelled;
    println!("已取消订单能否取消: {}", order.can_cancel());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    fn can_cancel(&self) -> bool {
        match self {
            OrderStatus::Pending => true,
            OrderStatus::Cancelled => {
                println!("订单已取消，无需重复操作");
                false
            }
            _ => false,
        }
    }
}

fn main() {
    let order = OrderStatus::Pending;
    println!("待支付订单能否取消: {}", order.can_cancel());
    
    let order = OrderStatus::Shipped;
    println!("已发货订单能否取消: {}", order.can_cancel());
    
    let order = OrderStatus::Cancelled;
    println!("已取消订单能否取消: {}", order.can_cancel());
}
```

**说明：** 将状态相关的行为封装在枚举的方法中，是 Rust 的常见模式。`can_cancel` 根据当前状态返回 `bool`，调用者根据返回值决定是否执行取消操作。`match` 确保所有状态都被考虑。
</details>

### 练习 05-13: 嵌套枚举——枚举中包含枚举

> 难度：⭐⭐
> 类似 Java 的嵌套 sealed class，Rust 枚举可以直接包含另一个枚举作为变体的数据类型

定义 `Employee` 枚举，其变体 `Developer` 和 `Manager` 各自包含一个 `Role` 子枚举。

```rust
// TODO: 定义 Role 枚举（Junior, Mid, Senior）
// TODO: 定义 Employee 枚举
//   Developer(Role) - 携带角色
//   Manager { team_size: u32 } - 携带团队人数

// TODO: 为 Employee 实现 describe 方法
// Developer + Junior -> "初级开发"
// Developer + Mid -> "中级开发"
// Developer + Senior -> "高级开发"
// Manager -> 返回 "管理者，团队 {n} 人"

fn main() {
    let dev = Employee::Developer(Role::Senior);
    let mgr = Employee::Manager { team_size: 8 };
    
    println!("{}", dev.describe());
    println!("{}", mgr.describe());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Role {
    Junior,
    Mid,
    Senior,
}

enum Employee {
    Developer(Role),
    Manager { team_size: u32 },
}

impl Employee {
    fn describe(&self) -> String {
        match self {
            Employee::Developer(role) => match role {
                Role::Junior => "初级开发".to_string(),
                Role::Mid => "中级开发".to_string(),
                Role::Senior => "高级开发".to_string(),
            },
            Employee::Manager { team_size } => format!("管理者，团队 {} 人", team_size),
        }
    }
}

fn main() {
    let dev = Employee::Developer(Role::Senior);
    let mgr = Employee::Manager { team_size: 8 };
    
    println!("{}", dev.describe());
    println!("{}", mgr.describe());
}
```

**说明：** 嵌套枚举（一个枚举变体包含另一个枚举）在 Rust 中非常自然。模式匹配时可以使用嵌套 `match` 逐层解构。这是表达分层数据结构的清晰方式，比类继承体系更简洁。
</details>

### 练习 05-14: 嵌套枚举——通信协议

> 难度：⭐⭐
> 类似 Java 的嵌套 sealed class 表达协议层次

定义一个通信协议枚举 `Packet`，包含 `Data` 和 `Control` 两种类型的数据包，其中 `Data` 和 `Control` 自身也是枚举。

```rust
// TODO: 定义 DataType 枚举（Text(String), Binary(Vec<u8>)）
// TODO: 定义 ControlType 枚举（Ack, Nack, Ping, Pong）
// TODO: 定义 Packet 枚举（Data(DataType), Control(ControlType)）

// TODO: 为 Packet 实现 summary 方法，返回数据包的简短描述

fn main() {
    let p1 = Packet::Data(DataType::Text(String::from("Hello")));
    let p2 = Packet::Control(ControlType::Ack);
    let p3 = Packet::Data(DataType::Binary(vec![0, 1, 2, 255]));
    let p4 = Packet::Control(ControlType::Ping);
    
    println!("{}", p1.summary());
    println!("{}", p2.summary());
    println!("{}", p3.summary());
    println!("{}", p4.summary());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum DataType {
    Text(String),
    Binary(Vec<u8>),
}

enum ControlType {
    Ack,
    Nack,
    Ping,
    Pong,
}

enum Packet {
    Data(DataType),
    Control(ControlType),
}

impl Packet {
    fn summary(&self) -> String {
        match self {
            Packet::Data(dt) => match dt {
                DataType::Text(text) => format!("数据包: 文本 \"{}\"", text),
                DataType::Binary(bytes) => format!("数据包: 二进制 ({} 字节)", bytes.len()),
            },
            Packet::Control(ct) => match ct {
                ControlType::Ack => "控制包: 确认 (ACK)".to_string(),
                ControlType::Nack => "控制包: 否定确认 (NACK)".to_string(),
                ControlType::Ping => "控制包: Ping".to_string(),
                ControlType::Pong => "控制包: Pong".to_string(),
            },
        }
    }
}

fn main() {
    let p1 = Packet::Data(DataType::Text(String::from("Hello")));
    let p2 = Packet::Control(ControlType::Ack);
    let p3 = Packet::Data(DataType::Binary(vec![0, 1, 2, 255]));
    let p4 = Packet::Control(ControlType::Ping);
    
    println!("{}", p1.summary());
    println!("{}", p2.summary());
    println!("{}", p3.summary());
    println!("{}", p4.summary());
}
```

**说明：** 嵌套枚举非常适合建模层次化的协议或消息格式。每一层枚举只关心自己的变体，通过 `match` 嵌套可以逐层处理。这种设计比类继承体系更安全——编译器确保所有可能的组合都被处理。
</details>

### 练习 05-15: 支付系统综合练习

> 难度：⭐⭐⭐
> 类似 Java 中用 sealed class 建模支付方式，Rust 枚举是建模"互斥但不同数据"的自然选择

用枚举建模一个简单的支付系统：
1. 定义 `Payment` 枚举，包含三种支付方式：
   - `Cash`：无额外数据
   - `CreditCard { card_number: String, holder: String, cvv: String }`：信用卡信息
   - `WeChat { open_id: String, nickname: String }`：微信支付信息
2. 为 `Payment` 实现 `description` 方法，返回支付方式描述
3. 实现一个函数 `process_payment(amount: f64, payment: Payment) -> bool`，模拟支付处理

```rust
// TODO: 定义 Payment 枚举

// TODO: 实现 description 方法
// Cash -> "现金支付"
// CreditCard -> "信用卡支付（持卡人: XXX，卡号: ****XXXX）"（只显示后四位）
// WeChat -> "微信支付（用户: XXX）"

// TODO: 实现 process_payment 函数
// 如果金额 > 0 则打印处理信息并返回 true，否则返回 false

fn main() {
    let cash = Payment::Cash;
    let card = Payment::CreditCard {
        card_number: String::from("6222021234567890"),
        holder: String::from("张三"),
        cvv: String::from("123"),
    };
    let wechat = Payment::WeChat {
        open_id: String::from("oxU9T5n8abc123"),
        nickname: String::from("路人甲"),
    };

    println!("{}", cash.description());
    println!("{}", card.description());
    println!("{}", wechat.description());

    println!("\n--- 处理支付 ---");
    println!("处理结果: {}", process_payment(299.0, cash));
    println!("处理结果: {}", process_payment(-50.0, card)); // 无效金额
    println!("处理结果: {}", process_payment(888.0, wechat));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum Payment {
    Cash,
    CreditCard {
        card_number: String,
        holder: String,
        cvv: String,
    },
    WeChat {
        open_id: String,
        nickname: String,
    },
}

impl Payment {
    fn description(&self) -> String {
        match self {
            Payment::Cash => "现金支付".to_string(),
            Payment::CreditCard { card_number, holder, .. } => {
                let last_four = if card_number.len() >= 4 {
                    &card_number[card_number.len() - 4..]
                } else {
                    card_number.as_str()
                };
                format!("信用卡支付（持卡人: {}，卡号: ****{}）", holder, last_four)
            }
            Payment::WeChat { nickname, .. } => {
                format!("微信支付（用户: {}）", nickname)
            }
        }
    }
}

fn process_payment(amount: f64, payment: Payment) -> bool {
    if amount <= 0.0 {
        println!("❌ 金额无效: {}", amount);
        return false;
    }
    
    println!("✅ 成功处理 {:.2} 元 - {}", amount, payment.description());
    true
}

fn main() {
    let cash = Payment::Cash;
    let card = Payment::CreditCard {
        card_number: String::from("6222021234567890"),
        holder: String::from("张三"),
        cvv: String::from("123"),
    };
    let wechat = Payment::WeChat {
        open_id: String::from("oxU9T5n8abc123"),
        nickname: String::from("路人甲"),
    };

    println!("{}", cash.description());
    println!("{}", card.description());
    println!("{}", wechat.description());

    println!("\n--- 处理支付 ---");
    println!("处理结果: {}", process_payment(299.0, cash));
    println!("处理结果: {}", process_payment(-50.0, card));
    println!("处理结果: {}", process_payment(888.0, wechat));
}
```

**说明：** 这是一个综合练习，展示了 Rust 枚举在实际业务场景中的强大建模能力。`Payment` 枚举用三个变体表达了三种完全不同的支付方式，每种携带不同的数据。`description` 方法利用模式匹配解构不同变体中的数据。`process_payment` 展示了接收枚举作为参数的函数设计。这种用枚举建模"互斥但不同"实体的方式，比类继承体系更安全、更简洁，且编译器能保证所有情况都被覆盖。
</details>
