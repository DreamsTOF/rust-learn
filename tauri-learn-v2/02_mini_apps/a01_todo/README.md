# 练习 A01: 待办清单（练习版）

**目标：** 做第一个"有状态"的应用——待办清单。数据放在**后端**（`manage` + `State<T>` + `Mutex`），前后端用结构体传递，后端每次操作主动 `emit` 日志、前端 `listen` 展示。

**新增知识：** `Builder::manage()` / `State<T>` / `Mutex`、结构体 + serde 序列化、`listen` / `emit`。

**TODO（共 8 处）：**

- `src-tauri/src/lib.rs`（5 处）
  - 步骤 2：`.manage(TodoState::default())` 注册共享状态
  - 步骤 3-5：三个命令里各发一条 `app.emit("todo-log", ...)`
  - 步骤 6：在 `generate_handler!` 登记 add/toggle/delete
- `src/main.ts`（3 处）
  - 步骤 2：调用 `add_todo` 并渲染
  - 步骤 3：事件委托里调用 toggle/delete
  - 步骤 4：把 `listen("todo-log", ...)` 的占位改成追加日志

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1422
- identifier: com.taurilearn.a01
