# 练习 E16: setup 钩子

**知识点：** Builder::setup() / 异步初始化 / run_on_main_thread / emit 事件

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- setup 闭包中注入 SetupState、spawn 异步任务 1 秒后 emit `init-done` 事件、执行主线程回调
- 前端 `listen("init-done")` 实时更新状态显示