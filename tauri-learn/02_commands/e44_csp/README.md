# 练习 E44: 内容安全策略（CSP）

## 知识点
- `app.security.csp` 配置（tauri.conf.json）
- `default-src` / `style-src` / `connect-src` / `img-src` 指令
- 内联样式与 `'unsafe-inline'`
- `connect-src` 必须包含 `ipc:` 与 `http://ipc.localhost` 才能 invoke
- 用命令读取配置：`app.config().app.security.csp`

## 任务
1. 补全 `src-tauri/src/lib.rs` 的 `get_csp` 命令并注册
2. 补全 `src/main.ts` 的 invoke 与展示逻辑
3. 补全 `index.html` 说明卡片（对照答案版）
4. 把 CSP 字符串配置到 `tauri.conf.json` 的 `app.security.csp`（答案版配置见对照）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照答案

`../e44_csp_answer/`（devUrl: http://localhost:1507）