# 练习 E46: 自定义错误传播

## 知识点
- `thiserror` 错误枚举与 `Display` 消息
- 错误码映射：`AppError::code()` → 400/404/500
- 自定义 `Serialize`：错误序列化为 `{ code, message }` 对象
- 前端按 `code` 分类处理，避免猜测错误字符串

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照

- devUrl: http://localhost:1511
- identifier: com.taurilearn.e46a
- 练习版: `../e46_error_propagation/`