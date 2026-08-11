# 练习 E26: 无边框窗口

知识点: decorations: false / data-tauri-drag-region 拖拽 / 自定义标题栏最小化与关闭

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- tauri.conf.json 主窗口设置 `decorations: false`，系统标题栏消失
- 标题栏元素标 data-tauri-drag-region 可拖拽；按钮标 data-tauri-drag-region="false" 避免误拖
- capabilities 增加了 `core:window:allow-start-dragging` / `allow-minimize` / `allow-close`
- identifier: com.taurilearn.e26a