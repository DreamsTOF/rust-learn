# 练习 E25: 窗口事件处理

知识点: on_window_event / Resized / Moved / Focused / 状态管理（Mutex + State）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 拖拽窗口 / 调整大小 / 切换焦点会触发事件，后端记录后前端拉取展示（最新在前）
- identifier: com.taurilearn.e25a