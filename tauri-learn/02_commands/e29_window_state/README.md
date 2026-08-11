# 练习 E29: 窗口状态持久化（window_state）

## 知识点

- tauri-plugin-window-state 插件：窗口位置 / 大小 / 最大化状态自动保存与恢复
- `AppHandleExt::save_window_state(StateFlags::all())` 手动保存
- 状态文件 `.window-state.json` 位于 app_config_dir

## 运行

```bash
pnpm install
cargo tauri dev
```

## 填空任务

1. lib.rs：注册 window-state 插件
2. lib.rs：补全 save_window_state 命令（StateFlags::all()）
3. lib.rs：补全 clear_window_state 命令（删除状态文件）
4. main.ts：两个按钮的 invoke 调用

对照答案: `../e29_window_state_answer/`