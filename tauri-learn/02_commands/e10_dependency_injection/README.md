# 练习 E10: 依赖注入

**知识点：** AppHandle / WebviewWindow / State&lt;T&gt; 多依赖组合注入

**版本：** 练习版（TODO 填空）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 完成 src-tauri/src/lib.rs 与 src/main.ts 中的 TODO 填空
- 查看注入信息：同时注入 AppHandle、WebviewWindow、State&lt;Counter&gt;
- 计数 +1：修改由 manage() 注入的共享状态
- 对照答案：e10_dependency_injection_answer/