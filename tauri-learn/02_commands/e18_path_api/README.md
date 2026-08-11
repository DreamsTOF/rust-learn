# 练习 E18: 路径 API

**知识点：** PathResolver（app.path()）/ app_data_dir 等目录解析 / 路径拼接

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- `app.path()` 返回 PathResolver，可解析 app_data_dir / app_config_dir / app_log_dir / app_cache_dir / resource_dir / temp_dir
- 前端表格渲染全部目录路径
- 对照答案：e18_path_api_answer/