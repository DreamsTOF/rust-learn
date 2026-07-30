 # 模块 01: 基础与环境
 
 ## 📋 模块概览
 
 本模块涵盖 Leptos 框架最基础的概念，共 **20 个练习**，从 Hello World 开始，逐步深入到组件系统、条件渲染、构建器模式等。
 
 | # | 题目 | 难度 | 核心知识点 |
 |--|------|------|-----------|
 | 01 | Hello World | ⭐ | `mount_to_body`, `view!` 宏, `#[component]` |
 | 02 | 文本节点 | ⭐ | 字符串文本 `"..."`、多行文本拼接 |
 | 03 | HTML 元素与属性 | ⭐ | `class`, `id`, `style`, `<a>`, `<img>` |
 | 04 | 元素嵌套 | ⭐ | `<div>`, `<section>`, 层级结构 |
 | 05 | 组件定义 | ⭐⭐ | 函数组件、`impl IntoView`、组件属性 |
 | 06 | 组件嵌套 | ⭐⭐ | `<Header/>`、`<Main/>`、`<Footer/>` 布局拆分 |
 | 07 | Fragment 语法 | ⭐⭐ | `<></>` 多根节点 |
 | 08 | 注释写法 | ⭐ | `view!` 内的 `/* */` 和 `//` 注释 |
 | 09 | Rust 表达式嵌入 | ⭐⭐ | `{ }` 块、变量插值、`format!` |
 | 10 | 块级表达式 | ⭐⭐ | `{ let x = 1; x + 2 }`、块内 if/else |
 | 11 | 条件 if | ⭐⭐⭐ | `{ if cond { "A" } else { "B" } }` |
 | 12 | match 匹配 | ⭐⭐⭐ | `{ match x { 1 => "一", _ => "其他" } }` |
 | 13 | 索引与方法调用 | ⭐⭐ | `{ items.len() }`, `{ items[0] }` |
 | 14 | 构建器模式初阶 | ⭐⭐⭐ | `div().child("text").on(ev::click, ...)` |
 | 15 | 浏览器开发者工具 | ⭐⭐ | WASM 调试、`tracing::info!`、console.log |
 | 16 | SVG 元素 | ⭐⭐ | `<svg>`、`<circle>`、`<rect>`、`<text>` |
 | 17 | 原始 HTML 渲染 | ⭐⭐ | `inner_html`、XSS 防范 |
 | 18 | Fragment 多根节点 | ⭐ | `view!` 中多个 Fragment 并列 |
 | 19 | 动态标签名 | ⭐⭐ | `leptos::html::h1`、`.into_any()` |
 | 20 | 构建器模式高级 | ⭐⭐⭐ | 纯构建器 API 完整组件 |
 
 ## 🎯 学习目标
 
 完成本模块后，你应该能够：
 
 - 使用 `view!` 宏创建声明式 UI
 - 编写 Leptos 函数组件并组合使用
 - 在 `view!` 中嵌入 Rust 表达式、条件、匹配
 - 使用构建器 API 替代 `view!` 宏
 - 通过 `tracing` 在浏览器控制台调试

## 📖 章节结构

每个练习包含**概念讲解**部分，专注解释本节核心知识点。

练习代码位于 `leptos-learn/01_basics/` 目录，使用 Cursor 编辑，trunk 自动编译预览。
