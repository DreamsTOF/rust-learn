# 练习 E15: 前后端类型同步

**知识点：** TS 接口定义 / invoke&lt;T&gt;() 泛型 / snake_case ↔ camelCase 自动转换

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- Rust 端 `user_id` / `display_name` 序列化后自动转为 JS 的 `userId` / `displayName`
- 前端用 `invoke&lt;UserProfile&gt;` 泛型获得类型提示，渲染 JSON 观察字段名