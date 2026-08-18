# 练习 A01: 待办清单

## 为什么要学这个

计数器（练习 01）的致命缺陷是：**数字存在前端变量里，页面一刷新就没了。** 真正的应用要跨操作、跨时间地"记住"数据。这一章要回答三个问题：

1. **数据该放哪？** —— 为什么要把数据交给后端存，而不是放在前端？
2. **多条命令怎么共享同一份数据？** —— `manage` + `State<T>` + `Mutex` 三件套怎么配合？
3. **后端怎么"主动"告诉前端？** —— 命令返回值是"前端问了才答"，事件是"后端自己推"，两者什么区别？

搞懂这三件事，"有状态的应用"的大门就打开了——后面所有项目（记事本、番茄钟、记账本）都是"放数据 + 改数据 + 通知前端"的组合。

---

## 从问题出发

练习 A01 要做的事：**一个待办清单——能加、能勾掉、能删**。

**核心矛盾：** 待办数据必须"记住"。如果还像计数器那样把 `count` 放在前端变量里，那么：
- 点一次按钮 → 数据变一次 → 页面一刷新，一切归零；
- 两个按钮同时想改数据，前端自己先打架。

所以数据要放在**后端**：后端进程活着，数据就在；所有改数据的操作都走后端，大家看到的就是同一份。

```text
前端 (WebView)                    Rust 进程
┌─────────────────────┐          ┌──────────────────────────┐
│ 输入框 + 按钮         │ invoke  │  TodoState (manage 注册)  │
│ 列表渲染              │ ──────► │   items: Mutex<Vec<...>>  │
│ 日志面板 (listen)     │ ◄────── │   next_id: Mutex<u64>    │
└─────────────────────┘  事件推  └──────────────────────────┘
```

本课新增四样东西，环环相扣：**`manage` 把状态注册进后端 → `State<T>` 让命令拿到它 → `Mutex` 保证一次只有一个人改 → `emit` 让后端主动通知前端。**

---

<a id="sec-a01-manage"></a>
## 1. `manage` + `State<T>` — 数据放后端，命令来取

### 第一步：定义"整个应用的数据"

```rust
#[derive(Default)]
struct TodoState {
    items: Mutex<Vec<TodoItem>>,
    next_id: Mutex<u64>,
}
```

`TodoState` 就是"整个应用的数据"：待办列表 + 自增 id 计数器。它和后端进程同生共死——应用启动时创建，应用关闭时销毁。

### 第二步：`manage` 把数据注册进 Builder

```rust
tauri::Builder::default()
    .manage(TodoState::default())
```

`.manage(...)` 把 `TodoState` 的实例注册成 Tauri 管理下的共享状态。**不注册，命令就拿不到它。**

### 第三步：命令用 `State<'_, T>` 参数取数据

```rust
#[tauri::command]
fn list_todos(state: State<'_, TodoState>) -> Vec<TodoItem> {
    state.items.lock().unwrap().clone()
}
```

`State<'_, TodoState>` 是 Tauri 自动注入的特殊参数——**不需要前端传**，框架把 `manage` 注册的那个实例递进来。命令里 `state.items` 就能读到列表。

> **关键理解：** `manage` 只注册一次，但**每个命令都能拿到同一个实例**。这正是"共享状态"的含义——a01 加一条，a01_answer 打开同一个数据？不，每个项目是独立进程；这里的"共享"是指**同一次运行里，所有命令看到的是同一份数据**。

### 没注册会怎样？

命令参数里写了 `State<TodoState>` 但 Builder 没 `.manage(...)`——运行时命令会报"state 不存在"的错误，前端显示"调用失败"。这是本课埋的第一个坑。

<a id="sec-a01-mutex"></a>
## 2. `Mutex` — 一次只有一个人能改

为什么 `items` 外面要包一层 `Mutex`？

考虑两个按钮同时操作：用户飞快地点"添加"和"删除"，两条 `invoke` 几乎同时到达后端，两个命令同时读改写 `Vec`——Rust 的内存安全规则**禁止两个可变借用同时存在**，直接编译都过不去。

`Mutex`（互斥锁）的解决办法：**想要改数据，先 `lock()` 拿钥匙；拿到钥匙的人改完 `drop`（解锁），下一个人才能拿。** 同一时刻最多一个人持有锁：

```rust
let mut items = state.items.lock().unwrap();
items.push(item);
```

- `lock()` 返回 `MutexGuard`，它可以当作 `&mut Vec` 用
- `.unwrap()`：锁被占用时阻塞等待，正常不会失败；出问题（比如持锁者 panic）就 panic
- 作用域结束，guard 自动解锁——所以**尽量在小的代码块里 lock，改完立刻释放**

> **为什么这里敢 `unwrap`？** 因为锁竞争只有"等"，没有"毒"（除非持锁线程 panic）。教学起见直接 unwrap；生产代码会用 `lock().map_err(...)` 优雅处理。

<a id="sec-a01-serde-struct"></a>
## 3. 结构体 + serde — 一条待办长什么样

待办不只是字符串，它有"形状"：**id（谁）、text（说什么）、done（完成没）**。用结构体表达：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub id: u64,
    pub text: String,
    pub done: bool,
}
```

- `Serialize`：作为命令返回值，穿越进程边界时**序列化成 JSON**
- `Deserialize`：本课前端不传结构体进来，但双向派生子是标准姿势（后面练习会用到）
- 序列化规则：**字段名就是 JSON 的 key**

```text
Rust:  TodoItem { id: 0, text: "买菜", done: false }
         │ serde 序列化
         ▼
JSON:  {"id":0,"text":"买菜","done":false}
         │ 反序列化
         ▼
TS:    { id: 0, text: "买菜", done: false }   ← interface TodoItem 就是照它写的
```

前端 `interface TodoItem { id: number; text: string; done: boolean }` 与 Rust 结构体**字段一一对应**——这就是 a01 前后端传递的"同一种语言"。

<a id="sec-a01-emit-listen"></a>
## 4. `emit` + `listen` — 后端主动推，前端躺着收

前面学的 `invoke` 是**请求-响应**：前端问一句，后端答一句。但待办清单里有个场景它办不到——**后端改了数据，要主动告诉前端"我刚加了条日志"**。总不能前端每隔一秒问一次（轮询）吧？

Tauri 提供了**事件**：后端 `emit` 广播，前端 `listen` 收听。

### 后端：`app.emit`

```rust
#[tauri::command]
fn add_todo(app: AppHandle, state: State<'_, TodoState>, text: String) -> Vec<TodoItem> {
    // ...push 进列表...
    let _ = app.emit("todo-log", format!("添加：{text}"));
    items.clone()
}
```

- `app: AppHandle`：**应用句柄**，Tauri 自动注入。要"发广播"得先拿到这个"总台"
- `app.emit("事件名", 载荷)`：向所有前端广播事件。载荷可以是任意可序列化类型，这里是一条 `String`
- `let _ = ...`：emit 的返回值是 `Result`，教学场景忽略（`_`），不忽略也行

### 前端：`listen`

```typescript
import { listen } from "@tauri-apps/api/event";

listen<string>("todo-log", (event) => {
  logEl!.insertAdjacentHTML("beforeend", `<li>${event.payload}</li>`);
});
```

- `listen("事件名", 回调)`：订阅事件，后端每次 `emit` 都触发一次回调
- `event.payload`：就是后端 `emit` 的那个载荷（泛型 `<string>` 声明它的类型）
- 回调里 `event` 对象还有 `id`、`event` 等字段，本课只用 `payload`

> **对比总结：**

| | 命令返回值 | 事件（emit / listen） |
|---|---|---|
| 谁发起 | 前端发起 | 后端主动发起 |
| 方向 | 前端 → 后端 → 前端（一来一回） | 后端 → 前端（单向广播） |
| 典型场景 | 前端要数据 | 后端有变化要通知 |
| 本课例子 | `list_todos` 返回列表 | 每次操作推一条"操作日志" |

本课待办清单**两个都用**：点按钮用 invoke（请求-响应），操作日志用事件（后端推送）——一道菜同时学会两种通信方式。

---

## 练习指引

**作业范围：** 只动 2 个文件，共 8 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 2 | `.manage(TodoState::default())` 注册共享状态 |
| `src-tauri/src/lib.rs` | 3-5 | 三个命令里各补一条 `app.emit("todo-log", ...)` |
| `src-tauri/src/lib.rs` | 6 | 登记 add/toggle/delete 三个命令 |
| `src/main.ts` | 2 | 调 `add_todo` 并渲染返回的列表 |
| `src/main.ts` | 3 | 事件委托里调 toggle/delete |
| `src/main.ts` | 4 | `listen("todo-log", ...)` 追加日志 |

**怎么验证：**

```bash
cd 02_mini_apps/a01_todo
cargo tauri dev
```

输入内容点"添加"，列表出现待办；点"完成"文字划线；点"删除"消失；下方日志面板逐条追加"添加：xxx / 完成：xxx / 删除：xxx"。

**故意踩坑看效果：** 不改 `.manage` → 任何操作都显示"调用失败"；不注册 add_todo → 点添加报"命令未找到"。这两个坑每个练习都会埋。

---

## 知识点连起来看

```text
manage(TodoState::default())        ← 注册：数据住进后端
        │
State<'_, TodoState>                ← 注入：命令拿到同一份数据
        │
Mutex<Vec<TodoItem>>                ← 并发：一次只有一个人改
        │
Vec<TodoItem> (serde)               ← 传输：结构体穿越进程边界
        │
app.emit("todo-log", ...)           ← 通知：后端主动推给前端
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 状态 | 数据放哪、怎么共享 | `manage`、`State<T>` |
| 并发 | 同时改怎么办 | `Mutex`、`lock()` |
| 传输 | 结构体怎么过边界 | serde、JSON、字段名=key |
| 通信 | 后端怎么主动说 | `AppHandle`、`emit` / `listen` |

**一通百通的核心：** 这一课完成了一次"质的升级"——从"前后端通话"（练习 01）到"**应用有状态**"。往后所有项目的骨架都是这四层：**注册状态 → 命令读写 → 结构体传输 → 事件通知**。变的是业务数据，不变的是这条链路。

**递进关系：** 练习 A02（记事本）将回答"关掉软件，数据还在吗？"——状态存在内存里，应用一关就没了；要持久化，就得落盘（路径 API + fs 插件）。
