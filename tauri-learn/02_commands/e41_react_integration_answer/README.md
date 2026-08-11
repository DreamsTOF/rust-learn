# 练习 E41: React 集成

**知识点：** React 挂载 / `useState` + `invoke` / 事件监听 hook 化（`useEffect`）/ 组件卸载时 `unlisten` / 后端 `Mutex` 管理状态

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 点击「+1」→ 后端 `Counter(Mutex<u32>)` 自增，返回新值并广播 `counter-changed` 事件
2. 前端收到事件（或 invoke 返回值）后 `setCount` 更新，React 重渲染
3. 「重置」→ 后端计数置 0
4. 观察 `useEffect` cleanup：组件卸载时调用 `unlisten()` 取消监听

## 说明

- 前端改为 React：`main.tsx` 入口挂载 `App`，`App.tsx` 用 `useState` 驱动 UI
- 事件监听封装进 `useEffect`，cleanup 返回 `unlisten` 防止监听器泄漏（React StrictMode 开发模式会挂载两次，代码已处理）
- 后端每次变更后 `app.emit("counter-changed", ...)` 广播，前端 `listen` 同步
- 依赖：`react` / `react-dom` / `@vitejs/plugin-react`（package.json），`vite.config.ts` 加 `react()` 插件，`tsconfig.json` 加 `"jsx": "react-jsx"`
- 入口文件由 `src/main.ts` 改为 `src/main.tsx`

## 信息

- devUrl: http://localhost:1501
- identifier: com.taurilearn.e41a