# 练习 E18: 路径 API

**知识点：** PathResolver（app.path()）/ app_data_dir 等目录解析 / 路径拼接

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- `app.path()` 返回 PathResolver，可解析 app_data_dir / app_config_dir / app_log_dir / app_cache_dir / resource_dir / temp_dir
- 前端表格渲染全部目录路径