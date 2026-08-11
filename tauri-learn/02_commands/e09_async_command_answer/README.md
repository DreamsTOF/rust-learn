# 练习 E09: 异步命令

**知识点：** async fn 命令 / tokio::time::sleep / tokio::time::timeout 超时

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- 慢速回显：sleep 2000ms 后返回；超时演示：1 秒超时中断 3 秒任务