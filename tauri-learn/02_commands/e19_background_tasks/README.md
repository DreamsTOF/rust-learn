# 练习 E19: 后台任务

**知识点：** async_runtime::spawn / spawn_blocking / 任务完成结果事件回传

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- `start_async_task` 用 `tauri::async_runtime::spawn` 跑异步任务（sleep 后 emit `task-done`），不阻塞 UI
- `start_blocking_task` 用 `tauri::async_runtime::spawn_blocking` 在独立线程池计算平方和（emit `blocking-done`），适合 CPU 密集
- 前端用 `listen`（@tauri-apps/api/event）接收两个完成事件并追加到列表
- 对照答案：e19_background_tasks_answer/