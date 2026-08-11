# 练习 E11: 错误处理

**知识点：** thiserror 错误枚举 / #[from] 错误链 / map_err / ? 传播

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- 解析数字：非法输入 → `AppError::InvalidInput`
- 读取文件：`?` 自动经 `#[from]` 把 io::Error 转为 `AppError::Io`
- 前端统一展示 `err: ${e}`（Tauri 会把 Err 序列化为字符串）