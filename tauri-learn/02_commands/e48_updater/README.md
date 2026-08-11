# 练习 E48: 自动更新（updater）

## 知识点
- 更新流程：版本检查 → 下载 → 安装 → 重启
- `tauri-plugin-updater` 集成：`.plugin(tauri_plugin_updater::Builder::new().build())`
- `plugins.updater` 配置：pubkey / endpoints
- endpoints 模板变量：`{{target}}` / `{{arch}}` / `{{current_version}}`
- pubkey 签名验证：未签名更新会被拒绝
- 命令：`app.updater()`（UpdaterExt trait）→ `updater.check()`

## 任务
1. `src-tauri/src/lib.rs`：
   - 补全 `check_update` 命令（`app.updater()` 获取实例 + `check()` 三分支）并注册
   - 取消注释 `use tauri_plugin_updater::UpdaterExt;`
2. `src/main.ts`：补全 invoke 调用与结果展示（成功 ✅ / 失败 ❌）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照答案

`../e48_updater_answer/`（devUrl: http://localhost:1515）