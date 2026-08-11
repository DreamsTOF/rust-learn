# 练习 E47: 打包发布（packaging）

## 知识点
- 打包元信息读取：`app.config().identifier` / `app.package_info()`
- 多平台产物：Windows（MSI/NSIS）、Linux（AppImage/deb/rpm）、macOS（DMG/AppBundle）
- `tauri build` 与 `--target` 交叉编译
- `cargo tauri icon` 生成全套图标（回顾 E08）
- 体积优化：release profile 的 lto / codegen-units / strip + 前端产物压缩
- 包名规范：productName 不含下划线

## 任务
1. `src-tauri/src/lib.rs`：
   - 补全 `BundleInfo` 结构体字段并派生 `serde::Serialize`
   - 补全 `bundle_info` 命令体（identifier / product_name / version / platform 的取值）并注册
2. `src/main.ts`：调用 `bundle_info` 并把四个字段渲染到表格

## 运行

```bash
pnpm install
cargo tauri dev
# 打包当前平台：
pnpm tauri build
```

## 对照答案

`../e47_packaging_answer/`（devUrl: http://localhost:1513）