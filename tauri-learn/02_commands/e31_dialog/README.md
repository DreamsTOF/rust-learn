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

## 填空任务（src/main.ts）

1. open 带文件过滤器（txt/md）
2. open 取消时的 null 判断
3. open 多选 / 目录模式
4. save 保存对话框
5. ask 确认对话框（message 已保留作为参照）

对照答案: `../e31_dialog_answer/`