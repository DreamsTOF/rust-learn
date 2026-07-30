# 16 智能指针

Rust 的智能指针是实现了 `Deref` 和 `Drop` trait 的数据结构，它们不仅拥有数据的所有权，还提供了额外的元数据或功能。与 C++ 的智能指针类似，Rust 提供了 `Box<T>`（类似 `std::unique_ptr`）、`Rc<T>`（类似单线程 `std::shared_ptr`）、`Arc<T>`（类似多线程 `std::shared_ptr`）和 `Weak<T>`（类似 `std::weak_ptr`）。本章练习将帮助你掌握各种智能指针的用法及其适用场景。

### 练习 16-01: 使用 Box 在堆上分配数据

> 难度：⭐⭐
> 类似 C++ 的 std::unique_ptr（make_unique）

填空，使用 `Box::new` 在堆上分配一个整数，并通过解引用修改其值。

```rust
fn main() {
    // TODO: 使用 Box::new 在堆上分配整数 42
    let x = ??;

    // TODO: 通过解引用修改 x 的值为 100
    ??;

    println!("x = {}", x);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut x = Box::new(42);
    *x = 100;
    println!("x = {}", x);
}
```

**说明：** `Box::new` 在堆上分配内存并返回一个指向堆数据的智能指针。通过 `*` 解引用操作符可以读写堆上的数据，类似 C++ 中 `*unique_ptr`。
</details>

### 练习 16-02: 使用 Box 创建递归类型（链表）

> 难度：⭐⭐
> 类似 C++ 的 unique_ptr 实现链表节点

补全代码，使用 `Box` 定义一个递归的链表类型，并创建一个包含三个节点的链表。

```rust
// TODO: 使用 Box 定义 List 枚举
// 提示：需要两个变体：Cons(i32, List) 和 Nil
// 但 List 出现在自身定义中，需要用 Box 间接存储
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // TODO: 创建链表 1 -> 2 -> 3 -> Nil
    let list = ??;

    // 遍历链表（已实现）
    let mut current = &list;
    loop {
        match current {
            List::Cons(value, next) => {
                print!("{} ", value);
                current = next;
            }
            List::Nil => {
                println!();
                break;
            }
        }
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    let mut current = &list;
    loop {
        match current {
            List::Cons(value, next) => {
                print!("{} ", value);
                current = next;
            }
            List::Nil => {
                println!();
                break;
            }
        }
    }
}
```

**说明：** Rust 需要在编译时知道所有类型的大小。递归类型的大小无法直接确定，因此需要用 `Box`（堆分配，大小固定为指针大小）来间接存储递归部分。这类似于 C++ 中 `unique_ptr<ListNode>` 指向下一个节点。
</details>

### 练习 16-03: 使用 Box 作为 trait 对象

> 难度：⭐⭐
> 类似 C++ 中通过 unique_ptr<Base> 调用虚函数

补全代码，使用 `Box<dyn Animal>` 创建动物集合，实现运行时多态。

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("旺旺！");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("喵喵！");
    }
}

fn main() {
    // TODO: 创建一个 Vec<Box<dyn Animal>>，放入一只 Dog 和一只 Cat
    // 然后遍历并调用 speak
    
    let animals: Vec<Box<dyn Animal>> = ??;

    for animal in animals {
        animal.speak();
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("旺旺！");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("喵喵！");
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals {
        animal.speak();
    }
}
```

**说明：** `Box<dyn Trait>` 是 Rust 中实现运行时多态的方式之一，类似 C++ 中通过 `unique_ptr<Base>` 调用虚函数。`dyn` 关键字表示动态分发，编译器会为 trait 对象生成虚表（vtable），在运行时确定具体调用哪个方法。
</details>

### 练习 16-04: Box 的所有权和移动语义

> 难度：⭐⭐
> 类似 C++ unique_ptr 的移动语义

填空，观察 Box 的所有权移动行为。

```rust
fn main() {
    let a = Box::new(5);
    println!("a = {}", a);

    // TODO: 将 a 移动到 b（Box 只有移动语义，没有拷贝语义）
    let b = ??;

    // 这行如果取消注释会编译错误，因为 a 已被移动
    // println!("a = {}", a);

    println!("b = {}", b);

    // TODO: 从函数返回 Box
    let c = create_box(10);
    println!("c = {}", c);
}

// TODO: 补全函数签名，返回一个 Box<i32>
fn create_box(value: i32) ?? {
    Box::new(value)
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let a = Box::new(5);
    println!("a = {}", a);

    let b = a;
    // println!("a = {}", a); // 编译错误：a 已被移动

    println!("b = {}", b);

    let c = create_box(10);
    println!("c = {}", c);
}

fn create_box(value: i32) -> Box<i32> {
    Box::new(value)
}
```

**说明：** `Box<T>` 拥有唯一所有权，类似 C++ 的 `unique_ptr`。赋值操作 `let b = a` 会转移所有权（move），而不是拷贝。函数返回 `Box<T>` 时所有权也会转移给调用者。
</details>

### 练习 16-05: 挑战 — 实现简单的 MyBox

> 难度：⭐⭐
> 类似 C++ 中实现简易 unique_ptr

实现一个自定义的 `MyBox<T>`，支持创建、解引用和自动释放。

```rust
use std::ops::Deref;

// TODO: 定义 MyBox<T> 结构体，包含一个 Box<T> 成员
struct MyBox<T>(??);

impl<T> MyBox<T> {
    // TODO: 实现 new 关联函数
    fn new(x: T) -> MyBox<T> {
        ???
    }
}

// TODO: 为 MyBox<T> 实现 Deref trait，使其可以被 * 解引用
impl<T> Deref for MyBox<T> {
    type Target = ??;

    fn deref(&self) -> &Self::Target {
        ???
    }
}

fn main() {
    let x = MyBox::new(42);
    assert_eq!(*x, 42);
    println!("通过测试！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::ops::Deref;

struct MyBox<T>(Box<T>);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(Box::new(x))
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = MyBox::new(42);
    assert_eq!(*x, 42);
    println!("通过测试！");
}
```

**说明：** 通过实现 `Deref` trait，编译器允许对 `MyBox<T>` 使用 `*` 解引用操作符。`deref` 返回内部数据的引用，使得 `*x` 自动转换为 `*(x.deref())`。这类似于 C++ 中 `unique_ptr` 重载了 `operator*` 和 `operator->`。
</details>

### 练习 16-06: Deref 强制转换（函数传参）

> 难度：⭐⭐
> 类似 C++ 中 unique_ptr 到原始指针的隐式转换

填空，观察 Deref 强制转换如何使 `&Box<T>` 自动变为 `&T`。

```rust
fn hello(name: &str) {
    println!("你好，{}！", name);
}

fn main() {
    let m = Box::new(String::from("Rust"));

    // TODO: 调用 hello 函数，传入 &m
    // 这里会发生 Deref 强制转换：&Box<String> -> &String -> &str
    hello(??);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn hello(name: &str) {
    println!("你好，{}！", name);
}

fn main() {
    let m = Box::new(String::from("Rust"));
    hello(&m);
}
```

**说明：** Deref 强制转换（Deref Coercion）是 Rust 在传参、赋值等场景下自动将实现了 `Deref` 的类型的引用转换为目标类型的引用。这里 `&Box<String>` 通过 `Deref` 转换为 `&String`，再通过 `String` 的 `Deref` 转换为 `&str`。这个机制类似 C++ 中 `unique_ptr` 隐式转换为原始指针，但 Rust 的转换是类型安全的。
</details>

### 练习 16-07: 连续的 Deref 强制转换

> 难度：⭐⭐
> 类似 C++ 的 operator-> 链式调用

补全代码，定义多层包装类型，观察 Rust 如何自动进行多层 Deref 转换。

```rust
use std::ops::Deref;

struct A(String);
struct B(A);
struct C(B);

impl Deref for A {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl Deref for B {
    type Target = A;
    fn deref(&self) -> &A {
        &self.0
    }
}

impl Deref for C {
    type Target = B;
    fn deref(&self) -> &B {
        &self.0
    }
}

fn main() {
    let c = C(B(A(String::from("Hello"))));

    // TODO: 直接调用 len() 方法，利用 Deref 强制转换
    // C -> B -> A -> String -> &str（调用 len）
    let length = ??;
    println!("length = {}", length);

    // TODO: 直接调用 is_empty() 方法
    let empty = ??;
    println!("empty = {}", empty);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::ops::Deref;

struct A(String);
struct B(A);
struct C(B);

impl Deref for A {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl Deref for B {
    type Target = A;
    fn deref(&self) -> &A {
        &self.0
    }
}

impl Deref for C {
    type Target = B;
    fn deref(&self) -> &B {
        &self.0
    }
}

fn main() {
    let c = C(B(A(String::from("Hello"))));
    let length = c.len();
    println!("length = {}", length);

    let empty = c.is_empty();
    println!("empty = {}", empty);
}
```

**说明：** Rust 的 Deref 强制转换可以连续进行多层，直到找到所需的类型。`c.len()` 会沿着 `C → B → A → String` 的 Deref 链一路转换，最终调用 `String::len()`。这类似于 C++ 中 `operator->` 的链式调用。
</details>

### 练习 16-08: Drop trait 基础

> 难度：⭐⭐
> 类似 C++ 的析构函数（destructor）

填空，实现 `Drop` trait 并观察变量离开作用域时的自动清理。

```rust
struct Timer {
    name: String,
}

// TODO: 为 Timer 实现 Drop trait
// 在 drop 方法中打印 "{name} 计时器已销毁"
impl Drop for Timer {
    fn drop(&mut self) {
        ???
    }
}

fn main() {
    let _a = Timer { name: String::from("A") };
    let _b = Timer { name: String::from("B") };
    println!("main 函数执行中");
    // _b 和 _a 会在 main 末尾按逆序自动销毁
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Timer {
    name: String,
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("{} 计时器已销毁", self.name);
    }
}

fn main() {
    let _a = Timer { name: String::from("A") };
    let _b = Timer { name: String::from("B") };
    println!("main 函数执行中");
}
```

**说明：** `Drop` trait 类似 C++ 的析构函数，当值离开作用域时自动调用。Rust 不允许手动调用 `drop` 方法（但提供了 `std::mem::drop` 函数），变量按创建顺序的逆序销毁（后进先出）。
</details>

### 练习 16-09: std::mem::drop 提前释放

> 难度：⭐⭐
> 类似 C++ 中通过 std::unique_ptr::reset 提前释放

填空，使用 `std::mem::drop` 函数在变量离开作用域之前提前释放资源。

```rust
struct Resource {
    id: u32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("资源 #{} 已释放", self.id);
    }
}

fn main() {
    let a = Resource { id: 1 };
    let b = Resource { id: 2 };

    // TODO: 提前释放 b，使其不再等待作用域结束
    ???

    println!("b 已被提前释放，但 a 还在作用域中");
    // a 在这里自动释放
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Resource {
    id: u32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("资源 #{} 已释放", self.id);
    }
}

fn main() {
    let a = Resource { id: 1 };
    let b = Resource { id: 2 };

    drop(b);

    println!("b 已被提前释放，但 a 还在作用域中");
}
```

**说明：** `std::mem::drop` 接受一个值并立即调用其 `Drop::drop` 方法，释放资源。这类似 C++ 中 `unique_ptr::reset()`。drop 后该值不再可用，Rust 编译器会阻止后续使用。
</details>

### 练习 16-10: 组合 Deref 和 Drop

> 难度：⭐⭐
> 类似 C++ 实现 RAII 包装器

补全代码，实现一个日志智能指针，记录值的创建、访问和销毁。

```rust
use std::ops::Deref;

struct LogBox<T> {
    value: T,
    name: String,
}

impl<T> LogBox<T> {
    fn new(value: T, name: &str) -> LogBox<T> {
        println!("创建 LogBox<{}>", name);
        LogBox { value, name: String::from(name) }
    }
}

// TODO: 实现 Deref，访问时打印 "访问 LogBox<{name}>"
impl<T> Deref for LogBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        ???
    }
}

// TODO: 实现 Drop，销毁时打印 "销毁 LogBox<{name}>"
impl<T> Drop for LogBox<T> {
    fn drop(&mut self) {
        ???
    }
}

fn main() {
    let x = LogBox::new(42, "x");
    println!("x = {}", *x);
    println!("x = {}", *x);
    // 离开作用域时自动销毁
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::ops::Deref;

struct LogBox<T> {
    value: T,
    name: String,
}

impl<T> LogBox<T> {
    fn new(value: T, name: &str) -> LogBox<T> {
        println!("创建 LogBox<{}>", name);
        LogBox { value, name: String::from(name) }
    }
}

impl<T> Deref for LogBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("访问 LogBox<{}>", self.name);
        &self.value
    }
}

impl<T> Drop for LogBox<T> {
    fn drop(&mut self) {
        println!("销毁 LogBox<{}>", self.name);
    }
}

fn main() {
    let x = LogBox::new(42, "x");
    println!("x = {}", *x);
    println!("x = {}", *x);
}
```

**说明：** 智能指针通常同时实现 `Deref`（提供透明访问）和 `Drop`（提供自动清理）。这是 Rust RAII（资源获取即初始化）哲学的典型体现，与 C++ 智能指针的设计理念一致。
</details>

### 练习 16-11: 使用 Rc 共享数据

> 难度：⭐⭐
> 类似 C++ 的 std::shared_ptr（单线程版）

填空，使用 `Rc<T>` 在多个变量之间共享数据的所有权。

```rust
use std::rc::Rc;

fn main() {
    // TODO: 使用 Rc::new 创建共享数据
    let data = Rc::new(String::from("共享数据"));

    // TODO: 克隆 Rc 指针（非克隆数据本身）
    let a = ??;
    let b = ??;

    // 验证所有引用都指向同一份数据
    println!("data: {}", data);
    println!("a: {}", a);
    println!("b: {}", b);

    // TODO: 打印当前引用计数（应输出 3）
    println!("引用计数: {}", ??);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("共享数据"));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);

    println!("data: {}", data);
    println!("a: {}", a);
    println!("b: {}", b);

    println!("引用计数: {}", Rc::strong_count(&data));
}
```

**说明：** `Rc<T>`（Reference Counted）通过引用计数实现单线程下的数据共享。`Rc::clone` 只复制指针并增加引用计数，不拷贝数据。`Rc::strong_count` 返回当前强引用数。类似 C++ 的 `shared_ptr`，但仅限单线程使用。
</details>

### 练习 16-12: 验证 Rc 引用计数变化

> 难度：⭐⭐
> 类似 C++ shared_ptr.use_count()

补全代码，观察 Rc 引用计数在克隆和变量离开作用域时的变化。

```rust
use std::rc::Rc;

fn main() {
    let original = Rc::new(42);
    println!("A: 引用计数 = {}", Rc::strong_count(&original));

    {
        // TODO: 在内部作用域中克隆 original
        let inner = ??;
        println!("B: 引用计数 = {}", Rc::strong_count(&original));

        // TODO: 再克隆一次
        let inner2 = ??;
        println!("C: 引用计数 = {}", Rc::strong_count(&original));
    }
    // inner 和 inner2 离开作用域后销毁

    // TODO: 打印离开内部作用域后的引用计数
    println!("D: 引用计数 = {}", ??);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

fn main() {
    let original = Rc::new(42);
    println!("A: 引用计数 = {}", Rc::strong_count(&original));

    {
        let inner = Rc::clone(&original);
        println!("B: 引用计数 = {}", Rc::strong_count(&original));

        let inner2 = Rc::clone(&original);
        println!("C: 引用计数 = {}", Rc::strong_count(&original));
    }

    println!("D: 引用计数 = {}", Rc::strong_count(&original));
}
```

**说明：** 每次 `Rc::clone` 引用计数 +1，每个克隆离开作用域时引用计数 -1。当引用计数归零时，数据被自动释放。这与 C++ `shared_ptr` 的 `use_count()` 行为一致。
</details>

### 练习 16-13: Rc 配合 RefCell 实现内部可变性

> 难度：⭐⭐
> 类似 C++ shared_ptr 配合 mutable 成员

补全代码，使用 `Rc<RefCell<T>>` 实现多个引用共享并修改同一份数据。

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // TODO: 创建 Rc<RefCell<i32>>，初始值为 0
    let shared = Rc::new(??);

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    // TODO: 通过 a 修改值为 10
    ???

    // TODO: 通过 b 修改值为 20
    ???

    // TODO: 通过 shared 打印最终值
    println!("最终值: {}", ??);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() = 10;
    *b.borrow_mut() = 20;

    println!("最终值: {}", shared.borrow());
}
```

**说明：** `Rc<T>` 本身只提供共享只读访问。配合 `RefCell<T>`（提供运行时借用检查的内部可变性），可以实现多个所有者共享并修改数据。`borrow_mut()` 获取可变引用，`borrow()` 获取不可变引用。这类似 C++ 中 `shared_ptr` 配合 `mutable` 成员或在互斥锁保护下修改数据。
</details>

### 练习 16-14: Rc 克隆 vs 深度拷贝

> 难度：⭐⭐
> 对比 shared_ptr 的拷贝和 deep copy

填空，区分 `Rc::clone`（增加引用计数）和普通 `.clone()`（深度拷贝）的区别。

```rust
use std::rc::Rc;

fn main() {
    let original = Rc::new(vec![1, 2, 3, 4, 5]);

    // Rc::clone — 只复制指针，不复制数据
    let rc_clone = Rc::clone(&original);
    // TODO: 验证 Rc::clone 后 original 和 rc_clone 指向同一地址
    // 提示：比较两个 Rc 中 Vec 的指针
    println!("rc_clone 引用计数: {}", ??);
    println!("original 和 rc_clone 指向相同数据: {}", 
        format!("{:p}", &original[0]) == format!("{:p}", &rc_clone[0]));

    // Vec 的 clone — 深度拷贝整个数据
    let deep_clone = (*original).clone();
    // TODO: 验证 deep_clone 是独立拷贝
    println!("deep_clone: {:?}", deep_clone);
    println!("深度拷贝是独立数据: {}", 
        format!("{:p}", &original[0]) != format!("{:p}", &deep_clone[0]));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

fn main() {
    let original = Rc::new(vec![1, 2, 3, 4, 5]);

    let rc_clone = Rc::clone(&original);
    println!("rc_clone 引用计数: {}", Rc::strong_count(&original));
    println!("original 和 rc_clone 指向相同数据: {}", 
        format!("{:p}", &original[0]) == format!("{:p}", &rc_clone[0]));

    let deep_clone = (*original).clone();
    println!("deep_clone: {:?}", deep_clone);
    println!("深度拷贝是独立数据: {}", 
        format!("{:p}", &original[0]) != format!("{:p}", &deep_clone[0]));
}
```

**说明：** `Rc::clone` 只复制指针并增加引用计数（O(1) 操作），类似 C++ `shared_ptr` 的拷贝构造。而 `Vec::clone` 会深度拷贝所有数据（O(n) 操作）。在需要共享数据时应优先使用 `Rc::clone` 以提高性能。
</details>

### 练习 16-15: 多个不可变引用的共享

> 难度：⭐⭐
> 类似 C++ 中多个 shared_ptr 指向同一对象

补全代码，多个函数共享同一个不可变数据，验证 Rc 的引用计数正确维护。

```rust
use std::rc::Rc;

struct Config {
    name: String,
    version: u32,
}

fn print_config(config: Rc<Config>) {
    println!("{} v{}", config.name, config.version);
    // 函数结束后 config 的引用计数 -1
}

fn main() {
    let config = Rc::new(Config {
        name: String::from("Rust学习"),
        version: 1,
    });

    println!("初始引用计数: {}", Rc::strong_count(&config));

    // TODO: 调用 print_config 三次，每次都传入 Rc::clone
    // 注意：由于 print_config 获取所有权，需要提前克隆
    ???

    println!("最终引用计数: {}", Rc::strong_count(&config));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

struct Config {
    name: String,
    version: u32,
}

fn print_config(config: Rc<Config>) {
    println!("{} v{}", config.name, config.version);
}

fn main() {
    let config = Rc::new(Config {
        name: String::from("Rust学习"),
        version: 1,
    });

    println!("初始引用计数: {}", Rc::strong_count(&config));

    print_config(Rc::clone(&config));
    print_config(Rc::clone(&config));
    print_config(Rc::clone(&config));

    println!("最终引用计数: {}", Rc::strong_count(&config));
}
```

**说明：** 每次调用 `print_config` 时传入 `Rc::clone`，函数接收克隆的所有权，执行完毕后释放，引用计数相应增减。最终 `main` 中还有一个引用，所以最终计数为 1。这是 Rust 所有权系统与引用计数结合的典型用法。
</details>

### 练习 16-16: 使用 Arc 在线程间共享数据

> 难度：⭐⭐⭐
> 类似 C++ 的 std::shared_ptr（多线程版）

填空，使用 `Arc<T>` 在多个线程间安全地共享不可变数据。

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    // TODO: 使用 Arc::new 创建线程安全的数据
    let data = Arc::new(String::from("多线程共享数据"));

    let mut handles = vec![];

    for i in 0..3 {
        // TODO: 克隆 Arc，移动到线程中
        let shared = ??;

        let handle = thread::spawn(move || {
            println!("线程 {}: 得到数据 '{}'", i, shared);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("主线程: 所有子线程已完成");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(String::from("多线程共享数据"));

    let mut handles = vec![];

    for i in 0..3 {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {}: 得到数据 '{}'", i, shared);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("主线程: 所有子线程已完成");
}
```

**说明：** `Arc<T>`（Atomic Reference Counted）使用原子操作维护引用计数，保证在多线程环境下线程安全。`Arc::clone` 是原子操作，性能略低于 `Rc::clone`。这类似 C++ 中 `shared_ptr` 的线程安全引用计数。
</details>

### 练习 16-17: Arc 配合 Mutex 实现可变共享

> 难度：⭐⭐⭐
> 类似 C++ 中 shared_ptr 配合 mutex

补全代码，使用 `Arc<Mutex<T>>` 在多线程间安全地修改共享数据。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: 创建 Arc<Mutex<i32>>，初始值为 0
    let counter = Arc::new(??);

    let mut handles = vec![];

    for _ in 0..5 {
        // TODO: 克隆 Arc
        let shared = ??;

        let handle = thread::spawn(move || {
            // TODO: 获取互斥锁并修改值（+1）
            let mut num = ??;
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: 打印最终结果
    println!("最终计数: {}", ??);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let shared = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = shared.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());
}
```

**说明：** `Arc<Mutex<T>>` 是 Rust 中多线程共享可变数据的标准组合。`Mutex` 提供互斥锁保证同一时间只有一个线程能访问数据，`Arc` 提供线程安全的引用计数。这类似 C++ 中 `shared_ptr` 配合 `std::mutex` 使用。
</details>

### 练习 16-18: Arc 的原子引用计数

> 难度：⭐⭐⭐
> 对比 Rc 的单线程计数和 Arc 的原子计数

填空，理解 `Arc` 使用原子操作维护引用计数，而 `Rc` 使用非原子操作（因此不能跨线程）。

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for _ in 0..3 {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // TODO: 在线程中打印 Arc 的引用计数
            // 注意：Arc::strong_count 返回当前线程看到的计数
            println!("线程内引用计数: {}", ??);
            println!("数据: {:?}", shared);
        });
        handles.push(handle);
    }

    // TODO: 在主线程中也打印引用计数
    println!("主线程引用计数: {}", ??);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成后引用计数: {}", Arc::strong_count(&data));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];

    for _ in 0..3 {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程内引用计数: {}", Arc::strong_count(&shared));
            println!("数据: {:?}", shared);
        });
        handles.push(handle);
    }

    println!("主线程引用计数: {}", Arc::strong_count(&data));

    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成后引用计数: {}", Arc::strong_count(&data));
}
```

**说明：** `Arc` 使用 `std::sync::atomic` 中的原子操作来维护引用计数，确保在多线程并发环境下的正确性。而 `Rc` 使用普通整数计数，性能更高但不支持线程安全。选择 `Rc` 还是 `Arc` 取决于是否需要跨线程共享。
</details>

### 练习 16-19: Rc 不能跨线程（编译错误）

> 难度：⭐⭐⭐
> 对比 C++ 中 shared_ptr 本身线程安全

补全代码，验证 `Rc<T>` 不能跨线程使用（`Send` trait 限制），而 `Arc<T>` 可以。

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let rc_data = Rc::new(42);
    let arc_data = Arc::new(42);

    // 下面的代码会编译错误，因为 Rc<i32> 没有实现 Send
    // let handle = thread::spawn(move || {
    //     println!("Rc 值: {}", rc_data);
    // });

    // TODO: 使用 Arc 替代 Rc，使其可以在线程间传递
    let handle = thread::spawn(move || {
        // TODO: 打印 arc_data 的值
        ???
    });

    handle.join().unwrap();

    println!("成功使用 Arc 在线程间共享数据");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let rc_data = Rc::new(42);
    let arc_data = Arc::new(42);

    // Rc<i32> 没有实现 Send，不能跨线程
    // let handle = thread::spawn(move || {
    //     println!("Rc 值: {}", rc_data);
    // });

    let handle = thread::spawn(move || {
        println!("Arc 值: {}", arc_data);
    });

    handle.join().unwrap();

    println!("成功使用 Arc 在线程间共享数据");
}
```

**说明：** `Rc<T>` 没有实现 `Send` trait，因为非原子引用计数在多线程中会导致数据竞争。`Arc<T>` 实现了 `Send` 和 `Sync`，可以安全地在线程间传递和共享。这类似于 C++ 中 `shared_ptr` 的引用计数操作是原子的，天生支持多线程。
</details>

### 练习 16-20: 多线程并行计算

> 难度：⭐⭐⭐
> 类似 C++ 中多个线程共享 large data

补全代码，使用 `Arc<Vec<i32>>` 让多个线程并行处理同一份大型数据。

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<i32> = (1..=100).collect();
    // TODO: 将 numbers 包装到 Arc 中
    let shared_numbers = Arc::new(numbers);

    let mut handles = vec![];
    let chunk_size = 25;

    for i in 0..4 {
        // TODO: 克隆 Arc
        let chunk = ??;
        let start = i * chunk_size;
        let end = start + chunk_size;

        let handle = thread::spawn(move || {
            let sum: i32 = chunk[start..end].iter().sum();
            println!("区块 {} ({}..{}): 总和 = {}", i, start, end, sum);
            sum
        });
        handles.push(handle);
    }

    // TODO: 汇总所有线程的结果
    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("最终总和: {}", total);
    assert_eq!(total, 5050);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<i32> = (1..=100).collect();
    let shared_numbers = Arc::new(numbers);

    let mut handles = vec![];
    let chunk_size = 25;

    for i in 0..4 {
        let chunk = Arc::clone(&shared_numbers);
        let start = i * chunk_size;
        let end = start + chunk_size;

        let handle = thread::spawn(move || {
            let sum: i32 = chunk[start..end].iter().sum();
            println!("区块 {} ({}..{}): 总和 = {}", i, start, end, sum);
            sum
        });
        handles.push(handle);
    }

    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("最终总和: {}", total);
    assert_eq!(total, 5050);
}
```

**说明：** `Arc` 允许多个线程共享同一份只读数据而无需拷贝。每个线程获取 `Arc::clone` 后获得独立的智能指针，但指向同一堆数据。这种方式在大型数据集并行处理时非常高效，类似 C++ 中多个线程持有指向同一大数组的 `shared_ptr`。
</details>

### 练习 16-21: Weak 引用不增加计数

> 难度：⭐⭐⭐
> 类似 C++ 的 std::weak_ptr

填空，理解 `Weak<T>` 不增加引用计数，只增加弱引用计数。

```rust
use std::rc::Rc;

fn main() {
    let strong = Rc::new(String::from("Hello"));

    // TODO: 使用 Rc::downgrade 创建弱引用
    let weak = ??;

    println!("强引用计数: {}", Rc::strong_count(&strong));
    // TODO: 打印弱引用计数（应输出 1）
    println!("弱引用计数: {}", ??);

    // 即使有弱引用，强引用归零时数据仍会被释放
    // drop(strong); // 取消注释会怎样？
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

fn main() {
    let strong = Rc::new(String::from("Hello"));
    let weak = Rc::downgrade(&strong);

    println!("强引用计数: {}", Rc::strong_count(&strong));
    println!("弱引用计数: {}", Rc::weak_count(&strong));
}
```

**说明：** `Weak<T>` 通过 `Rc::downgrade` 创建，调用 `weak_count` 可查看弱引用计数。弱引用不影响数据的生命周期——只有当所有强引用（`Rc<T>`）都被销毁后，数据才会被释放。这类似于 C++ 的 `weak_ptr` 不影响 `shared_ptr` 的引用计数。
</details>

### 练习 16-22: Weak::upgrade 获取共享所有权

> 难度：⭐⭐⭐
> 类似 C++ weak_ptr::lock

补全代码，使用 `Weak::upgrade` 尝试获取 `Rc<T>`，并处理数据可能已被释放的情况。

```rust
use std::rc::Rc;

fn try_read(weak: &Rc<i32>) {
    // TODO: 使用 upgrade 尝试获取强引用
    // 如果成功，打印值；如果失败，打印"数据已被释放"
    match ?? {
        Some(data) => println!("值: {}", data),
        None => println!("数据已被释放"),
    }
}

fn main() {
    let strong = Rc::new(42);
    let weak = Rc::downgrade(&strong);

    try_read(&strong); // 强引用存在，可以访问

    drop(strong); // 强引用被销毁

    // TODO: 在 strong 被 drop 后再次调用 try_read
    // 注意：weak 本身已失效，需要重新处理
    ???
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

fn try_read(weak: &Rc<i32>) {
    // 注意：这里函数签名接受 &Rc<i32> 是为了示例简单
    // 实际使用 Weak 时，应该接受 &Weak<i32>
    // 但我们这里为了展示 upgrade 的行为
}

fn main() {
    let strong = Rc::new(42);
    let weak = Rc::downgrade(&strong);

    // 方法一：直接演示 Weak::upgrade 的正确用法
    println!("--- 正确使用 Weak::upgrade 示例 ---");
    
    let strong2 = Rc::new(100);
    let weak2 = Rc::downgrade(&strong2);

    // 方式一：强引用存在时 upgrade 成功
    if let Some(data) = weak2.upgrade() {
        println!("方式一: 值 = {}", data);
    }

    drop(strong2);

    // 方式二：强引用已释放时 upgrade 返回 None
    if let Some(_data) = weak2.upgrade() {
        println!("方式二: 不应该执行到这里");
    } else {
        println!("方式二: 强引用已释放，无法获取数据");
    }

    println!("--- 练习原始场景 ---");
    let strong3 = Rc::new(42);
    let weak3 = Rc::downgrade(&strong3);

    // 使用 match 处理 Weak::upgrade 的返回值
    match weak3.upgrade() {
        Some(data) => println!("值: {}", data),
        None => println!("数据已被释放"),
    }

    drop(strong3);

    match weak3.upgrade() {
        Some(data) => println!("值: {}", data),
        None => println!("强引用已释放，无法获取数据"),
    }
}
```

**说明：** `Weak::upgrade` 返回 `Option<Rc<T>>`。如果仍有强引用存活，返回 `Some(Rc<T>)`；如果数据已被释放，返回 `None`。这类似于 C++ 中 `weak_ptr::lock()` 的行为。注意：`Weak` 不持有所有权，不能直接解引用。
</details>

### 练习 16-23: 树结构 — 父节点持有子节点的强引用

> 难度：⭐⭐⭐
> 类似 C++ 中父节点持有 shared_ptr 到子节点

补全代码，使用 `Rc` 构建树结构，父节点通过强引用持有子节点。

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    // TODO: 子节点列表，通过 Rc 共享所有权
    children: Vec<Rc<Node>>,
}

fn main() {
    // TODO: 创建一个叶子节点（value=3，无子节点）
    let leaf = Rc::new(Node {
        value: 3,
        children: vec![],
    });

    // TODO: 创建一个分支节点（value=1），将 leaf 作为子节点
    let branch = Rc::new(Node {
        value: 1,
        // TODO: 添加 leaf 作为子节点
        children: ??,
    });

    println!("branch: {:?}", branch);
    println!("leaf 强引用计数: {}", Rc::strong_count(&leaf));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: vec![],
    });

    let branch = Rc::new(Node {
        value: 1,
        children: vec![Rc::clone(&leaf)],
    });

    println!("branch: {:?}", branch);
    println!("leaf 强引用计数: {}", Rc::strong_count(&leaf));
}
```

**说明：** 树结构中父节点持有子节点的 `Rc` 强引用。当一个节点作为多个父节点的子节点时，其强引用计数会相应增加。类似 C++ 中父节点持有 `shared_ptr` 指向子节点。
</details>

### 练习 16-24: 树结构 — 子节点通过 Weak 引用父节点

> 难度：⭐⭐⭐
> 类似 C++ 中 child 通过 weak_ptr 指向 parent

补全代码，使用 `Weak<T>` 让子节点持有父节点的引用，防止循环引用导致内存泄漏。

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // TODO: 创建叶子节点 leaf（value=3）
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf 创建后");
    println!("  parent 强引用: {:?}", leaf.parent.borrow().upgrade());

    // TODO: 创建分支节点 branch（value=1），将 leaf 加入其子节点
    // 同时设置 leaf 的 parent 为 branch（通过 Weak）
    let branch = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // TODO: 设置 leaf 的 parent 为 branch 的弱引用
    *leaf.parent.borrow_mut() = ??;

    // 验证父子关系
    println!("leaf 的父节点值: {:?}", leaf.parent.borrow().upgrade().map(|n| n.value));
    println!("branch 的强引用计数: {}", Rc::strong_count(&branch));
    println!("leaf 的强引用计数: {}", Rc::strong_count(&leaf));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf 创建后");
    println!("  parent 强引用: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf 的父节点值: {:?}", leaf.parent.borrow().upgrade().map(|n| n.value));
    println!("branch 的强引用计数: {}", Rc::strong_count(&branch));
    println!("leaf 的强引用计数: {}", Rc::strong_count(&leaf));
}
```

**说明：** 树结构中，父节点通过强引用（`Rc`）持有子节点，子节点通过弱引用（`Weak`）持有父节点。这避免了循环引用：`branch → leaf（强）`且 `leaf → branch（弱）`，因此当 `branch` 的强引用归零时可以正确释放。类似 C++ 中父节点持有 `shared_ptr`、子节点持有 `weak_ptr` 的设计模式。
</details>

### 练习 16-25: 检测并防止循环引用

> 难度：⭐⭐⭐
> 类似 C++ 中 weak_ptr 打破循环引用

补全代码，创建一个循环引用的场景，观察内存泄漏问题，然后使用 `Weak<T>` 修复。

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Person {
    name: String,
    // TODO: 好友列表，使用 RefCell 实现内部可变性
    friends: RefCell<Vec<Rc<Person>>>,
}

fn main() {
    // 创建两个人
    let alice = Rc::new(Person {
        name: String::from("Alice"),
        friends: RefCell::new(vec![]),
    });

    let bob = Rc::new(Person {
        name: String::from("Bob"),
        friends: RefCell::new(vec![]),
    });

    // TODO: 让 Alice 和 Bob 互为好友（双方 friends 字段都包含对方）
    // 这时形成了一个循环引用，即使离开作用域也不会释放
    alice.friends.borrow_mut().push(Rc::clone(&bob));
    bob.friends.borrow_mut().push(Rc::clone(&alice));

    println!("Alice 的好友: {:?}", alice.friends.borrow().iter().map(|p| &p.name).collect::<Vec<_>>());
    println!("Bob 的好友: {:?}", bob.friends.borrow().iter().map(|p| &p.name).collect::<Vec<_>>());

    println!("\nalice 强引用计数: {}", Rc::strong_count(&alice));
    println!("bob 强引用计数: {}", Rc::strong_count(&bob));
    println!("（双方计数为 2：一个来自变量，一个来自好友字段——形成循环引用）");

    // TODO: 思考如何用 Weak 修复上述循环引用？
    // 提示：将 Person 的 friends 字段改为 Vec<Weak<Person>>
    println!("\n修复方案：将 friends 改为 Vec<Weak<Person>>，则一方释放后另一方不会阻止内存回收。");
}

// TODO: 创建一个修复后的版本 FixedPerson，使用 Weak 避免循环引用
#[derive(Debug)]
struct FixedPerson {
    name: String,
    friends: RefCell<Vec<Weak<FixedPerson>>>,
}

// 补全 main_fixed 函数，展示无循环引用的版本
fn main_fixed() {
    let alice = Rc::new(FixedPerson {
        name: String::from("Alice"),
        friends: RefCell::new(vec![]),
    });

    let bob = Rc::new(FixedPerson {
        name: String::from("Bob"),
        friends: RefCell::new(vec![]),
    });

    // TODO: 使用 Weak::downgrade 添加好友
    alice.friends.borrow_mut().push(Rc::downgrade(&bob));
    bob.friends.borrow_mut().push(Rc::downgrade(&alice));

    // 通过 upgrade 访问好友
    let alice_friends: Vec<String> = alice.friends.borrow()
        .iter()
        .filter_map(|w| w.upgrade())
        .map(|p| p.name.clone())
        .collect();
    println!("Alice 的好友（new 版本）: {:?}", alice_friends);

    println!("\nalice 强引用计数: {}", Rc::strong_count(&alice));
    println!("bob 强引用计数: {}", Rc::strong_count(&bob));
    println!("（双方计数为 1：只有变量持有强引用，好友字段仅持有弱引用）");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Person {
    name: String,
    friends: RefCell<Vec<Rc<Person>>>,
}

fn main() {
    let alice = Rc::new(Person {
        name: String::from("Alice"),
        friends: RefCell::new(vec![]),
    });

    let bob = Rc::new(Person {
        name: String::from("Bob"),
        friends: RefCell::new(vec![]),
    });

    alice.friends.borrow_mut().push(Rc::clone(&bob));
    bob.friends.borrow_mut().push(Rc::clone(&alice));

    println!("Alice 的好友: {:?}", alice.friends.borrow().iter().map(|p| &p.name).collect::<Vec<_>>());
    println!("Bob 的好友: {:?}", bob.friends.borrow().iter().map(|p| &p.name).collect::<Vec<_>>());

    println!("\nalice 强引用计数: {}", Rc::strong_count(&alice));
    println!("bob 强引用计数: {}", Rc::strong_count(&bob));
    println!("（双方计数为 2：一个来自变量，一个来自好友字段——形成循环引用）");

    println!("\n修复方案：将 friends 改为 Vec<Weak<Person>>，则一方释放后另一方不会阻止内存回收。");
}

#[derive(Debug)]
struct FixedPerson {
    name: String,
    friends: RefCell<Vec<Weak<FixedPerson>>>,
}

fn main_fixed() {
    let alice = Rc::new(FixedPerson {
        name: String::from("Alice"),
        friends: RefCell::new(vec![]),
    });

    let bob = Rc::new(FixedPerson {
        name: String::from("Bob"),
        friends: RefCell::new(vec![]),
    });

    alice.friends.borrow_mut().push(Rc::downgrade(&bob));
    bob.friends.borrow_mut().push(Rc::downgrade(&alice));

    let alice_friends: Vec<String> = alice.friends.borrow()
        .iter()
        .filter_map(|w| w.upgrade())
        .map(|p| p.name.clone())
        .collect();
    println!("Alice 的好友（new 版本）: {:?}", alice_friends);

    println!("\nalice 强引用计数: {}", Rc::strong_count(&alice));
    println!("bob 强引用计数: {}", Rc::strong_count(&bob));
    println!("（双方计数为 1：只有变量持有强引用，好友字段仅持有弱引用）");
}
```

**说明：** 循环引用是引用计数智能指针的常见问题。当 A 持有 B 的强引用，B 也持有 A 的强引用时，双方的引用计数永不为零，导致内存泄漏。解决方法是让一方持有弱引用（`Weak`）：选择"从属"方使用 `Weak`，"主导"方使用 `Rc`。在 C++ 中同样使用 `weak_ptr` 打破 `shared_ptr` 的循环引用。
</details>