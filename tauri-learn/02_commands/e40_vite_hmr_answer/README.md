# 练习 E40: Vite 与 HMR

**知识点：** `vite.config.ts` 的 Tauri 集成 / `devUrl` 与端口一致 / `strictPort` / `TAURI_DEV_HOST` / HMR / `watch.ignored`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 页面显示 `import.meta.env` 的 MODE / DEV / PROD 值
2. 修改 `index.html` 或 `src/main.ts` → 应用即时热更新，无需重启
3. 把端口改成已被占用的值（配合 strictPort）→ 观察启动报错而不是换端口
4. 局域网调试：设置 `TAURI_DEV_HOST`（如 `192.168.1.5`）后运行，手机可访问 devUrl

## 说明

- 本练习无 Rust 命令，纯 vite 配置 + 前端信息页
- `devUrl`（tauri.conf.json `build.devUrl`）必须与 vite server `port` 一致，否则 tauri dev 加载不到页面
- `strictPort: true` 端口被占用时报错退出，而不是自动换端口
- `TAURI_DEV_HOST` 有值时 Vite 监听局域网地址，HMR 绑定同一主机（专用端口 1421）
- `watch.ignored: ["**/src-tauri/**"]`：Rust 代码变化不触发前端热更新

## 信息

- devUrl: http://localhost:1499
- identifier: com.taurilearn.e40a