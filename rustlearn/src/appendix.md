# 附录：常见陷阱与 Cargo 技巧

## 常见编译错误与陷阱

### 1. 所有权移动（borrow of moved value）

```rust
let s = String::from("hello");
let t = s;
println!("{}", s); // 编译错误！s 已移动到 t
```

**修复**：使用 `t` 或调用 `.clone()`。

### 2. 同时存在可变和不可变引用

```rust
let mut v = vec![1, 2, 3];
let r = &v[0];
v.push(4); // 编译错误！&v[0] 的借用仍在作用域内
println!("{}", r);
```

**修复**：在 `r` 不再被使用之后再进行 `v.push(4)`。

### 3. 生命周期省略引起歧义

```rust
fn foo(x: &str, y: &str) -> &str { // 编译错误：需要显式生命周期标注
    if x.len() > y.len() { x } else { y }
}
```

**修复**：标注 `<'a>` 并说明返回值与输入的关系。

### 4. 闭包中的所有权问题

```rust
let s = String::from("hello");
let f = || println!("{}", s);
std::thread::spawn(f); // 编译错误：闭包未实现 Send
```

**修复**：使用 `move` 关键字：`let f = move || ...`

### 5. 在 for 循环中使用 `&` vs 值

```rust
let v = vec![1, 2, 3];
for x in v { ... }  // v 的所有权被移动
for x in &v { ... } // 不可变借用
for x in &mut v { ... } // 可变借用
```

## Cargo 实用技巧

### 常用命令

```bash
cargo new my_project                     # 创建新项目
cargo build                              # 编译
cargo run                                # 编译并运行
cargo test                               # 运行测试
cargo check                              # 快速检查编译（不生成二进制）
cargo add serde                          # 添加依赖（Cargo 编辑）
cargo doc --open                         # 生成并打开文档
```

### 工作空间（Workspace）

```toml
# Cargo.toml
[workspace]
members = ["project-a", "project-b"]
```

### 常用 Crate

| crate | 用途 |
|-------|------|
| `anyhow` | 简化错误处理 |
| `thiserror` | 自定义错误类型的 derive 宏 |
| `serde / serde_json` | 序列化/反序列化 |
| `tokio` | 异步运行时 |
| `rayon` | 并行迭代器 |
| `clap` | 命令行参数解析 |
| `chrono` | 日期时间处理 |
| `regex` | 正则表达式 |
| `rand` | 随机数生成 |

## 推荐学习资源

- [The Rust Book](https://doc.rust-lang.org/book/) — 官方入门书籍
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — 实例学 Rust
- [Rustlings](https://github.com/rust-lang/rustlings) — 小练习集
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — API 设计规范
- [Cargo Book](https://doc.rust-lang.org/cargo/) — Cargo 完整手册
