# 练习 E13: 命令模块化

**知识点：** commands/ 目录拆分 / 子模块定义命令 / 跨模块注册

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs、src-tauri/src/commands/ 与 src/main.ts 中的 TODO 填空
- 命令按领域拆分到 commands/ 子模块（math.rs / text.rs），lib.rs 用完整路径注册
- 前端仍按命令名（add / to_upper 等）调用
- 对照答案：e13_command_modules_answer/