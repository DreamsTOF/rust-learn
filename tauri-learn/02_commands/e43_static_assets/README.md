# 练习 E43: 静态资源

**知识点：** `public/` 静态资源 / `src/assets` import 引用 / asset 协议 `convertFileSrc` / `resource_dir`

## TODO（练习版）

按注释提示补全：

1. `index.html`：补全 `<img>` 的 `src="/tauri-logo.svg"`（public 目录资源）
2. `src-tauri/src/lib.rs`：补全 `resource_info` 命令（`app.path().resource_dir()`）
3. `src-tauri/src/lib.rs`：注册 `resource_info` 到 `generate_handler!`
4. `src/main.ts`：按钮点击后 `invoke("resource_info")` 并展示

（`convertFileSrc` 的 asset 协议用法已作为注释示例保留在 `src/main.ts` 与 `index.html` 说明卡片中）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- `public/` 下资源打包时原样复制到 dist，直接用 `/路径` 引用
- `src/assets/` 下资源需 import 引用，打包时自动带内容 hash
- 大文件/动态文件用 asset 协议：`convertFileSrc(filePath)`（来自 `@tauri-apps/api/core`）把磁盘路径转成可加载 URL
- `resource_dir` 是打包后存放额外资源的位置，运行时用 `app.path().resource_dir()` 获取（需要 `use tauri::Manager`）
- 对照答案: `e43_static_assets_answer/`

## 信息

- devUrl: http://localhost:1504
- identifier: com.taurilearn.e43