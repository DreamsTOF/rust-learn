# 练习 E35: 通知

**知识点：** 插件注册（Rust + capabilities）/ `isPermissionGranted` / `requestPermission` / `sendNotification` / `onAction` 点击监听

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 页面加载自动检查权限，徽标显示「已授权 / 未授权」
2. 未授权时点击「请求权限」→ 授权后显示「发送通知」按钮
3. 输入标题/正文 → 发送（Windows 上显示在系统通知中心）
4. 点击系统通知 → 前端日志区显示「通知被点击」

## 说明

- `onAction` 回调收到通知对象（含 title/body）；旧版回调带 `{ type: 'clicked' }` 字段，新版已改为通知对象
- 对比答案: `e35_notification_answer/`

## 信息

- devUrl: http://localhost:1488
- identifier: com.taurilearn.e35