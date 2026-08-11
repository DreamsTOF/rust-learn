# 练习 E46: 自定义错误传播

## 知识点
- `thiserror` 错误枚举与 `Display` 消息
- 错误码映射：`AppError::code()` → 400/404/500
- 自定义 `Serialize`：错误序列化为 `{ code, message }` 对象
- 前端按 `code` 分类处理，避免猜测错误字符串

## 任务
1. `src-tauri/src/lib.rs`：
   - 补全 `AppError` 枚举的 `NotFound` / `Internal` 变体
   - 补全 `code()` 的 404 / 500 分支
   - 补全 `Serialize` 中 `ErrorBody` 的构造（`code: self.code()`）
   - 补全 `risky_operation` 的 `missing` / `boom` 分支
2. `src/main.ts`：
   - 补全 `codeMessages` 的 404 / 500 映射
   - 补全 `invokeWithError` 的 invoke 调用与 catch 展示（错误码徽标 + 中文提示 + 原始信息）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照答案

`../e46_error_propagation_answer/`（devUrl: http://localhost:1511）