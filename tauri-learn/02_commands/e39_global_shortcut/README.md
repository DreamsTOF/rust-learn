# 练习 E39: 全局快捷键

**知识点：** `tauri-plugin-global-shortcut` / `register` / `unregister` / `is_registered` / `with_handler` 事件回调 / `app.emit` 广播

## TODO（练习版）

按注释提示补全：

1. `with_handler` 回调：判断 `ShortcutState::Pressed` 并 `app.emit("shortcut-pressed", ...)`
2. `register_shortcut`：`Shortcut::from_str` + `register`
3. `unregister_shortcut`：`Shortcut::from_str` + `unregister`
4. `is_shortcut_registered`：`Shortcut::from_str` + `is_registered`
5. 前端 `listen("shortcut-pressed")` 展示按下日志 + 三个按钮的 `invoke`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 全局快捷键由操作系统层处理，应用失焦/最小化时依然生效
- 与系统或其他应用冲突的快捷键注册失败，错误会经 invoke 返回给前端
- 组合键写法：`Ctrl+Shift+Space`、`Alt+1`、`CmdOrCtrl+P`（跨平台修饰键）
- 对照答案: `e39_global_shortcut_answer/`

## 信息

- devUrl: http://localhost:1496
- identifier: com.taurilearn.e39