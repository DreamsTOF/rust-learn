# 练习 E21: 前端事件

**知识点：** 前端 listen / once / unlisten / 类型化 payload

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- Rust 端 `EventPayload`（Serialize + Deserialize）作为命令参数，`app.emit` 发送事件
- 前端 `listen` 多次接收、`once` 只收一次、`unlisten()` 取消监听
- payload 用 TS `interface EventPayload` 类型化，`e.payload` 直接带类型访问