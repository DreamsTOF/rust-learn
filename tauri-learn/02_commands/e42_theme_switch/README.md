# 练习 E42: 主题切换

**知识点：** CSS 变量 / `prefers-color-scheme` / `matchMedia` / `localStorage`

## TODO（练习版）

按注释提示补全：

1. `src/styles.css`：补全 `--btn-bg` 变量定义（浅色主题按钮主色）
2. `src/styles.css`：补全 `[data-theme='dark']` 中 `--bg` 覆盖（深色背景）
3. `src/main.ts`：补全 `applyTheme` 中 dark 的计算（matchMedia 检测系统主题）
4. `src/main.ts`：把 mode 写入 `localStorage`（记忆选择）
5. `src/main.ts`：初始化读取 localStorage（无则 auto）
6. `src/main.ts`：matchMedia change 监听（auto 模式实时跟随系统）
7. `src/main.ts`：绑定三个主题按钮的点击事件

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- 本练习无 Rust 命令，纯前端主题切换
- `:root` 定义浅色变量，`[data-theme='dark']` 覆盖深色变量；页面颜色全部引用 `var(--xxx)`
- `matchMedia("(prefers-color-scheme: dark)")` 检测系统主题；change 事件监听系统切换
- 对照答案: `e42_theme_switch_answer/`

## 信息

- devUrl: http://localhost:1502
- identifier: com.taurilearn.e42