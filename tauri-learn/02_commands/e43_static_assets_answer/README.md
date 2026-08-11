# 练习 E43: 静态资源

**知识点：** `public/` 静态资源 / `src/assets` import 引用 / asset 协议 `convertFileSrc` / `resource_dir`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 页面展示 `public/tauri-logo.svg`（`/` 路径直接引用）
2. 点击「查看 resource 目录」→ invoke `resource_info` 显示 resource_dir 路径

## 说明

- `public/` 下资源打包时原样复制到 dist，直接用 `/路径` 引用
- `src/assets/` 下资源需 import 引用，打包时自动带内容 hash
- 大文件/动态文件用 asset 协议：`convertFileSrc(filePath)`（来自 `@tauri-apps/api/core`）把磁盘路径转成可加载 URL
- `resource_dir` 是打包后存放额外资源的位置，运行时用 `app.path().resource_dir()` 获取（需要 `use tauri::Manager`）

## 信息

- devUrl: http://localhost:1505
- identifier: com.taurilearn.e43a