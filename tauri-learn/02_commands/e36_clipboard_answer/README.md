# 练习 E36: 剪贴板

**知识点：** 插件注册（Rust + capabilities）/ `writeText` / `readText` / `clear`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 输入内容 → 「写入剪贴板」（可在记事本等应用中 Ctrl+V 粘贴验证）
2. 「读取剪贴板」显示当前剪贴板文本
3. 「清空剪贴板」清空

## 说明

- 本练习全程前端实现，Rust 侧只负责注册插件
- 对比答案: `e36_clipboard_answer/`

## 信息

- devUrl: http://localhost:1490
- identifier: com.taurilearn.e36