# 模块 01: 入门

## 📋 模块概览

本模块带你从零走通 Tauri 开发的完整闭环：环境准备、项目结构、运行构建、第一个命令、参数传递、窗口配置、调试与打包，共 **8 个练习**。

| # | 题目 | 难度 | 核心知识点 |
|--|------|------|-----------|
| 01 | 环境准备与项目创建 | ⭐ | `#[tauri::command]`、`invoke()`、`serde::Serialize` 结构体返回 |
| 02 | 项目结构 | ⭐ | `src/` 与 `src-tauri/` 分工、`lib.rs` vs `main.rs` |
| 03 | 运行与构建 | ⭐⭐ | `tauri dev` / `tauri build`、`devUrl` / `frontendDist`、AppHandle 注入 |
| 04 | 第一个命令 | ⭐⭐ | 命令定义、注册、`invoke()` 传参与错误处理 |
| 05 | 参数与返回值 | ⭐⭐⭐ | 多类型参数、`Deserialize` / `Serialize`、snake_case ↔ camelCase |
| 06 | 窗口配置 | ⭐⭐⭐ | `tauri.conf.json` 主窗口、`WebviewWindowBuilder` 多窗口 |
| 07 | 调试 | ⭐ | Web Inspector、`println!` 终端日志、`console.log` |
| 08 | 打包与图标 | ⭐⭐ | bundle 产物、identifier 规范、`cargo tauri icon` |

## 🎯 学习目标

完成本模块后，你应该能够：

- 理解 Tauri 的双进程架构（WebView 前端 + Rust 核心）
- 编写并注册 `#[tauri::command]` 命令
- 在前端通过 `invoke()` 调用后端命令，传递参数并处理返回值
- 使用 `cargo tauri dev` 运行应用、`cargo tauri build` 打包应用
- 理解 Tauri 项目结构与关键配置文件

## 📖 章节结构

每个练习包含**概念讲解**部分，专注解释本节核心知识点。

练习代码位于 `tauri-learn/01_getting_started/` 目录（与本书同级的练习仓库），每个练习提供练习版（含 TODO）与答案版（完整代码）两个项目，答案版在练习版端口基础上 +1、identifier 带 `a` 后缀，可同时运行互不冲突。