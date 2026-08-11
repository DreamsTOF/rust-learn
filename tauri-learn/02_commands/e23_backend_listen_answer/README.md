# 练习 E23: 后端监听

**知识点：** app_handle.listen / 事件 payload 解析 / 后端转发到窗口

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- setup 中用 `handle.listen("ping-a" / "ping-b")` 注册后端监听（use tauri::Listener），`event.payload().to_string()` 解析负载
- 监听器收到后打印到终端，并用 `emit_to("main", "pong", ...)` 把回应转发回主窗口
- 前端 `listen('pong')` 展示后端回应列表