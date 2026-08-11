# Tauri 实战练习

欢迎来到 **Tauri 实战练习**！这是一本通过动手练习学习 [Tauri](https://tauri.app/) v2 桌面应用框架的互动教程。

## 关于本书

本书采用 **练习→解答** 模式：

1. 每节先讲解核心概念
2. 然后给出带有 TODO 提示的练习模板
3. 最后提供完整的参考答案

建议先独立完成练习，再对照答案巩固理解。

## 模块结构

| 模块 | 内容 | 练习数 |
|------|------|--------|
| 01 入门 | 环境准备、项目结构、运行构建、命令、窗口、调试、打包 | 8 |
| 02 命令 | 异步命令、状态、事件、窗口操作、插件 API | — |
| 03 简单项目 | 综合小项目 | — |
| 04 超级项目 | 串行递进的完整应用（React） | — |

> 更多模块持续更新中...

## 环境要求

- Rust（建议最新 stable）
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/)（包管理器）
- Windows 需要 [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) 运行时（Windows 11 已内置）
- Tauri CLI（`cargo install tauri-cli` 或通过 npm 安装 `@tauri-apps/cli`）

## 快速开始

每个练习都是一个独立的 Tauri 项目，可直接运行：

```bash
# 在 tauri-learn 仓库根目录安装前端依赖（只需一次）
pnpm install

cd 01_getting_started/e01_hello_world
cargo tauri dev
```

练习代码位于 `tauri-learn/01_getting_started/` 目录（与本书同级的练习仓库），每个练习同时提供练习版与答案版两个项目。