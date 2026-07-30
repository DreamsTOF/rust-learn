# 18 多线程并发

Rust 的所有权系统和类型系统为并发编程提供了强有力的安全保证。通过 `std::thread` 创建线程、`mpsc` 进行消息传递、`Arc<Mutex<T>>` 实现共享状态，以及 `Send` 和 `Sync` 这两个标记 trait 在编译期确保线程安全。本章练习将帮助你掌握 Rust 并发编程的核心概念与实践技巧。

### 练习 18-01: 创建第一个线程

> 难度：⭐⭐
> 类似 Java 的 `Thread.start()` / 类似 C++11 的 `std::thread`

补全代码，使用 `thread::spawn` 创建一个新线程，在新线程中打印一条消息。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // TODO: 使用 thread::spawn 创建一个新线程，打印 "你好，来自新线程！"
    
    // 主线程等待一段时间，让子线程有机会执行
    thread::sleep(Duration::from_millis(100));
    println!("主线程结束");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("你好，来自新线程！");
    });
    
    thread::sleep(Duration::from_millis(100));
    println!("主线程结束");
}
```

**说明：** `thread::spawn` 接收一个闭包作为参数，闭包中的代码将在新线程中执行。主线程通过 `thread::sleep` 等待子线程完成——但这不是可靠的方式，实际应使用 `join` 句柄来等待线程结束。
</details>

### 练习 18-02: 使用 move 闭包

> 难度：⭐⭐
> 类似 Java 的传递参数给线程 / 类似 C++11 的 `std::thread` 传参

补全代码，使用 `move` 关键字将数据从主线程移动到子线程中。

```rust
use std::thread;

fn main() {
    let message = String::from("你好，世界！");
    
    // TODO: 使用 thread::spawn + move 闭包，打印 message
    // 提示：闭包前需要加 move 关键字
    
    // 注意：这里不能再用 message 了，因为所有权已转移
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;

fn main() {
    let message = String::from("你好，世界！");
    
    let handle = thread::spawn(move || {
        println!("{}", message);
    });
    
    handle.join().unwrap();
}
```

**说明：** `move` 关键字强制闭包获取其捕获变量的所有权。在多线程场景中，这确保数据安全地从主线程移动到子线程，避免了生命周期和借用冲突。如果没有 `move`，闭包只会借用 `message`，而编译器无法保证借用在线程执行期间始终有效。
</details>

### 练习 18-03: 等待线程——join 基础

> 难度：⭐⭐
> 类似 Java 的 `thread.join()` / 类似 C++11 的 `std::thread::join()`

补全代码，使用 `join` 等待子线程执行完毕，并获取线程的返回值。

```rust
use std::thread;

fn main() {
    // TODO: 创建一个线程，计算 1 到 100 的和，返回结果
    // 使用 join 获取返回值并打印
    let handle = thread::spawn(|| {
        // 在这里计算 1+2+...+100
        // TODO
    });
    
    // TODO: 使用 handle.join() 获取结果并打印
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let sum: i32 = (1..=100).sum();
        sum
    });
    
    let result = handle.join().unwrap();
    println!("1 到 100 的和是: {}", result);
}
```

**说明：** `thread::spawn` 返回一个 `JoinHandle<T>`，调用其 `join()` 方法会阻塞当前线程直到子线程执行完毕。`join()` 返回 `Result<T, Box<dyn Any + Send>>`，如果子线程 panic，则返回 `Err`。
</details>

### 练习 18-04: 创建多个线程

> 难度：⭐⭐
> 类似 Java 循环启动多个线程 / 类似 C++11 的线程数组

补全代码，创建多个线程分别计算不同范围内的和，然后汇总结果。

```rust
use std::thread;

fn main() {
    let mut handles = vec![];
    
    // TODO: 创建 4 个线程，每个线程计算一段连续数字的和
    // 线程 0: 1..=25, 线程 1: 26..=50, 线程 2: 51..=75, 线程 3: 76..=100
    for i in 0..4 {
        // TODO: 计算每个线程的起始和结束范围
        // 使用 thread::spawn，返回该段的和
        // 将 handle 存入 handles 向量
    }
    
    // TODO: 遍历 handles，使用 join 收集每个线程的结果，累加并打印总和
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;

fn main() {
    let mut handles = vec![];
    
    for i in 0..4 {
        let start = i * 25 + 1;
        let end = (i + 1) * 25;
        let handle = thread::spawn(move || {
            (start..=end).sum::<i32>()
        });
        handles.push(handle);
    }
    
    let total: i32 = handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum();
    
    println!("1 到 100 的总和: {}", total);
}
```

**说明：** 可以将 `JoinHandle` 存入向量来管理多个线程。遍历向量并依次 `join` 可确保所有线程执行完毕。`move` 闭包捕获循环变量 `start` 和 `end`，每个线程获得自己独立的副本。
</details>

### 练习 18-05: 线程返回值与错误处理挑战

> 难度：⭐⭐
> 类似 Java 的 `Callable` / 类似 C++11 的 `std::future`

实现一个函数 `parallel_map`，对向量中的每个元素在一个独立线程中应用转换函数，返回一个包含所有结果的新向量。

```rust
use std::thread;

// TODO: 实现 parallel_map 函数
// 接收一个 Vec<i32> 和一个闭包，对每个元素在线程中应用闭包
// 返回 Vec<i32>，保持元素的原始顺序
fn parallel_map<F>(data: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Copy + 'static,
{
    // TODO
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = parallel_map(data, |x| x * x);
    println!("平方结果: {:?}", result); // [1, 4, 9, 16, 25]
    
    let data2 = vec![10, 20, 30];
    let result2 = parallel_map(data2, |x| x + 5);
    println!("加 5 结果: {:?}", result2); // [15, 25, 35]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;

fn parallel_map<F>(data: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32 + Send + Copy + 'static,
{
    let mut handles = vec![];
    
    for x in data {
        let handle = thread::spawn(move || {
            f(x)
        });
        handles.push(handle);
    }
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = parallel_map(data, |x| x * x);
    println!("平方结果: {:?}", result);
    
    let data2 = vec![10, 20, 30];
    let result2 = parallel_map(data2, |x| x + 5);
    println!("加 5 结果: {:?}", result2);
}
```

**说明：** 泛型约束 `Send` 确保闭包可以安全地跨线程传递，`Copy` 允许闭包被多次 `move`，`'static` 保证闭包不包含非静态引用。通过收集所有 `JoinHandle` 再依次 `join`，可以维持原始顺序——因为向量按插入顺序迭代，而插入顺序与输入顺序一致。
</details>

### 练习 18-06: 发送与接收——mpsc 基础

> 难度：⭐⭐
> 类似 Java 的 `BlockingQueue` / 类似 C++11 的 `std::queue` + 条件变量

补全代码，使用 `mpsc::channel` 创建一个通道，在子线程中发送消息，在主线程中接收。

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    // TODO: 使用 mpsc::channel() 创建 (发送端, 接收端)
    // let (tx, rx) = ...
    
    thread::spawn(move || {
        // TODO: 发送 "你好" 字符串到通道
    });
    
    // TODO: 在主线程中接收消息并打印
    // 使用 rx.recv() 或 rx.try_recv()
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("你好").unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("收到消息: {}", received);
}
```

**说明：** `mpsc` 是 "Multiple Producer, Single Consumer"（多生产者，单消费者）的缩写。`channel()` 返回 `(Sender<T>, Receiver<T>)` 元组。`send()` 发送消息，`recv()` 阻塞等待消息。`send` 返回 `Result` 因为如果接收端已丢弃，发送会失败。
</details>

### 练习 18-07: 发送多条消息

> 难度：⭐⭐
> 类似 Java 连续向队列发送多条消息

补全代码，在子线程中发送多条消息，主线程逐条接收。

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["消息1", "消息2", "消息3", "消息4", "消息5"];
        // TODO: 循环发送 messages 中的每条消息
        // 每条消息发送后睡眠 100ms 模拟工作
        for msg in messages {
            // TODO
        }
    });
    
    // TODO: 在主线程中循环接收 5 条消息并打印
    // 提示：可以逐条调用 recv()，也可以将 rx 当作迭代器使用
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["消息1", "消息2", "消息3", "消息4", "消息5"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
}
```

**说明：** `Receiver` 实现了 `Iterator` trait，可以直接在 `for` 循环中使用。当所有发送端（`Sender`）被丢弃后，迭代器会自动结束。这种方式比逐个调用 `recv()` 更简洁。
</details>

### 练习 18-08: 多个发送者

> 难度：⭐⭐
> 类似 Java 的多个生产者线程 / 类似 C++ 多生产者-单消费者

补全代码，创建多个线程作为发送者，每个线程发送消息到同一个通道。

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    // TODO: 创建通道
    // 注意：mpsc 只支持一个消费者，但支持多个生产者
    
    // TODO: 创建 3 个发送者线程，每个线程发送 2 条消息
    // 提示：需要使用 tx.clone() 复制发送端
    let mut handles = vec![];
    for i in 0..3 {
        // TODO: clone 发送端，在线程中发送消息
        // 消息格式: "线程{i} 的消息{j}"
    }
    
    // TODO: 等待所有线程结束
    // 并接收所有消息，打印总数
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let mut handles = vec![];
    for i in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..2 {
                tx_clone.send(format!("线程{} 的消息{}", i, j)).unwrap();
            }
        });
        handles.push(handle);
    }
    
    // 丢弃原始发送端，否则接收端会一直等待
    drop(tx);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mut count = 0;
    for msg in rx {
        println!("{}", msg);
        count += 1;
    }
    println!("共计接收 {} 条消息", count);
}
```

**说明：** `Sender<T>` 实现了 `Clone` trait，可以通过克隆创建多个发送者。必须丢弃（`drop`）所有发送端（包括原始发送端），接收端的迭代器才会结束。`mpsc` 的名称正是"多生产者，单消费者"。
</details>

### 练习 18-09: 发送多种类型——枚举消息

> 难度：⭐⭐
> 类似 Java 的消息对象多态 / 类似 C++ 的 `std::variant`

补全代码，使用枚举在一个通道中发送不同类型的消息。

```rust
use std::thread;
use std::sync::mpsc;

// TODO: 定义 Message 枚举，包含以下变体：
// - Text(String)
// - Number(i32)
// - Exit

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let sender = thread::spawn(move || {
        // TODO: 发送 Text("开始处理")，Number(42)，Text("继续")，Number(100)，Exit
    });
    
    // TODO: 在主线程中使用循环接收消息
    // 遇到 Exit 时终止循环
    // Text 时打印 "文本: {内容}"
    // Number 时打印 "数字: {值}"
    
    sender.join().unwrap();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::mpsc;

enum Message {
    Text(String),
    Number(i32),
    Exit,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    let sender = thread::spawn(move || {
        tx.send(Message::Text("开始处理".to_string())).unwrap();
        tx.send(Message::Number(42)).unwrap();
        tx.send(Message::Text("继续".to_string())).unwrap();
        tx.send(Message::Number(100)).unwrap();
        tx.send(Message::Exit).unwrap();
    });
    
    loop {
        match rx.recv().unwrap() {
            Message::Text(text) => println!("文本: {}", text),
            Message::Number(n) => println!("数字: {}", n),
            Message::Exit => {
                println!("收到退出信号");
                break;
            }
        }
    }
    
    sender.join().unwrap();
}
```

**说明：** 使用枚举作为通道消息类型是一种常见模式，可以在单一通道中传递多种类型的数据。配合 `match` 表达式，接收端可以清晰处理每种消息变体。`Exit` 消息常用于优雅地通知工作线程结束。
</details>

### 练习 18-10: 消息通道综合挑战

> 难度：⭐⭐
> 类似 Java 的生产者-消费者模式

实现一个简单的工作分发系统：主线程生成任务，多个工作线程从通道接收任务并处理，将结果通过另一个通道返回给主线程。

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    // TODO: 创建两个通道：
    // job_tx / job_rx: 用于发送任务（String）
    // result_tx / result_rx: 用于接收结果（String）
    
    // TODO: 启动 3 个工作线程，每个线程从 job_rx 接收任务
    // 处理方式：将收到的字符串转换为大写
    // 结果通过 result_tx 发回
    
    // TODO: 主线程发送 5 个任务
    // 然后丢弃 job_tx（关闭任务通道）
    
    // TODO: 从 result_rx 接收并打印所有结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (job_tx, job_rx) = mpsc::channel::<String>();
    let (result_tx, result_rx) = mpsc::channel::<String>();
    
    let job_rx = std::sync::Arc::new(std::sync::Mutex::new(job_rx));
    let mut handles = vec![];
    
    for id in 0..3 {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();
        let handle = thread::spawn(move || {
            loop {
                let job = {
                    let lock = job_rx.lock().unwrap();
                    lock.recv()
                };
                match job {
                    Ok(task) => {
                        let result = task.to_uppercase();
                        result_tx.send(format!("线程{} 处理: {}", id, result)).unwrap();
                    }
                    Err(_) => break, // 通道关闭，退出循环
                }
            }
        });
        handles.push(handle);
    }
    
    // 发送任务
    for i in 0..5 {
        job_tx.send(format!("任务-{}", i)).unwrap();
    }
    drop(job_tx); // 关闭任务通道
    
    // 丢弃结果发送端（克隆的），否则接收端不会结束
    drop(result_tx);
    
    // 接收结果
    for result in result_rx {
        println!("{}", result);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

**说明：** 这个练习展示了经典的生产者-消费者模式。任务通道使用 `Arc<Mutex<Receiver>>` 实现多线程共享接收端——因为 `Receiver` 本身不是 `Sync` 的，需要加锁保护。工作线程从任务通道接收任务，处理后将结果通过另一个通道返回。当所有发送端丢弃后，通道自动关闭，工作线程通过 `recv()` 返回的 `Err` 检测到关闭并退出。
</details>

### 练习 18-11: Arc 基础——原子引用计数

> 难度：⭐⭐
> 类似 Java 的 `AtomicInteger` 计数 / 类似 C++11 的 `std::shared_ptr`

补全代码，使用 `Arc`（原子引用计数）在多个线程间共享数据。

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    // TODO: 创建一个 Arc<String> 包裹的字符串 "共享数据"
    let data = // TODO
    
    let mut handles = vec![];
    for i in 0..5 {
        // TODO: clone Arc 并在线程中打印数据
        let data_clone = data.clone();
        handles.push(thread::spawn(move || {
            // TODO: 打印 "线程{i}: {data}"
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let data = Arc::new(String::from("共享数据"));
    
    let mut handles = vec![];
    for i in 0..5 {
        let data_clone = data.clone();
        handles.push(thread::spawn(move || {
            println!("线程{}: {}", i, data_clone);
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
}
```

**说明：** `Arc<T>`（Atomic Reference Counting）是线程安全的引用计数智能指针。`Arc::clone()` 增加引用计数，每个 `Arc` 被丢弃时计数减 1，当计数归零时数据被释放。与 `Rc<T>` 不同，`Arc` 使用原子操作更新计数，因此是 `Send` 和 `Sync` 的，可以在线程间安全共享。
</details>

### 练习 18-12: Mutex 基础——互斥锁

> 难度：⭐⭐
> 类似 Java 的 `synchronized` / 类似 C++11 的 `std::mutex`

补全代码，使用 `Mutex` 保护共享数据，在不同线程中修改它。

```rust
use std::sync::Mutex;

fn main() {
    // TODO: 创建一个 Mutex<i32>，初始值为 0
    let counter = // TODO
    
    {
        // TODO: 使用 lock() 获取数据并修改为 10
        let mut guard = // TODO
        // TODO
    } // 锁在此处自动释放
    
    // TODO: 再次上锁，读取并打印 counter 的值
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    
    {
        let mut guard = counter.lock().unwrap();
        *guard = 10;
    }
    
    let guard = counter.lock().unwrap();
    println!("counter = {}", *guard);
}
```

**说明：** `Mutex<T>` 提供互斥锁，确保同一时间只有一个线程可以访问内部数据。`lock()` 返回 `MutexGuard<T>`，这是一个智能指针，通过 `Deref` 提供对内部数据的访问。当 `MutexGuard` 离开作用域时自动释放锁。`lock()` 返回 `Result`，因为如果持有锁的线程 panic，其他线程会收到 `PoisonError`。
</details>

### 练习 18-13: `Arc<Mutex<T>>`——在多个线程中修改共享数据

> 难度：⭐⭐
> 类似 Java 的 `synchronized` + 共享对象 / 类似 C++11 的 `std::shared_mutex`

补全代码，使用 `Arc<Mutex<i32>>` 让多个线程安全地递增一个共享计数器。

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // TODO: 创建 Arc<Mutex<i32>>，初始值为 0
    let counter = // TODO
    
    let mut handles = vec![];
    for _ in 0..10 {
        // TODO: clone Arc，在每个线程中将 counter 递增 10 次
    }
    
    // TODO: 等待所有线程结束，打印最终的 counter 值
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数: {}", *counter.lock().unwrap());
}
```

**说明：** `Arc<Mutex<T>>` 是 Rust 多线程共享可变数据最常用的组合。`Arc` 负责跨线程共享所有权，`Mutex` 负责互斥访问。10 个线程各递增 10 次，最终结果应为 100。注意每次循环中都要先 `lock()` 再修改，确保操作是原子的。
</details>

### 练习 18-14: `Arc<Mutex<Vec<T>>>`——共享集合

> 难度：⭐⭐
> 类似 Java 的 `synchronizedList` / 类似 C++ 的线程安全队列

补全代码，多个线程向同一个共享的 `Vec` 中添加元素。

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // TODO: 创建 Arc<Mutex<Vec<i32>>>，初始为空向量
    let shared_vec = // TODO
    
    let mut handles = vec![];
    for i in 0..5 {
        // TODO: 每个线程将自己的线程编号 i 添加到共享向量中（添加 10 次）
    }
    
    // TODO: 等待所有线程，打印向量长度和内容
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let shared_vec = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = vec![];
    for i in 0..5 {
        let shared_vec = shared_vec.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut vec = shared_vec.lock().unwrap();
                vec.push(i);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let vec = shared_vec.lock().unwrap();
    println!("向量长度: {}", vec.len()); // 应为 50
    println!("向量内容: {:?}", vec);
}
```

**说明：** `Arc<Mutex<Vec<T>>>` 允许跨线程共享并修改一个集合。每个线程通过 `lock()` 获取独占访问权后向向量添加元素。由于 `Mutex` 的互斥性，不需要担心数据竞争。最终长度为 5 × 10 = 50。
</details>

### 练习 18-15: 共享状态综合挑战——银行转账系统

> 难度：⭐⭐
> 类似 Java 的线程安全账户操作

实现一个简单的银行系统，包含多个账户，支持线程安全的转账操作。使用 `Arc<Mutex<HashMap>>` 存储账户余额。

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// TODO: 实现 Bank 结构体
// 内部使用 Arc<Mutex<HashMap<String, f64>>> 存储账户余额
struct Bank {
    accounts: Arc<Mutex<HashMap<String, f64>>>,
}

impl Bank {
    fn new() -> Self {
        // TODO: 创建 Bank，初始化几个账户
    }
    
    fn transfer(&self, from: &str, to: &str, amount: f64) -> Result<(), String> {
        // TODO: 实现转账操作
        // 1. 上锁
        // 2. 检查 from 账户余额是否足够
        // 3. 扣除 from 账户，增加 to 账户
        // 4. 返回 Ok(()) 或 Err(错误信息)
    }
    
    fn balance(&self, name: &str) -> Option<f64> {
        // TODO: 查询账户余额
    }
}

fn main() {
    let bank = Arc::new(Bank::new());
    let mut handles = vec![];
    
    // 启动多个线程执行转账
    let transfers = vec![
        ("Alice", "Bob", 100.0),
        ("Bob", "Charlie", 50.0),
        ("Charlie", "Alice", 30.0),
        ("Alice", "Charlie", 200.0),
        ("Bob", "Alice", 20.0),
    ];
    
    for (from, to, amount) in transfers {
        let bank = bank.clone();
        handles.push(thread::spawn(move || {
            match bank.transfer(from, to, amount) {
                Ok(()) => println!("转账成功: {} -> {}: {:.1}", from, to, amount),
                Err(e) => println!("转账失败: {}", e),
            }
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 打印最终余额
    for name in &["Alice", "Bob", "Charlie"] {
        if let Some(bal) = bank.balance(name) {
            println!("{} 余额: {:.1}", name, bal);
        }
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct Bank {
    accounts: Arc<Mutex<HashMap<String, f64>>>,
}

impl Bank {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("Alice".to_string(), 1000.0);
        map.insert("Bob".to_string(), 500.0);
        map.insert("Charlie".to_string(), 300.0);
        Bank {
            accounts: Arc::new(Mutex::new(map)),
        }
    }
    
    fn transfer(&self, from: &str, to: &str, amount: f64) -> Result<(), String> {
        let mut accounts = self.accounts.lock().unwrap();
        
        let from_balance = accounts.get(from)
            .ok_or_else(|| format!("账户 {} 不存在", from))?;
        
        if *from_balance < amount {
            return Err(format!("账户 {} 余额不足", from));
        }
        
        // 先扣减 from 账户
        if let Some(bal) = accounts.get_mut(from) {
            *bal -= amount;
        }
        
        // 增加 to 账户
        if let Some(bal) = accounts.get_mut(to) {
            *bal += amount;
        } else {
            // 如果 to 账户不存在，回滚 from 账户的扣减
            if let Some(bal) = accounts.get_mut(from) {
                *bal += amount;
            }
            return Err(format!("账户 {} 不存在", to));
        }
        
        Ok(())
    }
    
    fn balance(&self, name: &str) -> Option<f64> {
        let accounts = self.accounts.lock().unwrap();
        accounts.get(name).copied()
    }
}

fn main() {
    let bank = Arc::new(Bank::new());
    let mut handles = vec![];
    
    let transfers = vec![
        ("Alice", "Bob", 100.0),
        ("Bob", "Charlie", 50.0),
        ("Charlie", "Alice", 30.0),
        ("Alice", "Charlie", 200.0),
        ("Bob", "Alice", 20.0),
    ];
    
    for (from, to, amount) in transfers {
        let bank = bank.clone();
        handles.push(thread::spawn(move || {
            match bank.transfer(from, to, amount) {
                Ok(()) => println!("转账成功: {} -> {}: {:.1}", from, to, amount),
                Err(e) => println!("转账失败: {}", e),
            }
        }));
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    for name in &["Alice", "Bob", "Charlie"] {
        if let Some(bal) = bank.balance(name) {
            println!("{} 余额: {:.1}", name, bal);
        }
    }
}
```

**说明：** 银行转账系统展示了 `Arc<Mutex<HashMap>>` 的实际应用。整个账户集合在一次 `lock()` 中完成读写操作，确保转账的原子性。在错误处理中（目标账户不存在），需要回滚已扣减的金额，避免数据不一致。生产系统中还应考虑死锁预防（例如对锁进行排序）和更细粒度的锁策略。
</details>

### 练习 18-16: 理解 Send trait

> 难度：⭐⭐⭐
> Rust 独有的并发安全 trait / 类似 C++ 的线程安全类型标记

`Send` 是一个标记 trait，表示类型的所有权可以安全地在线程间转移。如果一个类型实现了 `Send`，它的值可以安全地移动到另一个线程。补全代码，理解哪些类型是 `Send` 的。

```rust
use std::thread;
use std::rc::Rc;

// TODO: 判断以下哪些类型实现了 Send，在注释中写出你的推理

// 类型 A: 只包含 i32 字段
struct A(i32);

// 类型 B: 包含 Rc<i32> 字段
struct B(Rc<i32>);

// 类型 C: 包含 Arc<i32> 字段
struct C(std::sync::Arc<i32>);

// 类型 D: 包含 *mut i32 原始指针
struct D(*mut i32);

fn is_send<T: Send>() {}

fn main() {
    // TODO: 对以下类型调用 is_send，看哪些能编译通过
    // 能编译的表示该类型实现了 Send
    
    // is_send::<A>();
    // is_send::<B>();
    // is_send::<C>();
    // is_send::<D>();
    
    // TODO: 取消编译成功的行的注释，观察编译器的行为
    // 对于不 Send 的类型，尝试在线程间发送会发生什么？
    // 补一个尝试在线程中发送 Rc 的代码，观察编译错误
    
    let data = Rc::new(42);
    // TODO: 尝试将 Rc 发送到另一个线程中，看看编译器的错误信息
    // thread::spawn(move || {
    //     println!("{}", data);
    // });
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::rc::Rc;

struct A(i32);
struct B(Rc<i32>);
struct C(std::sync::Arc<i32>);
struct D(*mut i32);

fn is_send<T: Send>() {}

fn main() {
    // A 只包含 i32（实现了 Send），所以 A 是 Send
    is_send::<A>();
    
    // B 包含 Rc<i32>，Rc 不是 Send，所以 B 也不是 Send
    // 如果取消注释下一行，会编译错误
    // is_send::<B>(); // 编译错误: Rc<i32> 未实现 Send
    
    // C 包含 Arc<i32>，Arc 实现了 Send，所以 C 是 Send
    is_send::<C>();
    
    // D 包含原始指针 *mut i32，原始指针既不是 Send 也不是 Sync
    // 如果取消注释下一行，会编译错误
    // is_send::<D>(); // 编译错误: *mut i32 未实现 Send
    
    // 尝试将 Rc 发送到另一个线程——编译错误!
    // 因为 Rc 不是 Send，不能跨线程转移所有权
    // let data = Rc::new(42);
    // thread::spawn(move || {
    //     println!("{}", data);
    // });
    // 错误: `Rc<i32>` 未实现 `Send`
}
```

**说明：** `Send` 是一个标记 trait（没有方法），表示类型的所有权可以安全地在线程间转移。几乎所有标准库类型都实现了 `Send`，但以下例外：`Rc<T>`（非原子引用计数，跨线程转移可能导致引用计数竞态）、原始指针、以及包含非 `Send` 成员的类型。编译器会自动为类型推导 `Send`——如果所有字段都是 `Send`，则该类型自动为 `Send`。
</details>

### 练习 18-17: 理解 Sync trait

> 难度：⭐⭐⭐
> Rust 独有的并发安全 trait / 类似 C++ 的线程安全访问标记

`Sync` 是另一个标记 trait，表示类型可以安全地通过引用在线程间共享（即 `&T` 是 `Send` 的）。补全代码，理解 `Sync` 和 `Send` 的区别。

```rust
use std::sync::{Mutex, Arc};
use std::rc::Rc;
use std::cell::RefCell;

// TODO: 判断以下类型是否实现了 Sync，并在注释中写出理由

struct MyType {
    data: i32,
}

fn is_sync<T: Sync>() {}

fn main() {
    // 测试常见的 Sync 和 !Sync 类型
    // 下面哪些能编译通过？
    
    // is_sync::<i32>();
    // is_sync::<Mutex<i32>>();
    // is_sync::<RefCell<i32>>();
    // is_sync::<Rc<i32>>();
    // is_sync::<Arc<i32>>();
    // is_sync::<*mut i32>();
    
    // TODO: 取消能编译的行的注释
    
    // TODO: 解释为什么 RefCell 和 Rc 不是 Sync 的
    // 提示：考虑多线程同时调用 borrow_mut 或 clone 会发生什么
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::{Mutex, Arc};
use std::rc::Rc;
use std::cell::RefCell;

struct MyType {
    data: i32,
}

fn is_sync<T: Sync>() {}

fn main() {
    // i32 是 Sync——多个线程可以同时读取 i32
    is_sync::<i32>();
    
    // Mutex<i32> 是 Sync——Mutex 确保互斥访问
    is_sync::<Mutex<i32>>();
    
    // RefCell<i32> 不是 Sync——运行时借用检查非线程安全
    // 取消注释会编译错误
    // is_sync::<RefCell<i32>>();
    
    // Rc<i32> 不是 Sync——非原子引用计数
    // 取消注释会编译错误
    // is_sync::<Rc<i32>>();
    
    // Arc<i32> 是 Sync——原子引用计数
    is_sync::<Arc<i32>>();
    
    // *mut i32 不是 Sync——原始指针
    // 取消注释会编译错误
    // is_sync::<*mut i32>();
    
    // 解释:
    // - RefCell 不是 Sync，因为它的运行时借用检查不是线程安全的，
    //   多线程同时调用 borrow_mut 会导致数据竞争
    // - Rc 不是 Sync，因为引用计数操作不是原子的，
    //   多线程同时 clone/drop 会导致引用计数错误
    // - Arc 是 Sync，因为使用原子操作管理引用计数
}
```

**说明：** `Sync` 表示 `&T` 可以安全地在线程间共享（即 `&T` 实现了 `Send`）。如果一个类型是 `Sync`，多个线程可以同时通过不可变引用访问它而不导致数据竞争。`Send` 和 `Sync` 的关系：`T: Sync` 等价于 `&T: Send`。常见的非 `Sync` 类型包括：`RefCell<T>`（运行时借用检查非线程安全）、`Cell<T>`、`Rc<T>`、原始指针。`Mutex<T>` 是 `Sync` 的，即使 `T` 不是 `Sync`。
</details>

### 练习 18-18: 实现 Send 和 Sync——自定义类型

> 难度：⭐⭐⭐
> Rust 独有的 trait 实现 / 类似 C++ 的显式线程安全声明

补全代码，为自定义类型标注 `Send` 和 `Sync`。注意：绝大多数情况下，如果类型的所有字段都实现了 `Send`/`Sync`，编译器会自动推导。只有当类型包含原始指针或非 Send/Sync 的字段时，才需要手动 `unsafe impl`。

```rust
use std::cell::RefCell;
use std::sync::Mutex;
use std::rc::Rc;

// 类型 A: 所有字段都是 Send 和 Sync
// 编译器会自动推导
struct A {
    x: i32,
    y: Mutex<f64>,
}

// 类型 B: 包含 Rc，不是 Send 也不是 Sync
// 编译器会自动推导出 !Send + !Sync
struct B {
    name: String,
    rc: Rc<i32>,
}

// 类型 C: 持有原始指针，默认不是 Send 也不是 Sync
// TODO: 使用 unsafe impl 手动标记为 Send 和 Sync
// 注意：这需要你确保该类型在多线程使用中确实是安全的
struct C {
    ptr: *mut i32,
    len: usize,
}

// TODO: 为 C 实现 Send 和 Sync
// 提示：需要 unsafe impl

fn main() {
    // 验证
    fn check_send_sync<T: Send + Sync>() {}
    
    // check_send_sync::<A>(); // A 应该可以
    // check_send_sync::<B>(); // B 应该不行
    // check_send_sync::<C>(); // C 如果手动实现了就可以
    
    // TODO: 取消能编译的行的注释
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::cell::RefCell;
use std::sync::Mutex;
use std::rc::Rc;

struct A {
    x: i32,
    y: Mutex<f64>,
}

struct B {
    name: String,
    rc: Rc<i32>,
}

struct C {
    ptr: *mut i32,
    len: usize,
}

// 手动标注 Send 和 Sync——这是 unsafe 的，需要开发者确保线程安全
// 这里 C 实际上只用于读访问（len 控制范围），且不会在多个线程中同时写 ptr，
// 我们在逻辑上保证了它的安全性
unsafe impl Send for C {}
unsafe impl Sync for C {}

fn main() {
    fn check_send_sync<T: Send + Sync>() {}
    
    check_send_sync::<A>();
    // check_send_sync::<B>(); // 编译错误：Rc<i32> 不是 Send
    
    // C 已手动标记为 Send + Sync
    check_send_sync::<C>();
}
```

**说明：** 只有当你能确保类型在多线程使用中绝对安全时，才应使用 `unsafe impl Send` 和 `unsafe impl Sync`。编译器信任开发者的判断，错误的标记会导致未定义行为。大多数情况下不需要手动实现——如果所有字段都是 `Send`/`Sync`，编译器会自动推导。需要手动实现的典型场景包括：包含原始指针的类型（如自定义分配器、FFI 类型）等。
</details>

### 练习 18-19: Rc 与 Arc 的区别理解

> 难度：⭐⭐⭐
> 对比 Rust 的 Rc（单线程）和 Arc（多线程）/ 类似 C++11 的 shared_ptr（线程安全版本）

补全代码，通过编译行为理解 `Rc` 和 `Arc` 在线程安全上的本质区别。

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // 场景 1: 在同一线程中，Rc 可以正常工作
    let rc_data = Rc::new(42);
    let rc_clone = rc_data.clone();
    println!("Rc 在同一线程: {} {}", rc_data, rc_clone);
    
    // TODO: 场景 2: 尝试将 Rc 发送到另一线程（取消注释观察编译错误）
    // let rc_data = Rc::new(42);
    // let handle = thread::spawn(move || {
    //     println!("Rc 在另一线程: {}", rc_data);
    // });
    // handle.join().unwrap();
    
    // TODO: 场景 3: 使用 Arc 代替 Rc，使其能跨线程
    // 补全代码：创建 Arc<i32> 并发送到另一个线程
    let arc_data = Arc::new(42);
    // TODO: clone Arc 并在线程中打印
    
    // TODO: 场景 4: 尝试在线程中修改 Arc 内部的值（需要使用 Mutex）
    // 创建 Arc<Mutex<i32>>，在两个线程中分别递增
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 场景 1: Rc 在同一线程中
    let rc_data = Rc::new(42);
    let rc_clone = rc_data.clone();
    println!("Rc 在同一线程: {} {}", rc_data, rc_clone);
    
    // 场景 2: Rc 跨线程——编译错误! Rc 不是 Send
    // 取消注释将看到: `Rc<i32>` 未实现 `Send`
    // let rc_data = Rc::new(42);
    // let handle = thread::spawn(move || {
    //     println!("Rc 在另一线程: {}", rc_data);
    // });
    // handle.join().unwrap();
    
    // 场景 3: Arc 可以跨线程
    let arc_data = Arc::new(42);
    let arc_clone = arc_data.clone();
    let handle = thread::spawn(move || {
        println!("Arc 在另一线程: {}", arc_clone);
    });
    handle.join().unwrap();
    println!("Arc 在主线程: {}", arc_data);
    
    // 场景 4: Arc<Mutex<T>> 实现跨线程可变访问
    let shared = Arc::new(Mutex::new(0));
    let shared_clone = shared.clone();
    
    let handle = thread::spawn(move || {
        let mut val = shared_clone.lock().unwrap();
        *val += 1;
    });
    handle.join().unwrap();
    
    println!("最终值: {}", *shared.lock().unwrap());
}
```

**说明：** `Rc<T>` 使用非原子引用计数，只能在单线程中使用（不是 `Send`）。`Arc<T>` 使用原子引用计数，可以在线程间共享（实现了 `Send` 和 `Sync`）。如果要修改 `Arc` 内部的值，需要结合 `Mutex<T>` 提供互斥访问。这是 Rust 并发设计的关键思想：不同类型的"细胞"提供不同的线程安全保证，编译器在编译期强制执行。
</details>

### 练习 18-20: Send 与 Sync 综合理解挑战

> 难度：⭐⭐⭐
> Rust 独有的并发安全模型验证

设计一个自定义线程安全计数器 `ThreadSafeCounter`，内部使用原子操作，并验证其 `Send` 和 `Sync` 性质。然后设计一个包含原始指针的类型，通过 `unsafe impl Send` 和 `unsafe impl Sync` 手动标记为线程安全。

```rust
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

// TODO: 实现 ThreadSafeCounter
// 内部使用 AtomicI32 实现无锁线程安全计数器
struct ThreadSafeCounter {
    // TODO
}

impl ThreadSafeCounter {
    fn new() -> Self {
        // TODO
    }
    
    fn increment(&self) {
        // TODO: 使用 fetch_add 原子增加
    }
    
    fn get(&self) -> i32 {
        // TODO: 使用 load 读取值
    }
}

// TODO: 验证 ThreadSafeCounter 是 Send + Sync
// 写一个函数来测试它可以在线程间共享和发送

// TODO: 创建 UnsafeCounter，包含一个 *mut i32
// 使用 unsafe impl Send/Sync 标记（仅用于学习目的）
struct UnsafeCounter {
    ptr: *mut i32,
}

// TODO: unsafe impl Send for UnsafeCounter
// TODO: unsafe impl Sync for UnsafeCounter

fn main() {
    // 测试 ThreadSafeCounter
    let counter = std::sync::Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                counter.increment();
            }
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("线程安全计数器: {}", counter.get()); // 应输出 1000
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

struct ThreadSafeCounter {
    value: AtomicI32,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            value: AtomicI32::new(0),
        }
    }
    
    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }
    
    fn get(&self) -> i32 {
        self.value.load(Ordering::SeqCst)
    }
}

// ThreadSafeCounter 只包含 AtomicI32（实现了 Send + Sync），
// 所以 ThreadSafeCounter 自动为 Send + Sync

struct UnsafeCounter {
    ptr: *mut i32,
}

// 不安全：手动标记为 Send + Sync
// 只有在我们确保不会出现数据竞争时才应该这样做
unsafe impl Send for UnsafeCounter {}
unsafe impl Sync for UnsafeCounter {}

fn main() {
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                counter.increment();
            }
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("线程安全计数器: {}", counter.get());
}
```

**说明：** `AtomicI32` 是 Rust 标准库提供的原子类型，它既是 `Send` 又是 `Sync`，因此包含它的 `ThreadSafeCounter` 自动获得这些 trait。原子操作使用 CPU 级别的指令保证操作的原子性，无需使用操作系统锁，性能更好。`Ordering` 参数控制内存排序保证（这里使用最严格的 `SeqCst`，即顺序一致性）。`UnsafeCounter` 展示了如何手动标注 `Send`/`Sync`——这仅用于学习，生产代码中应优先使用标准的线程安全类型。
</details>

### 练习 18-21: 任务池基础——任务定义与存储

> 难度：⭐⭐⭐
> 类似 Java 的 `ThreadPoolExecutor` / 类似 C++ 的线程池

实现一个简单的任务池，包含任务定义和并发执行机制。先从任务池的基础数据结构开始。

```rust
use std::sync::{Arc, Mutex};

// TODO: 定义 Task trait，包含一个 run 方法
trait Task {
    // TODO: 定义 run 方法，返回 String
}

// TODO: 实现一个 PrintTask，打印一条消息
struct PrintTask {
    message: String,
}

// TODO: 为 PrintTask 实现 Task trait
// run 方法返回 format!("打印: {}", self.message)

// TODO: 实现一个 ComputeTask，执行计算
struct ComputeTask {
    a: i32,
    b: i32,
    op: char, // '+', '-', '*'
}

// TODO: 为 ComputeTask 实现 Task trait
// 根据 op 执行计算，返回格式如 "计算: 3 + 4 = 7"

fn main() {
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(PrintTask { message: "你好".to_string() }),
        Box::new(ComputeTask { a: 3, b: 4, op: '+' }),
        Box::new(PrintTask { message: "世界".to_string() }),
        Box::new(ComputeTask { a: 10, b: 5, op: '*' }),
    ];
    
    for task in tasks {
        println!("{}", task.run());
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::{Arc, Mutex};

trait Task {
    fn run(&self) -> String;
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

struct ComputeTask {
    a: i32,
    b: i32,
    op: char,
}

impl Task for ComputeTask {
    fn run(&self) -> String {
        match self.op {
            '+' => format!("计算: {} + {} = {}", self.a, self.b, self.a + self.b),
            '-' => format!("计算: {} - {} = {}", self.a, self.b, self.a - self.b),
            '*' => format!("计算: {} * {} = {}", self.a, self.b, self.a * self.b),
            _ => format!("计算: 未知操作符 {}", self.op),
        }
    }
}

fn main() {
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(PrintTask { message: "你好".to_string() }),
        Box::new(ComputeTask { a: 3, b: 4, op: '+' }),
        Box::new(PrintTask { message: "世界".to_string() }),
        Box::new(ComputeTask { a: 10, b: 5, op: '*' }),
    ];
    
    for task in tasks {
        println!("{}", task.run());
    }
}
```

**说明：** 任务池的第一步是定义统一的任务接口。使用 `trait` 定义 `run` 方法，不同的任务类型（打印、计算）各自实现 `run`。通过 `Box<dyn Task>` 实现多态，将不同类型的任务放入同一个容器中。这是后续构建并发任务池的基础。
</details>

### 练习 18-22: 任务通道——发送与接收任务

> 难度：⭐⭐⭐
> 类似 Java 的 `BlockingQueue<Runnable>` / 类似 C++ 的任务队列

扩展练习 18-21，使用 `mpsc` 通道在不同线程间发送和接收任务。

```rust
use std::thread;
use std::sync::mpsc;

// 复用练习 18-21 的 Task trait 和实现
trait Task {
    fn run(&self) -> String;
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

// TODO: 创建一个发送任务的线程，发送 5 个 PrintTask
// 主线程从通道接收任务并执行

fn main() {
    // TODO: 创建 mpsc 通道，发送端发送 Box<dyn Task>
    let (tx, rx) = // TODO
    
    let sender = thread::spawn(move || {
        // TODO: 发送 5 个 PrintTask 实例
        // 消息: "任务1" 到 "任务5"
    });
    
    // TODO: 主线程接收并执行所有任务
    // 提示：使用 for 循环迭代 rx
    
    sender.join().unwrap();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::mpsc;

trait Task {
    fn run(&self) -> String;
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<Box<dyn Task + Send>>();
    
    let sender = thread::spawn(move || {
        for i in 1..=5 {
            let task = PrintTask {
                message: format!("任务{}", i),
            };
            tx.send(Box::new(task)).unwrap();
        }
    });
    
    for task in rx {
        println!("{}", task.run());
    }
    
    sender.join().unwrap();
}
```

**说明：** 使用 `mpsc::channel::<Box<dyn Task + Send>>()` 创建一条可以发送 trait 对象的通道。注意 `Task` 后面加了 `Send` 约束，因为 `Box<dyn Task>` 默认不是 `Send` 的——需要显式要求 trait 对象满足 `Send`。发送端线程创建任务并发送，主线程通过迭代 `rx` 依次接收并执行。
</details>

### 练习 18-23: 工作线程池——Worker 实现

> 难度：⭐⭐⭐
> 类似 Java 的 `Worker` 线程 / 类似 C++ 线程池的工作线程

实现工作线程（Worker），它从共享的任务通道中获取任务并执行。

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

trait Task {
    fn run(&self) -> String;
}

// TODO: 定义 Worker 结构体
// 包含 id: usize 和 thread: Option<thread::JoinHandle<>>
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // TODO: 实现 new 方法
    // 接收 id 和 Arc<Mutex<mpsc::Receiver<Box<dyn Task + Send>>>>
    // 使用 thread::spawn 启动一个线程循环接收任务
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn Task + Send>>>>) -> Worker {
        // TODO: 在线程中循环调用 receiver.lock().unwrap().recv()
        // 收到任务则执行，通道关闭则退出循环
    }
    
    // TODO: 实现 join 方法，等待线程结束
    fn join(&mut self) {
        // 使用 self.thread.take().unwrap().join()
    }
}

// 简单的打印任务
struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: 创建 3 个 Worker
    let mut workers = vec![];
    for id in 0..3 {
        // TODO: 创建 Worker 并添加到 workers 向量
    }
    
    // 发送 6 个任务
    for i in 0..6 {
        tx.send(Box::new(PrintTask {
            message: format!("任务{}", i),
        }) as Box<dyn Task + Send>).unwrap();
    }
    drop(tx); // 关闭通道
    
    // 等待所有 worker 结束
    for worker in &mut workers {
        worker.join();
    }
    println!("所有任务完成");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

trait Task {
    fn run(&self) -> String;
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn Task + Send>>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let task = {
                    let lock = receiver.lock().unwrap();
                    lock.recv()
                };
                match task {
                    Ok(task) => {
                        let result = task.run();
                        println!("Worker {}: {}", id, result);
                    }
                    Err(_) => {
                        println!("Worker {}: 通道关闭，退出", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
    
    fn join(&mut self) {
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
    }
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    let mut workers = vec![];
    for id in 0..3 {
        workers.push(Worker::new(id, rx.clone()));
    }
    
    for i in 0..6 {
        tx.send(Box::new(PrintTask {
            message: format!("任务{}", i),
        }) as Box<dyn Task + Send>).unwrap();
    }
    drop(tx);
    
    for worker in &mut workers {
        worker.join();
    }
    println!("所有任务完成");
}
```

**说明：** Worker 是线程池的核心组件。每个 Worker 拥有一个线程，该线程循环从共享的任务通道中获取任务并执行。`Arc<Mutex<Receiver>>` 使多个 Worker 可以共享同一个接收端——`Mutex` 确保每次只有一个 Worker 获取任务，`Arc` 实现所有权共享。当通道关闭且所有任务被取完后，Worker 线程自动退出。
</details>

### 练习 18-24: 任务池——收集执行结果

> 难度：⭐⭐⭐
> 类似 Java 的 `Future` / 类似 C++ 的 `std::future`

扩展 Worker，使任务执行后可以将结果通过另一个通道返回给主线程。

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

trait Task {
    fn run(&self) -> String;
}

// TODO: 修改 Worker，接收一个 result_tx: mpsc::Sender<String>
// 任务执行后将 run() 的返回值通过 result_tx 发送

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Box<dyn Task + Send>>>>,
        result_tx: mpsc::Sender<String>,
    ) -> Worker {
        // TODO: 在线程中执行任务，将结果通过 result_tx 发送
    }
    
    fn join(&mut self) {
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
    }
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<Box<dyn Task + Send>>();
    let (result_tx, result_rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: 创建 3 个 Worker，传入 result_tx 的克隆
    
    // 发送任务
    for i in 0..6 {
        tx.send(Box::new(PrintTask {
            message: format!("任务{}", i),
        })).unwrap();
    }
    drop(tx);
    
    // TODO: 先 drop result_tx 再开始接收结果
    // 这样才能让结果通道的迭代器结束
    
    // TODO: 从 result_rx 接收并打印所有结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

trait Task {
    fn run(&self) -> String;
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Box<dyn Task + Send>>>>,
        result_tx: mpsc::Sender<String>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let task = {
                    let lock = receiver.lock().unwrap();
                    lock.recv()
                };
                match task {
                    Ok(task) => {
                        let result = task.run();
                        result_tx.send(format!("Worker{}: {}", id, result)).unwrap();
                    }
                    Err(_) => break,
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
    
    fn join(&mut self) {
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }
    }
}

struct PrintTask {
    message: String,
}

impl Task for PrintTask {
    fn run(&self) -> String {
        format!("打印: {}", self.message)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<Box<dyn Task + Send>>();
    let (result_tx, result_rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));
    
    let mut workers = vec![];
    for id in 0..3 {
        workers.push(Worker::new(id, rx.clone(), result_tx.clone()));
    }
    
    for i in 0..6 {
        tx.send(Box::new(PrintTask {
            message: format!("任务{}", i),
        })).unwrap();
    }
    drop(tx);
    
    drop(result_tx); // 关闭结果通道，使迭代器能结束
    
    for result in result_rx {
        println!("结果: {}", result);
    }
    
    for worker in &mut workers {
        worker.join();
    }
    println!("所有任务完成");
}
```

**说明：** 添加结果通道后，Worker 执行任务后会将结果发送回主线程。主线程通过迭代 `result_rx` 来收集所有结果。关键点：必须 `drop(result_tx)`（丢弃主线程持有的发送端副本）后，结果通道的迭代器才能正常结束——否则 `result_rx` 会一直等待更多的消息。
</details>

### 练习 18-25: 完整任务池实现挑战

> 难度：⭐⭐⭐
> 类似 Java 的 `ThreadPoolExecutor` 简化版 / 类似 C++ 的线程池完整实现

综合前面所有练习，实现一个完整的 `ThreadPool` 结构体，包含以下方法：

1. `new(size: usize)` — 创建指定线程数的线程池
2. `execute<T>(task: T)` — 提交任务，其中 `T: Task + Send + 'static`
3. `join_all()` — 等待所有任务完成

任务类型使用泛型 + 闭包（类似标准库 `thread::spawn` 的签名）。

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

// TODO: 定义 ThreadPool 结构体
// 包含 workers 向量和任务发送端
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Box<dyn FnOnce() + Send>>>,
}

// TODO: 定义 Worker 结构体
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // TODO: 实现 Worker::new
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send>>>>) -> Worker {
        // TODO
    }
}

impl ThreadPool {
    // TODO: 创建线程池
    fn new(size: usize) -> ThreadPool {
        // 1. 创建通道
        // 2. 创建 size 个 Worker
        // 3. 返回 ThreadPool
    }
    
    // TODO: 提交任务
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 通过 sender 发送任务
    }
}

// TODO: 为 ThreadPool 实现 Drop trait
// 丢弃 sender 并 join 所有 worker
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO
    }
}

fn main() {
    // 创建 4 个线程的线程池
    let pool = ThreadPool::new(4);
    
    // 提交 8 个任务
    for i in 0..8 {
        pool.execute(move || {
            println!("任务 {} 在线程 {:?} 上执行", i, thread::current().id());
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }
    
    // pool 在这里被 drop，会等待所有任务完成
    println!("所有任务已提交，等待完成...");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Box<dyn FnOnce() + Send>>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send>>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = {
                    let lock = receiver.lock().unwrap();
                    lock.recv()
                };
                match job {
                    Ok(job) => {
                        println!("Worker {} 开始执行任务", id);
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} 退出", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 丢弃发送端，关闭通道
        drop(self.sender.take());
        
        // 等待所有 worker 完成
        for worker in &mut self.workers {
            println!("等待 Worker {} 结束...", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);
    
    for i in 0..8 {
        pool.execute(move || {
            println!("任务 {} 在线程 {:?} 上执行", i, thread::current().id());
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }
    
    println!("所有任务已提交，等待完成...");
}
```

**说明：** 这是一个完整的线程池实现。`ThreadPool::new` 创建指定数量的 Worker 线程，每个 Worker 循环从共享的任务队列中获取任务并执行。`execute` 方法接收闭包（`FnOnce`），将其装箱后通过通道发送给 Worker。通过 `Drop` trait 的优雅处理：丢弃发送端关闭通道 → Worker 收到 `Err` 后退出循环 → 主线程 `join` 所有 Worker。这个实现使用了 `FnOnce`（而非自定义 `Task` trait），使其用起来更接近标准库的 `thread::spawn`，具有更好的通用性。
</details>
