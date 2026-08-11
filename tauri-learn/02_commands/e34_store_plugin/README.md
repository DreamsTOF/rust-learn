# 练习 E34: Store 插件

**知识点：** 插件注册（Rust + capabilities）/ `load` / `set` / `get` / `has` / `delete` / `save` / `onChange` 监听

## TODO（练习版）

在 `src/main.ts` 中按注释提示补全：

1. 加载 store（`load("settings.json", { autoSave: false })`）
2. `set` + `save` 写入并持久化
3. `get` 读取与 `null`（实际为 `undefined`）处理
4. `has` 判断存在
5. `delete` + `save` 删除
6. `onChange` 监听变化（旧名 `watch`）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- v2.2+ 中监听变化的 API 由 `watch` 更名为 `onChange`（按单个 key 监听用 `onKeyChange`）
- 对照答案: `e34_store_plugin_answer/`

## 信息

- devUrl: http://localhost:1486
- identifier: com.taurilearn.e34