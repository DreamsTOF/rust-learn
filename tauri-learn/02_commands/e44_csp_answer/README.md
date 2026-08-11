# 练习 E44: 内容安全策略（CSP）

## 知识点
- `app.security.csp` 配置（tauri.conf.json）
- `default-src` / `style-src` / `connect-src` / `img-src` 指令
- 内联样式与 `'unsafe-inline'`
- `connect-src` 必须包含 `ipc:` 与 `http://ipc.localhost` 才能 invoke
- 用命令读取配置：`app.config().app.security.csp`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照

- devUrl: http://localhost:1507
- identifier: com.taurilearn.e44a
- 练习版: `../e44_csp/`（csp 保持 null，由练习者自行配置）