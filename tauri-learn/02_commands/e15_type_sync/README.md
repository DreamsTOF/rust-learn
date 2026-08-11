# 练习 E15: 前后端类型同步

**知识点：** TS 接口定义 / invoke&lt;T&gt;() 泛型 / snake_case ↔ camelCase 自动转换

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- Rust 端 `user_id` / `display_name` 序列化后自动转为 JS 的 `userId` / `displayName`
- 前端用 `invoke&lt;UserProfile&gt;` 泛型获得类型提示，渲染 JSON 观察字段名
- 对照答案：e15_type_sync_answer/