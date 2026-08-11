# 练习 E42: 主题切换

**知识点：** CSS 变量 / `prefers-color-scheme` / `matchMedia` / `localStorage`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 点击「跟随系统 / 浅色 / 深色」切换主题
2. 选择「跟随系统」后，切换系统深浅色 → 页面实时跟随
3. 重启应用 → 仍是上次的选择（localStorage 记忆）

## 说明

- 本练习无 Rust 命令，纯前端主题切换
- `:root` 定义浅色变量，`[data-theme='dark']` 覆盖深色变量；页面颜色全部引用 `var(--xxx)`
- `matchMedia("(prefers-color-scheme: dark)")` 检测系统主题；change 事件监听系统切换
- 切换本质是设置 `<html>` 的 `data-theme` 属性，无需重新加载页面

## 信息

- devUrl: http://localhost:1503
- identifier: com.taurilearn.e42a