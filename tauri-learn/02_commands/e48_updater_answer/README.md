# 练习 E48: 自动更新（updater）

## 知识点
- 更新流程：版本检查 → 下载 → 安装 → 重启
- `tauri-plugin-updater` 集成：`.plugin(tauri_plugin_updater::Builder::new().build())`
- `plugins.updater` 配置：pubkey / endpoints
- endpoints 模板变量：`{{target}}` / `{{arch}}` / `{{current_version}}`
- pubkey 签名验证：未签名更新会被拒绝
- 命令：`app.updater()`（UpdaterExt trait）→ `updater.check()`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照

- devUrl: http://localhost:1515
- identifier: com.taurilearn.e48a
- 练习版: `../e48_updater/`