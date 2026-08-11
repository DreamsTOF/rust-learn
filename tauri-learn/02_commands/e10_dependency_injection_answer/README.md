# 练习 E10: 依赖注入

**知识点：** AppHandle / WebviewWindow / State&lt;T&gt; 多依赖组合注入

**版本：** 答案版（完整代码）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 后端命令定义在 src-tauri/src/lib.rs，前端逻辑在 src/main.ts
- 查看注入信息：同时注入 AppHandle、WebviewWindow、State&lt;Counter&gt;
- 计数 +1：修改由 manage() 注入的共享状态