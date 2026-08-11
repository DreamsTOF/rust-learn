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

## 操作验证

1. 拖动窗口、调整大小、最大化
2. 点击「保存窗口状态」
3. 完全退出应用，再次 `cargo tauri dev` 启动，窗口自动恢复
4. 点击「清除已保存的状态」后再启动，窗口回到默认位置

对照练习版: `../e29_window_state/`