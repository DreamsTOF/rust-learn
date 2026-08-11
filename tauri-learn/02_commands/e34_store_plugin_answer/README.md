# 练习 E34: Store 插件

**知识点：** 插件注册（Rust + capabilities）/ `load` / `set` / `get` / `has` / `delete` / `save` / `onChange` 监听

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 输入 key 和 value → 「set + save」写入并持久化（`autoSave: false` 时必须手动 `save()`）
2. 「get」读取（key 不存在时返回 `undefined`，按无值处理）
3. 「has」判断 key 是否存在；「delete + save」删除
4. 每次 set/delete 都会触发 `onChange` 回调，日志区显示变化（delete 时 value 为 `null`）
5. 重启应用后 `get` 仍能读回数据（数据保存在 `app_data_dir/settings.json`）

## 说明

- v2.2+ 中监听变化的 API 由 `watch` 更名为 `onChange`（按单个 key 监听用 `onKeyChange`）
- 对比答案: `e34_store_plugin_answer/`

## 信息

- devUrl: http://localhost:1486
- identifier: com.taurilearn.e34