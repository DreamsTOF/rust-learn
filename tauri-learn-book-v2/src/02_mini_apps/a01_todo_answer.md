# 练习 A01 答案讲解：待办清单

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/main.ts`（前端），共 8 处 TODO。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | `TodoItem` / `TodoState` / `list_todos` 已给 | `.manage`、3 条 `emit`、3 个命令的注册 |
| `src/main.ts` | 渲染、刷新、事件委托骨架已给 | 3 处 `invoke` 调用 + 日志追加 |

## lib.rs TODO 2：注册共享状态

### 练习版这里是什么

```rust
tauri::Builder::default()
    // TODO: .manage(TodoState::default())
    .invoke_handler(...)
```

### 答案版填了什么

```rust
tauri::Builder::default()
    .manage(TodoState::default())
    .invoke_handler(...)
```

### 为什么

`manage` 把 `TodoState` 实例注册进 Builder，命令里的 `State<'_, TodoState>` 才能拿到它。**不注册 → 运行时命令报"状态不存在"，前端显示"调用失败"。**

### 回查文档

[《练习 A01》第 1 节：manage + State\<T\>](a01_todo.md#sec-a01-manage)。

## lib.rs TODO 3-5：三条 `emit`

### 练习版这里是什么

```rust
// 步骤 3（add_todo 里，push 之后）
// TODO: app.emit("todo-log", format!("添加：{text}"));
items.clone()
```

### 答案版填了什么

```rust
let _ = app.emit("todo-log", format!("添加：{text}"));
items.clone()
```

toggle 里是 `format!("{action}：{}", item.text)`，delete 里是 `format!("删除：{}", item.text)`——三条格式串不同，套路完全一样。

### 为什么

- `app: AppHandle` 是 Tauri 自动注入的"总台"，发广播必须通过它
- `emit("todo-log", 载荷)` 把载荷广播给所有前端；前端 `listen` 同一个名字就能收到
- `let _ =`：忽略 `Result`（教学场景）；不忽略就得处理失败
- **注意 `app` 在 add/toggle/delete 里都作为参数注入**——这是"依赖注入"的第一次正式出现

### 回查文档

[《练习 A01》第 4 节：emit + listen](a01_todo.md#sec-a01-emit-listen)。

## lib.rs TODO 6：登记三个命令

### 练习版这里是什么

```rust
.invoke_handler(tauri::generate_handler![
    list_todos,
    // TODO: add_todo, toggle_todo, delete_todo,
])
```

### 答案版填了什么

```rust
.invoke_handler(tauri::generate_handler![list_todos, add_todo, toggle_todo, delete_todo])
```

### 为什么

同练习 01：命令定义 ≠ 命令可用，必须登记进注册表，前端按名字查找。**没登记的 `add_todo` 会产生 `dead_code` 警告**——编译器也在提示你"这函数没人用"。

### 回查文档

[《练习 01》第 3 节：登记电话号码簿](../01_first_app/01_counter.md#sec-01-register)（沿用）。

## main.ts TODO 2：调用 add_todo

### 练习版这里是什么

```typescript
render([]); // ← 替换成你的代码
inputEl!.value = "";
```

### 答案版填了什么

```typescript
render(await invoke<TodoItem[]>("add_todo", { text }));
inputEl!.value = "";
```

### 为什么

- 参数 `{ text }` 的 key 必须等于 Rust 参数名 `text`
- 返回值是 `Vec<TodoItem>`，泛型写 `TodoItem[]`；后端已经把**最新列表**作为返回值——渲染它，列表立刻更新，**不需要再单独查一次**

### 回查文档

[《练习 01》第 1 节：invoke 的三件事](../01_first_app/01_counter.md#sec-01-invoke)（沿用）。

## main.ts TODO 3：事件委托里调 toggle / delete

### 练习版这里是什么

```typescript
const btn = (ev.target as HTMLElement).closest<HTMLButtonElement>("[data-action]");
if (!btn) return;
// TODO: 取 id + 根据 data-action 调 toggle_todo / delete_todo
render([]); // ← 替换成你的代码
```

### 答案版填了什么

```typescript
const id = Number(btn.dataset.id);
if (btn.dataset.action === "toggle") {
  render(await invoke<TodoItem[]>("toggle_todo", { id }));
} else if (btn.dataset.action === "delete") {
  render(await invoke<TodoItem[]>("delete_todo", { id }));
}
```

### 为什么

- **事件委托**：不在每个 `<li>` 上绑监听，只给父级 `#todo-list` 绑一个。点哪个按钮用 `closest("[data-action]")` 找出来，再用 `data-id` 告诉后端"是哪一条"
- `Number(btn.dataset.id)`：HTML 属性是字符串，后端要 `u64`，先转数字
- `id` 类型是 `number`，与 Rust `u64` 对应

## main.ts TODO 4：listen 追加日志

### 练习版这里是什么

```typescript
listen<string>("todo-log", (event) => {
  logEl!.textContent = `（TODO：把日志追加成 <li>，事件内容：${event.payload}）`;
});
```

### 答案版填了什么

```typescript
listen<string>("todo-log", (event) => {
  logEl!.insertAdjacentHTML("beforeend", `<li>${event.payload}</li>`);
});
```

### 为什么

- `insertAdjacentHTML("beforeend", ...)`：在 `#log` 末尾插入一段 HTML——**每次收到事件追加一条，而不是覆盖**（`textContent = ...` 是覆盖，占位版用它是为了能编译）
- `` `<li>${event.payload}</li>` ``：模板字符串把后端推来的日志包成一个列表项

### 回查文档

[《练习 A01》第 4 节：emit + listen](a01_todo.md#sec-a01-emit-listen)。

## 验收标准

```bash
cd 02_mini_apps/a01_todo
cargo tauri dev
```

输入内容 → 添加 → 列表出现；点"完成"划线、再点"重开"恢复；点"删除"消失；日志面板逐条显示"添加 / 完成 / 删除"。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 删掉 `.manage(...)` → 任何操作显示"调用失败"（验证 State 必须注册）
- 把 `generate_handler!` 里的 `delete_todo` 删掉 → 点删除报"命令未找到"（验证注册的必要性）
- 把 emit 的事件名改成 `"todo-logx"` → 列表照常工作，但日志面板没动静（验证事件名必须前后端一致）
- 把 `insertAdjacentHTML` 改成 `textContent = ...` → 日志只剩最后一条（验证追加 vs 覆盖）

## 升级挑战（可选）

- 加一个"清空已完成"按钮：后端加 `clear_done` 命令，复用"定义 → 注册 → 调用"三件套
- 让 `toggle` 不发日志，观察日志面板（体会"哪些事值得通知前端"）
