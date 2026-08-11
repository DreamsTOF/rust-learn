# 练习 E20: 单实例

**知识点：** tauri-plugin-single-instance / 重复启动拦截回调 / 共享状态

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- `tauri_plugin_single_instance::init` 注册拦截回调：重复启动时打印参数/工作目录并聚焦已有 main 窗口
- `InstanceId` 以 State 注入，命令 `get_instance_id` 返回进程 ID
- 测试：运行中再次 `cargo tauri dev`（或运行 exe），新进程被拦截，读取到的仍是首个进程的实例 ID