# 练习 E21: 前端事件

**知识点：** 前端 listen / once / unlisten / 类型化 payload

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- Rust 端 `EventPayload`（Serialize + Deserialize）作为命令参数，`app.emit` 发送事件
- 前端 `listen` 多次接收、`once` 只收一次、`unlisten()` 取消监听
- payload 用 TS `interface EventPayload` 类型化，`e.payload` 直接带类型访问
- 对照答案：e21_frontend_events_answer/