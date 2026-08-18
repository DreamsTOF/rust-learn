# Tauri v2 实战练习（v2）

欢迎来到 **Tauri v2 实战练习**！这是一本通过动手做应用学习 [Tauri](https://tauri.app/) v2 桌面框架的教程。

## 这本书怎么教

**每课一道菜**：每一课都从一句真实需求出发，交付一个完整、能跑、能玩的小应用。知识在用到的时候讲——**只讲本课新增的部分，并且讲透**：每个新 API 都讲清"解决什么问题 → 怎么用 → 有哪几种写法 → 为什么选这种"。

## 每课两个文档

| 文档 | 什么时候读 | 讲什么 |
|------|-----------|--------|
| 练习讲解（`NN_topic.md`） | 先读 | 原理讲透 + 作业指引（动哪些文件、每处 TODO） |
| 答案讲解（`NN_topic_answer.md`） | 卡住时再看 | 逐 TODO 对照（练习版缺什么 → 答案版填了什么 → 为什么）+ 验收标准 + 破坏性验证 |

答案讲解每处 TODO 都有"回查文档"锚点，链回练习讲解的对应小节——两文档单向依赖，内容不重复。

## 练习版 / 答案版

每个练习对应两个独立项目（代码位于 `tauri-learn-v2/`，与本书同级）：

- **练习版**（如 `e01_counter`）：只留骨架 + 中文 TODO 注释，你来填
- **答案版**（如 `e01_counter_answer`）：完整代码

## 环境要求

- Rust（stable）
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/)
- Windows 需要 [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)（Windows 11 已内置）
- Tauri CLI（`cargo install tauri-cli` 或 `pnpm dlx tauri`）

## 快速开始

```bash
# 在 tauri-learn-v2 仓库根目录安装前端依赖（只需一次）
pnpm install

cd 01_first_app/e01_counter
cargo tauri dev
```

## 学习路径

```
阶段 1  环境 + 第一个窗口（计数器）        跑起来
阶段 2  八道小菜（a01-a08）                边做边学，越做越复杂
阶段 3  超级项目：Markdown 编辑器（26 步） 可安装分发的完整产品
```
