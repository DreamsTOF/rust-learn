# 练习 A07: 批量重命名

## 为什么要学这个

A03 的番茄钟解决了"长时间任务不卡界面"，但还差一环：**用户要看到任务做到哪了**。A03 用事件推秒数，但事件是"广播"；批量处理场景更常见的是**点对点的进度流**。这一章要回答三个问题：

1. **怎么把进度一条条推给前端？** —— `Channel` 和命令返回值、事件有什么区别？
2. **任务放后台跑，命令怎么立刻返回？** —— `async_runtime::spawn` 是干什么的？
3. **怎么让用户选文件夹？** —— dialog 插件一行代码搞定。

学完你会：做出"先预览、后执行、带进度条"的批量工具——这是文件管理类应用的经典形态。

---

## 从问题出发

练习 A07 要做的事：**选一个文件夹，把里面所有 `IMG_` 开头的文件改成 `photo_` 开头，几百个文件批量执行，界面上有进度条。**

**核心矛盾：** 批量处理有两个要求，和之前学到的方式都对不上：

1. **过程要可见**：命令返回值只能给"最终结果"（改完了才返回）；A03 的事件是"广播给所有人"。批量任务要的是**指向特定这次调用的进度流**——`Channel` 正好是这个角色。
2. **不能卡界面**：几百个 `std::fs::rename` 串行跑会占住命令，界面卡死。要把任务**丢到后台**（`async_runtime::spawn`），命令立即返回。

```text
前端 (React)                                Rust 进程
┌──────────────────────────┐               ┌───────────────────────────────┐
│ open() 选文件夹           │ ──dialog──►  │ dialog 插件（系统对话框）      │
│ invoke("preview_rename") │ ───────────► │ 列出文件 → 计算新旧名（预览）   │
│ invoke("run_rename",     │ ───────────► │ async_runtime::spawn 后台任务   │
│   { onProgress: channel })│              │   逐个 rename → channel.send   │
│ channel.onmessage 进度条  │ ◄──进度流─── │                               │
└──────────────────────────┘               └───────────────────────────────┘
```

---

<a id="sec-a07-channel"></a>
## 1. Channel — 命令的"单向进度管道"

### 三种"后端 → 前端"的方式对比

| 方式 | 谁发起 | 特点 | 适合 |
|------|--------|------|------|
| 命令返回值 | 前端问 | 一次只能回一个结果 | 请求-响应 |
| 事件（A01/A03） | 后端推 | 广播给所有监听者 | 全局通知 |
| **Channel（本课）** | 后端推 | **点对点**，绑在一次调用上 | 进度、流式数据 |

### 前端：建通道、注册回调、随 invoke 传过去

```typescript
import { invoke, Channel } from "@tauri-apps/api/core";

const channel = new Channel<RenameProgress>();   // ① 建一个"管道"
channel.onmessage = (m) => setProgress(m);        // ② 收到一条就更新 UI
await invoke("run_rename", { dir, find, replace, onProgress: channel }); // ③ 把管道塞进参数
```

### 后端：`Channel<T>` 参数 + `send`

```rust
use tauri::ipc::Channel;

#[tauri::command]
fn run_rename(
    ...
    on_progress: Channel<RenameProgress>,   // Tauri 自动把前端的管道接过来
) -> Result<(), String> {
    // 任务进行中……每完成一步：
    let _ = on_progress.send(RenameProgress { done, total, current, finished });
}
```

- 前端 `onProgress: channel` 对应后端 `on_progress` 参数（camelCase ↔ snake_case 自动转换）
- **`channel.send(payload)` 多少次都行**——这就是"流"：一条命令调用的生命周期内，后端可以推任意多条进度
- `Channel<T>` 是 `Clone + Send` 的，可以 move 进后台任务里用

> **关键理解：** 事件是"广播"（一个 emit，所有 listen 都收到）；Channel 是"点对点管道"（这一次调用的接收端唯一）。批量任务的进度天然属于某一次调用，用 Channel 才对。

<a id="sec-a07-spawn"></a>
## 2. 后台任务 — `async_runtime::spawn`

`run_rename` 里改文件要跑一会儿。两种做法：

```rust
// 做法 A：async 命令里直接循环（命令要等它跑完才返回）
// 做法 B（本课）：spawn 一个后台任务，命令立刻返回 Ok(())
tauri::async_runtime::spawn(async move {
    for path in files {
        // ... rename + on_progress.send(...)
    }
});
Ok(())  // 命令立刻返回，重命名在后台继续
```

- `tauri::async_runtime::spawn(闭包/async块)`：把任务丢到 Tauri 的运行时线程池上，**fire-and-forget**
- 命令立刻返回 `Ok(())`——前端 `await invoke(...)` 马上结束，进度靠 Channel 继续收
- 需要 move 进任务的 `files`、`find`、`replace`、`on_progress` 都会被移动进去

> **对比 A03：** 番茄钟的倒计时命令是 async 且 `await` 到结束（因为结束时要发通知、要收尾）。批量任务更适合 spawn：**发起即返回，结果靠管道汇报**。哪种更合适取决于"命令结束时要不要做收尾"。

<a id="sec-a07-dialog"></a>
## 3. dialog 插件 — 让用户选文件夹

让用户输入文件夹路径太不友好。dialog 插件直接调系统对话框：

```rust
// Rust 端注册（练习版已配好）
tauri::Builder::default().plugin(tauri_plugin_dialog::init())
```

```typescript
// 前端（JS 包 @tauri-apps/plugin-dialog）
import { open } from "@tauri-apps/plugin-dialog";

const selected = await open({ directory: true });  // 只允许选文件夹
if (typeof selected === "string") {
  setDir(selected);   // 拿到选中的目录路径
}
```

- `open({ directory: true })`：弹出系统目录选择器；取消时返回 `null`
- 权限：`dialog:default`（练习版已配好）——**插件 API 要权限，自己写的命令不要**（A02 的结论再次适用）

<a id="sec-a07-preview"></a>
## 4. 预览 → 执行 — 危险操作先看清楚

批量改名是"不可轻易撤销"的操作（改坏了 500 个文件很麻烦）。所以分成两步：

1. **`preview_rename`（预览）**：只计算不改动——列出"旧名 → 新名"，返回给前端展示
2. **`run_rename`（执行）**：用户确认后再真正 `rename`

```rust
// 预览：算新名，只收集会改名的
if new_name != file_name {
    items.push(PreviewItem { old: file_name.to_string(), new: new_name });
}
```

前端"开始重命名"按钮在 `preview.length === 0` 时禁用——**先看清单，再动手**。

---

## 练习指引

**作业范围：** 动 2 个文件，共 9 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | `build_new_name` 字符串替换 |
| `src-tauri/src/lib.rs` | 2 | `list_files` 遍历目录 |
| `src-tauri/src/lib.rs` | 3 | `preview_rename` 收集会改名的文件 |
| `src-tauri/src/lib.rs` | 4 | `run_rename` 里改名 + `on_progress.send` |
| `src-tauri/src/lib.rs` | 5 | 注册 dialog 插件 + 登记命令 |
| `src/App.tsx` | 1-4 | 导入、选文件夹、预览、Channel 进度 + 执行 |

**怎么验证：**

```bash
cd 02_mini_apps/a07_batch_rename
cargo tauri dev
```

选一个含多张图片的文件夹 → 点预览 → 看到"旧名 → 新名"清单 → 点开始重命名 → 进度条滚动、显示"正在处理 xxx" → 完成后到文件夹里确认文件名都变了。

**故意踩坑看效果：** 把 `on_progress` 参数从 invoke 里去掉 → 进度条不动（验证 Channel 是参数传过去的）；把 spawn 改成直接在命令里循环 → 按钮卡住直到改完（验证后台任务）。

---

## 知识点连起来看

```text
open({ directory: true })             ← dialog 插件：系统选文件夹
        │
preview_rename（只算不改）            ← 预览：危险操作先看清单
        │
async_runtime::spawn                   ← 后台：命令立刻返回
        │
std::fs::rename + channel.send(进度)   ← 执行：逐个改 + 点对点推进度
        │
Channel<T> onmessage                    ← 前端：进度条
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 选目录 | 用户怎么给路径 | dialog 插件、`open({ directory: true })` |
| 预览 | 怎么避免误操作 | 只计算不改动、`preview` |
| 后台 | 长任务不卡界面 | `async_runtime::spawn` |
| 进度 | 过程怎么让用户看见 | `Channel`、`send` / `onmessage` |

**一通百通的核心：** 这一课补齐了"**长任务的反馈**"。凡是"要跑一会儿、用户得看着"的操作（下载、转换、批量处理），骨架都是：**spawn 后台 + Channel 推进度 + 进度条**。超级项目 P19 导出、P21 拼写检查会直接复用。

**递进关系：** 练习 A08（剪贴板历史）把"桌面三件套"凑齐——剪贴板、全局快捷键、应用菜单。学完八道小菜，你就有能力独立做各种桌面小工具了。
