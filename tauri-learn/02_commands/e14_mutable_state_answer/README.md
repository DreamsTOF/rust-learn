# 练习 E14: 可变状态

**知识点：** manage() / State&lt;T&gt; / Mutex / RwLock / 多 State 并存

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- Counter 用 Mutex 保护，Tags 用 RwLock 保护，两个 State 同时注入
- 标签支持添加 / 清空 / 刷新列表