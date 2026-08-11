# 练习 E14: 可变状态

**知识点：** manage() / State&lt;T&gt; / Mutex / RwLock / 多 State 并存

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- Counter 用 Mutex 保护，Tags 用 RwLock 保护，两个 State 同时注入
- 标签支持添加 / 清空 / 刷新列表
- 对照答案：e14_mutable_state_answer/