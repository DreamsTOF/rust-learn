# 练习 E28: 系统托盘

知识点: TrayIconBuilder / 托盘菜单与事件 / CloseRequested 拦截隐藏到托盘

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 点 X 关闭 = 隐藏到托盘；左键单击托盘图标恢复；右键菜单：显示主窗口 / 退出
- 「退出应用」按钮调用 quit_app 命令真正退出
- identifier: com.taurilearn.e28a