# 练习 E12: Channel 流式传输

**知识点：** Channel::new / send 推送 / onmessage 前端消费

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- 进度流：后端循环 send 0-100 的进度值，前端 onmessage 实时更新进度条
- 消息流：后端连续发送 5 条结构化消息，前端追加到列表