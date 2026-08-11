# 练习 E27: 应用菜单

知识点: Menu / Submenu / MenuItem / CheckMenuItem / PredefinedMenuItem::separator / 快捷键 / on_menu_event

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 「文件」菜单：新建（Ctrl+N）/ 打开（Ctrl+O）/ 深色模式勾选项 / 退出（Ctrl+Q）
- 菜单事件通过 emit("menu-action") 通知前端展示
- identifier: com.taurilearn.e27a