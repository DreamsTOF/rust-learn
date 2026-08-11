# 练习 E22: 窗口级事件

**知识点：** emit_to 定向发送 / 窗口级 vs 全局事件差异 / 多窗口

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- `open_chat_window` 用 WebviewWindowBuilder 创建 chat 窗口（与主窗共用 index.html，按 label 渲染不同 UI）
- `emit_to` 定向发送给指定窗口；`emit` 全局广播给所有窗口
- 前端 `getCurrentWindow().listen` 是窗口级监听，`listen` 是全局监听
- capabilities 中把 chat 窗口加入 windows 列表