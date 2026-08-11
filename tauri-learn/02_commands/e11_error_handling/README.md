# 练习 E11: 错误处理

**知识点：** thiserror 错误枚举 / #[from] 错误链 / map_err / ? 传播

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- 解析数字：非法输入 → `AppError::InvalidInput`
- 读取文件：`?` 自动经 `#[from]` 把 io::Error 转为 `AppError::Io`
- 前端统一展示 `err: ${e}`（Tauri 会把 Err 序列化为字符串）
- 对照答案：e11_error_handling_answer/