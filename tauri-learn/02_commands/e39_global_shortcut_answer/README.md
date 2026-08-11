# 练习 E39: 全局快捷键

**知识点：** `tauri-plugin-global-shortcut` / `register` / `unregister` / `is_registered` / `with_handler` 事件回调 / `app.emit` 广播

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 输入组合键（默认 `Ctrl+Shift+Space`），点击「注册」
2. 按下该组合键 → 「按下日志」出现记录（应用失焦也生效）
3. 「查询状态」查看是否已注册；「注销」后按键不再触发
4. 注册与系统冲突的快捷键（如 `Ctrl+C`）→ 观察注册失败的错误展示

## 说明

- 全局快捷键由操作系统层处理，应用失焦/最小化时依然生效
- 与系统或其他应用冲突的快捷键注册失败，错误会经 invoke 返回给前端
- 后端 `with_handler` 回调中判断 `ShortcutState::Pressed`，再用 `app.emit("shortcut-pressed", ...)` 广播
- 前端 `listen("shortcut-pressed")` 接收并展示日志
- 组合键写法：`Ctrl+Shift+Space`、`Alt+1`、`CmdOrCtrl+P`（跨平台修饰键）

## 信息

- devUrl: http://localhost:1497
- identifier: com.taurilearn.e39a