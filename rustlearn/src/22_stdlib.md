# 22 标准库实战

Rust 标准库（`std`）提供了丰富且经过实战检验的基础设施，涵盖文件系统、I/O 流、进程管理、环境变量、时间操作等核心能力。与依赖第三方 crate 不同，标准库开箱即用、稳定可靠，是每个 Rust 开发者必须熟练掌握的工具集。本章 25 道练习将带你逐一攻克 `std::fs`、`std::io`、`std::env`、`std::process`、`std::time` 以及 `std::path` 等关键模块，从简单的文件遍历到完整的命令行工具，逐步提升实战能力。

> 所有练习均仅依赖 Rust 标准库，无需添加任何外部 crate。

---

### 练习 22-01: 列出当前目录下所有文件

> 难度：⭐⭐
> 类似 Java 的 `File.listFiles()` / 类似 C 的 `readdir`

使用 `std::fs::read_dir` 遍历当前目录，打印所有文件/目录的名称。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // TODO: 使用 read_dir 读取当前目录 (".")，遍历条目，打印每个条目的文件名（提示：使用 entry?.file_name()）
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}
```

**说明：** `read_dir` 返回迭代器，每个 `DirEntry` 的 `file_name()` 返回 `OsString`，通过 `to_string_lossy()` 转为可打印字符串。两层 `?`：第一层处理迭代时的 I/O 错误，第二层处理单个条目读取错误。
</details>

---

### 练习 22-02: 判断路径是文件还是目录

> 难度：⭐⭐
> 类似 Java 的 `Files.isDirectory()` / 类似 Python 的 `os.path.isdir()`

接收一个路径参数，判断它是文件还是目录，并打印相应信息。

```rust
use std::path::Path;

fn main() {
    let path = Path::new("."); // TODO: 也可以从命令行参数读取
    // TODO: 判断 path 是文件还是目录，打印 "xxx 是一个目录" 或 "xxx 是一个文件"
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::path::Path;

fn main() {
    let path = Path::new("Cargo.toml");
    if path.is_dir() {
        println!("{} 是一个目录", path.display());
    } else if path.is_file() {
        println!("{} 是一个文件", path.display());
    } else {
        println!("{} 不存在或是其他类型", path.display());
    }
}
```

**说明：** `Path::new` 创建路径对象，`display()` 提供跨平台的路径显示。`is_dir()`、`is_file()` 内部调用操作系统元数据查询，是 `metadata()` 的便捷封装。
</details>

---

### 练习 22-03: 递归遍历目录

> 难度：⭐⭐
> 类似 Python 的 `os.walk()`

递归遍历指定目录，打印所有文件和子目录的完整路径。

```rust
use std::fs;
use std::path::Path;

fn visit_dir(dir: &Path) -> std::io::Result<()> {
    // TODO: 遍历 dir，如果是目录则递归，如果是文件则打印路径
    Ok(())
}

fn main() -> std::io::Result<()> {
    visit_dir(Path::new("."))
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;
use std::path::Path;

fn visit_dir(dir: &Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path)?;
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    visit_dir(Path::new("."))
}
```

**说明：** 递归遍历是文件系统操作的经典模式。`entry.path()` 返回完整路径，注意递归时可能遇到符号链接循环——生产代码建议用 `walkdir` crate 或限制递归深度。
</details>

---

### 练习 22-04: 创建并写入文件

> 难度：⭐⭐
> 类似 Java 的 `Files.writeString()`

创建一个新文件并写入多行文本。

```rust
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // TODO: 创建文件 "hello.txt"，写入 "Hello\nRust\nStdlib\n"
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = fs::File::create("hello.txt")?;
    file.write_all(b"Hello\nRust\nStdlib\n")?;
    println!("文件已创建");
    Ok(())
}
```

**说明：** `File::create` 以写入模式打开文件（若存在则截断）。`write_all` 写入字节切片，确保全部写入或返回错误。`b"..."` 是字节字符串字面量。
</details>

---

### 练习 22-05: 读取文件全部内容

> 难度：⭐⭐
> 类似 Java 的 `Files.readString()`

读取指定文件的全部内容并打印。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // TODO: 读取 "hello.txt" 的全部内容并打印
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("hello.txt")?;
    println!("{}", content);
    Ok(())
}
```

**说明：** `read_to_string` 是最便捷的文件读取方式，内部自动分配 `String` 并处理编码。适合中小文件，超大文件请使用 `BufReader` 逐行读取。
</details>

---

### 练习 22-06: 使用 BufReader 逐行读取

> 难度：⭐⭐
> 类似 Java 的 `BufferedReader.readLine()`

使用 `BufReader` 逐行读取文件并加上行号打印。

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // TODO: 打开 "Cargo.toml"，用 BufReader 逐行读取，打印行号+内容
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("Cargo.toml")?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{:>4}: {}", i + 1, line);
    }
    Ok(())
}
```

**说明：** `BufReader` 为底层文件提供缓冲，减少系统调用。`lines()` 返回 `Result<String>` 迭代器，每行不包含换行符。`enumerate()` 从 0 开始计数，因此行号显示为 `i + 1`。
</details>

---

### 练习 22-07: 使用 BufWriter 缓冲写入

> 难度：⭐⭐
> 类似 Java 的 `BufferedWriter.write()`

使用 `BufWriter` 向文件写入大量数据，体验缓冲带来的性能提升。

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    // TODO: 创建 "output.txt"，使用 BufWriter 写入 10000 行 "Hello\n"
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);
    for _ in 0..10_000 {
        writer.write_all(b"Hello\n")?;
    }
    println!("写入完成");
    Ok(())
}
```

**说明：** `BufWriter` 将多次小写入合并为一次大写入，显著减少系统调用次数。写入完成后 `BufWriter` 被 drop 时会自动 flush，但显式调用 `flush()` 是更好的实践。
</details>

---

### 练习 22-08: 标准输入读取

> 难度：⭐⭐
> 类似 Java 的 `System.in` / 类似 C 的 `scanf`

从标准输入读取一行用户输入并回显。

```rust
use std::io;

fn main() -> io::Result<()> {
    // TODO: 提示用户输入，读取一行，打印 "你输入的是: xxx"
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    println!("请输入一段文字：");
    io::stdin().read_line(&mut input)?;
    println!("你输入的是: {}", input.trim());
    Ok(())
}
```

**说明：** `io::stdin()` 返回标准输入句柄，`read_line` 读取到第一个换行符（包含 `\n`），因此通常需要 `trim()` 去掉末尾空白字符。标准输入是全局共享资源，多次调用间共享同一个缓冲区。
</details>

---

### 练习 22-09: 读取指定字节数

> 难度：⭐⭐
> 类似 C 的 `fread`

从文件中精确读取前 N 字节并打印为十六进制。

```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // TODO: 打开 "Cargo.toml"，读取前 16 字节，以十六进制格式打印
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("Cargo.toml")?;
    let mut buffer = [0u8; 16];
    let n = file.read(&mut buffer)?;
    println!("读取了 {} 字节：{:02X?}", n, &buffer[..n]);
    Ok(())
}
```

**说明：** `read(&mut buf)` 从当前偏移量读取最多 `buf.len()` 字节，返回实际读取的字节数。返回值可能小于缓冲区大小（非阻塞或文件末尾）。`{:02X?}` 格式说明符以两位大写十六进制显示。
</details>

---

### 练习 22-10: 文件复制

> 难度：⭐⭐
> 类似 Java 的 `Files.copy()`

不使用 `std::fs::copy`，手动实现文件复制：逐块读取源文件并写入目标文件。

```rust
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // TODO: 手动复制 "src.txt" 到 "dst.txt"，每次读取 1024 字节
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs::File;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut src = File::open("src.txt")?;
    let mut dst = File::create("dst.txt")?;
    let mut buffer = [0u8; 1024];
    loop {
        let n = src.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        dst.write_all(&buffer[..n])?;
    }
    println!("复制完成");
    Ok(())
}
```

**说明：** 手动复制展示了 `Read` 和 `Write` 的基本用法。循环调用 `read` 直到返回 `0`（EOF），`write_all` 确保完整写入。生产环境直接用 `std::fs::copy` 更高效（可能利用操作系统特性）。
</details>

---

### 练习 22-11: 读取命令行参数

> 难度：⭐⭐
> 类似 C 的 `argv` / 类似 Java 的 `main(String[] args)`

接收命令行参数并逐个打印。

```rust
// TODO: 收集命令行参数并打印 "第 N 个参数: xxx" 的格式
fn main() {
    // ...
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("第 {} 个参数: {}", i, arg);
    }
}
```

**说明：** `std::env::args()` 返回迭代器，第一个参数（索引 0）始终是程序路径。`collect()` 将其收集为 `Vec<String>`。如需处理无效 Unicode，使用 `args_os()` 获取 `OsString`。
</details>

---

### 练习 22-12: 读取环境变量

> 难度：⭐⭐
> 类似 Java 的 `System.getenv()`

读取并打印指定环境变量的值，若不存在则给出提示。

```rust
use std::env;

fn main() {
    // TODO: 读取 "PATH" 环境变量，打印其值；若不存在则提示
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::env;

fn main() {
    match env::var("PATH") {
        Ok(val) => println!("PATH = {}", val),
        Err(e) => println!("无法读取 PATH 环境变量: {}", e),
    }
}
```

**说明：** `env::var` 返回 `Result<String, VarError>`。`VarError::NotPresent` 表示变量不存在，`VarError::NotUnicode` 表示值包含无效 UTF-8。对于非 UTF-8 场景，使用 `env::var_os` 获取 `OsString`。
</details>

---

### 练习 22-13: 设置环境变量

> 难度：⭐⭐
> 类似 Java 的 `System.setProperty()`

在当前进程中临时设置环境变量，然后读取验证。

```rust
use std::env;

fn main() {
    // TODO: 设置环境变量 MY_APP_MODE = "debug"，然后读取并打印
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::env;

fn main() {
    env::set_var("MY_APP_MODE", "debug");
    let mode = env::var("MY_APP_MODE").unwrap_or_else(|_| "unknown".to_string());
    println!("MY_APP_MODE = {}", mode);
}
```

**说明：** `set_var` 修改**当前进程**的环境变量，不影响父进程或系统环境。**安全警告：** 环境变量是全局可变的，多线程中调用 `set_var` 可能导致数据竞争（unsound）。仅在 `main` 函数启动时使用，不要在库代码中调用。
</details>

---

### 练习 22-14: 运行外部命令并等待

> 难度：⭐⭐
> 类似 Java 的 `ProcessBuilder.start()`

使用 `std::process::Command` 运行 `echo` 命令并等待其完成。

```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    // TODO: 运行 "echo Hello from Rust"，等待完成并打印状态码
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let status = Command::new("cmd")
        .args(["/C", "echo", "Hello from Rust"])
        .status()?;
    println!("子进程退出码: {:?}", status.code());
    Ok(())
}
```

**说明：** Windows 上需通过 `cmd /C` 运行内置命令。`status()` 等待子进程退出并返回 `ExitStatus`。`code()` 返回 `Option<i32>`，Unix 信号终止时返回 `None`。
</details>

---

### 练习 22-15: 捕获命令输出

> 难度：⭐⭐
> 类似 Java 的 `Process.getInputStream()`

运行命令并捕获其标准输出。

```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    // TODO: 运行 "rustc --version"，捕获输出并打印
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let output = Command::new("rustc")
        .arg("--version")
        .output()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        eprintln!("命令执行失败");
    }
    Ok(())
}
```

**说明：** `output()` 捕获子进程的 stdout 和 stderr，返回 `Output` 结构体。`String::from_utf8_lossy` 将字节转换为字符串，遇到无效 UTF-8 时用 `�` 替换。`status.success()` 判断退出码是否为 0。
</details>

---

### 练习 22-16: 测量代码执行时间

> 难度：⭐⭐⭐
> 类似 Java 的 `System.nanoTime()`

使用 `std::time::Instant` 测量一段代码的执行时间。

```rust
use std::time::Instant;

fn main() {
    // TODO: 记录开始时间，执行一个 1 亿次循环的加法运算，打印耗时
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..100_000_000 {
        sum += i;
    }
    let duration = start.elapsed();
    println!("结果: {}, 耗时: {:?}", sum, duration);
}
```

**说明：** `Instant` 表示单调递增的时间点，适用于测量间隔。`elapsed()` 返回 `Duration`。`Instant` 保证不受系统时间调整影响，是基准测试的首选。`{:?}` 格式化 `Duration` 显示为类似 `2.345s` 的格式。
</details>

---

### 练习 22-17: Duration 时间运算

> 难度：⭐⭐⭐
> 类似 Java 的 `Duration`

练习 `std::time::Duration` 的创建、运算和格式化。

```rust
use std::time::Duration;

fn main() {
    // TODO: 创建 2 分 30 秒的 Duration，加上 45 秒，打印结果总秒数
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::time::Duration;

fn main() {
    let d1 = Duration::from_secs(2 * 60 + 30); // 2 分 30 秒
    let d2 = Duration::from_secs(45);
    let total = d1 + d2;
    println!("总秒数: {}", total.as_secs_f64());
    println!("可读格式: {:?}", total);
}
```

**说明：** `Duration` 支持 `+`、`-`、`*`、`/` 等运算。`as_secs_f64()` 返回浮点秒数（如 `195.0`）。`{:?}` 格式化为 `195s`。注意 `Duration` 不能为负，减法在结果小于零时会 panic。
</details>

---

### 练习 22-18: 超时控制（循环检查）

> 难度：⭐⭐⭐
> 类似 Java 的 `Future.get(timeout)`

不使用线程，通过在循环中检查 `Instant` 差值来实现超时等待。

```rust
use std::time::{Duration, Instant};

fn main() {
    // TODO: 循环等待直到超时 2 秒，在循环中打印 "等待中..."
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::time::{Duration, Instant};

fn main() {
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut count = 0;
    while Instant::now() < deadline {
        println!("等待中... ({})", count);
        count += 1;
        // 忙等待，实际项目中应使用 thread::sleep
    }
    println!("超时！共循环 {} 次", count);
}
```

**说明：** 通过 `Instant::now() + Duration` 计算截止时间。忙等待浪费 CPU，生产代码应使用 `thread::sleep(Duration)` 让出时间片。此练习仅演示 `Instant` 比较用法。
</details>

---

### 练习 22-19: 系统时间戳

> 难度：⭐⭐⭐
> 类似 Java 的 `System.currentTimeMillis()`

使用 `std::time::SystemTime` 获取自 Unix 纪元以来的秒数。

```rust
use std::time::SystemTime;

fn main() {
    // TODO: 获取当前系统时间，计算并打印 Unix 时间戳（秒）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)
        .expect("系统时间早于 Unix 纪元");
    println!("Unix 时间戳: {} 秒", since_epoch.as_secs());
}
```

**说明：** `SystemTime` 表示系统时钟，受系统时间调整影响（用户修改时间、NTP 同步等）。`UNIX_EPOCH` 是 1970-01-01 00:00:00 UTC。`duration_since` 返回 `Result`，因为系统时间可能早于纪元（理论上可能，实践中罕见）。
</details>

---

### 练习 22-20: 格式化时间输出

> 难度：⭐⭐⭐
> 类似 Java 的 `SimpleDateFormat`

将 `SystemTime` 格式化为可读的日期时间字符串。

```rust
use std::time::SystemTime;

fn main() {
    // TODO: 获取当前时间，格式化为 "YYYY-MM-DD HH:MM:SS" 格式打印
    // 提示：标准库没有直接格式化 API，需手动计算年月日
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("时间错误");
    let total_secs = now.as_secs();

    // 计算年月日时分秒（忽略闰秒，适用于大多数场景）
    let days = total_secs / 86400;
    let time_secs = total_secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    // 从 Unix 纪元（1970-01-01）计算当前日期
    let mut y = 1970i64;
    let mut remaining = days as i64;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let month_days = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0;
    for (i, &md) in month_days.iter().enumerate() {
        if remaining < md as i64 {
            m = i + 1;
            break;
        }
        remaining -= md as i64;
    }
    let d = remaining + 1;

    println!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        y, m, d, hours, minutes, seconds
    );
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
```

**说明：** Rust 标准库没有日期格式化 API（这是 `chrono` crate 的领域）。此练习手动实现了公历日期计算，展示了纯标准库的能力。**ponytail:** 生产环境请用 `chrono` crate，这里仅为教学目的展示算法。
</details>

---

### 练习 22-21: 简易 wc 命令行工具

> 难度：⭐⭐⭐
> 类似 Unix 的 `wc` 命令

实现一个简易版 `wc`，统计文件的行数、单词数和字符数。

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // TODO: 从命令行参数获取文件名，统计行数、单词数、字符数并打印
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <文件名>", args[0]);
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    for line in reader.lines() {
        let line = line?;
        lines += 1;
        chars += line.chars().count() + 1; // +1 算换行符
        words += line.split_whitespace().count();
    }

    println!("{:>8} {:>8} {:>8} {}", lines, words, chars, &args[1]);
    Ok(())
}
```

**说明：** 综合运用了 `env::args`、`File::open`、`BufReader`、`lines()` 以及字符串分割。`chars().count()` 统计 Unicode 字符数（不是字节数）。错误处理：无参数时打印用法并退出。
</details>

---

### 练习 22-22: 文件搜索工具（简易 grep）

> 难度：⭐⭐⭐
> 类似 Unix 的 `grep`

实现一个简易文件搜索工具，在指定文件中搜索包含关键字的行并打印。

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // TODO: 接收关键字和文件名作为命令行参数，打印包含关键字的行及行号
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: {} <关键字> <文件名>", args[0]);
        std::process::exit(1);
    }
    let keyword = &args[1];
    let file = File::open(&args[2])?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(keyword) {
            println!("{}:{}", i + 1, line);
        }
    }
    Ok(())
}
```

**说明：** 结合 `env::args`、`BufReader`、`lines()` 和字符串 `contains` 方法。`enumerate()` 提供行号。此版本区分大小写，扩展方向：支持大小写不敏感（`to_lowercase`）、正则匹配（`regex` crate）。
</details>

---

### 练习 22-23: 批量文件重命名工具

> 难度：⭐⭐⭐
> 类似 bash 的 `rename`

批量将当前目录下所有 `.txt` 文件重命名为 `.bak` 后缀。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // TODO: 遍历当前目录，找到所有 .txt 文件，重命名为 .bak
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let new_path = path.with_extension("bak");
            fs::rename(&path, &new_path)?;
            println!("重命名: {} -> {}", path.display(), new_path.display());
        }
    }
    Ok(())
}
```

**说明：** `path.extension()` 返回 `Option<&OsStr>`，用 `and_then` 配合 `to_str` 转为 `Option<&str>` 进行比较。`with_extension` 智能替换扩展名。`fs::rename` 在同一文件系统内移动/重命名。
</details>

---

### 练习 22-24: 目录大小计算器

> 难度：⭐⭐⭐
> 类似 `du -sh` 命令

递归计算指定目录的总大小（以字节为单位），并转换为可读格式（KB/MB/GB）。

```rust
use std::fs;
use std::path::Path;

fn dir_size(path: &Path) -> std::io::Result<u64> {
    // TODO: 递归遍历目录，累加所有文件大小
    Ok(0)
}

fn format_size(size: u64) -> String {
    // TODO: 将字节数转换为 KB/MB/GB 可读格式
    String::new()
}

fn main() -> std::io::Result<()> {
    // TODO: 接收目录路径作为命令行参数（默认为当前目录），计算并打印总大小
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;
use std::path::Path;

fn dir_size(path: &Path) -> std::io::Result<u64> {
    let mut total = 0u64;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                total += dir_size(&entry_path)?;
            } else {
                total += entry.metadata()?.len();
            }
        }
    }
    Ok(total)
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.2} {}", size, UNITS[unit_idx])
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let target = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };
    let total = dir_size(target)?;
    println!("总大小: {} ({})", format_size(total), total);
    Ok(())
}
```

**说明：** 综合运用了递归遍历、`metadata().len()` 获取文件大小、以及单位换算。`format_size` 使用循环逐步除以 1024 确定合适的单位。注意符号链接可能导致的重复计数或无限递归——生产环境可用 `walkdir` crate 处理。
</details>

---

### 练习 22-25: 简易文件同步工具

> 难度：⭐⭐⭐
> 类似 `rsync` 的精简版

实现一个简易同步工具：比较源目录和目标目录，将源目录中新增或修改的文件复制到目标目录。

```rust
use std::fs;
use std::path::Path;

fn sync_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    // TODO: 遍历源目录，对于每个文件检查目标目录中是否存在且一致
    // 若不存在或修改时间不同，则复制
    Ok(())
}

fn main() -> std::io::Result<()> {
    // TODO: 接收源目录和目标目录两个命令行参数，执行同步
    Ok(())
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::fs;
use std::path::Path;

fn sync_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
            sync_dir(&src_path, &dst_path)?;
        } else {
            // 判断是否需要复制：目标不存在或修改时间不同
            let need_copy = if dst_path.exists() {
                let src_meta = src_path.metadata()?;
                let dst_meta = dst_path.metadata()?;
                src_meta.modified()? != dst_meta.modified()?
                    || src_meta.len() != dst_meta.len()
            } else {
                true
            };

            if need_copy {
                fs::copy(&src_path, &dst_path)?;
                println!("复制: {} -> {}", src_path.display(), dst_path.display());
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: {} <源目录> <目标目录>", args[0]);
        std::process::exit(1);
    }
    let src = Path::new(&args[1]);
    let dst = Path::new(&args[2]);
    if !src.is_dir() {
        eprintln!("错误: 源目录不存在或不是目录");
        std::process::exit(1);
    }
    sync_dir(src, dst)?;
    println!("同步完成！");
    Ok(())
}
```

**说明：** 综合了递归遍历、`fs::create_dir_all`、元数据比较（`modified()` 和 `len()`）、`fs::copy` 复制文件。通过比较修改时间和文件大小决定是否需要复制。此版本是单向同步（源 → 目标），不处理删除操作。**ponytail:** 生产环境请用 `rsync` 或 `diff` crate。
</details>

---

以上就是标准库实战的 25 道练习题。通过这些练习，你应当能够熟练运用 Rust 标准库处理文件系统、I/O 流、进程管理、环境变量和时间操作等日常开发任务。所有代码均不依赖任何第三方 crate，仅使用 `std` 即可编译运行。建议在本地创建一个新的二进制项目（`cargo new --bin stdlib_practice`）逐个练习验证。
