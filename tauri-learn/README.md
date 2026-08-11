# Tauri v2 练习项目

共 **84 个练习**（每练习含练习版 + 答案版，共 168 个独立项目），按四大块组织：

| 块 | 目录 | 题量 | 前端 |
| :-: | ---- | :--: | :--: |
| 入门 | 01_getting_started/ | 8 | Vanilla TS + Vite |
| 基本命令和语法 | 02_commands/ | 40 | Vanilla TS + Vite |
| 简单项目 | 03_simple_projects/ | 10 | Vanilla TS + Vite |
| 超级项目 | 04_super_project/ | 26 步 | React |

## 端口分配

全部项目端口按次序递增（练习版与答案版各占一个），从 1420 到 1587。完整端口表见 [00_preface/index.html](00_preface/index.html)。

## 运行单个练习

``bash
cd 02_commands/e10_dependency_injection
pnpm install
cargo tauri dev
``

## 目录约定

- 每个练习两个项目：练习版（NN_name，含 TODO）与答案版（NN_name_answer）
- 超级项目 26 步为串行递进（p01 → p26），每步依赖前一步代码
- 所有项目已预注册为 Cargo workspace members

## 相关文档

- 内容规划: .trae/documents/tauri-learn-plan.md
- Agent 编写流水线: .trae/documents/tauri-learn-agent-plan.md