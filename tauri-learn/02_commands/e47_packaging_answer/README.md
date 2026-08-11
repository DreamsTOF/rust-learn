# 练习 E47: 打包发布（packaging）

## 知识点
- 打包元信息读取：`app.config().identifier` / `app.package_info()`
- 多平台产物：Windows（MSI/NSIS）、Linux（AppImage/deb/rpm）、macOS（DMG/AppBundle）
- `tauri build` 与 `--target` 交叉编译
- `cargo tauri icon` 生成全套图标（回顾 E08）
- 体积优化：release profile 的 lto / codegen-units / strip + 前端产物压缩
- 包名规范：productName 不含下划线

## 运行

```bash
pnpm install
cargo tauri dev
# 打包当前平台：
pnpm tauri build
```

## 对照

- devUrl: http://localhost:1513
- identifier: com.taurilearn.e47a
- 练习版: `../e47_packaging/`