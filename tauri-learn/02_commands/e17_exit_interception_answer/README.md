# 练习 E17: 退出拦截

**知识点：** CloseRequested / prevent_close / emit 通知前端 / destroy 退出

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- `on_window_event` 匹配 CloseRequested：`api.prevent_close()` 阻止默认关闭，并 `emit("close-requested")` 通知前端
- 前端弹 confirm 确认框，确认后调用 `confirm_close` 命令 `destroy()` 窗口