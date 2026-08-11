# 练习 E40: Vite 与 HMR

**知识点：** `vite.config.ts` 的 Tauri 集成 / `devUrl` 与端口一致 / `strictPort` / `TAURI_DEV_HOST` / HMR / `watch.ignored`

## TODO（练习版）

按注释提示补全：

1. `vite.config.ts`：`port: 0` → 改为 1498（与 devUrl 一致）
2. `vite.config.ts`：取消注释 `strictPort: true` 并说明作用
3. `vite.config.ts`：取消注释 `hmr` 配置并说明 TAURI_DEV_HOST 场景
4. `src/main.ts`：展示 `import.meta.env.MODE` / `PROD`（DEV 已作为参考保留）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 本练习无 Rust 命令，纯 vite 配置 + 前端信息页
- `devUrl`（tauri.conf.json `build.devUrl`）必须与 vite server `port` 一致，否则 tauri dev 加载不到页面
- `strictPort: true` 端口被占用时报错退出，而不是自动换端口
- `TAURI_DEV_HOST` 有值时 Vite 监听局域网地址，HMR 绑定同一主机（专用端口 1421）
- 对照答案: `e40_vite_hmr_answer/`

## 信息

- devUrl: http://localhost:1498
- identifier: com.taurilearn.e40