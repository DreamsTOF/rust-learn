# 08 泛型

泛型（Generics）是 Rust 中实现代码复用的核心机制，允许你编写适用于多种类型的函数、结构体、枚举和方法。Rust 的泛型在编译时进行单态化（monomorphization），因此不会带来运行时开销。本章练习将帮助你掌握泛型函数、泛型结构体、泛型约束（Trait Bound）、where 子句以及泛型 impl 块。

### 练习 08-01: 泛型函数——填空类型参数

> 难度：⭐⭐
> 类似 C++ 的 `template<typename T>`，在函数名后声明类型参数

补全代码中的泛型声明，使 `identity` 函数能够接受任意类型并原样返回。

```rust
// TODO: 在函数名后添加泛型参数声明
fn identity(value: T) -> T {
    value
}

fn main() {
    let n = identity(42);
    let s = identity("hello");
    let b = identity(true);
    println!("{n} {s} {b}");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let n = identity(42);
    let s = identity("hello");
    let b = identity(true);
    println!("{n} {s} {b}");
}
```

**说明：** 泛型函数在函数名后使用尖括号声明类型参数：`fn 函数名<T>(参数: T) -> T`。调用时 Rust 会根据传入参数自动推断类型，无需手动指定。
</details>

### 练习 08-02: 泛型函数——填空返回类型

> 难度：⭐⭐
> 类似 C++ 的 `template<typename T>`，但 Rust 返回类型也需要标注泛型

补全代码中的返回类型声明，使 `make_pair` 函数返回一个元组。

```rust
fn make_pair<T, U>(a: T, b: U) -> // TODO: 填写返回类型 {
    (a, b)
}

fn main() {
    let pair = make_pair(10, "ten");
    println!("{:?}", (pair.0, pair.1));
    // 注: 此处使用 Debug 格式化，仅用于验证
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn make_pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

fn main() {
    let pair = make_pair(10, "ten");
    println!("{:?}", (pair.0, pair.1));
}
```

**说明：** 当函数有多个泛型参数时，返回类型可以包含这些泛型参数的组合。这里 `(T, U)` 表示返回一个元组，元素类型分别为 `T` 和 `U`。
</details>

### 练习 08-03: 泛型函数——补全实现

> 难度：⭐⭐
> 类似 C++ 的 template<typename T>，编译器单态化生成具体代码

补全 `first` 函数体，使其返回切片中第一个元素的引用。

```rust
fn first<T>(slice: &[T]) -> &T {
    // TODO: 返回 slice 的第一个元素
}

fn main() {
    let numbers = vec![10, 20, 30];
    let words = vec!["apple", "banana", "cherry"];
    println!("{}", first(&numbers));
    println!("{}", first(&words));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn first<T>(slice: &[T]) -> &T {
    &slice[0]
}

fn main() {
    let numbers = vec![10, 20, 30];
    let words = vec!["apple", "banana", "cherry"];
    println!("{}", first(&numbers));
    println!("{}", first(&words));
}
```

**说明：** 泛型函数可以接受任意类型的切片引用。`&[T]` 表示任意类型 `T` 的切片引用，返回类型 `&T` 表示引用切片中的某个元素。输入输出的生命周期由 Rust 自动推断（生命周期省略规则）。
</details>

### 练习 08-04: 泛型函数——多个类型参数

> 难度：⭐⭐
> 类似 C++ 的 `template<typename T, typename U>`，支持多个泛型参数

补全 `swap` 函数体，交换两个可变引用指向的值。

```rust
fn swap<T>(a: &mut T, b: &mut T) {
    // TODO: 交换 a 和 b 指向的值
}

fn main() {
    let mut x = 10;
    let mut y = 20;
    swap(&mut x, &mut y);
    println!("x = {x}, y = {y}"); // 应输出 x = 20, y = 10

    let mut a = "hello";
    let mut b = "world";
    swap(&mut a, &mut b);
    println!("a = {a}, b = {b}"); // 应输出 a = world, b = hello
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

fn main() {
    let mut x = 10;
    let mut y = 20;
    swap(&mut x, &mut y);
    println!("x = {x}, y = {y}");

    let mut a = "hello";
    let mut b = "world";
    swap(&mut a, &mut b);
    println!("a = {a}, b = {b}");
}
```

**说明：** 函数内部也可以使用标准库的 `std::mem::swap` 来实现交换，但练习的目标是理解泛型参数使函数适用于任意类型。两个参数必须是**相同类型** `T`，若类型不同则编译错误。
</details>

### 练习 08-05: 挑战——实现泛型最大值函数

> 难度：⭐⭐⭐
> 类似 C++ 的 std::max<T>，但 Rust 需要 PartialOrd 约束

实现一个泛型函数 `max_val`，接受两个参数并返回较大的那个。注意：不是所有类型都可以比较大小，需要使用 trait 约束。

```rust
// TODO: 实现 max_val 泛型函数（需要添加合适的 trait 约束）

fn main() {
    println!("{}", max_val(10, 20));       // 20
    println!("{}", max_val(3.14, 2.72));   // 3.14
    println!("{}", max_val("abc", "xyz")); // xyz
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn max_val<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn main() {
    println!("{}", max_val(10, 20));
    println!("{}", max_val(3.14, 2.72));
    println!("{}", max_val("abc", "xyz"));
}
```

**说明：** 并非所有类型都能比较大小，因此需要在泛型参数上添加 `PartialOrd` 约束（对应 `>`、`<` 运算符）。这里返回值直接使用 `a` 或 `b` 会发生所有权移动，但函数签名中 `T` 不涉及引用，所以没有问题。如果 `T` 是 `&str`，那么比较的是字符串的字典序。
</details>

### 练习 08-06: 定义泛型结构体

> 难度：⭐⭐
> 类似 C++ 的 template<typename T> class，Rust 泛型结构体在名字后加 <T>

定义一个泛型结构体 `Point<T>`，包含 `x` 和 `y` 两个字段，类型均为 `T`。

```rust
// TODO: 定义 Point<T> 结构体

fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 3.2 };
    println!("int: ({}, {}), float: ({}, {})", int_point.x, int_point.y, float_point.x, float_point.y);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 3.2 };
    println!("int: ({}, {}), float: ({}, {})", int_point.x, int_point.y, float_point.x, float_point.y);
}
```

**说明：** 泛型结构体使用 `struct 名称<T> { 字段: T }` 定义。这里 `x` 和 `y` 都是 `T` 类型，因此它们必须是相同类型。若需要不同类型，可以定义多个泛型参数如 `Point<T, U>`。
</details>

### 练习 08-07: 泛型结构体——不同类型字段

> 难度：⭐⭐
> 类似 C++ 的 pair<T, U>，Rust 结构体支持多个泛型参数

定义一个泛型结构体 `Pair<T, U>`，包含 `first` 和 `second` 两个字段，类型分别为 `T` 和 `U`。

```rust
// TODO: 定义 Pair<T, U> 结构体

fn main() {
    let pair = Pair { first: 42, second: "answer" };
    println!("Pair({}, {})", pair.first, pair.second);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let pair = Pair { first: 42, second: "answer" };
    println!("Pair({}, {})", pair.first, pair.second);
}
```

**说明：** 多个泛型参数用逗号分隔。`Pair<T, U>` 允许 `first` 和 `second` 是不同类型，Rust 会为每个具体组合生成独立的类型。这类似于 C++ 的 `std::pair<T, U>`。
</details>

### 练习 08-08: 泛型结构体——补全方法

> 难度：⭐⭐
> 类似 C++ 模板类的成员函数，Rust 在 impl<T> 块中添加方法

为 `Point<T>` 实现一个 `x` 方法，返回 `x` 字段的引用。

```rust
struct Point<T> {
    x: T,
    y: T,
}

// TODO: 为 Point<T> 实现一个方法 x(&self) -> &T

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("p.x = {}", p.x());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("p.x = {}", p.x());
}
```

**说明：** 为泛型结构体实现方法时，`impl<T>` 中的 `<T>` 表示实现块也是泛型的。`Point<T>` 中的 `T` 与 `impl<T>` 中的 `T` 是同一个类型参数。方法签名中的 `&self` 隐含有生命周期，因此返回 `&T` 是合法的。
</details>

### 练习 08-09: 泛型结构体——为具体类型实现方法

> 难度：⭐⭐
> 类似 C++ 的模板特化，但 Rust 的 impl 块可以针对具体类型

为 `Point<f64>` 实现一个 `distance_from_origin` 方法，计算到原点的距离。

```rust
struct Point<T> {
    x: T,
    y: T,
}

// TODO: 只为 Point<f64> 实现 distance_from_origin 方法
// 提示: 使用 sqrt 方法计算平方根，需要调用 .sqrt()

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("distance = {:.1}", p.distance_from_origin()); // 应输出 5.0
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("distance = {:.1}", p.distance_from_origin());
}
```

**说明：** 与 `impl<T> Point<T>` 不同，`impl Point<f64>` 只为 `T = f64` 这一具体类型实现方法。这是 Rust 泛型的特色之一——你可以为泛型类型限定某些方法只存在于特定的类型参数上。
</details>

### 练习 08-10: 挑战——泛型结构体综合

> 难度：⭐⭐⭐
> 类似 C++ 模板类，但 Rust 需要在 impl 块重复声明泛型参数

定义一个泛型结构体 `Wrapper<T>`，它包装一个 `T` 类型的值。为它实现：
1. `new(value: T) -> Wrapper<T>` 关联函数
2. `value(&self) -> &T` 方法，返回内部值的引用
3. `map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U>` 方法，对内部值应用一个函数并返回新的 `Wrapper`

```rust
// TODO: 定义 Wrapper<T> 并实现上述方法

fn main() {
    let w = Wrapper::new(42);
    println!("{}", w.value()); // 42
    let w2 = w.map(|x| x * 2);
    println!("{}", w2.value()); // 84
    let w3 = w2.map(|x| format!("答案是: {x}"));
    println!("{}", w3.value()); // 答案是: 84
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Wrapper<T> {
        Wrapper { value }
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn map<U>(self, f: impl FnOnce(T) -> U) -> Wrapper<U> {
        Wrapper { value: f(self.value) }
    }
}

fn main() {
    let w = Wrapper::new(42);
    println!("{}", w.value());
    let w2 = w.map(|x| x * 2);
    println!("{}", w2.value());
    let w3 = w2.map(|x| format!("答案是: {x}"));
    println!("{}", w3.value());
}
```

**说明：** `map` 方法引入了新的泛型参数 `U`，使得 `Wrapper<T>` 可以转换为 `Wrapper<U>`。`impl FnOnce(T) -> U` 是闭包参数的简写，表示接受一个 `T` 类型参数并返回 `U` 类型的可调用对象。注意 `map` 消耗 `self`（所有权转移），因为内部值被移出后用于构造新的包装器。
</details>

### 练习 08-11: 泛型约束——Display

> 难度：⭐⭐
> 类似 C++ 的 template<typename T> 配合 requires，Rust 用冒号指定 trait 约束

补全代码，为泛型参数添加 `std::fmt::Display` 约束，使 `print_val` 函数能使用 `{}` 格式化打印。

```rust
// TODO: 添加 trait 约束使函数能打印值
fn print_val(val: T) {
    println!("值: {val}");
}

fn main() {
    print_val(42);
    print_val(3.14);
    print_val("hello");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn print_val<T: std::fmt::Display>(val: T) {
    println!("值: {val}");
}

fn main() {
    print_val(42);
    print_val(3.14);
    print_val("hello");
}
```

**说明：** `T: std::fmt::Display` 是 trait 约束，表示 `T` 必须实现了 `Display` trait。这确保了 `println!("值: {val}")` 能正常工作。Rust 中基本类型和字符串都实现了 `Display`。
</details>

### 练习 08-12: where 子句

> 难度：⭐⭐
> Rust 特色语法，当约束较多时使用 where 子句更清晰

将以下函数的 trait 约束改写为 `where` 子句形式。

```rust
fn print_pairs<T: std::fmt::Display, U: std::fmt::Display>(a: T, b: U) {
    println!("({a}, {b})");
}

// TODO: 使用 where 子句重写上面的 print_pairs

fn main() {
    print_pairs(10, "hello");
    print_pairs(3.14, true);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn print_pairs<T, U>(a: T, b: U)
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    println!("({a}, {b})");
}

fn main() {
    print_pairs(10, "hello");
    print_pairs(3.14, true);
}
```

**说明：** `where` 子句放在函数签名尾部、返回值之后。当泛型参数多或约束复杂时，`where` 子句让函数签名更清晰。多个约束用逗号分隔，每个约束一行是惯例写法。
</details>

### 练习 08-13: 多约束

> 难度：⭐⭐
> 类似 C++ 的多个概念约束，Rust 用 + 连接多个 trait

补全函数，要求 `T` 同时实现 `Display` 和 `PartialOrd`，使得函数既能打印又能比较大小。

```rust
// TODO: 为 T 添加 Display + PartialOrd 约束
fn describe_cmp<T>(a: T, b: T) {
    if a >= b {
        println!("{a} >= {b}");
    } else {
        println!("{a} < {b}");
    }
}

fn main() {
    describe_cmp(10, 20);
    describe_cmp("abc", "abc");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn describe_cmp<T: std::fmt::Display + PartialOrd>(a: T, b: T) {
    if a >= b {
        println!("{a} >= {b}");
    } else {
        println!("{a} < {b}");
    }
}

fn main() {
    describe_cmp(10, 20);
    describe_cmp("abc", "abc");
}
```

**说明：** 多个 trait 约束用 `+` 连接：`T: Display + PartialOrd`。这意味着 `T` 必须同时实现这两个 trait。标准库中的 `Display` 和 `PartialOrd` 是最常用的约束之一。
</details>

### 练习 08-14: where 子句与多类型参数

> 难度：⭐⭐
> Rust 特色语法，where 子句可以为不同泛型参数指定不同约束

补全函数，使用 `where` 子句要求 `T` 实现 `Display`，`U` 实现 `Debug`。

```rust
fn inspect<T, U>(t: T, u: U)
// TODO: 使用 where 子句添加约束
// T: Display, U: Debug
{
    println!("T: {t}");
    println!("U: {:?}", u);
}

fn main() {
    inspect(42, vec![1, 2, 3]);
    inspect("hello", [4, 5, 6]);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn inspect<T, U>(t: T, u: U)
where
    T: std::fmt::Display,
    U: std::fmt::Debug,
{
    println!("T: {t}");
    println!("U: {:?}", u);
}

fn main() {
    inspect(42, vec![1, 2, 3]);
    inspect("hello", [4, 5, 6]);
}
```

**说明：** `where` 子句可以为每个泛型参数分别指定不同约束。这里 `T` 需要 `Display`（用 `{}` 输出），`U` 需要 `Debug`（用 `{:?}` 输出）。`Vec` 和数组实现了 `Debug` 但未实现 `Display`，因此不能混用约束。
</details>

### 练习 08-15: 挑战——多约束综合

> 难度：⭐⭐⭐
> 类似 C++ 的 requires 子句，Rust 的 where 子句支持复杂约束

实现一个泛型函数 `top_two`，接收一个可变的切片引用，将最大的两个元素按降序放入前两个位置。需要 `T` 支持比较和交换。

```rust
// TODO: 实现 top_two 函数
// 要求: 修改 slice，使前两个元素是最大的两个（降序）
// 约束: T 需要 PartialOrd（比较）和 Clone（避免所有权问题）
// 提示: 可以用冒泡思路或直接找出最大和第二大

fn main() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    top_two(&mut data);
    println!("{:?}", &data[..2]); // 应输出 [9, 6]（顺序可能不同）
    
    let mut words = vec!["banana", "apple", "cherry", "date"];
    top_two(&mut words);
    println!("{:?}", &words[..2]); // 按字典序最大的两个
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn top_two<T>(slice: &mut [T])
where
    T: PartialOrd,
{
    if slice.len() < 2 {
        return;
    }
    // 冒泡一次将最大值移到开头，第二次找到第二大
    for i in 1..slice.len() {
        if slice[i] > slice[0] {
            slice.swap(0, i);
        }
    }
    for i in 2..slice.len() {
        if slice[i] > slice[1] {
            slice.swap(1, i);
        }
    }
}

fn main() {
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    top_two(&mut data);
    println!("{:?}", &data[..2]);
    
    let mut words = vec!["banana", "apple", "cherry", "date"];
    top_two(&mut words);
    println!("{:?}", &words[..2]);
}
```

**说明：** `where T: PartialOrd` 约束使得 `>` 比较可用。`slice.swap(i, j)` 是切片的内置方法，交换两个位置的元素。这里不需要 `Clone`，因为 `swap` 直接交换内存而不需要复制值。`&data[..2]` 是切片语法，取前两个元素。
</details>

### 练习 08-16: 泛型 impl——基本

> 难度：⭐⭐
> Rust 特色语法，impl<T> 为整个泛型类型注入方法

为 `Container<T>` 实现一个 `new` 关联函数和一个 `get` 方法。

```rust
struct Container<T> {
    value: T,
}

// TODO: 为 Container<T> 实现 new 和 get 方法

fn main() {
    let c = Container::new(100);
    println!("{}", c.get());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    let c = Container::new(100);
    println!("{}", c.get());
}
```

**说明：** `impl<T> Container<T>` 表示这个实现块适用于所有 `T` 类型。`new` 是关联函数（不接收 `self`），`get` 是方法（接收 `&self`）。注意 `get` 返回 `&T` 以避免所有权转移。
</details>

### 练习 08-17: 泛型 impl——with 约束

> 难度：⭐⭐
> Rust 特色语法，impl<T: Trait> 只有在 T 满足约束时才有对应方法

为 `Container<T>` 实现一个 `describe` 方法，要求 `T` 实现 `Display`。同时为 `Container<T>` 实现一个 `debug` 方法，要求 `T` 实现 `Debug`。

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
}

// TODO: 为 Container<T> 实现 describe 方法（要求 T: Display）
// TODO: 为 Container<T> 实现 debug 方法（要求 T: Debug）

fn main() {
    let c = Container::new(42);
    c.describe(); // 输出: 值是 42
    c.debug();    // 输出: 值是 42
    
    let c2 = Container::new(vec![1, 2, 3]);
    // c2.describe(); // Vec<i32> 未实现 Display，此行取消注释会编译错误
    c2.debug();    // 输出: 值是 [1, 2, 3]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt;

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
}

impl<T: fmt::Display> Container<T> {
    fn describe(&self) {
        println!("值是 {value}", value = self.value);
    }
}

impl<T: fmt::Debug> Container<T> {
    fn debug(&self) {
        println!("值是 {:?}", self.value);
    }
}

fn main() {
    let c = Container::new(42);
    c.describe();
    c.debug();
    
    let c2 = Container::new(vec![1, 2, 3]);
    // c2.describe(); // Vec<i32> 未实现 Display
    c2.debug();
}
```

**说明：** Rust 允许为同一个泛型类型编写多个 `impl` 块，每个块可以有不同的 trait 约束。这使得只有在 `T` 满足特定约束时，对应的方法才可用。这是一种条件性方法实现的机制，是 Rust 泛型的强大特性。
</details>

### 练习 08-18: 泛型 impl——为具体类型添加额外方法

> 难度：⭐⭐
> Rust 特色语法，可为特定泛型参数实现独有方法

为 `Container<String>` 实现一个 `len` 方法，返回内部字符串的长度。

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
}

// TODO: 为 Container<String> 实现 len 方法

fn main() {
    let c = Container::new(String::from("hello"));
    println!("length = {}", c.len());
    
    let c2 = Container::new(42);
    // c2.len(); // ❌ Container<i32> 没有 len 方法
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
}

impl Container<String> {
    fn len(&self) -> usize {
        self.value.len()
    }
}

fn main() {
    let c = Container::new(String::from("hello"));
    println!("length = {}", c.len());
    
    let c2 = Container::new(42);
    // c2.len(); // Container<i32> 没有 len 方法
}
```

**说明：** `impl Container<String>` 为 `T = String` 这一具体类型添加方法。这种机制让你能为特定类型参数添加额外功能，而不影响其他类型参数的实例。类似 C++ 的模板特化，但 Rust 通过独立的 `impl` 块实现。
</details>

### 练习 08-19: 泛型 impl——where 子句

> 难度：⭐⭐
> Rust 特色语法，impl 块也可以使用 where 子句简化约束

使用 `where` 子句改写以下代码，使 `is_same` 方法能够比较两个 `Container<T>` 是否相等。

```rust
use std::fmt::Debug;

struct Container<T> {
    value: T,
}

// TODO: 使用 where 子句添加 PartialEq 约束，实现 is_same 方法

fn main() {
    let c1 = Container { value: 10 };
    let c2 = Container { value: 10 };
    let c3 = Container { value: 20 };
    println!("c1 == c2: {}", c1.is_same(&c2)); // true
    println!("c1 == c3: {}", c1.is_same(&c3)); // false
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fmt::Debug;

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
}

impl<T: PartialEq> Container<T> {
    fn is_same(&self, other: &Container<T>) -> bool {
        self.value == other.value
    }
}

fn main() {
    let c1 = Container { value: 10 };
    let c2 = Container { value: 10 };
    let c3 = Container { value: 20 };
    println!("c1 == c2: {}", c1.is_same(&c2));
    println!("c1 == c3: {}", c1.is_same(&c3));
}
```

**说明：** `impl<T: PartialEq> Container<T>` 表示只有 `T` 实现了 `PartialEq` 时，`Container<T>` 才有 `is_same` 方法。`self.value == other.value` 依赖于 `PartialEq` trait 提供的 `eq` 方法。也可以写为 `impl<T> Container<T> where T: PartialEq { ... }`。
</details>

### 练习 08-20: 挑战——设计一个泛型容器

> 难度：⭐⭐⭐
> 综合运用，类似设计一个简化版的 Option 或简单的泛型容器

设计一个泛型枚举 `Result<T, E>` 的简化版本——`SimpleResult<T, E>`，包含 `Ok(T)` 和 `Err(E)` 两个变体。为它实现：
1. `is_ok(&self) -> bool` 和 `is_err(&self) -> bool`
2. `ok(self) -> Option<T>` —— 将 `SimpleResult` 转换为 `Option<T>`（Ok 返回 `Some(T)`，Err 返回 `None`）
3. `unwrap(self) -> T` —— Ok 时返回值，Err 时调用 `panic!`
4. `map<U>(self, f: impl FnOnce(T) -> U) -> SimpleResult<U, E>` —— 只对 Ok 变体应用函数

```rust
// TODO: 定义 SimpleResult<T, E> 枚举

// TODO: 实现上述方法

fn main() {
    let ok: SimpleResult<i32, &str> = SimpleResult::Ok(42);
    let err: SimpleResult<i32, &str> = SimpleResult::Err("出错了");
    
    println!("ok.is_ok() = {}", ok.is_ok());
    println!("err.is_err() = {}", err.is_err());
    
    println!("ok.ok() = {:?}", ok.ok());
    println!("err.ok() = {:?}", err.ok());
    
    // println!("err.unwrap()"); // 会 panic
    
    let mapped = SimpleResult::Ok(10).map(|x| x * 2);
    println!("mapped.ok() = {:?}", mapped.ok()); // Some(20)
    
    let mapped_err = SimpleResult::Err("error").map(|x: i32| x * 2);
    println!("mapped_err.is_err() = {}", mapped_err.is_err()); // true
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum SimpleResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> SimpleResult<T, E> {
    fn is_ok(&self) -> bool {
        matches!(self, SimpleResult::Ok(_))
    }

    fn is_err(&self) -> bool {
        matches!(self, SimpleResult::Err(_))
    }

    fn ok(self) -> Option<T> {
        match self {
            SimpleResult::Ok(val) => Some(val),
            SimpleResult::Err(_) => None,
        }
    }

    fn unwrap(self) -> T {
        match self {
            SimpleResult::Ok(val) => val,
            SimpleResult::Err(e) => panic!("called unwrap on an Err value: {:?}", e),
        }
    }

    fn map<U>(self, f: impl FnOnce(T) -> U) -> SimpleResult<U, E> {
        match self {
            SimpleResult::Ok(val) => SimpleResult::Ok(f(val)),
            SimpleResult::Err(e) => SimpleResult::Err(e),
        }
    }
}

// 为 Err 添加 Debug 约束以支持 unwrap 中的 panic 输出
impl<T: std::fmt::Debug, E: std::fmt::Debug> SimpleResult<T, E> {
    fn unwrap_debug(self) -> T {
        match self {
            SimpleResult::Ok(val) => val,
            SimpleResult::Err(e) => panic!("called unwrap on an Err value: {:?}", e),
        }
    }
}

fn main() {
    let ok: SimpleResult<i32, &str> = SimpleResult::Ok(42);
    let err: SimpleResult<i32, &str> = SimpleResult::Err("出错了");
    
    println!("ok.is_ok() = {}", ok.is_ok());
    println!("err.is_err() = {}", err.is_err());
    
    println!("ok.ok() = {:?}", ok.ok());
    println!("err.ok() = {:?}", err.ok());
    
    let mapped = SimpleResult::Ok(10).map(|x| x * 2);
    println!("mapped.ok() = {:?}", mapped.ok());
    
    let mapped_err = SimpleResult::Err("error").map(|x: i32| x * 2);
    println!("mapped_err.is_err() = {}", mapped_err.is_err());
}
```

**说明：** 这是标准库 `Result<T, E>` 的简化实现。`matches!` 宏可以简洁地匹配枚举变体。`map` 方法接收一个闭包，只在 `Ok` 变体上应用，`Err` 变体保持不变。`impl<T, E>` 中的泛型参数在整个 impl 块中可见，而 `map<U>` 引入了额外的泛型参数 `U`。注意 `unwrap` 如果直接 panic 使用 `{:?}` 输出 `E`，需要 `E: Debug` 约束，因此可以用单独的 impl 块处理，或者简化 panic 消息。
</details>
