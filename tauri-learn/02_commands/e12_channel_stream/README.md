# 练习 E12: Channel 流式传输

**知识点：** Channel::new / send 推送 / onmessage 前端消费

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- 进度流：后端循环 send 0-100 的进度值，前端 onmessage 实时更新进度条
- 消息流：后端连续发送 5 条结构化消息，前端追加到列表
- 对照答案：e12_channel_stream_answer/