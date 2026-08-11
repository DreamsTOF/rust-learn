# 练习 E09: 异步命令

**知识点：** async fn 命令 / tokio::time::sleep / tokio::time::timeout 超时

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- 慢速回显：sleep 2000ms 后返回；超时演示：1 秒超时中断 3 秒任务
- 对照答案：e09_async_command_answer/