# 练习 E01: 计数器（练习版）

**目标：** 做一个"点一下 +1"的计数器桌面应用，学会 Tauri 的三层骨架：`invoke`（前端调用）、`#[tauri::command]`（定义命令）、`generate_handler!`（注册命令）。

**TODO（共 4 处）：**

- `src-tauri/src/lib.rs`
  - 步骤 1：给 `count_up` 函数加 `#[tauri::command]` 属性
  - 步骤 2：在 `generate_handler![...]` 中登记 `count_up`
- `src/main.ts`
  - 步骤 1：导入 `invoke`
  - 步骤 2：调用 `invoke<number>("count_up", { current: count })`

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/01_first_app/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1420
- identifier: com.taurilearn.e01
