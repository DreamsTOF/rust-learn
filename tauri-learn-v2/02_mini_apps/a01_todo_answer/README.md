# 练习 A01: 待办清单（答案版）

**目标：** 做第一个"有状态"的应用——待办清单。数据放在**后端**（`manage` + `State<T>` + `Mutex`），前后端用结构体传递，后端每次操作主动 `emit` 日志、前端 `listen` 展示。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `.manage(TodoState::default())` 注册共享状态
  - 三个命令里各补一条 `let _ = app.emit("todo-log", format!(...));`
  - `generate_handler![list_todos, add_todo, toggle_todo, delete_todo]`
- `src/main.ts`
  - 步骤 2/3：`invoke` 调用 add/toggle/delete 并渲染返回值
  - 步骤 4：`listen<string>("todo-log", ...)` 用 `insertAdjacentHTML` 追加日志

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a01_todo_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1423
- identifier: com.taurilearn.a01a
