# 19 异步编程

Rust 的异步编程基于 `Future` trait 和 `async`/`.await` 语法，提供了一种零成本抽象来编写高效的并发 I/O 密集型代码。与传统的线程并发不同，异步任务在单个线程（或少量线程）上协作式调度，每个 `.await` 点都可能让出控制权。本章需要 `tokio` 运行时支持（`tokio = { version = "1", features = ["full"] }`），所有练习代码均假设在 `#[tokio::main]` 上下文中运行。通过本章练习，你将掌握 `async fn`、`.await`、`tokio::spawn`、异步 channel、`select!` 宏以及任务编排等核心异步编程技巧。

---

### 练习 19-01: 定义第一个异步函数

> 难度：⭐⭐
> 类似 Java 的 `CompletableFuture.supplyAsync` / 类似 C++20 的 `co_await`

填空，定义一个 `async fn` 并调用它等待结果。

```rust
// TODO: 补全 async fn greet，返回 String "你好，异步！"
// async fn greet() -> String { ... }

#[tokio::main]
async fn main() {
    // TODO: 调用 greet() 并用 .await 获取结果，打印它
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
async fn greet() -> String {
    String::from("你好，异步！")
}

#[tokio::main]
async fn main() {
    let msg = greet().await;
    println!("{}", msg);
}
```

**说明：** `async fn` 定义了一个返回 `Future` 的异步函数，调用它不会立即执行，而是返回一个实现了 `Future` trait 的值。只有在 `.await` 时才会真正驱动 Future 执行直到完成。`#[tokio::main]` 宏将 `main` 函数包装为 tokio 运行时的入口点。
</details>

---

### 练习 19-02: 顺序等待多个异步操作

> 难度：⭐⭐
> 类似 Java 的 `CompletableFuture.thenApply`

补全代码，定义两个异步函数并依次 `.await`。

```rust
async fn step1() -> i32 {
    10
}

async fn step2(input: i32) -> i32 {
    input * 2
}

#[tokio::main]
async fn main() {
    // TODO: 依次调用 step1() 和 step2()，将 step1 的结果传入 step2，打印最终结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
async fn step1() -> i32 {
    10
}

async fn step2(input: i32) -> i32 {
    input * 2
}

#[tokio::main]
async fn main() {
    let v = step1().await;
    let result = step2(v).await;
    println!("结果: {}", result); // 20
}
```

**说明：** 多个 `.await` 默认是**顺序执行**的——每个 `.await` 会阻塞当前任务直到对应的 Future 完成。如果希望并发执行，需要使用 `tokio::join!` 或先启动再等待。
</details>

---

### 练习 19-03: async 块与 Future 变量

> 难度：⭐⭐
> 类似 Java 的 `CompletableFuture` 变量引用

填空，使用 `async` 块创建 Future 并存储到变量中。

```rust
#[tokio::main]
async fn main() {
    let x = 42;
    // TODO: 创建一个 async 块，返回 x * 2，并 .await 打印结果
    // let fut = async { ... };
    // let result = fut.await;
    // println!("{}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[tokio::main]
async fn main() {
    let x = 42;
    let fut = async { x * 2 };
    let result = fut.await;
    println!("{}", result); // 84
}
```

**说明：** `async` 块和 `async fn` 一样会创建一个 `Future`。`async` 块可以捕获环境中的变量，就像闭包一样。注意 `async` 块不会执行任何代码，直到被 `.await` 轮询。
</details>

---

### 练习 19-04: 返回 Result 的异步函数

> 难度：⭐⭐
> 类似 Java 的 `CompletableFuture` 异常处理

补全代码，定义一个返回 `Result<i32, String>` 的异步函数，并用 `?` 传播错误。

```rust
async fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

// TODO: 定义 async fn compute() -> Result<f64, String>，调用 divide 并使用 ? 传播错误

#[tokio::main]
async fn main() {
    match compute().await {
        Ok(v) => println!("计算成功: {}", v),
        Err(e) => println!("计算失败: {}", e),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
async fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

async fn compute() -> Result<f64, String> {
    let a = divide(10.0, 2.0).await?;
    let b = divide(a, 5.0).await?;
    Ok(b)
}

#[tokio::main]
async fn main() {
    match compute().await {
        Ok(v) => println!("计算成功: {}", v),
        Err(e) => println!("计算失败: {}", e),
    }
}
```

**说明：** 在 `async fn` 中可以使用 `?` 操作符传播 `Result` 类型的错误——`?` 在 `Err` 时会提前返回，在 `Ok` 时解包出值。但 `main` 函数一般使用 `match` 处理最终结果，因为 `#[tokio::main]` 不支持返回 `Result` 的 main（可以用 `#[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>>` 绕过）。
</details>

---

### 练习 19-05: 模拟异步等待

> 难度：⭐⭐
> 类似 Java 的 `Thread.sleep` 但非阻塞

挑战：编写一个异步函数 `async fn delayed_greet(msg: &str, ms: u64) -> String`，使用 `tokio::time::sleep` 模拟延迟后返回问候语。

```rust
use tokio::time::{sleep, Duration};

// TODO: 实现 delayed_greet

#[tokio::main]
async fn main() {
    // TODO: 调用 delayed_greet("你好", 100)，等待后打印结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

async fn delayed_greet(msg: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("延迟 {}ms 后: {}", ms, msg)
}

#[tokio::main]
async fn main() {
    let result = delayed_greet("你好", 100).await;
    println!("{}", result);
}
```

**说明：** `tokio::time::sleep` 返回一个 Future，`.await` 时会**让出当前任务的控制权**，而不是阻塞线程。这是异步编程的核心优势：在 I/O 等待期间，tokio 运行时可以调度其他任务执行。相比之下，`std::thread::sleep` 会阻塞整个线程。
</details>

---

### 练习 19-06: tokio::spawn 基础

> 难度：⭐⭐
> 类似 Java 的 `ExecutorService.submit` / 类似 C++20 的 `std::async`

填空，使用 `tokio::spawn` 启动一个并发任务。

```rust
#[tokio::main]
async fn main() {
    // TODO: 使用 tokio::spawn 启动一个异步任务，打印 "并发任务执行中"
    // 提示: tokio::spawn(async { ... })
    
    // 等待任务完成
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    println!("主任务结束");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        println!("并发任务执行中");
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    println!("主任务结束");
}
```

**说明：** `tokio::spawn` 将异步任务提交给 tokio 运行时，任务会在后台并发执行。它返回一个 `JoinHandle`，可以通过 `.await` 等待任务结束。注意如果主任务结束，所有 spawn 的子任务会被立即取消，因此通常需要等待。
</details>

---

### 练习 19-07: 等待 JoinHandle

> 难度：⭐⭐
> 类似 Java 的 `Future.get()`

补全代码，使用 `JoinHandle` 等待 spawn 任务的返回值。

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // TODO: spawn 一个异步任务，计算 1+2+3+4+5 并返回结果
    // 使用 JoinHandle 获取返回值并打印
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // 模拟一些计算
        sleep(Duration::from_millis(50)).await;
        1 + 2 + 3 + 4 + 5
    });

    let result = handle.await.unwrap();
    println!("计算结果: {}", result);
}
```

**说明：** `tokio::spawn` 返回 `JoinHandle<T>`，它实现了 `Future<Output = Result<T, JoinError>>`。`.await` JoinHandle 会等待任务完成并获取返回值。外层 `unwrap()` 用于处理任务 panic 的情况（`JoinError`）。
</details>

---

### 练习 19-08: move 闭包与 spawn

> 难度：⭐⭐
> 类似 Java 的 lambda 传递变量到线程

填空，使用 `async move` 将数据移动到 spawn 的任务中。

```rust
#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let key = String::from("my-key");

    // TODO: 使用 async move 将 data 和 key 移动到异步任务中，打印它们
    // let handle = tokio::spawn(async move { ... });

    // handle.await.unwrap();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let key = String::from("my-key");

    let handle = tokio::spawn(async move {
        println!("key: {}, data: {:?}", key, data);
    });

    handle.await.unwrap();
}
```

**说明：** `async move` 块会**将捕获的变量所有权移动到异步任务中**。这和线程中的 `move` 闭包同理——如果不用 `move`，编译器会报错说闭包可能比捕获的变量活得更久。`move` 保证了异步任务拥有数据的完全所有权，可以安全地在后台执行。
</details>

---

### 练习 19-09: 并发执行多个 spawn 任务

> 难度：⭐⭐
> 类似 Java 的 `ExecutorService.invokeAll`

补全代码，spawn 三个任务并发执行，收集所有结果。

```rust
use tokio::time::{sleep, Duration};

async fn task(id: u32, ms: u64) -> u32 {
    sleep(Duration::from_millis(ms)).await;
    id * 10
}

#[tokio::main]
async fn main() {
    // TODO: spawn 三个任务，分别延迟 100ms/50ms/80ms，收集所有返回值并打印
    // 提示: 先 spawn 得到三个 JoinHandle，再依次 .await
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

async fn task(id: u32, ms: u64) -> u32 {
    sleep(Duration::from_millis(ms)).await;
    id * 10
}

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(task(1, 100));
    let h2 = tokio::spawn(task(2, 50));
    let h3 = tokio::spawn(task(3, 80));

    let r1 = h1.await.unwrap();
    let r2 = h2.await.unwrap();
    let r3 = h3.await.unwrap();

    println!("结果: {}, {}, {}", r1, r2, r3);
}
```

**说明：** 先全部 `spawn` 再依次 `.await` 是让多个任务**并发执行**的关键模式。如果先 `.await` 第一个再 spawn 第二个，就变成了顺序执行。三个任务的总耗时约等于最长的那个（100ms），而不是它们的和。
</details>

---

### 练习 19-10: 使用 tokio::join! 并发等待

> 难度：⭐⭐
> 类似 Java 的 `CompletableFuture.allOf`

挑战：使用 `tokio::join!` 宏同时等待多个 Future。

```rust
use tokio::time::{sleep, Duration};

async fn fetch_data(id: &str) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("数据 {}", id)
}

#[tokio::main]
async fn main() {
    // TODO: 使用 tokio::join! 同时获取 "A"、"B"、"C" 三个数据，打印结果
    // 提示: let (a, b, c) = tokio::join!(fetch_data("A"), fetch_data("B"), fetch_data("C"));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

async fn fetch_data(id: &str) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("数据 {}", id)
}

#[tokio::main]
async fn main() {
    let (a, b, c) = tokio::join!(fetch_data("A"), fetch_data("B"), fetch_data("C"));
    println!("{}, {}, {}", a, b, c);
}
```

**说明：** `tokio::join!` 宏一次性传入多个 Future，**同时轮询它们**，等待所有完成。相比于手动收集 JoinHandle，`join!` 更简洁且没有 spawn 的开销。`join!` 支持最多 16 个 Future。注意 `join!` 要求所有 Future 的 Output 类型可以不同。
</details>

---

### 练习 19-11: mpsc 发送与接收

> 难度：⭐⭐
> 类似 Java 的 `BlockingQueue` / 类似 Go 的 channel

填空，使用 `tokio::sync::mpsc` 创建通道并在异步任务间传递消息。

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // TODO: 创建一个容量为 32 的 mpsc channel，发送 "hello" 并接收打印
    // let (tx, mut rx) = mpsc::channel(32);
    // tokio::spawn(async move { ... });
    // let msg = rx.recv().await...;
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("hello".to_string()).await.unwrap();
    });

    let msg = rx.recv().await.unwrap();
    println!("收到: {}", msg);
}
```

**说明：** `mpsc`（Multi-Producer, Single-Consumer）是多生产者单消费者通道。`tx` 是发送端（可克隆用于多生产者），`rx` 是接收端（唯一）。`send()` 是一个异步方法，当通道满时会阻塞等待；`recv()` 返回 `Option<T>`，通道关闭且消息耗尽时返回 `None`。
</details>

---

### 练习 19-12: mpsc 多生产者

> 难度：⭐⭐
> 类似 Java 的多个线程发送到同一个队列

补全代码，创建两个发送任务向同一个接收端发送数据。

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // TODO: 创建 mpsc channel，spawn 两个任务各发送一条消息，主任务接收两条消息
    // 提示: tx 需要 clone
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.send("来自任务1").await.unwrap();
    });

    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx2.send("来自任务2").await.unwrap();
    });

    // 丢弃原始 tx，否则 rx 会一直等待
    drop(tx);

    while let Some(msg) = rx.recv().await {
        println!("收到: {}", msg);
    }
}
```

**说明：** `mpsc::Sender` 实现了 `Clone`，可以创建多个发送端。注意每个 `async move` 闭包会拿走自己那份 `tx` 的所有权。主任务中的原始 `tx` 也要 `drop` 掉，否则 `rx.recv()` 会永远等待（因为还有发送端存活）。
</details>

---

### 练习 19-13: tokio::sync::Mutex 与 Arc 共享状态

> 难度：⭐⭐
> 类似 Java 的 `ReentrantLock` / 类似 C++ 的 `std::mutex`

填空，使用 `tokio::sync::Mutex` 和 `Arc` 在多个并发任务间共享数据。

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // TODO: 创建一个 Arc<Mutex<i32>>，在两个并发任务中各加 1，最终值应为 2
    // let counter = Arc::new(Mutex::new(0));
    // 提示: 使用 Arc::clone 共享，.lock().await 获取锁
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let counter1 = Arc::clone(&counter);
    let h1 = tokio::spawn(async move {
        let mut val = counter1.lock().await;
        *val += 1;
    });

    let counter2 = Arc::clone(&counter);
    let h2 = tokio::spawn(async move {
        let mut val = counter2.lock().await;
        *val += 1;
    });

    h1.await.unwrap();
    h2.await.unwrap();

    println!("最终值: {}", *counter.lock().await);
}
```

**说明：** `tokio::sync::Mutex` 和 `std::sync::Mutex` 的区别在于 `lock()` 是异步的——它在 `.await` 时让出控制权，而不是阻塞线程。在异步代码中，如果需要跨 `.await` 持有锁，**必须**使用 `tokio::sync::Mutex`。在多任务间共享 `Mutex` 时，需要用 `Arc`（原子引用计数）包裹，因为 `tokio::spawn` 无法借用局部变量的引用（`async move` 会拿走所有权）。注意这里不能使用 `std::sync::Mutex` 的引用方式，因为 `tokio::spawn` 的生命周期要求是 `'static`。
</details>

---

### 练习 19-14: 异步生产者-消费者模型

> 难度：⭐⭐
> 类似 Java 的 `BlockingQueue` 生产者消费者

补全代码，一个生产者不断发送数字，消费者接收并处理。

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // TODO: 创建 channel，spawn 生产者发送 1..=5，消费者接收并打印每个数字
    // 生产者每个数字发送后延迟 50ms
    // 提示: 使用 for 循环发送，drop(tx) 后消费者自然结束
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
    });

    while let Some(num) = rx.recv().await {
        println!("收到: {}", num);
    }
}
```

**说明：** 这是典型的异步生产者-消费者模式。生产者循环调用 `send()`（通道满时自动等待），消费者使用 `while let Some(v) = rx.recv().await` 持续接收。当所有发送端 `drop` 后，`rx.recv()` 返回 `None`，循环自然结束。
</details>

---

### 练习 19-15: oneshot 通道

> 难度：⭐⭐
> 类似 Java 的 `CompletableFuture` 单次通知

挑战：使用 `tokio::sync::oneshot` 实现一次性请求-响应模式。

```rust
use tokio::sync::oneshot;

async fn compute_square(x: i32, tx: oneshot::Sender<i32>) {
    // TODO: 计算 x*x 并通过 tx 发送
}

#[tokio::main]
async fn main() {
    // TODO: 创建 oneshot channel，将发送端传入 compute_square，主任务使用接收端获取结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::oneshot;

async fn compute_square(x: i32, tx: oneshot::Sender<i32>) {
    let result = x * x;
    tx.send(result).unwrap(); // 发送一次，通道即关闭
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(compute_square(7, tx));

    let result = rx.await.unwrap();
    println!("7 的平方是: {}", result);
}
```

**说明：** `oneshot` 是只发送一次的通道，非常适合请求-响应场景。`Sender::send()` 返回 `Result<(), T>`，如果接收端已 drop 则返回错误。`Receiver::await` 返回 `Result<T, RecvError>`，如果发送端 drop 且未发送则返回错误。相比于 `mpsc`，`oneshot` 更轻量且语义明确。
</details>

---

### 练习 19-16: select! 宏基础

> 难度：⭐⭐⭐
> 类似 Go 的 `select` / 类似 Java 的 ` CompletableFuture.anyOf`

填空，使用 `tokio::select!` 等待多个 Future 中第一个完成的。

```rust
use tokio::time::{sleep, Duration};

async fn slow() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "slow"
}

async fn fast() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "fast"
}

#[tokio::main]
async fn main() {
    // TODO: 使用 select! 同时等待 slow() 和 fast()，打印先完成的结果
    // tokio::select! {
    //     val = slow() => println!("slow 完成: {}", val),
    //     val = fast() => println!("fast 完成: {}", val),
    // }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

async fn slow() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "slow"
}

async fn fast() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "fast"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        val = slow() => println!("slow 完成: {}", val),
        val = fast() => println!("fast 完成: {}", val),
    }
    // 输出: fast 完成: fast （因为 fast 先完成，另一个分支被取消）
}
```

**说明：** `tokio::select!` 会同时轮询所有分支，当**任意一个** Future 完成时，执行该分支的代码，**其他分支的 Future 会被取消**。这是实现超时控制、竞争请求等模式的基石。注意 `select!` 会剥夺所有未选中分支的 Future，确保资源不会泄漏。
</details>

---

### 练习 19-17: 超时控制

> 难度：⭐⭐⭐
> 类似 Java 的 `CompletableFuture.orTimeout`

补全代码，使用 `tokio::time::timeout` 为异步操作设置超时。

```rust
use tokio::time::{sleep, Duration, timeout};

async fn long_operation() -> &'static str {
    sleep(Duration::from_millis(300)).await;
    "操作完成"
}

#[tokio::main]
async fn main() {
    // TODO: 使用 timeout 为 long_operation 设置 200ms 超时，处理超时情况
    // 提示: timeout(Duration::from_millis(200), long_operation()).await
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration, timeout};

async fn long_operation() -> &'static str {
    sleep(Duration::from_millis(300)).await;
    "操作完成"
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(200), long_operation()).await {
        Ok(result) => println!("成功: {}", result),
        Err(_) => println!("操作超时了！"),
    }
}
```

**说明：** `tokio::time::timeout` 返回 `Result<Future::Output, Elapsed>`——如果在指定时间内 Future 完成则返回 `Ok`，否则返回 `Err`。它内部就是基于 `select!` 实现的：同时等待原始 Future 和一个 `sleep`，谁先完成就选谁。这是 Rust 异步中实现超时的标准做法。
</details>

---

### 练习 19-18: JoinSet 管理多任务

> 难度：⭐⭐⭐
> 类似 Java 的 `ExecutorCompletionService`

填空，使用 `tokio::task::JoinSet` 管理多个并发任务。

```rust
use tokio::time::{sleep, Duration};
use tokio::task::JoinSet;

async fn work(id: u32, ms: u64) -> u32 {
    sleep(Duration::from_millis(ms)).await;
    id
}

#[tokio::main]
async fn main() {
    // TODO: 使用 JoinSet 添加三个任务，依次获取并打印它们的结果
    // let mut set = JoinSet::new();
    // set.spawn(work(1, 100));
    // ...
    // while let Some(res) = set.join_next().await { ... }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};
use tokio::task::JoinSet;

async fn work(id: u32, ms: u64) -> u32 {
    sleep(Duration::from_millis(ms)).await;
    id
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    set.spawn(work(1, 100));
    set.spawn(work(2, 50));
    set.spawn(work(3, 80));

    while let Some(res) = set.join_next().await {
        match res {
            Ok(id) => println!("任务 {} 完成", id),
            Err(e) => println!("任务失败: {}", e),
        }
    }
}
```

**说明：** `JoinSet` 是动态管理多个 `JoinHandle` 的工具。相比手动收集 `Vec<JoinHandle>`，`JoinSet` 的优势是：1）按完成顺序返回结果（不一定是添加顺序）；2）支持动态添加新任务；3）可以在任务完成时立即处理，无需等待所有任务结束。
</details>

---

### 练习 19-19: select! 与 channel 混合

> 难度：⭐⭐⭐
> 类似 Go 的 `select` 监听多个 channel

补全代码，使用 `select!` 同时监听两个 channel 的消息。

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);

    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx1.send("来自通道1").await.unwrap();
    });

    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        tx2.send("来自通道2").await.unwrap();
    });

    // TODO: 使用 select! 同时等待两个通道的消息，打印先到达的那个
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);

    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx1.send("来自通道1").await.unwrap();
    });

    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        tx2.send("来自通道2").await.unwrap();
    });

    tokio::select! {
        msg = rx1.recv() => println!("通道1: {}", msg.unwrap()),
        msg = rx2.recv() => println!("通道2: {}", msg.unwrap()),
    }
    // 输出: 通道2: 来自通道2（通道2 50ms 先到）
}
```

**说明：** `select!` 可以同时监听多个 channel 的 `recv()`，哪个先收到数据就处理哪个，另一个被取消。这和 Go 语言的 `select` 非常相似。注意 `rx.recv()` 返回 `Option<T>`，需要 `unwrap()`。
</details>

---

### 练习 19-20: 带超时的 select 循环

> 难度：⭐⭐⭐
> 类似 Java 的 `ScheduledExecutorService` 带超时的轮询

挑战：实现一个循环，每隔 100ms 打印一次心跳，同时监听 channel 消息，当收到 "stop" 时退出。

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // 模拟 250ms 后发送停止信号
    tokio::spawn(async move {
        sleep(Duration::from_millis(250)).await;
        tx.send("stop").await.unwrap();
    });

    // TODO: 使用 loop + select!，每个循环：
    // - 分支1: rx.recv() 收到消息，如果是 "stop" 就 break
    // - 分支2: sleep(100ms) 到时间，打印 "心跳..."
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        sleep(Duration::from_millis(250)).await;
        tx.send("stop").await.unwrap();
    });

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Some(s) if s == "stop" => {
                        println!("收到停止信号，退出");
                        break;
                    }
                    Some(s) => println!("收到消息: {}", s),
                    None => break, // 通道关闭
                }
            }
            _ = sleep(Duration::from_millis(100)) => {
                println!("心跳...");
            }
        }
    }
}
```

**说明：** 这个模式实现了**定时心跳 + 消息监听**的混合循环。`select!` 的每个分支都是独立的 Future，`sleep(Duration)` 返回一个 Future，它在指定时间后完成。如果既没有消息也没有超时，循环会一直等待——但这里 sleep 分支保证至少每 100ms 唤醒一次。这种模式在实现 WebSocket 心跳保活、任务超时取消等场景中非常常见。
</details>

---

### 练习 19-21: 模拟并发 HTTP 请求

> 难度：⭐⭐⭐
> 类似 Java 的 `CompletableFuture` 并发请求

填空，模拟并发发起多个 HTTP 请求并汇总结果。

```rust
use tokio::time::{sleep, Duration};

/// 模拟 HTTP 请求，返回响应体长度
async fn mock_http_request(url: &str) -> usize {
    // 模拟网络延迟
    sleep(Duration::from_millis(100)).await;
    // 模拟响应体长度
    url.len() * 10
}

#[tokio::main]
async fn main() {
    let urls = vec!["https://example.com", "https://rust-lang.org", "https://tokio.rs"];

    // TODO: 并发发起所有请求，汇总所有响应体长度并打印总和
    // 提示: 使用 futures::future::join_all 或者 Vec<JoinHandle>
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

async fn mock_http_request(url: &str) -> usize {
    sleep(Duration::from_millis(100)).await;
    url.len() * 10
}

#[tokio::main]
async fn main() {
    let urls = vec!["https://example.com", "https://rust-lang.org", "https://tokio.rs"];

    let mut handles = Vec::new();
    for url in urls {
        handles.push(tokio::spawn(mock_http_request(url)));
    }

    let mut total = 0;
    for handle in handles {
        total += handle.await.unwrap();
    }

    println!("总响应体长度: {}", total);
}
```

**说明：** 通过 `tokio::spawn` 将每个请求变成独立任务并发执行，总耗时约等于最慢的请求（本例 ~100ms），而不是请求数 × 100ms。如果使用标准库中的 `futures::future::join_all`（需要 `futures` crate），可以避免 spawn 的开销，直接并发等待多个 Future。
</details>

---

### 练习 19-22: 竞速模式——第一个成功的结果

> 难度：⭐⭐⭐
> 类似 Java 的 `CompletableFuture.anyOf` / 类似 Go 的 `select`

补全代码，向多个数据源并发请求，使用最先返回的成功结果。

```rust
use tokio::time::{sleep, Duration};

async fn fetch_from(source: &str, data: &str, ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(ms)).await;
    if data.is_empty() {
        Err(format!("{} 返回空数据", source))
    } else {
        Ok(format!("来自 {}: {}", source, data))
    }
}

// TODO: 定义函数 first_success，并发调用多个 fetch，返回第一个 Ok 的结果
// 如果全部失败则返回最后一个 Err

#[tokio::main]
async fn main() {
    // let result = first_success().await;
    // println!("{}", result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::time::{sleep, Duration};

async fn fetch_from(source: &str, data: &str, ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(ms)).await;
    if data.is_empty() {
        Err(format!("{} 返回空数据", source))
    } else {
        Ok(format!("来自 {}: {}", source, data))
    }
}

async fn first_success() -> String {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let sources = vec![
        ("Server-A", "hello", 200),
        ("Server-B", "", 50),   // 最快但失败
        ("Server-C", "world", 150),
    ];

    for (name, data, ms) in sources {
        let tx = tx.clone();
        tokio::spawn(async move {
            let result = fetch_from(name, data, ms).await;
            tx.send(result).await.unwrap();
        });
    }
    drop(tx); // 让 rx 知道所有发送者已结束

    let mut last_err = String::new();
    while let Some(result) = rx.recv().await {
        match result {
            Ok(msg) => return msg,
            Err(e) => last_err = e,
        }
    }
    last_err // 全部失败返回最后的错误
}

#[tokio::main]
async fn main() {
    let result = first_success().await;
    println!("{}", result);
    // 输出: 来自 Server-C: world （Server-B 最快但失败，Server-C 次快但成功）
}
```

**说明：** 竞速模式（Race Pattern）通过 `mpsc::channel(1)`（容量为 1，第一个 send 成功即可）收集多个并发任务的结果，一旦收到 `Ok` 就立即返回。这类似于微服务中的"请求备份"(hedged requests)策略——向多个副本发送请求，用最先成功的响应。注意 `drop(tx)` 很重要，否则 `rx.recv()` 不会结束。
</details>

---

### 练习 19-23: 异步任务流水线

> 难度：⭐⭐⭐
> 类似 Java 的 `CompletionStage` 编排

填空，实现一个三阶段异步流水线：读取 → 处理 → 输出，每个阶段通过 channel 连接。

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn stage_read(output: mpsc::Sender<String>) {
    // 模拟读取数据
    for i in 1..=3 {
        sleep(Duration::from_millis(30)).await;
        output.send(format!("原始数据 {}", i)).await.unwrap();
    }
}

async fn stage_process(input: mpsc::Receiver<String>, output: mpsc::Sender<String>) {
    // TODO: 接收数据，转换为大写，发送到下一阶段
}

async fn stage_output(input: mpsc::Receiver<String>) {
    // TODO: 接收并打印处理后的数据
}

#[tokio::main]
async fn main() {
    // TODO: 创建两个 channel，串联三个阶段
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn stage_read(output: mpsc::Sender<String>) {
    for i in 1..=3 {
        sleep(Duration::from_millis(30)).await;
        output.send(format!("原始数据 {}", i)).await.unwrap();
    }
}

async fn stage_process(mut input: mpsc::Receiver<String>, output: mpsc::Sender<String>) {
    while let Some(data) = input.recv().await {
        let processed = data.to_uppercase();
        output.send(processed).await.unwrap();
    }
}

async fn stage_output(mut input: mpsc::Receiver<String>) {
    while let Some(data) = input.recv().await {
        println!("输出: {}", data);
    }
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = mpsc::channel(32);
    let (tx2, rx2) = mpsc::channel(32);

    tokio::spawn(stage_read(tx1));
    tokio::spawn(stage_process(rx1, tx2));
    tokio::spawn(stage_output(rx2));

    // 等待所有任务完成（简单方式：sleep 足够长时间）
    sleep(Duration::from_millis(200)).await;
}
```

**说明：** 这是异步流水线（Pipeline）的经典模式：每个阶段是一个独立的任务，通过 `mpsc` channel 连接。数据在阶段间流动，天然实现了并行的生产者-消费者。每个阶段可以独立伸缩，且由于 channel 的缓冲作用，各阶段可以以不同速度运行。这是 Rust 异步实现数据流处理的基石。
</details>

---

### 练习 19-24: 优雅关闭与取消任务

> 难度：⭐⭐⭐
> 类似 Java 的 `ExecutorService.shutdown` / 类似 Go 的 `context.WithCancel`

补全代码，实现一个可被优雅关闭的后台工作任务。

```rust
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

async fn worker(mut rx: watch::Receiver<bool>) {
    // TODO: 循环监听，每 50ms 打印 "工作中..."，当 rx 收到 true 时打印 "正在关闭..." 并退出
    // 提示: 使用 select! 同时等待 rx.changed() 和 sleep
}

#[tokio::main]
async fn main() {
    // let (tx, rx) = watch::channel(false);
    // TODO: spawn worker，主任务 sleep 150ms 后发送关闭信号，等待 worker 结束
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use tokio::sync::watch;
use tokio::time::{sleep, Duration};

async fn worker(mut rx: watch::Receiver<bool>) {
    loop {
        tokio::select! {
            // 当 rx 的值变化时，changed() 返回 Ok(())
            _ = rx.changed() => {
                if *rx.borrow() {
                    println!("正在关闭...");
                    break;
                }
            }
            _ = sleep(Duration::from_millis(50)) => {
                println!("工作中...");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = watch::channel(false);

    let handle = tokio::spawn(worker(rx));

    sleep(Duration::from_millis(150)).await;
    tx.send(true).unwrap();

    handle.await.unwrap();
    println!("主任务结束");
}
```

**说明：** `tokio::sync::watch` 是一个单生产者多消费者的广播通道，适合发送关闭信号。`rx.changed()` 是一个 Future，在值变化时完成。通过 `select!` 将工作循环和关闭信号结合起来，就实现了**可优雅停止的后台任务**。`watch` 的优点是所有 `Receiver` 都能收到相同的信号（不同于 mpsc 的一条消息只能被一个消费者接收）。
</details>

---

### 练习 19-25: 简易任务调度器

> 难度：⭐⭐⭐
> 类似 Java 的 `ScheduledExecutorService` / 类似 Go 的 `time.Ticker`

挑战：实现一个简易的任务调度器，可以按指定间隔周期执行任务，并支持取消。

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration, Instant};

// TODO: 实现 Ticker，每 interval 时间返回一次 ()
// 提示: 可以实现一个自定义 Future，或者组合 sleep
struct Ticker {
    interval: Duration,
    // 可以添加需要的字段
}

impl Future for Ticker {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // TODO: 实现轮询逻辑，每 interval 返回 Poll::Ready(())
        todo!()
    }
}

#[tokio::main]
async fn main() {
    // TODO: 使用 Ticker 每 100ms 打印一次 "tick"，共打印 3 次后退出
    // 提示: 可以用 tokio::time::timeout 或一个计数器来实现退出条件
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration, Instant, Sleep};

/// 周期性 tick 的 Future，每 interval 完成一次
struct Ticker {
    interval: Duration,
    sleep: Pin<Box<Sleep>>,
}

impl Ticker {
    fn new(interval: Duration) -> Self {
        Self {
            interval,
            sleep: Box::pin(sleep(interval)),
        }
    }
}

impl Future for Ticker {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // 检查当前 sleep 是否完成
        if self.sleep.as_mut().poll(cx).is_ready() {
            // 重置下一次 sleep
            self.sleep = Box::pin(sleep(self.interval));
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// 更简化的方式：直接用循环 + sleep，不需要自定义 Future
async fn ticker_simple(interval: Duration, mut count: usize) {
    while count > 0 {
        sleep(interval).await;
        println!("tick");
        count -= 1;
    }
}

#[tokio::main]
async fn main() {
    // 使用 Ticker 自定义 Future
    let mut ticker = Ticker::new(Duration::from_millis(100));
    for _ in 0..3 {
        ticker.await;
        println!("tick (自定义)");
    }

    println!("---");

    // 使用更简单的方式
    ticker_simple(Duration::from_millis(100), 3).await;
}
```

**说明：** 自定义 `Future` 是 Rust 异步编程的高级技巧。`Ticker` 内部维护一个 `Sleep` Future，每次轮询时检查它是否完成，完成时立即重置并返回 `Ready(())`。不过在实际应用中，更常见的做法是直接在循环中用 `sleep` 来实现周期性任务——更简单且不易出错。自定义 `Future` 通常只在对性能或语义有特殊要求时才使用。

**调度器扩展思路：** 可以在 `Ticker` 基础上添加 `cancel` 信号（使用 `watch` 或 `oneshot`），或者支持动态调整间隔。但 YAGNI——除非确实需要，否则循环 + `sleep` 足够了。
</details>

---

> **注意：** 所有异步练习均需要 `tokio` crate。在 `Cargo.toml` 中添加：
> ```toml
> [dependencies]
> tokio = { version = "1", features = ["full"] }
> ```
> 
> **核心概念回顾：**
> - **`Future`**：一个代表**将来某个时刻会完成的值**的 trait。Rust 的 Future 是**惰性**的——除非被轮询，否则什么都不会发生。
> - **`async fn`**：语法糖，将函数体转为返回 `impl Future` 的状态机。每个 `.await` 点都是一个状态转换点。
> - **`.await`**：在当前任务上**异步等待** Future 完成。如果 Future 未就绪，当前任务会**让出控制权**，运行时可以调度其他任务。
> - **`tokio::spawn`**：将 Future 提交给运行时作为**独立任务**并发执行。
> - **异步运行时（tokio）**：负责驱动所有 Future 的轮询、调度任务、管理 I/O 事件。Rust 标准库不包含运行时，你需要选择 tokio、async-std 等。
> - **关键区别**：异步 ≠ 并行。异步任务在单线程上协作式调度（当然 tokio 支持多线程工作窃取），让 I/O 等待不阻塞线程；而 `std::thread` 是真正的并行执行，有线程创建和上下文切换的开销。
