# 练习 E31: 对话框（dialog）

## 知识点

- @tauri-apps/plugin-dialog：open / save / ask / message
- open 的 filters 文件过滤器、multiple 多选、directory 目录模式
- 取消处理：open/save 返回 null，ask 返回 false

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作验证

1. 打开文件（txt/md 过滤器）→ 取消对话框观察 "已取消"
2. 多选文件、选择目录
3. 保存为（只返回路径，不写文件）
4. ask 确认（warning 样式）、message 消息

对照练习版: `../e31_dialog/`