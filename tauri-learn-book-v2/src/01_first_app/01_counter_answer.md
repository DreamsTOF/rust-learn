# 练习 01 答案讲解：计数器

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/main.ts`（前端），共 4 处 TODO。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | `count_up` 函数体已写好 | `#[tauri::command]` 标注 + 注册 |
| `src/main.ts` | 界面绑定、错误兜底已写好 | `import { invoke }` + 真正的 invoke 调用 |

> **前端基础提示**：本练习前端是 Vanilla TS。`querySelector`、`addEventListener`、`textContent` 不熟的话，先读 [Vanilla TS 速成（给 Vue 开发者）](../00_vanilla_ts_primer.md)。

## lib.rs TODO 1：给 `count_up` 加 `#[tauri::command]`

### 练习版这里是什么

```rust
fn count_up(current: i32) -> i32 {
    current + 1
}
```

### 答案版填了什么

```rust
#[tauri::command]
fn count_up(current: i32) -> i32 {
    current + 1
}
```

### 为什么

`#[tauri::command]` 是**属性宏**：编译期把普通函数"升级"成可被前端调用的命令，自动生成参数反序列化、返回值序列化与注册信息。

**不加会发生什么**：函数编译通过，但前端 `invoke("count_up")` 时后端查无此命令，页面显示"调用失败"。

### 回查文档

[《练习 01》第 2 节：把函数升级成命令](01_counter.md#sec-01-command)。

## lib.rs TODO 2：登记命令

### 练习版这里是什么

```rust
.invoke_handler(tauri::generate_handler![
    // count_up,
])
```

### 答案版填了什么

```rust
.invoke_handler(tauri::generate_handler![count_up])
```

### 为什么

- `generate_handler![...]` 把命令打包成"命令注册表"
- `.invoke_handler(...)` 把表挂到 Builder 上
- 前端调用请求在表里**按名字查找**——写了 `#[tauri::command]` 但忘了登记，照样报"命令未找到"。这是刻意埋的练习点

### 回查文档

[《练习 01》第 3 节：登记电话号码簿](01_counter.md#sec-01-register)。

## main.ts TODO 1：导入 invoke

### 练习版这里是什么

```typescript
// import { invoke } from "@tauri-apps/api/core";
```

### 答案版填了什么

```typescript
import { invoke } from "@tauri-apps/api/core";
```

### 为什么

`invoke` 是前端调用后端的唯一入口。不 import，运行时 `invoke` 就是未定义变量，点击按钮会抛 `ReferenceError`，页面直接显示"调用失败"。

### 回查文档

[《练习 01》第 1 节：前端给后端打电话](01_counter.md#sec-01-invoke)。

## main.ts TODO 2：调用 `count_up`

### 练习版这里是什么

```typescript
count = 0; // ← 替换成你的代码
```

### 答案版填了什么

```typescript
count = await invoke<number>("count_up", { current: count });
```

### 为什么

- **`await` 必须有**：`invoke` 返回 Promise（跨进程请求，跟 `fetch` 同一个模型）。不 `await`，`count` 拿到的是 Promise 对象而不是数字。
- **`<number>` 泛型**：声明返回值类型，对应 Rust 的 `i32`（见 [类型映射表](01_counter.md#sec-01-serde)）。只是类型声明，运行时不校验。
- **`"count_up"` 是命令名**：必须和后端 `generate_handler![count_up]` 一致，拼错就"命令未找到"。
- **`{ current: count }` 的 key 必须等于 Rust 参数名 `current`**：Tauri 按参数名反序列化，key 拼错后端报参数缺失。

> **Vanilla TS 注解**：`await` 是"等结果回来"。`count` 是模块级 `let` 变量，每次都重新赋值；`valueEl!.textContent = String(count)` 把数字**当纯文字**显示（不是 HTML）。不熟的话看 [异步：async/await 与错误兜底](../00_vanilla_ts_primer.md)。

### 回查文档

[《练习 01》第 1 节：invoke 的三件事](01_counter.md#sec-01-invoke)、[第 4 节：serde 与序列化](01_counter.md#sec-01-serde)。

## 验收标准

```bash
cd 01_first_app/e01_counter
cargo tauri dev
```

窗口出现大数字（初始 0）与 +1 按钮；每点一次按钮，数字 +1，状态行显示"后端返回: N"。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把命令名 `"count_up"` 改成 `"count_up_xxx"` → 点按钮显示"调用失败"（验证命令名是字符串、按名字查找）
- 把 `lib.rs` 的注册注释掉（改回 `// count_up,`）→ 同样的"调用失败"（验证注册的必要性）
- 把参数 key 改成 `{ cur: count }` → 点按钮显示"调用失败"（验证参数名必须等于 Rust 参数名）
- 把 `await` 去掉 → 界面不报错，但数字永远是 `[object Promise]` 或不变（验证 invoke 必须异步）

## 升级挑战（可选）

- 加一个"清零"按钮：清空时调用一次后端，把数字归零
- 改成两个按钮（+1 / -1）：后端加一个 `count_down` 命令，重复练习"定义 → 注册 → 调用"三件套
