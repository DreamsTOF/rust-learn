# 练习 E01: 计数器（答案版）

**目标：** 做一个"点一下 +1"的计数器桌面应用，学会 Tauri 的三层骨架：`invoke`（前端调用）、`#[tauri::command]`（定义命令）、`generate_handler!`（注册命令）。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `fn count_up` 上加 `#[tauri::command]`
  - `generate_handler![count_up]` 完成注册
- `src/main.ts`
  - `import { invoke } from "@tauri-apps/api/core"`
  - `count = await invoke<number>("count_up", { current: count })`

**完整讲解见：** `tauri-learn-book-v2/src/01_first_app/01_counter_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1421
- identifier: com.taurilearn.e01a
