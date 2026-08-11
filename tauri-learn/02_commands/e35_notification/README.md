# 练习 E35: 通知

**知识点：** 插件注册（Rust + capabilities）/ `isPermissionGranted` / `requestPermission` / `sendNotification` / `onAction` 点击监听

## TODO（练习版）

在 `src/main.ts` 中按注释提示补全：

1. 权限状态变量 `granted`
2. 页面加载时 `isPermissionGranted()` 检查权限
3. `requestPermission()` 请求权限
4. `sendNotification({ title, body })` 发送通知
5. 权限状态徽标与按钮显隐更新
6. `onAction` 监听通知点击

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- `onAction` 回调收到通知对象（含 title/body）；旧版回调带 `{ type: 'clicked' }` 字段，新版已改为通知对象
- 对照答案: `e35_notification_answer/`

## 信息

- devUrl: http://localhost:1488
- identifier: com.taurilearn.e35