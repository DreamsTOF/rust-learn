# 练习 E13: 命令模块化

**知识点：** commands/ 目录拆分 / 子模块定义命令 / 跨模块注册

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 命令按领域拆分到 src-tauri/src/commands/ 子模块（math.rs / text.rs）
- lib.rs 通过 `commands::math::add` 等完整路径注册
- 前端仍按命令名（add / to_upper 等）调用，模块路径只在 Rust 侧体现