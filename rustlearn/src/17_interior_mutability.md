# 17 内部可变性

内部可变性（Interior Mutability）是 Rust 中一种重要的设计模式，它允许你在拥有不可变引用的同时修改内部数据。这通过 `Cell<T>`、`RefCell<T>`、`Mutex<T>` 和 `RwLock<T>` 等类型实现，它们将借用检查从编译时推迟到运行时，在受限的场景下提供了更大的灵活性。本章练习将帮助你深入理解这些类型的适用场景与用法。

### 练习 17-01: Cell 的基本使用

> 难度：⭐⭐
> 这个概念的运行时借用检查是 Rust 独有的设计

`Cell<T>` 适用于实现了 `Copy` 的类型。它通过 `get()` 和 `set()` 方法提供内部可变性。补全代码，使用 `Cell` 来追踪调用次数。

```rust
use std::cell::Cell;

struct Counter {
    count: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { count: Cell::new(0) }
    }

    fn increment(&self) {
        // TODO: 使用 Cell 的 set 方法将 count 加 1
        // 提示：先 get 再 set
    }

    fn get(&self) -> i32 {
        // TODO: 获取当前计数值
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("计数: {}", counter.get());
    assert_eq!(counter.get(), 3);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::Cell;

struct Counter {
    count: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter { count: Cell::new(0) }
    }

    fn increment(&self) {
        self.count.set(self.count.get() + 1);
    }

    fn get(&self) -> i32 {
        self.count.get()
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("计数: {}", counter.get());
    assert_eq!(counter.get(), 3);
}
```

**说明：** `Cell::get()` 返回内部值的克隆（要求 `T: Copy`），`Cell::set()` 替换内部值。注意 `increment` 方法接收 `&self` 而非 `&mut self`，却能修改内部状态——这就是内部可变性的核心。
</details>

### 练习 17-02: 用 Cell 缓存计算结果

> 难度：⭐⭐
> C++/Java 中可以用 mutable 成员实现类似效果

补全代码，实现一个带缓存的计算器。`get_value` 方法如果缓存为空则计算并缓存结果，否则直接返回缓存值。

```rust
use std::cell::Cell;

struct Calculator {
    input: i32,
    cache: Cell<Option<i32>>,
}

impl Calculator {
    fn new(input: i32) -> Self {
        Calculator { input, cache: Cell::new(None) }
    }

    fn get_value(&self) -> i32 {
        // TODO: 如果 cache 有值则返回，否则计算 input * input 存入缓存再返回
        // 提示：使用 match 或 if let
    }
}

fn main() {
    let calc = Calculator::new(5);
    // 第一次调用会计算，第二次直接返回缓存
    println!("结果: {}", calc.get_value());
    println!("结果: {}", calc.get_value());
    assert_eq!(calc.get_value(), 25);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::Cell;

struct Calculator {
    input: i32,
    cache: Cell<Option<i32>>,
}

impl Calculator {
    fn new(input: i32) -> Self {
        Calculator { input, cache: Cell::new(None) }
    }

    fn get_value(&self) -> i32 {
        match self.cache.get() {
            Some(val) => val,
            None => {
                let result = self.input * self.input;
                self.cache.set(Some(result));
                result
            }
        }
    }
}

fn main() {
    let calc = Calculator::new(5);
    println!("结果: {}", calc.get_value());
    println!("结果: {}", calc.get_value());
    assert_eq!(calc.get_value(), 25);
}
```

**说明：** `Cell<Option<i32>>` 是一个常见用法——`Option<i32>` 本身也实现了 `Copy`（当 `T: Copy` 时），因此可以用 `Cell` 包装来实现可选的缓存值。`Cell::get()` 返回 `Option<i32>`，`Cell::set()` 替换整个值。
</details>

### 练习 17-03: 用 Cell 实现不可变结构体中的可变字段

> 难度：⭐⭐
> Java 的 final 类中无法修改字段，但 Rust 可以用 Cell 突破

补全代码，实现一个日志统计器，记录 info、warn、error 三种级别的日志数量。结构体字段是不可变引用，但需要通过 `Cell` 实现计数。

```rust
use std::cell::Cell;

struct LogStats {
    info_count: Cell<u32>,
    warn_count: Cell<u32>,
    error_count: Cell<u32>,
}

impl LogStats {
    fn new() -> Self {
        LogStats {
            info_count: Cell::new(0),
            warn_count: Cell::new(0),
            error_count: Cell::new(0),
        }
    }

    fn log_info(&self) { self.info_count.set(self.info_count.get() + 1); }
    fn log_warn(&self) { self.warn_count.set(self.warn_count.get() + 1); }
    fn log_error(&self) { self.error_count.set(self.error_count.get() + 1); }

    fn total(&self) -> u32 {
        // TODO: 返回三种级别计数之和
    }
}

fn main() {
    let stats = LogStats::new();
    stats.log_info();
    stats.log_info();
    stats.log_warn();
    stats.log_error();
    stats.log_error();
    stats.log_error();
    println!("总计: {}", stats.total());
    assert_eq!(stats.total(), 6);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::Cell;

struct LogStats {
    info_count: Cell<u32>,
    warn_count: Cell<u32>,
    error_count: Cell<u32>,
}

impl LogStats {
    fn new() -> Self {
        LogStats {
            info_count: Cell::new(0),
            warn_count: Cell::new(0),
            error_count: Cell::new(0),
        }
    }

    fn log_info(&self) { self.info_count.set(self.info_count.get() + 1); }
    fn log_warn(&self) { self.warn_count.set(self.warn_count.get() + 1); }
    fn log_error(&self) { self.error_count.set(self.error_count.get() + 1); }

    fn total(&self) -> u32 {
        self.info_count.get() + self.warn_count.get() + self.error_count.get()
    }
}

fn main() {
    let stats = LogStats::new();
    stats.log_info();
    stats.log_info();
    stats.log_warn();
    stats.log_error();
    stats.log_error();
    stats.log_error();
    println!("总计: {}", stats.total());
    assert_eq!(stats.total(), 6);
}
```

**说明：** 多个 `Cell` 字段可以共存于同一个结构体中，每个字段独立提供内部可变性。这使得不可变结构体也能拥有可变的计数器字段——在需要共享引用的场景中非常有用。
</details>

### 练习 17-04: RefCell 的运行时借用

> 难度：⭐⭐
> C++/Java 没有编译期借用检查，感受不到这种限制

`RefCell<T>` 适用于非 `Copy` 类型，它在运行时执行借用规则：同一时刻要么有多个不可变借用，要么有一个可变借用。补全代码实现一个带有内部状态的 `Messenger`。

```rust
use std::cell::RefCell;

struct Messenger {
    messages: RefCell<Vec<String>>,
}

impl Messenger {
    fn new() -> Self {
        Messenger { messages: RefCell::new(Vec::new()) }
    }

    fn send(&self, msg: &str) {
        // TODO: 使用 borrow_mut 获取可变引用，将 msg 转换为 String 后推入 messages
    }

    fn message_count(&self) -> usize {
        // TODO: 使用 borrow 获取不可变引用，返回 messages 长度
    }
}

fn main() {
    let messenger = Messenger::new();
    messenger.send("Hello");
    messenger.send("World");
    println!("消息数: {}", messenger.message_count());
    assert_eq!(messenger.message_count(), 2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;

struct Messenger {
    messages: RefCell<Vec<String>>,
}

impl Messenger {
    fn new() -> Self {
        Messenger { messages: RefCell::new(Vec::new()) }
    }

    fn send(&self, msg: &str) {
        self.messages.borrow_mut().push(msg.to_string());
    }

    fn message_count(&self) -> usize {
        self.messages.borrow().len()
    }
}

fn main() {
    let messenger = Messenger::new();
    messenger.send("Hello");
    messenger.send("World");
    println!("消息数: {}", messenger.message_count());
    assert_eq!(messenger.message_count(), 2);
}
```

**说明：** `borrow_mut()` 返回 `RefMut<T>`，在运行时获取可变借用；`borrow()` 返回 `Ref<T>`，获取不可变借用。如果运行时违反借用规则（例如同时对同一 `RefCell` 持有 `borrow` 和 `borrow_mut`），程序会 panic。
</details>

### 练习 17-05: RefCell 运行时借用检查导致的 panic

> 难度：⭐⭐
> 这是 Rust 独有的运行时借用检查机制

以下代码会在运行时 panic，因为同时存在活跃的可变借用和不可变借用。补全代码，使其能够正常运行——通过缩小可变借用的作用域。

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    // 当前这段代码会 panic，因为 borrow_mut 和 borrow 同时活跃
    let mut mut_borrow = data.borrow_mut();
    mut_borrow.push(4);
    let immut_borrow = data.borrow();
    println!("len = {}", immut_borrow.len());

    // TODO: 重写上面的逻辑，使得程序不 panic（提示：使用作用域 {} 或 drop）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    // 方案一：使用作用域限制可变借用的生命周期
    {
        let mut mut_borrow = data.borrow_mut();
        mut_borrow.push(4);
    } // 可变借用在这里释放

    let immut_borrow = data.borrow();
    println!("len = {}", immut_borrow.len());

    // 方案二：也可以使用 drop 显式释放
    // let mut mut_borrow = data.borrow_mut();
    // mut_borrow.push(4);
    // drop(mut_borrow);
    // let immut_borrow = data.borrow();
    // println!("len = {}", immut_borrow.len());
}
```

**说明：** `RefCell` 在运行时追踪借用计数。`borrow_mut()` 返回的 `RefMut` 被 drop 后，可变借用才会释放。利用作用域或显式 `drop` 可以控制借用生命周期，避免运行时的 panic。
</details>

### 练习 17-06: RefCell 与特征约束

> 难度：⭐⭐
> Java/C++ 中 trait 可以随意修改内部状态

补全代码，实现一个 `Logger` trait 及其实现，其中实现者需要在不可变方法中修改内部状态——这正是 `RefCell` 的典型应用场景。

```rust
use std::cell::RefCell;

trait Logger {
    fn log(&self, message: &str);
    fn get_logs(&self) -> Vec<String>;
}

struct VecLogger {
    logs: RefCell<Vec<String>>,
}

impl Logger for VecLogger {
    fn log(&self, message: &str) {
        // TODO: 将消息推入 logs
    }

    fn get_logs(&self) -> Vec<String> {
        // TODO: 返回 logs 的克隆
    }
}

fn main() {
    let logger = VecLogger { logs: RefCell::new(Vec::new()) };
    logger.log("第一行");
    logger.log("第二行");
    let logs = logger.get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "第一行");
    println!("日志: {:?}", logs);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;

trait Logger {
    fn log(&self, message: &str);
    fn get_logs(&self) -> Vec<String>;
}

struct VecLogger {
    logs: RefCell<Vec<String>>,
}

impl Logger for VecLogger {
    fn log(&self, message: &str) {
        self.logs.borrow_mut().push(message.to_string());
    }

    fn get_logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }
}

fn main() {
    let logger = VecLogger { logs: RefCell::new(Vec::new()) };
    logger.log("第一行");
    logger.log("第二行");
    let logs = logger.get_logs();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0], "第一行");
    println!("日志: {:?}", logs);
}
```

**说明：** 特征方法签名中的 `&self` 是不可变引用，但内部实现需要修改字段时，`RefCell` 是理想的选择。这在实现缓存、日志、统计等横切关注点时非常常用。
</details>

### 练习 17-07: `Rc<RefCell<T>>` 共享可变数据

> 难度：⭐⭐
> Java/C++ 中所有引用默认就是共享可变的

`Rc<T>` 允许多重所有权（共享），`RefCell<T>` 提供内部可变性。组合 `Rc<RefCell<T>>` 可以实现多个所有者共享可变数据。补全代码。

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_value = Rc::new(RefCell::new(42));

    let a = Rc::clone(&shared_value);
    let b = Rc::clone(&shared_value);

    // 通过 a 修改值
    // TODO: 使用 borrow_mut 将值改为 100

    // 通过 b 读取值
    // TODO: 使用 borrow 读取并打印

    // 验证 shared_value 的值也是 100
    assert_eq!(*shared_value.borrow(), 100);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_value = Rc::new(RefCell::new(42));

    let a = Rc::clone(&shared_value);
    let b = Rc::clone(&shared_value);

    // 通过 a 修改值
    *a.borrow_mut() = 100;

    // 通过 b 读取值
    println!("b 读取的值: {}", b.borrow());

    // 验证 shared_value 的值也是 100
    assert_eq!(*shared_value.borrow(), 100);
}
```

**说明：** `Rc<RefCell<T>>` 是 Rust 中实现"共享可变性"的经典组合。`Rc` 提供共享所有权，`RefCell` 提供内部可变性。注意解引用：`*a.borrow_mut() = 100` 是对 `RefMut` 解引用后赋值。
</details>

### 练习 17-08: `Rc<RefCell<T>>` 双向链接

> 难度：⭐⭐
> Java 中的对象引用天然就是共享可变的

在图形用户界面中，父节点持有子节点的引用，子节点也可能持有父节点的引用。补全代码，实现一个简单的单向子节点引用。

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, children: Vec::new() }))
    }

    fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        // TODO: 将 child 添加到 parent 的 children 中
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1);
    Node::add_child(&root, child2);

    println!("根节点: {:?}", root.borrow());
    assert_eq!(root.borrow().children.len(), 2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, children: Vec::new() }))
    }

    fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        parent.borrow_mut().children.push(child);
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1);
    Node::add_child(&root, child2);

    println!("根节点: {:?}", root.borrow());
    assert_eq!(root.borrow().children.len(), 2);
}
```

**说明：** `Rc<RefCell<Node>>` 是构建树、图等复杂数据结构的常用模式。`Rc` 让节点可以被多处持有（共享所有权），`RefCell` 让节点可以在共享引用下被修改。
</details>

### 练习 17-09: 用 `Rc<RefCell<T>>` 实现观察者模式

> 难度：⭐⭐
> 观察者模式在 Java/C++ 中通常用 mutable 引用实现

补全代码，实现一个简单的观察者模式：Subject 持有观察者列表，通知时更新所有观察者。

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Observer {
    id: u32,
    value: RefCell<i32>,
}

impl Observer {
    fn new(id: u32) -> Rc<Self> {
        Rc::new(Observer { id, value: RefCell::new(0) })
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}

struct Subject {
    observers: RefCell<Vec<Rc<Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: RefCell::new(Vec::new()) }
    }

    fn attach(&self, observer: Rc<Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn notify(&self, new_value: i32) {
        // TODO: 遍历所有观察者，更新它们的 value 为 new_value
    }
}

fn main() {
    let subject = Subject::new();
    let obs1 = Observer::new(1);
    let obs2 = Observer::new(2);

    subject.attach(obs1.clone());
    subject.attach(obs2.clone());

    subject.notify(42);
    assert_eq!(obs1.get_value(), 42);
    assert_eq!(obs2.get_value(), 42);
    println!("两个观察者都已更新为 42");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Observer {
    id: u32,
    value: RefCell<i32>,
}

impl Observer {
    fn new(id: u32) -> Rc<Self> {
        Rc::new(Observer { id, value: RefCell::new(0) })
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}

struct Subject {
    observers: RefCell<Vec<Rc<Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: RefCell::new(Vec::new()) }
    }

    fn attach(&self, observer: Rc<Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn notify(&self, new_value: i32) {
        for observer in self.observers.borrow().iter() {
            *observer.value.borrow_mut() = new_value;
        }
    }
}

fn main() {
    let subject = Subject::new();
    let obs1 = Observer::new(1);
    let obs2 = Observer::new(2);

    subject.attach(obs1.clone());
    subject.attach(obs2.clone());

    subject.notify(42);
    assert_eq!(obs1.get_value(), 42);
    assert_eq!(obs2.get_value(), 42);
    println!("两个观察者都已更新为 42");
}
```

**说明：** 观察者模式中，`Subject` 持有 `Rc<Observer>` 的列表（共享所有权），每个 `Observer` 内部的 `RefCell<i32>` 允许在 `&self` 方法中修改值。这是 `Rc<RefCell<T>>` 在事件驱动系统中的典型应用。
</details>

### 练习 17-10: Mutex 的基本使用

> 难度：⭐⭐⭐
> 类似 C++ 的 std::mutex / Java 的 synchronized

`Mutex<T>` 是线程安全的内部可变性类型，提供跨线程的互斥访问。补全代码，使用 `Mutex` 来保护共享计数器。

```rust
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    // TODO: lock 获取锁，然后将值加 1，最后打印结果
    // 提示: lock() 返回 MutexGuard，解引用后可以修改内部值

    // 验证
    let final_val = counter.lock().unwrap();
    println!("最终值: {}", final_val);
    assert_eq!(*final_val, 1);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    {
        let mut val = counter.lock().unwrap();
        *val += 1;
    } // MutexGuard 在这里被 drop，锁被释放

    let final_val = counter.lock().unwrap();
    println!("最终值: {}", final_val);
    assert_eq!(*final_val, 1);
}
```

**说明：** `Mutex::lock()` 返回 `Result<MutexGuard<T>, PoisonError>`。当持有锁的线程 panic 时，Mutex 会中毒（poisoned）。`unwrap()` 在生产环境中应替换为更健壮的错误处理。`MutexGuard` 实现了 `Deref` 和 `DerefMut`，可以像引用一样使用。
</details>

### 练习 17-11: `Arc<Mutex<T>>` 跨线程共享

> 难度：⭐⭐⭐
> C++ 中需要手动管理互斥锁和线程安全

`Arc<T>` 是 `Rc<T>` 的线程安全版本。组合 `Arc<Mutex<T>>` 可以在多个线程间安全地共享可变数据。补全代码，启动多个线程并发累加计数器。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        // TODO: 启动一个线程，将 counter 的值加 1
        // 在线程内 lock 并递增
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", *counter.lock().unwrap());
    assert_eq!(*counter.lock().unwrap(), 10);
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

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut val = counter_clone.lock().unwrap();
            *val += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", *counter.lock().unwrap());
    assert_eq!(*counter.lock().unwrap(), 10);
}
```

**说明：** `Arc<T>` 提供原子引用计数，是线程安全的 `Rc`。`Arc::clone()` 增加引用计数，而 `move` 闭包将克隆的 `Arc` 移入线程。`Arc<Mutex<T>>` 是最常见的跨线程共享可变数据的方式。
</details>

### 练习 17-12: RwLock 多读单写

> 难度：⭐⭐⭐
> C++ 的 std::shared_mutex / Java 的 ReadWriteLock

`RwLock<T>` 允许多个读者同时访问，或一个写者独占访问。补全代码，模拟一个多线程读多、写少的配置中心。

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Arc::new(RwLock::new(String::from("初始配置")));
    let mut handles = vec![];

    // 读线程 1
    let r1 = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        // TODO: 使用 read() 获取读锁，打印配置
    }));

    // 写线程
    let w = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        // TODO: 使用 write() 获取写锁，将配置改为 "新配置"
    }));

    // 读线程 2
    let r2 = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        // TODO: 使用 read() 获取读锁，打印配置
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let final_config = config.read().unwrap();
    println!("最终配置: {}", *final_config);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let config = Arc::new(RwLock::new(String::from("初始配置")));
    let mut handles = vec![];

    let r1 = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        let cfg = r1.read().unwrap();
        println!("读线程 1 读取: {}", *cfg);
    }));

    let w = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        let mut cfg = w.write().unwrap();
        *cfg = String::from("新配置");
        println!("写线程已更新配置");
    }));

    let r2 = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        let cfg = r2.read().unwrap();
        println!("读线程 2 读取: {}", *cfg);
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    let final_config = config.read().unwrap();
    println!("最终配置: {}", *final_config);
}
```

**说明：** `RwLock::read()` 返回 `RwLockReadGuard`，多个读者可以同时持有；`RwLock::write()` 返回 `RwLockWriteGuard`，写者必须独占。在"读多写少"的场景下，`RwLock` 比 `Mutex` 有更好的并发性能。
</details>

### 练习 17-13: 选择 Cell 还是 RefCell

> 难度：⭐⭐⭐
> 这是 Rust 中需要根据类型特性做决策的场景

根据类型特征选择 `Cell` 或 `RefCell`。补全代码，在 `// TODO` 处填入正确的类型。

```rust
use std::cell::{Cell, RefCell};

// 场景 A: 包装一个 bool 标志位，经常需要 get 和 set
struct FlagHolder {
    flag: ____________, // TODO: 选择 Cell 或 RefCell
}

// 场景 B: 包装一个 String，需要在 &self 方法中修改
struct NameHolder {
    name: ____________, // TODO: 选择 Cell 或 RefCell
}

// 场景 C: 包装一个 Option<i32>，需要 get 和 set
struct OptHolder {
    opt: ____________, // TODO: 选择 Cell 或 RefCell
}

fn main() {
    let f = FlagHolder { flag: Cell::new(false) };
    f.flag.set(true);
    assert_eq!(f.flag.get(), true);

    let n = NameHolder { name: RefCell::new(String::from("Rust")) };
    n.name.borrow_mut().push_str(" 🦀");
    assert_eq!(*n.name.borrow(), "Rust 🦀");

    let o = OptHolder { opt: Cell::new(Some(42)) };
    assert_eq!(o.opt.get(), Some(42));

    println!("所有场景验证通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::{Cell, RefCell};

// bool 实现了 Copy，使用 Cell
struct FlagHolder {
    flag: Cell<bool>,
}

// String 没有实现 Copy，使用 RefCell
struct NameHolder {
    name: RefCell<String>,
}

// Option<i32> 实现了 Copy（因为 i32: Copy），使用 Cell
struct OptHolder {
    opt: Cell<Option<i32>>,
}

fn main() {
    let f = FlagHolder { flag: Cell::new(false) };
    f.flag.set(true);
    assert_eq!(f.flag.get(), true);

    let n = NameHolder { name: RefCell::new(String::from("Rust")) };
    n.name.borrow_mut().push_str(" 🦀");
    assert_eq!(*n.name.borrow(), "Rust 🦀");

    let o = OptHolder { opt: Cell::new(Some(42)) };
    assert_eq!(o.opt.get(), Some(42));

    println!("所有场景验证通过！");
}
```

**说明：** 选择 `Cell` 还是 `RefCell` 的关键在于内部类型是否实现了 `Copy`：
- `Cell<T>` 要求 `T: Copy`，提供 `get()`/`set()` 方法，无运行时开销（不会 panic）
- `RefCell<T>` 适用于非 `Copy` 类型，提供 `borrow()`/`borrow_mut()`，运行时检查借用规则
- 常见的 `Copy` 类型：基本类型、`Option<T>`（当 `T: Copy`）、元组（元素都 `Copy`）
</details>

### 练习 17-14: 选择单线程还是多线程内部可变性

> 难度：⭐⭐⭐
> C++/Java 需要开发者记住加锁，Rust 用类型系统引导选择

根据是否跨线程选择正确的内部可变性类型。补全代码。

```rust
// TODO: 补全 use 语句，导入需要的类型

// 场景 A: 单线程下统计函数调用次数
struct CallStats {
    count: ____________, // TODO: 选择 Cell / RefCell / Mutex / RwLock
}

// 场景 B: 多线程间共享配置
struct SharedConfig {
    data: ____________, // TODO: 选择 Cell / RefCell / Mutex / RwLock
}

// 场景 C: 单线程下的字符串构建器
struct StringBuilder {
    content: ____________, // TODO: 选择 Cell / RefCell / Mutex / RwLock
}

fn main() {
    // 单线程场景
    let stats = CallStats { count: Cell::new(0u64) };
    stats.count.set(5);
    assert_eq!(stats.count.get(), 5);

    let builder = StringBuilder { content: RefCell::new(String::new()) };
    builder.content.borrow_mut().push_str("Hello");
    assert_eq!(*builder.content.borrow(), "Hello");

    println!("所有场景验证通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::{Cell, RefCell};

// 场景 A: 单线程 + Copy 类型 → Cell
struct CallStats {
    count: Cell<u64>,
}

// 场景 B: 多线程需要 Arc<Mutex<T>> 或 Arc<RwLock<T>>，
// 但此处仅声明类型，跨线程场景需搭配 Arc
// 题设中未要求跨线程代码，留作设计讨论
struct SharedConfig {
    data: Mutex<String>,
}

// 场景 C: 单线程 + 非 Copy 类型 → RefCell
struct StringBuilder {
    content: RefCell<String>,
}

fn main() {
    let stats = CallStats { count: Cell::new(0u64) };
    stats.count.set(5);
    assert_eq!(stats.count.get(), 5);

    let builder = StringBuilder { content: RefCell::new(String::new()) };
    builder.content.borrow_mut().push_str("Hello");
    assert_eq!(*builder.content.borrow(), "Hello");

    println!("所有场景验证通过！");
}
```

**说明：** 选择策略总结：
- 单线程 + Copy 类型 → `Cell<T>`
- 单线程 + 非 Copy 类型 → `RefCell<T>`
- 多线程 + 共享可变 → `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`
- 多线程 + 读多写少 → `Arc<RwLock<T>>`
- 多线程 + 读写均匀 → `Arc<Mutex<T>>`
</details>

### 练习 17-15: 综合应用 — 银行账户系统

> 难度：⭐⭐⭐
> 综合运用多种内部可变性策略解决实际问题

实现一个简单的银行账户系统。账户可以在单线程中进行存取款（使用 `RefCell`），同时有一个跨线程的交易计数器（使用 `Arc<AtomicU64>`，或者用 `Arc<Mutex<u64>>`）。补全代码。

```rust
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

struct BankAccount {
    balance: RefCell<f64>,
    transaction_count: Arc<Mutex<u64>>,
}

impl BankAccount {
    fn new(initial: f64) -> Self {
        BankAccount {
            balance: RefCell::new(initial),
            transaction_count: Arc::new(Mutex::new(0)),
        }
    }

    fn deposit(&self, amount: f64) {
        // TODO: 增加余额，并递增交易计数
    }

    fn withdraw(&self, amount: f64) -> Result<(), String> {
        // TODO: 如果余额足够则扣减并递增交易计数，否则返回 Err
    }

    fn get_balance(&self) -> f64 {
        *self.balance.borrow()
    }

    fn get_transaction_count(&self) -> u64 {
        *self.transaction_count.lock().unwrap()
    }
}

fn main() {
    let account = BankAccount::new(100.0);

    // 单线程操作
    account.deposit(50.0);
    account.withdraw(30.0).unwrap();
    println!("余额: {}, 交易次数: {}", account.get_balance(), account.get_transaction_count());

    // 模拟多线程查询交易次数（不修改余额）
    let counter = Arc::clone(&account.transaction_count);
    let handle = thread::spawn(move || {
        let count = counter.lock().unwrap();
        println!("子线程读取交易次数: {}", *count);
    });
    handle.join().unwrap();

    assert_eq!(account.get_balance(), 120.0);
    assert_eq!(account.get_transaction_count(), 2);
    println!("账户系统验证通过！");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

struct BankAccount {
    balance: RefCell<f64>,
    transaction_count: Arc<Mutex<u64>>,
}

impl BankAccount {
    fn new(initial: f64) -> Self {
        BankAccount {
            balance: RefCell::new(initial),
            transaction_count: Arc::new(Mutex::new(0)),
        }
    }

    fn deposit(&self, amount: f64) {
        *self.balance.borrow_mut() += amount;
        *self.transaction_count.lock().unwrap() += 1;
    }

    fn withdraw(&self, amount: f64) -> Result<(), String> {
        if *self.balance.borrow() >= amount {
            *self.balance.borrow_mut() -= amount;
            *self.transaction_count.lock().unwrap() += 1;
            Ok(())
        } else {
            Err(format!("余额不足: 当前余额 {}, 需要 {}", *self.balance.borrow(), amount))
        }
    }

    fn get_balance(&self) -> f64 {
        *self.balance.borrow()
    }

    fn get_transaction_count(&self) -> u64 {
        *self.transaction_count.lock().unwrap()
    }
}

fn main() {
    let account = BankAccount::new(100.0);

    account.deposit(50.0);
    account.withdraw(30.0).unwrap();
    println!("余额: {}, 交易次数: {}", account.get_balance(), account.get_transaction_count());

    let counter = Arc::clone(&account.transaction_count);
    let handle = thread::spawn(move || {
        let count = counter.lock().unwrap();
        println!("子线程读取交易次数: {}", *count);
    });
    handle.join().unwrap();

    assert_eq!(account.get_balance(), 120.0);
    assert_eq!(account.get_transaction_count(), 2);
    println!("账户系统验证通过！");
}
```

**说明：** 这个练习综合运用了 `RefCell<T>`（单线程内部可变性）和 `Mutex<T>`（跨线程互斥访问）。`RefCell` 用于余额操作（账户本身不跨线程），`Arc<Mutex<u64>>` 用于交易计数器（需要被多个线程读取）。在实际应用中，`AtomicU64` 是更高效的计数器选择——这是一个 `ponytail` 留待后续优化。
</details>
