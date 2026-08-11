# 练习 E41: React 集成

**知识点：** React 挂载 / `useState` + `invoke` / 事件监听 hook 化（`useEffect`）/ 组件卸载时 `unlisten` / 后端 `Mutex` 管理状态

## TODO（练习版）

在 `src/App.tsx` 中按注释提示补全：

1. `useState` 声明 `count` / `setCount`（当前为本地占位）
2. `useEffect` 中 `listen("counter-changed")` 并返回 `unlisten` cleanup
3. 「+1」按钮：`invoke("increment")` 并把返回值写入状态
4. 「重置」按钮：`invoke("reset_counter")` 并把返回值写入状态
5. 渲染处显示 `count` 的值

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 前端为 React：`main.tsx` 入口挂载 `App`；Rust 侧与答案版一致（`Counter(Mutex)` + `increment`/`reset_counter`，每次变更广播 `counter-changed`）
- 事件监听封装进 `useEffect`，cleanup 返回 `unlisten` 防止监听器泄漏
- 依赖：`react` / `react-dom` / `@vitejs/plugin-react`（package.json），`vite.config.ts` 加 `react()` 插件，`tsconfig.json` 加 `"jsx": "react-jsx"`
- 入口文件由 `src/main.ts` 改为 `src/main.tsx`
- 对照答案: `e41_react_integration_answer/`

## 信息

- devUrl: http://localhost:1500
- identifier: com.taurilearn.e41