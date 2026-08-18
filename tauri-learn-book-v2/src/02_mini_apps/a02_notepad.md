# 练习 A02: 记事本

## 为什么要学这个

待办清单（A01）有个问题：**数据在内存里，应用一关就没了。** 真正的应用要"关掉字还在"——把数据**持久化到文件**。这一章要回答三个问题：

1. **文件该存哪？** —— 为什么不能随便存？"应用数据目录"是什么？
2. **怎么写代码读写文件？** —— fs 插件怎么用？`OpenOptions` 那些 `read / write / create / truncate` 是什么意思？
3. **出错了怎么办？** —— `Result` 错误处理怎么在前后端传"失败原因"？

另外，从这一课开始，前端从 Vanilla TS 换成 **React**——为什么？因为应用开始有"输入框 + 按钮 + 状态显示"这种交互，框架能省掉大量手写 DOM 代码。学完你会：用 React 写前端 + 用 Rust 读写文件，做出第一个"真正能用"的应用。

---

## 从问题出发

练习 A02 要做的事：**一个记事本——输入文字，点保存，关掉软件再打开，字还在。**

**核心矛盾：** A01 的数据在内存里（`Mutex<Vec<...>>`），内存断电即失。要把数据"留下来"，唯一可靠的办法是**写进磁盘文件**。但"写文件"背后有三个坑：

1. **存哪？** 写"当前目录"？打包安装后程序目录是只读的。每个系统都有规范的应用数据目录——Windows 的 `%APPDATA%`、macOS 的 `~/Library/Application Support`、Linux 的 `~/.local/share`。
2. **怎么读怎么写？** 打开文件、读内容、写内容、关文件——而且要有"创建 / 清空 / 追加"的控制。
3. **失败怎么办？** 磁盘满、文件被占用、目录不存在……任何一个错误都不能让程序崩溃，要变成一句能看懂的中文提示。

所以后端三件事：**`app.path()` 拿目录 → `app.fs()` 读写文件 → `Result` 兜住所有失败。**

```text
前端 (React)                        Rust 进程
┌──────────────────────┐           ┌─────────────────────────────┐
│ textarea (useState)   │  invoke  │  app.path().app_data_dir()   │ ← 存哪
│ 保存按钮               │ ───────► │  app.fs().open(OpenOptions)  │ ← 读写
│ 路径 + 状态显示         │ ◄────── │  Result<String, String>      │ ← 出错
└──────────────────────┘           └─────────────────────────────┘
```

---

<a id="sec-a02-path"></a>
## 1. 路径 API — 文件该存哪

### 为什么不能用相对路径

开发时 `./note.txt` 好像挺方便。但打包安装后，可执行文件所在的目录是**只读**的（Windows 在 `Program Files` 下），而且每个用户应该有自己的笔记。所以 Tauri 提供了一组**规范目录**：

| 方法 | 返回的目录 | 用途 |
|------|-----------|------|
| `app_data_dir()` | 应用专属数据目录（`%APPDATA%/com.taurilearn.a02`） | 本课用它 |
| `app_config_dir()` | 配置目录 | 设置文件 |
| `app_log_dir()` | 日志目录 | 日志文件 |
| `app_cache_dir()` | 缓存目录 | 可随时清掉的缓存 |
| `temp_dir()` | 系统临时目录 | 临时文件 |

### 怎么用

```rust
use tauri::Manager;

fn note_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()                       // 拿到路径解析器
        .app_data_dir()               // 应用数据目录
        .map_err(|e| format!("获取应用数据目录失败：{e}"))?;
    Ok(dir.join("note.txt"))          // 目录 + 文件名 → 完整路径
}
```

- `app.path()`：`Manager` trait 提供的方法，`AppHandle` 上直接可调
- `app_data_dir()` 返回 `Result<PathBuf, tauri::Error>`——**可能失败**（比如目录创建失败），所以用 `?` 传播
- `dir.join("note.txt")`：路径拼接，效果是 `<数据目录>/note.txt`

> **关键理解：** 目录名里的 `com.taurilearn.a02` 就是 `tauri.conf.json` 的 `identifier`——所以每个应用有自己的专属目录，互不污染。

<a id="sec-a02-fs"></a>
## 2. fs 插件 — 读写文件

### 为什么用插件而不是裸 `std::fs`

`std::fs` 当然能读写，但 Tauri 的 fs 插件（`tauri-plugin-fs`）提供**跨平台一致的体验**（Android/iOS 也支持）、统一的错误类型，并且 Rust 端方法直接可用。先注册插件：

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
```

### 读文件：`app.fs().open` + `read_to_string`

```rust
use std::io::Read;
use tauri_plugin_fs::{FsExt, OpenOptions};

let mut file = app
    .fs()                                                          // 拿到 fs 访问器
    .open(&path, OpenOptions { read: true, ..Default::default() }) // 以"只读"打开
    .map_err(|e| format!("打开文件失败：{e}"))?;
let mut content = String::new();
file.read_to_string(&mut content).map_err(|e| format!("读取失败：{e}"))?;
```

### 写文件：`OpenOptions` 三个开关

```rust
use std::io::Write;

let mut file = app
    .fs()
    .open(
        &path,
        OpenOptions {
            write: true,     // 可写
            create: true,    // 文件不存在就创建
            truncate: true,  // 打开时先清空（覆盖旧内容）
            ..Default::default()
        },
    )
    .map_err(|e| format!("创建文件失败：{e}"))?;
file.write_all(content.as_bytes()).map_err(|e| format!("写入失败：{e}"))?;
```

`OpenOptions` 是"打开方式"的声明，用 `OpenOptions::new()` 创建后用 builder 方法依次打开开关，四个最常用的：

| 开关 | 含义 | 本课 |
|------|------|------|
| `read: true` | 可读 | 读文件时开 |
| `write: true` | 可写 | 写文件时开 |
| `create: true` | 不存在则创建 | 写文件时开（第一次保存） |
| `truncate: true` | 打开即清空 | 写文件时开（整体覆盖，不残留旧内容） |

> **注意：** 插件里 `OpenOptions` 的字段是私有的，**不能用 `OpenOptions { read: true }` 结构体字面量**——必须 `OpenOptions::new()` 后用 `.read(true)` 这类方法链式打开。`..Default::default()` 也不能用（见 [答案讲解](a02_notepad_answer.md)）。fs 插件的 Rust 端方法（`app.fs()`）直接基于 `std::fs`，**不走权限 scope**——权限（capabilities）是给前端 JS 调用插件时用的。所以本课 capabilities 不需要额外加 `fs:*` 权限。

### 写之前确保目录存在

`create: true` 只创建文件，**不创建父目录**。应用数据目录未必存在，先手动建：

```rust
if let Some(dir) = path.parent() {
    std::fs::create_dir_all(dir).map_err(|e| format!("创建目录失败：{e}"))?;
}
```

<a id="sec-a02-result"></a>
## 3. `Result` 错误处理 — 出错也要体面

三个命令的返回值都是 `Result`：

```rust
#[tauri::command]
fn load_note(app: AppHandle) -> Result<String, String> { ... }

#[tauri::command]
fn save_note(app: AppHandle, content: String) -> Result<(), String> { ... }
```

- `Result<T, E>` 里 `E` 是错误类型，本课用 `String`——**最省事的做法**：错误消息直接就是一个能看懂的中文字符串
- 命令返回 `Err(...)` 时，Tauri 会把它转成 IPC 错误，前端 `invoke` 的 Promise 就会 reject，被 `.catch` 接到
- `map_err(...)` 把底层错误（`io::Error`）翻译成人话；`?` 遇到 `Err` 提前返回

```text
Rust 端:  Err("打开文件失败：No such file...") 
            │ Tauri 转成 IPC 错误
            ▼
前端:      invoke 的 Promise reject → .catch(e => setStatus(`读取失败: ${e}`))
```

**一个重要的设计：** `load_note` 里"文件不存在"**不是错误**，而是"第一次打开，内容为空"——所以用 `if !path.exists() { return Ok(String::new()); }` 提前返回空串。**什么时候算错误，是业务决定，不是技术决定。**

<a id="sec-a02-react"></a>
## 4. React — 前端换引擎

本课起前端改用 React。三个核心概念对照着看：

| 你要做的事 | Vanilla TS（A01） | React（本课） |
|----------|-------------------|--------------|
| 存数据 | 模块级 `let` 变量 | `useState` |
| 更新界面 | 手动 `render()` | 改 state，React 自动重渲染 |
| 启动时加载 | 直接调用 | `useEffect` |

```tsx
const [content, setContent] = useState("");   // 笔记内容

useEffect(() => {                              // 挂载后执行一次
  invoke<string>("load_note")
    .then((c) => setContent(c))                // 把读到的内容放进 state
    .catch((e) => setStatus(`读取失败: ${e}`));
}, []);

<textarea
  value={content}                              // 受控组件：值来自 state
  onChange={(e) => setContent(e.target.value)} // 输入 → 更新 state
/>
```

- `useState`：声明"一块会变的数据"；`content` 是当前值，`setContent` 是唯一合法的修改方式
- **受控组件**：`<textarea value={content}>`——输入框显示什么完全由 state 决定，state 变了界面自动变
- `useEffect(..., [])`：`[]` 表示"只在挂载时执行一次"（相当于"页面打开时做初始化"）

> **关键理解：** React 的心智模型是"**数据驱动界面**"——你只管改 state，界面由框架替你更新。相比 Vanilla 手动 `render()`，应用复杂后省力得多。超级项目（Markdown 编辑器）就是靠这个撑起来的。

---

## 练习指引

**作业范围：** 动 2 个文件，共 10 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | `load_note`：fs 插件打开 + `read_to_string` |
| `src-tauri/src/lib.rs` | 2 | `save_note`：`OpenOptions` 三开关 + `write_all` |
| `src-tauri/src/lib.rs` | 3 | `.plugin(tauri_plugin_fs::init())` |
| `src-tauri/src/lib.rs` | 4 | 登记三个命令 |
| `src/App.tsx` | 1-4 | 导入 invoke + 三处 invoke 调用 |

**怎么验证：**

```bash
cd 02_mini_apps/a02_notepad
cargo tauri dev
```

输入文字 → 保存 → 状态行显示"已保存 时间"；关闭窗口 → 再 `cargo tauri dev` 启动 → 文字还在。注意窗口底部显示的文件路径——用资源管理器打开那个目录，能看到 `note.txt`。

**故意踩坑看效果：** 保存前不建目录 → 报"创建目录失败"；`truncate: false` → 第二次保存时旧内容没清掉，新内容接在后面。

---

## 知识点连起来看

```text
app.path().app_data_dir()        ← 存哪：规范目录 + identifier
        │
dir.join("note.txt")             ← 路径拼接
        │
app.fs().open(OpenOptions{...})  ← 怎么开：read/write/create/truncate
        │
read_to_string / write_all       ← 读写本体
        │
Result<T, String> + map_err + ?  ← 出错：翻译成人话，前端 .catch 展示
        │
React: useState + useEffect      ← 界面：数据驱动渲染
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 路径 | 文件该存哪 | `app_data_dir`、`identifier`、`join` |
| 读写 | 怎么操作文件 | fs 插件、`OpenOptions` |
| 错误 | 失败了怎么办 | `Result`、`map_err`、`?`、业务 vs 技术错误 |
| 前端 | 交互怎么做 | React、`useState`、`useEffect`、受控组件 |

**一通百通的核心：** 这一课补上了"**持久化**"这一环。往后任何"要保存的应用"（记账本、笔记、设置）都是同一个套路：**拿到目录 → 读写文件 → 错误转中文 → 前端展示**。而 A01 学的事件、State，在超级项目里会跟它组合成完整的架构。

**递进关系：** 练习 A03（番茄钟）将引入"**时间**"——`async` 命令 + `tokio::time::sleep` 做倒计时，以及"应用在后台也要干活"的系统托盘。
