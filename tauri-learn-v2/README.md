# Tauri v2 练习项目（v2 直观版）

**每课一道菜**：每课一个完整可运行的小应用，知识在用到时讲、只讲新增、讲透为止。配套课程书在 [tauri-learn-book-v2/](../tauri-learn-book-v2/)。

| 阶段 | 目录 | 内容 | 前端 |
| :-: | ---- | ---- | :-: |
| 1 | 01_first_app/ | 环境 + 计数器（e01） | Vanilla TS + Vite |
| 2 | 02_mini_apps/ | 八道小菜（a01-a08） | a01 Vanilla，a02 起 React |
| 3 | 03_super_project/ | Markdown 编辑器（p01-p26） | React |

## 端口分配

全部项目端口按次序递增（练习版与答案版各占一个，答案版 +1）。完整端口表见 [00_preface/index.html](00_preface/index.html)。

## 运行单个练习

```bash
cd 01_first_app/e01_counter
pnpm install        # 或在工作区根目录执行一次
cargo tauri dev
```

## 目录约定

- 每个练习两个项目：练习版（`e01_counter`，含 TODO）与答案版（`e01_counter_answer`，完整代码）
- 所有项目已预注册为 Cargo workspace members
- 讲解文档：每课两个 mdbook 文档（练习讲解 + 答案讲解）

## 相关文档

- 计划: .trae/documents/tauri-learn-book-v2.md
