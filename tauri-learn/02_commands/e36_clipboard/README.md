# 练习 E36: 剪贴板

**知识点：** 插件注册（Rust + capabilities）/ `writeText` / `readText` / `clear`

## TODO（练习版）

在 `src/main.ts` 中按注释提示补全：

1. `writeText(text)` 写入剪贴板并展示结果
2. `readText()` 读取剪贴板并展示
3. `clear()` 清空剪贴板并展示结果

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 本练习全程前端实现，Rust 侧只负责注册插件
- 对照答案: `e36_clipboard_answer/`

## 信息

- devUrl: http://localhost:1490
- identifier: com.taurilearn.e36