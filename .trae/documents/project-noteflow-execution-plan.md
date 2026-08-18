# NoteFlow — 实时协作知识库 执行规划

## 1. 项目概述

| 维度 | 说明 |
|------|------|
| **项目名称** | NoteFlow — 支持 Markdown 实时协作、离线编辑、多端同步的现代知识库 |
| **技术栈** | Leptos 0.9 (nightly) + Thaw UI + leptos_router + leptos-use + WebSocket + IndexedDB + PWA + Y.js (CRDT) |
| **构建方式** | Trunk (CSR 为主) + cargo-leptos (SSR 分享页) |
| **步数** | 40 步，分 8 个阶段 |
| **执行模式** | **单 agent 串行**（强依赖链，每步依赖前一步代码） |
| **目录位置** | 练习: `leptos-learn/projects/noteflow/` — 答案: `leptos-learn/projects/noteflow_answer/` |
| **预计耗时** | ~6-8h 墙钟时间 |

### 1.1 练习/答案双文件夹结构

每个终极项目分两个独立文件夹，遵循与章节练习题一致的规范：

```
leptos-learn/projects/
├── noteflow/              # 练习项目（含 TODO，供学员逐步完成）
│   ├── Cargo.toml
│   ├── src/...
│   └── ...
└── noteflow_answer/       # 参考答案项目（完整可编译运行，无 TODO）
    ├── Cargo.toml
    ├── src/...
    └── ...
```

**核心原则：**
- **练习文件夹**：Agent 递进式编写，每步增量添加代码。最终结果是一个**包含 TODO 引导的练习项目**，学员按照 TODO 提示逐步补全。
- **答案文件夹**：40 步全部完成后，生成一个**完整自洽的独立项目**。`trunk build` 零错误零警告，可直接编译运行，不含任何 TODO。答案项目是练习项目的"最终形态参照"。
- 两个文件夹的内容在 workspace `Cargo.toml` 中**分别注册**为独立 member。

---

## 2. 核心依赖链

```
B-01(脚手架) → B-02(树) → B-03(编辑器) → B-04(高亮) → B-05(IndexedDB)
                                                          ↓
B-16(注册) ← B-15(收藏) ← B-14(筛选) ← B-13(搜索) ← B-12(分类) ← B-11(标签) ← B-10(导出) ← B-09(模板) ← B-08(拖拽) ← B-07(Tab) ← B-06(CRUD)
    ↓                                                                                                                                 ↓
B-17(工作区) → B-18(成员) → B-19(权限) → B-20(动态) → B-21(WS连接) → B-22(光标) → B-23(Y.js) → B-24(实时同步) → B-25(离线) → B-26(版本) → B-27(批注)
                                                                                                                            ↓
                                                                                                      B-32(看板) ← B-31(主题) ← B-30(快捷键) ← B-29(通知) ← B-28(PWA)
                                                                                                          ↓
                                                                                                      B-33(统计) → B-34(测试) → B-35(部署)
                                                                                                                                  ↓
                                                                                                          B-36(ToC) → B-37(链接) → B-38(分享) → B-39(专注) → B-40(AI)
```

**最关键依赖链：** B-03 编辑器 → B-05 持久化 → B-06 CRUD → B-13 搜索 → B-21 WebSocket → B-23 Y.js → B-24 实时同步。这条链上任意一步有 Bug，后续协作功能全崩。

**关键风险点：**
- B-03 编辑器输入/预览不同步 → 所有编辑功能全废
- B-23/B-24 Y.js 多人协作 → 文字不能丢失（最难关卡）
- B-28 PWA → Service Worker 缓存策略决定离线体验

---

## 3. 编写执行流程

### 3.1 总体策略

**单 agent 串行执行，最终产出两份项目。** 

Agent 在两个文件夹中同步编写：
- **练习项目** (`projects/noteflow/`)：递进式增量开发，每步代码中保留 TODO 注释引导学员补全。
- **答案项目** (`projects/noteflow_answer/`)：同步写入完整可编译代码，不含任何 TODO，作为练习的最终参照。

每步在练习项目中先写入含 TODO 的骨架代码，编译通过后，再将该步对应的完整实现同步到答案文件夹。

### 3.2 每步执行标准流程

```
1. 阅读本规划中该 Step 的描述、前置依赖、核心知识点
2. 在练习项目 (noteflow/) 中增量开发，代码中保留 TODO 注释
3. 编译验证练习: cd projects/noteflow && trunk build
4. 编译通过后，将该步的完整实现（去掉 TODO，补全答案）写入答案项目 (noteflow_answer/)
5. 编译验证答案: cd projects/noteflow_answer && trunk build
6. 如编译失败：
   - 分析错误信息 → 修复代码
   - 重试编译，最多 5 次
7. 两个项目都编译通过后: git add + git commit
8. 进入下一步
```

> **B-35 及 SSR 相关步骤：** 改用 `cargo leptos build` 验证。
> **注意：** 答案项目必须在 40 步全部完成后能作为一个完整项目独立编译运行，不得有未补全的 TODO 或缺失的模块。

### 3.3 分支策略

```
main
├── step-B-01  (脚手架)
├── step-B-02  (文档树)
├── step-B-03  (Markdown 编辑器)
├── ...        (增量提交)
└── step-B-40  (最终)
```

每步独立 commit。

---

## 4. 逐阶段详细规划

---

### 阶段 1：编辑器与文档核心（B-01 ~ B-05）

**目标：** 搭建 CSR 骨架，实现文档树侧边栏、Markdown 编辑器（分屏编辑+预览）、代码高亮+数学公式、IndexedDB 本地持久化。完成后可编辑文档、预览、刷新后恢复。

---

#### Step B-01: 项目初始化 + 文档路由

| 属性 | 内容 |
|------|------|
| **前置** | 无 |
| **难度** | ⭐ |
| **核心知识点** | Trunk CSR 脚手架、`<Router/>` 嵌套布局、Thaw UI 安装 |

**执行清单：**

1. 在 `leptos-learn/projects/noteflow/` 下创建 Trunk 项目：
   ```
   projects/noteflow/
   ├── Cargo.toml
   ├── index.html
   ├── Trunk.toml
   └── src/
       ├── main.rs     # mount_to_body
       ├── app.rs      # 根组件 + 路由
       └── lib.rs      # 模块导出
   ```
2. 在 workspace `Cargo.toml` 的 `members` 中注册 `projects/noteflow`
3. 配置依赖：`leptos`、`leptos_router`、`thaw`、`leptos-use`、`serde`、`serde_json`
4. 在 `app.rs` 配置路由骨架：
   ```rust
   <Router>
       <Routes>
           <Route path="" view=Layout>
               // 子路由占位
           </Route>
       </Routes>
   </Router>
   ```
5. **验证：** `trunk build` 零错误零警告

**产出：** 可编译运行的 CSR 空项目

---

#### Step B-02: 文档树侧边栏

| 属性 | 内容 |
|------|------|
| **前置** | B-01 |
| **难度** | ⭐⭐ |
| **核心知识点** | Thaw `<Tree/>` 递归组件、树形数据 Signal、折叠/展开状态 |

**执行清单：**

1. 定义文档节点数据结构：
   ```rust
   #[derive(Clone, Serialize, Deserialize)]
   struct DocNode {
       id: String,         // UUID
       title: String,
       is_folder: bool,
       children: Vec<DocNode>,
       parent_id: Option<String>,
       created_at: i64,
       updated_at: i64,
   }
   ```
2. 创建 `src/components/sidebar.rs`：
   - 使用 Thaw `<Tree/>` 递归渲染文档树
   - 根节点：工作区名称
   - 文件夹节点：可展开折叠
   - 文档节点：图标 + 标题
3. 创建树管理 Hook `use_doc_tree()`：
   - `treedata: RwSignal<Vec<DocNode>>`
   - `selected_doc_id: RwSignal<Option<String>>`
   - `expand_node(id)` / `collapse_node(id)`
4. 点击文档节点 → 设置 `selected_doc_id` → 路由跳转到 `/doc/:id`
5. 在 Layout 中组合：左侧侧边栏 + 右侧内容区
6. **验证：** 树可展开折叠，点击节点路由跳转正确

**产出：** 文档树侧边栏

---

#### Step B-03: Markdown 编辑器（受控）

| 属性 | 内容 |
|------|------|
| **前置** | B-02 |
| **难度** | ⭐⭐ |
| **核心知识点** | 受控 `<textarea/>` + 实时预览（`comrak` WASM）、分屏布局 |

**执行清单：**

1. 添加 `comrak` 依赖（带 WASM 兼容配置）
2. 创建编辑器页 `src/pages/editor.rs`：
   - 左侧：`<textarea/>` 编辑区（受控，绑定到文档内容的 Signal）
   - 右侧：HTML 预览区（通过 `comrak` 渲染 Markdown → 设置 `inner_html`）
3. 分屏布局：CSS Grid 或 flexbox，支持拖拽调整比例
4. Markdown 渲染管道：
   ```rust
   fn render_markdown(md: &str) -> String {
       comrak::markdown_to_html(md, &comrak::ComrakOptions::default())
   }
   ```
5. 预览区使用 `dangerous_inner_html`（因为编译后的 HTML 是可信的）
6. 编辑器高度占满可视区域（`calc(100vh - header_height)`）
7. **验证：** 左侧输入 Markdown，右侧实时预览，标题/列表/代码块渲染正确

**产出：** Markdown 编辑器（分屏编辑 + 实时预览）

---

#### Step B-04: 代码高亮 + 数学公式

| 属性 | 内容 |
|------|------|
| **前置** | B-03 |
| **难度** | ⭐⭐ |
| **核心知识点** | 语法高亮、KaTeX WASM 渲染 |

**执行清单：**

1. 代码高亮：
   - 添加 `syntect` 依赖（纯 Rust 语法高亮库）
   - 在 `render_markdown` 中启用 `comrak` 的 syntax highlighting 插件
   - 配置高亮主题（如 Monokai / GitHub）
   - CSS 样式覆盖代码块背景和行号
2. 数学公式：
   - 添加 `katex` 依赖或使用 `comrak` 的数学扩展
   - 渲染 `$$...$$` 块级公式和 `$...$` 行内公式
   - 加载 KaTeX CSS（内联在 index.html 或从 CDN）
3. 扩展 `render_markdown()` 的 ComrakOptions：
   ```rust
   let mut options = ComrakOptions::default();
   options.extension.strikethrough = true;
   options.extension.table = true;
   options.extension.tasklist = true;
   options.extension.footnotes = true;
   options.render.syntax_highlighting = true;
   ```
4. **验证：** 代码块 ` ```rust ` 语法高亮正确；`$E=mc^2$` 渲染为数学公式

**产出：** 代码高亮 + 数学公式支持

---

#### Step B-05: 本地持久化 IndexedDB

| 属性 | 内容 |
|------|------|
| **前置** | B-04 |
| **难度** | ⭐⭐ |
| **核心知识点** | leptos-use `use_indexed_db`、自动保存（`watch` + debounce）、草稿恢复 |

**执行清单：**

1. 使用 leptos-use 的 `use_indexed_db` Hook
2. 创建 `use_doc_storage()` Hook：
   - `save_doc(doc_id, content)` — 保存到 IndexedDB
   - `load_doc(doc_id) -> Option<String>` — 读取
   - `delete_doc(doc_id)` — 删除
   - `list_docs() -> Vec<DocMeta>` — 列出所有文档
3. 自动保存机制：
   - 使用 `watch` 监听编辑器内容变化
   - 配合 `use_debounce` 延迟 500ms 自动保存
   - 保存状态指示器（"已保存" / "保存中..." / "保存失败"）
4. 启动时自动恢复上次打开的文档
5. 错误处理：IndexedDB 写入失败时提示用户
6. **验证：** 编辑文档 → 关闭页面 → 重新打开 → 内容恢复；草稿自动保存

**产出：** IndexedDB 本地持久化 + 自动保存

---

### 阶段 2：文档管理（B-06 ~ B-10）

**目标：** 实现文档 CRUD、多标签页、拖拽排序、模板、导入导出。

---

#### Step B-06: 文档 CRUD

| 属性 | 内容 |
|------|------|
| **前置** | B-05 |
| **难度** | ⭐⭐ |
| **核心知识点** | 新建/删除/重命名、Thaw `<Modal/>` 确认弹窗、快捷键支持 |

**执行清单：**

1. 在侧边栏添加操作：
   - "新建文档"按钮 → 创建新 DocNode，路由跳转到新文档
   - "新建文件夹"按钮 → 创建文件夹节点
   - 右键菜单（上下文菜单）：重命名、删除、复制链接
2. 重命名：`<Modal/>` 弹窗 + `<Input/>` 确认
3. 删除：`<Modal/>` 二次确认（"确定删除 [文档名]？此操作不可撤销。"）
4. 快捷键：
   - `Ctrl+N` → 新建文档
   - `Ctrl+Shift+N` → 新建文件夹
   - `F2` → 重命名选中项
   - `Delete` → 删除选中项
5. 更新文档树 Signal 和 IndexedDB，确保前后端同步
6. **验证：** 删除文档后树更新；重命名后标题更新；快捷键生效

**产出：** 文档 CRUD 功能

---

#### Step B-07: 多标签页编辑

| 属性 | 内容 |
|------|------|
| **前置** | B-06 |
| **难度** | ⭐⭐ |
| **核心知识点** | Tab 组件 + URL 路由同步、Tab 关闭/切换/拖拽排序 |

**执行清单：**

1. 创建 Tab 管理 Hook `use_tabs()`：
   - `open_tabs: RwSignal<Vec<TabInfo>>`（打开的标签页列表）
   - `active_tab_id: RwSignal<Option<String>>`
   - `open_tab(doc_id)` — 打开/切换到标签页
   - `close_tab(doc_id)` — 关闭标签页
   - `close_other_tabs(doc_id)` — 关闭其他
   - `close_all_tabs()`
2. 编辑器区域顶部渲染 Tab 栏：
   - 每个 Tab 显示文档标题 + 关闭按钮 ×
   - 激活的 Tab 高亮
   - 未保存的 Tab 显示圆点标记 ●
   - 拖拽排序（HTML5 Drag & Drop）
3. Tab 关闭时检查是否有未保存内容，弹出提示
4. Tab 切换时 URL 同步到 `/doc/:id`
5. **验证：** 点击不同文档 → 新 Tab 打开；关闭 Tab 不丢失数据；拖拽排序正常

**产出：** 多标签页编辑器

---

#### Step B-08: 目录拖拽排序

| 属性 | 内容 |
|------|------|
| **前置** | B-07 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | HTML5 Drag & Drop API、`ondragstart`/`ondragover`/`ondrop`、树结构更新 |

**执行清单：**

1. 给文档树节点添加 HTML5 Drag & Drop 事件处理：
   - `ondragstart`：设置拖拽数据（节点 ID）
   - `ondragover`：显示拖放指示线（before/after/inside）
   - `ondrop`：重新计算树结构并更新 Signal
2. 拖放目标高亮（插入位置指示）
3. 拖放逻辑：
   - 拖到文件夹上 → 移入该文件夹
   - 拖到两个节点之间 → 插入到该位置
   - 不能将父文件夹拖入子节点（循环引用检测）
4. 拖放完成后自动保存新的树结构到 IndexedDB
5. **验证：** 拖拽节点到不同位置后树结构正确更新；父子关系正确；刷新后保持

**产出：** 目录拖拽排序

---

#### Step B-09: 文档模板

| 属性 | 内容 |
|------|------|
| **前置** | B-08 |
| **难度** | ⭐⭐ |
| **核心知识点** | 预置模板、模板变量替换 |

**执行清单：**

1. 创建模板数据文件 `src/templates/mod.rs`：
   ```rust
   struct Template {
       name: String,
       icon: &'static str,
       content: &'static str,
   }
   ```
2. 预置模板：
   - **会议记录**：日期、参会人、议题、决议、待办项
   - **周报**：本周工作、下周计划、风险与问题
   - **需求文档**：背景、目标、功能描述、验收标准
   - **空白文档**：空 Markdown
3. 新建文档时弹出模板选择面板（`<Modal/>` + 模板卡片）
4. 模板中的 `{{变量}}` 自动替换为当前日期等信息
5. **验证：** 选择"周报"模板 → 新建文档预填周报结构 → 变量已替换

**产出：** 文档模板系统

---

#### Step B-10: 导入/导出

| 属性 | 内容 |
|------|------|
| **前置** | B-09 |
| **难度** | ⭐⭐ |
| **核心知识点** | Markdown 文件导入拖拽上传、导出 `.md`/`.pdf` 文件下载 |

**执行清单：**

1. **导入 Markdown：**
   - 侧边栏或空白区域支持拖拽 `.md` 文件
   - 使用 FileReader API 读取文件内容
   - 解析文件名作为文档标题
   - 创建新文档并填入内容
2. **导出 Markdown：**
   - 编辑器菜单 → 导出 → 下载 `.md` 文件
   - 文件名 = 文档标题 + `.md`
   - 内容 = 原始 Markdown（非预览 HTML）
3. **导出 PDF（可选）：**
   - 使用 `window.print()` 触发浏览器打印为 PDF
   - 特殊 CSS `@media print` 样式
4. **验证：** 拖入 `.md` 文件 → 新文档创建且内容正确；导出的 `.md` 文件与原编辑内容一致

**产出：** Markdown 导入/导出

---

### 阶段 3：组织与检索（B-11 ~ B-15）

**目标：** 实现标签系统、分类与嵌套文件夹、全文搜索、高级筛选、最近访问与收藏。

---

#### Step B-11: 标签系统

| 属性 | 内容 |
|------|------|
| **前置** | B-10 |
| **难度** | ⭐⭐ |
| **核心知识点** | 标签 CRUD、Thaw `<Tag/>` `<Select/>` 多选、标签颜色 |

**执行清单：**

1. 扩展 DocNode 数据结构添加 `tags: Vec<String>` 字段
2. 标签管理 UI（编辑器顶部或侧边栏）：
   - 现有标签列表（`<Tag/>` 组件）
   - 添加标签：`<Select/>` 多选 + 输入创建新标签
   - 标签颜色：预设色板随机分配
3. 全局标签管理 Hook `use_tags()`：
   - `all_tags: RwSignal<Vec<TagInfo>>`
   - `add_tag(doc_id, tag)` / `remove_tag(doc_id, tag)`
4. 标签筛选侧边栏：点击标签 → 显示该标签下的所有文档
5. 标签保存到 IndexedDB
6. **验证：** 文档添加标签后标签筛选可找到该文档；删除标签不影响文档

**产出：** 标签系统

---

#### Step B-12: 分类 + 嵌套文件夹

| 属性 | 内容 |
|------|------|
| **前置** | B-11 |
| **难度** | ⭐⭐ |
| **核心知识点** | 文件夹树、移动文档、`<Breadcrumb/>` 面包屑 |

**执行清单：**

1. 文件夹功能已在 B-02 树下有基础，此步强化：
   - 文件夹右键菜单：新建子文档、新建子文件夹、重命名、删除（检查非空）
   - "移动到..."功能：右键文档 → 选择目标文件夹
2. 面包屑导航：编辑器顶部显示路径
   ```
   工作区 > 项目文档 > 技术方案 > [当前文档]
   ```
   每级可点击跳转
3. 文件夹删除前检查：
   - 如有子文档 → 提示"此文件夹含有 N 个文档，确定删除？"
   - 删除后子文档移至根目录或回收站
4. **验证：** 多级文件夹创建正常；移动文档后面包屑更新；删除文件夹不丢失文档

**产出：** 嵌套文件夹 + 面包屑

---

#### Step B-13: 全文搜索

| 属性 | 内容 |
|------|------|
| **前置** | B-12 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `use_debounce` + IndexedDB 全文索引、搜索结果高亮 |

**执行清单：**

1. 在顶部栏添加搜索框（`Cmd+K` 或 `Ctrl+K` 快捷键弹出）
2. 搜索逻辑：
   - 输入关键词 → `use_debounce` 300ms 防抖
   - 遍历 IndexedDB 中所有文档的标题和内容
   - 按匹配度排序：标题匹配 > 内容匹配；全匹配 > 部分匹配
3. 搜索结果面板（下拉列表）：
   - 每项显示：文档标题 + 匹配片段（截取关键词前后 50 字）
   - 关键词高亮（`<mark>` 标签）
   - 点击跳转到对应文档
4. 性能优化（文档量 > 100 时）：
   - 使用 IndexedDB 索引加速查询
   - 或使用 `fst` crate 构建本地搜索索引
5. **验证：** 搜索"Rust"匹配标题和正文中含"Rust"的文档；点击结果跳转正确

**产出：** 全文搜索

---

#### Step B-14: 高级筛选

| 属性 | 内容 |
|------|------|
| **前置** | B-13 |
| **难度** | ⭐⭐ |
| **核心知识点** | 标签/日期/类型组合筛选、保存筛选条件为"视图"、URL query |

**执行清单：**

1. 在侧边栏添加筛选面板：
   - 标签筛选（多选）
   - 类型筛选（文档/文件夹）
   - 日期筛选（今天/本周/本月/自定义范围）
   - 排序方式（按名称/按修改时间/按创建时间）
2. 筛选条件组合（AND 逻辑）
3. 保存筛选条件为"智能视图"：
   - 命名视图 → 保存筛选条件
   - 侧边栏显示保存的视图列表
   - 点击视图 → 应用筛选
4. 筛选状态同步到 URL query：`?tags=rust,leptos&sort=updated`
5. **验证：** 组合筛选后文档列表正确过滤；保存的视图可复现筛选结果

**产出：** 高级筛选 + 智能视图

---

#### Step B-15: 最近访问 + 收藏

| 属性 | 内容 |
|------|------|
| **前置** | B-14 |
| **难度** | ⭐⭐ |
| **核心知识点** | leptos-use `use_local_storage` 记录历史、星标收藏列表 |

**执行清单：**

1. **最近访问：**
   - 每次打开文档时记录到 localStorage（最多 20 条）
   - 侧边栏"最近访问"分组
   - 按访问时间倒序排列
2. **收藏（星标）：**
   - 文档标题旁星标按钮 ★ / ☆
   - 收藏列表（localStorage 持久化）
   - 侧边栏"收藏"分组
3. 数据格式：
   ```rust
   #[derive(Serialize, Deserialize, Clone)]
   struct FavoritesData {
       favorites: Vec<String>,       // doc_ids
       recent_docs: Vec<(String, i64)>, // (doc_id, last_access_ts)
   }
   ```
4. 使用 `use_local_storage::<FavoritesData>` 持久化
5. **验证：** 收藏文档后星标高亮；最近访问列表跨页面一致；收藏列表刷新后保持

**产出：** 最近访问 + 收藏

---

### 阶段 4：用户与团队（B-16 ~ B-20）

**目标：** 实现用户注册/登录、工作区管理、成员与角色、文档级权限、操作历史动态。

---

#### Step B-16: 用户注册/登录

| 属性 | 内容 |
|------|------|
| **前置** | B-15 |
| **难度** | ⭐⭐ |
| **核心知识点** | `use_local_storage` JWT token、受保护路由 |

**执行清单：**

1. 创建登录页面 `src/pages/auth/login.rs`
2. 创建注册页面 `src/pages/auth/register.rs`
3. 注意：本项目为 Trunk CSR，没有后端，因此采用以下方案：
   - **本地用户模拟**：用户名 + 密码哈希存储在 localStorage
   - 或**接入远程 API**：通过 `gloo-net` 调用外部认证服务
   - 生成模拟 JWT token 存 localStorage
4. 创建 `use_auth()` Hook：
   - `current_user: RwSignal<Option<UserInfo>>`
   - `is_authenticated: Signal<bool>`
   - `login(username, password)` / `register(username, email, password)` / `logout()`
5. **受保护路由：** 未登录时重定向到 `/login`
6. **验证：** 注册 → 登录 → 刷新保持登录态；未登录无法访问编辑器

**产出：** 用户认证系统（本地模式）

---

#### Step B-17: 工作区/团队管理

| 属性 | 内容 |
|------|------|
| **前置** | B-16 |
| **难度** | ⭐⭐ |
| **核心知识点** | 多 workspace 切换、邀请链接生成、Thaw `<Select/>` |

**执行清单：**

1. 定义工作区数据结构：
   ```rust
   struct Workspace {
       id: String,
       name: String,
       owner_id: String,
       member_ids: Vec<String>,
       created_at: i64,
   }
   ```
2. 工作区管理页面 `src/pages/workspace/mod.rs`：
   - 工作区列表（用户所在的所有工作区）
   - 创建工作区
   - 切换工作区：`<Select/>` 下拉切换
3. 切换工作区后：
   - 文档树刷新（对应工作区的文档）
   - 标签页关闭
4. 邀请功能：
   - 生成邀请链接（含 token）
   - 被邀请者打开链接 → 自动加入工作区
5. 工作区数据存储：IndexedDB（本地模式）/ 远程 API（协作模式）
6. **验证：** 创建多工作区；切换工作区后文档树变化；邀请链接可加入

**产出：** 多工作区管理

---

#### Step B-18: 成员管理 + 角色

| 属性 | 内容 |
|------|------|
| **前置** | B-17 |
| **难度** | ⭐⭐ |
| **核心知识点** | 成员列表、角色（所有者/管理员/编辑者/查看者）、权限枚举 |

**执行清单：**

1. 定义角色枚举：
   ```rust
   enum WorkspaceRole {
       Owner,   // 所有者 — 可管理成员、删除工作区
       Admin,   // 管理员 — 可管理成员
       Editor,  // 编辑者 — 可编辑文档
       Viewer,  // 查看者 — 只读
   }
   ```
2. 成员管理页面 `src/pages/workspace/members.rs`：
   - 成员列表（Thaw `<Table/>`：头像、名称、角色、加入时间）
   - 修改角色（`<Select/>`）
   - 移除成员（需确认）
3. 角色权限矩阵：
   | 操作 | Owner | Admin | Editor | Viewer |
   |------|-------|-------|--------|--------|
   | 查看文档 | ✓ | ✓ | ✓ | ✓ |
   | 编辑文档 | ✓ | ✓ | ✓ | ✗ |
   | 创建/删除文档 | ✓ | ✓ | ✓ | ✗ |
   | 邀请成员 | ✓ | ✓ | ✗ | ✗ |
   | 修改角色 | ✓ | ✓ | ✗ | ✗ |
   | 删除工作区 | ✓ | ✗ | ✗ | ✗ |
4. **验证：** 管理员可修改成员角色；查看者无编辑权限

**产出：** 成员管理 + 角色权限

---

#### Step B-19: 文档级权限

| 属性 | 内容 |
|------|------|
| **前置** | B-18 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 文档 ACL（继承+覆盖）、权限校验 Signal、UI 按钮条件禁用 |

**执行清单：**

1. 扩展 DocNode 添加权限字段：
   ```rust
   struct DocPermissions {
       can_read: Vec<String>,   // 用户/角色 ID 列表
       can_edit: Vec<String>,
       is_inherited: bool,      // 是否从父文件夹继承
   }
   ```
2. 权限继承规则：
   - 默认继承父文件夹权限
   - 文档可单独覆盖（`is_inherited = false`）
   - 根文件夹权限 = 工作区默认权限
3. 创建 `use_doc_permissions(doc_id)` Hook：
   - 返回 `can_read: Signal<bool>` / `can_edit: Signal<bool>`
   - 校验当前用户角色
4. UI 权限控制：
   - 无编辑权限 → 编辑器只读（`textarea readonly` + 灰度提示）
   - 无编辑权限 → 隐藏保存/删除按钮
   - 无查看权限 → 文档列表不显示
5. **验证：** 查看者打开文档时编辑器只读；权限变更后即时生效

**产出：** 文档级权限控制

---

#### Step B-20: 操作历史 + 动态 Feed

| 属性 | 内容 |
|------|------|
| **前置** | B-19 |
| **难度** | ⭐⭐ |
| **核心知识点** | 活动日志（编辑/评论/移动）、Thaw `<Timeline/>` 时间线 |

**执行清单：**

1. 定义活动事件：
   ```rust
   struct ActivityEvent {
       id: String,
       user_id: String,
       username: String,
       action: String,       // "created" / "edited" / "deleted" / "moved" / "commented"
       target_type: String,  // "doc" / "folder"
       target_id: String,
       target_name: String,
       timestamp: i64,
   }
   ```
2. 创建活动 Feed 页面 `src/pages/activity.rs`：
   - Thaw `<Timeline/>` 展示活动流
   - 过滤器：按用户、按操作类型、按时间
3. 每次文档操作时记录事件：
   - 创建/编辑/删除/移动文档
   - 后续 B-27 评论事件
4. 活动事件存储：IndexedDB（本地）/ 远程同步
5. **验证：** 编辑文档后活动流出现新事件；筛选功能正常

**产出：** 操作活动 Feed

---

### 阶段 5：高级协作（B-21 ~ B-27）

**目标：** WebSocket 连接、在线状态+光标同步、Y.js CRDT 集成、实时内容同步、离线编辑+同步、版本历史+时间旅行、评论/批注。这是整个项目最难的部分。

---

#### Step B-21: WebSocket 连接管理

| 属性 | 内容 |
|------|------|
| **前置** | B-20 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `ws://` 连接建立/重连/心跳、`on_cleanup` 断开清理 |

**执行清单：**

1. 安装 WebSocket 支持（`gloo-net` 的 `websocket` feature 或 `wasm-bindgen` 的 `web_sys::WebSocket`）
2. 创建 `use_websocket(url)` Hook：
   - 连接建立与关闭
   - 自动重连（指数退避：1s → 2s → 4s → max 30s）
   - 心跳 ping/pong
   - `send_message(msg)` / `on_message: Callback<Message>`
3. WebSocket 服务端（开发阶段可用简单的 `warp` 或 `axum` WebSocket echo server）：
   - 广播消息给同一文档的所有连接者
   - 消息类型：`cursor_update`、`content_update`、`presence`、`awareness`
4. 连接状态 UI 指示器（绿色连接/黄色重连中/红色断开）
5. `on_cleanup` 中关闭 WebSocket 连接
6. **验证：** 两个浏览器窗口连接同一 WS，发送消息可互收

**产出：** WebSocket 连接管理器

---

#### Step B-22: 在线状态 + 光标同步

| 属性 | 内容 |
|------|------|
| **前置** | B-21 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 在线用户列表、远程光标位置渲染（绝对定位叠加） |

**执行清单：**

1. 创建在线状态管理：
   - 进入文档时通过 WS 发送 `presence_join`
   - 定期发送 `presence_update`（光标位置、选择范围）
   - 离开文档时发送 `presence_leave`
2. 编辑器右侧显示在线用户面板：
   - 在线用户头像列表
   - 每个用户分配唯一颜色
3. 远程光标渲染：
   - 监听 `textarea` 的光标位置（`selectionStart`）
   - 通过 WS 广播给其他客户端
   - 其他客户端在预览区渲染彩色光标标签
   ```html
   <span class="remote-cursor" style="left: 100px; top: 50px; color: blue;">
       张三
   </span>
   ```
4. 远程光标使用 `position: absolute` 叠加在 Markdown 预览上
5. **验证：** 两个窗口编辑同一文档，能看到对方的彩色光标和用户名

**产出：** 在线状态 + 远程光标

---

#### Step B-23: Y.js CRDT 集成

| 属性 | 内容 |
|------|------|
| **前置** | B-22 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `y-crdt` WASM 绑定、`Y.Doc` 共享数据结构、操作合并 |

**执行清单：**

1. 添加 `yrs` (Y.js Rust 端口) 到依赖（`yrs = "0.18"`）
2. 创建 `use_yjs_doc(doc_id)` Hook：
   ```rust
   use yrs::{Doc, Text, Transact, Awareness, updates::decoder::Decode};
   
   let doc = Doc::new();
   let text = doc.get_or_insert_text("content");
   // ...
   ```
3. Y.Doc 操作：
   - `text.insert(offset, str)` — 插入文本
   - `text.remove_range(offset, len)` — 删除文本
   - `text.to_string()` — 获取当前内容
4. 监听 Y.Doc 变更：
   ```rust
   let sub = doc.observe_update_v1(move |_, update| {
       // 通过 WS 发送 update 二进制
       ws.send(update);
   });
   ```
5. 接收远程 update：
   ```rust
   // 收到 WS 消息 → 应用 update
   let update = Update::decode_v1(&msg.data);
   doc.transact_mut().apply_update(update);
   ```
6. Y.Doc 绑定到受控 `<textarea/>`：
   - 本地编辑 → `text.insert` / `text.remove_range`
   - 远程变更回调 → 更新 `textarea` 的 value
7. **验证：** 两个窗口编辑同一个 Y.Doc 文档，内容实时同步，不丢失文字

**产出：** Y.js CRDT 基础集成

---

#### Step B-24: 实时内容同步

| 属性 | 内容 |
|------|------|
| **前置** | B-23 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `y-textarea` 双向绑定、冲突自动解决、`watch` 响应远程变更 |

**执行清单：**

1. 将 B-23 的 Y.Doc 与 B-03 的编辑器整合：
   - `textarea` 绑定到 `Y.Text`
   - 本地输入事件 → `Y.Text` 操作 → WS 广播
   - WS 接收 → 更新 `Y.Text` → `textarea` 自动更新
2. 冲突解决：Y.js CRDT 自动合并操作
3. 性能优化：
   - 批量发送 update（合并 50ms 内的操作）
   - 避免不必要的 `get_string` 调用
4. 与 IndexedDB 持久化的协调：
   - 本地文档在 Y.Doc 中编辑
   - 定期将整个 Y.Doc 状态快照保存到 IndexedDB 作为备份
5. Markdown 预览实时更新
6. **验证：** 打开两个浏览器窗口编辑同一个文档，A 窗口输入的内容在 < 200ms 内出现在 B 窗口

**产出：** 实时协作编辑

---

#### Step B-25: 离线编辑 + 同步

| 属性 | 内容 |
|------|------|
| **前置** | B-24 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | Y.js offline 模式、IndexedDB 持久化、同步冲突 UI 提示 |

**执行清单：**

1. Y.js 离线支持：
   - 网络断开时，所有 Y.Doc 操作在本地缓存
   - 使用 `yrs` 的 `encode_state_as_update` 保存完整文档状态到 IndexedDB
2. 重连同步流程：
   ```
   网络恢复 → 从 IndexedDB 加载离线期间的 update 列表
           → 发送给服务端/其他 peers
           → 服务端合并 → 返回最新状态
           → 本地应用合并后的状态
   ```
3. 同步冲突 UI：
   - 如果离线期间的编辑与远程冲突 → 显示通知"检测到 N 处冲突，已自动解决"
   - 如有无法自动解决的冲突 → 显示冲突对比面板
4. 离线状态指示器（编辑器顶部黄色横幅"当前处于离线模式，编辑将在恢复网络后同步"）
5. **验证：** 断网编辑 → 重连 → 内容自动同步；两个窗口断网编辑同一文档 → 重连后合并

**产出：** 离线编辑 + 自动同步

---

#### Step B-26: 版本历史 + 时间旅行

| 属性 | 内容 |
|------|------|
| **前置** | B-25 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | Y.js `UndoManager`、版本滑动条、Diff 差异对比 |

**执行清单：**

1. 使用 `yrs::UndoManager` 管理撤销/重做：
   ```rust
   let undo_manager = UndoManager::new(&text, 30); // 保留 30 步历史
   // 快捷键绑定
   // Ctrl+Z → undo_manager.undo()
   // Ctrl+Shift+Z → undo_manager.redo()
   ```
2. 版本快照管理：
   - 每 5 分钟或每次关闭文档时创建一个版本快照
   - 快照存储为完整 Y.Doc state vector
3. 版本历史面板 `src/components/version_history.rs`：
   - 版本列表（时间 + 操作摘要 + 用户）
   - 滑动条选择版本 → 预览该版本的文档
   - "恢复到此版本"按钮
4. Diff 差异对比（可选）：
   - 使用 `similar` crate 计算两个版本差异
   - 绿色（新增）/ 红色（删除）高亮
5. 撤销不影响其他人的编辑（Y.js 的 UndoManager 只撤销本地操作）
6. **验证：** Ctrl+Z 撤销本地操作；版本历史选择可预览旧版本；恢复后内容正确

**产出：** 版本历史 + 时间旅行

---

#### Step B-27: 评论/批注系统

| 属性 | 内容 |
|------|------|
| **前置** | B-26 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 选中文本批注、评论线程、Thaw `<Comment/>` `<Popover/>`、@ 提及 |

**执行清单：**

1. 定义评论数据结构：
   ```rust
   struct Comment {
       id: String,
       doc_id: String,
       user_id: String,
       username: String,
       selected_text: String,   // 被评论的文本片段
       content: String,
       replies: Vec<CommentReply>,
       resolved: bool,
       created_at: i64,
   }
   ```
2. 选中文本 → 右键 → "添加批注" → 弹出评论输入框
3. 批注显示：
   - 被评论的文本区域用黄色高亮背景
   - 点击高亮文本 → 右侧弹出评论线程面板
4. 评论线程面板：
   - 原始评论 + 回复列表
   - 回复输入框
   - @ 提及：输入 `@` 触发用户搜索 → 自动补全
   - 标记为已解决
5. 评论通过 Y.js Awareness 协议或自定义 WS 消息同步
6. **验证：** 选中文本添加评论；打开同一文档的两个窗口都能看到评论

**产出：** 评论/批注系统

---

### 阶段 6：PWA 与用户体验（B-28 ~ B-31）

**目标：** PWA + Service Worker、桌面通知+后台同步、快捷键+命令面板、暗黑模式+主题。

---

#### Step B-28: PWA + Service Worker

| 属性 | 内容 |
|------|------|
| **前置** | B-27 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `manifest.json`、Service Worker 缓存策略、离线启动页 |

**执行清单：**

1. 创建 `manifest.json`：
   ```json
   {
       "name": "NoteFlow",
       "short_name": "NoteFlow",
       "description": "实时协作知识库",
       "start_url": "/",
       "display": "standalone",
       "background_color": "#ffffff",
       "theme_color": "#4f46e5",
       "icons": [
           { "src": "/icon-192.png", "sizes": "192x192", "type": "image/png" },
           { "src": "/icon-512.png", "sizes": "512x512", "type": "image/png" }
       ]
   }
   ```
2. 在 `index.html` 中链接 manifest 和注册 Service Worker
3. 编写 Service Worker（`sw.js`）：
   - **Cache-First**：静态资源（CSS/JS/WASM/字体）
   - **Network-First**：文档数据（API 请求）
   - **Stale-While-Revalidate**：图片
   - 离线回退页面
4. 使用 `trunk` 的 `data-cargo-features` 处理 SW 缓存版本管理
5. PWA 安装提示：检测 `beforeinstallprompt` 事件 → 显示安装按钮
6. **验证：** 打开 DevTools Application 面板 → Service Worker 状态 activated；离线刷新不白屏

**产出：** PWA 安装 + 离线访问

---

#### Step B-29: 桌面通知 + 后台同步

| 属性 | 内容 |
|------|------|
| **前置** | B-28 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `Notification API`、`Background Sync`、`on_visibility_change` |

**执行清单：**

1. 请求通知权限：
   ```rust
   // 通过 wasm-bindgen 调用 Notification.requestPermission()
   ```
2. 通知场景：
   - 有人在你的文档上添加了评论
   - 文档被其他人修改
   - 离线期间的变更是已同步
   - @ 提及通知
3. Background Sync（可选，浏览器支持有限）：
   - 注册 `navigator.serviceWorker.ready.then(reg => reg.sync.register('doc-sync'))`
   - Service Worker 中监听 `sync` 事件，执行同步逻辑
4. 页面可见性检测：
   ```rust
   use leptos_use::use_document_visibility;
   let visible = use_document_visibility();
   // 页面从隐藏恢复时：刷新在线状态 + 检查远程更新
   ```
5. **验证：** 其他人添加评论时收到桌面通知（需授予权限）

**产出：** 桌面通知 + 后台同步

---

#### Step B-30: 快捷键 + 命令面板

| 属性 | 内容 |
|------|------|
| **前置** | B-29 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `window_event_listener(keydown)`、Thaw `<Command/>` 面板、快捷键映射 |

**执行清单：**

1. 快捷键注册系统：
   ```rust
   struct Shortcut {
       keys: &'static str,      // "Ctrl+N"
       description: &'static str,
       action: Action,
   }
   ```
2. 全局快捷键列表：
   | 快捷键 | 功能 |
   |--------|------|
   | `Ctrl+N` | 新建文档 |
   | `Ctrl+Shift+N` | 新建文件夹 |
   | `Ctrl+S` | 保存（手动） |
   | `Ctrl+K` | 命令面板 |
   | `Ctrl+Z` | 撤销 |
   | `Ctrl+Shift+Z` | 重做 |
   | `Ctrl+F` | 搜索 |
   | `Ctrl+B` | 加粗 |
   | `Ctrl+I` | 斜体 |
   | `Ctrl+Shift+F` | 全屏专注模式 |
   | `F2` | 重命名 |
   | `Delete` | 删除 |
3. 命令面板（`Cmd+K` / `Ctrl+K`）：
   - Thaw `<Command/>` 组件（或自定义 modal）
   - 输入匹配：命令名称模糊搜索
   - 命令列表：新建文档、新建文件夹、搜索、切换暗黑模式、导出 Markdown、打开设置...
   - 选中执行 + 关闭面板
4. 使用 `window_event_listener(ev::keydown, ...)` 注册全局快捷键
5. **验证：** `Ctrl+K` 弹出命令面板；输入"新建文档"回车执行；`Ctrl+N` 新建文档

**产出：** 快捷键 + 命令面板

---

#### Step B-31: 暗黑模式 + 自定义主题

| 属性 | 内容 |
|------|------|
| **前置** | B-30 |
| **难度** | ⭐⭐ |
| **核心知识点** | CSS 变量体系、`use_media_query` 自动检测、主题 Signal 持久化 |

**执行清单：**

1. 定义 CSS 变量体系（亮色 / 暗色两套）：
   ```css
   :root {
       --bg-primary: #ffffff;
       --bg-secondary: #f5f5f5;
       --text-primary: #1a1a1a;
       --text-secondary: #666666;
       --border-color: #e5e5e5;
       --accent-color: #4f46e5;
       /* 编辑器特定 */
       --editor-bg: #ffffff;
       --editor-text: #1a1a1a;
   }
   
   [data-theme="dark"] {
       --bg-primary: #1a1a2e;
       --bg-secondary: #16213e;
       --text-primary: #e0e0e0;
       --text-secondary: #a0a0a0;
       --border-color: #2a2a4a;
       --editor-bg: #0f0f23;
       --editor-text: #e0e0e0;
   }
   ```
2. 主题管理 Hook `use_theme()`：
   - `theme: RwSignal<Theme>` (Light / Dark / System)
   - `effective_theme: Signal<Theme>` (跟随系统或手动)
   - 使用 `use_media_query("(prefers-color-scheme: dark)")` 自动检测
3. 主题切换按钮（命令面板 / 设置页）
4. 主题偏好持久化到 localStorage
5. 编辑器预览区的暗色适配（Markdown 渲染 HTML 的暗色样式覆盖）
6. **验证：** 切换暗黑模式后全局样式变化；编辑器内容在暗黑模式下可读；跟随系统主题切换

**产出：** 暗黑模式 + 自定义主题

---

### 阶段 7：工程化与部署（B-32 ~ B-35）

**目标：** 看板视图、统计分析、测试+性能分析、SSR 分享页+部署。

---

#### Step B-32: 看板视图（数据关联）

| 属性 | 内容 |
|------|------|
| **前置** | B-31 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 文档状态作为看板列、HTML5 Drag & Drop 跨列移动、状态同步回 Y.js |

**执行清单：**

1. 为文档添加 `status` 字段（可在文档开头 YAML front matter 中定义）：
   ```yaml
   ---
   status: todo  # todo / in_progress / review / done
   ---
   ```
2. 创建看板视图页面 `src/pages/board.rs`：
   - 列：待办 (todo) / 进行中 (in_progress) / 审核中 (review) / 已完成 (done)
   - 每列显示对应状态的文档卡片
3. 文档卡片：标题、标签、修改时间
4. HTML5 Drag & Drop 跨列移动：
   - 拖动文档卡片到另一列
   - 更新文档的 `status` YAML front matter
   - 状态变更通过 Y.js 同步给协作者
5. 看板数据源：根据筛选条件（工作区/文件夹/标签）获取文档列表
6. **验证：** 拖拽文档卡片跨列 → 状态更新 → 其他窗口看板同步刷新

**产出：** 看板视图

---

#### Step B-33: 统计分析

| 属性 | 内容 |
|------|------|
| **前置** | B-32 |
| **难度** | ⭐⭐ |
| **核心知识点** | 文档数/字数统计、编辑频率图表、Thaw `<Statistic/>` |

**执行清单：**

1. 创建统计页面 `src/pages/stats.rs`
2. 统计面板：
   - **概览：** 总文档数、总字数、今日编辑次数、工作区数
   - **编辑频率图表：** 近 7 天每天编辑的文档数量（柱状图）
   - **标签分布：** 饼图展示各标签下的文档数量
   - **字数排行：** 文档按字数的 Top 10
3. 数据来源：IndexedDB 中的文档元数据 + 活动日志
4. Thaw `<Statistic/>` + `<Card/>` 展示指标卡
5. **验证：** 统计数据与实际情况一致；编辑后统计刷新

**产出：** 统计分析页面

---

#### Step B-34: 测试 + 性能分析

| 属性 | 内容 |
|------|------|
| **前置** | B-33 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `wasm-bindgen-test`、`twiggy` WASM 体积分析、lazy loading 代码分割 |

**执行清单：**

1. 编写 WASM 测试：
   ```rust
   use wasm_bindgen_test::*;
   wasm_bindgen_test_configure!(run_in_browser);
   
   #[wasm_bindgen_test]
   fn test_markdown_render() {
       let html = render_markdown("# Hello");
       assert!(html.contains("<h1>"));
   }
   
   #[wasm_bindgen_test]
   fn test_doc_tree_operations() { /* ... */ }
   ```
2. 关键测试用例：
   - Markdown 渲染正确性
   - 文档 CRUD 操作
   - 标签添加/删除
   - 权限校验
3. WASM 体积分析：
   - 使用 `twiggy` 分析 `noteflow_bg.wasm`
   - 找出占用空间最大的函数/模块
   - 代码分割：对 `comrak`、`syntect` 等大型依赖使用 lazy loading
4. 配置代码分割（Trunk / `wasm-bindgen` 的 `--split-linked-modules`）
5. **验证：** 所有测试通过；WASM 产物 < 5MB

**产出：** 测试覆盖 + WASM 体积优化

---

#### Step B-35: SSR 分享页 + 部署

| 属性 | 内容 |
|------|------|
| **前置** | B-34 |
| **难度** | ⭐⭐ |
| **核心知识点** | `cargo-leptos` 补充 SSR 路由、SEO meta、Docker 部署 |

**执行清单：**

1. 为项目添加 `cargo-leptos` 支持（或创建独立的 SSR 分享子项目）：
   - 使用 cargo-leptos 项目结构
   - 添加 SSR 路由：`/share/:doc_id`（分享页面）
2. SSR 分享页功能：
   - 渲染 Markdown 为 HTML（服务端执行，不依赖 WASM）
   - 设置 SEO meta：`<Title/>`、`<Meta description/>`、OG 标签
   - 无需 JS 也能查看文档内容（纯 SSR HTML）
3. SEO 优化：
   - 动态标题：`[文档名] - NoteFlow`
   - Meta description：文档前 200 字
   - Open Graph 标签：标题、描述、缩略图
4. 部署方案（Docker）：
   ```dockerfile
   FROM rust:nightly AS builder
   # ... 编译 SSR 服务
   
   FROM debian:bookworm-slim
   COPY --from=builder /app/target/release/noteflow-server /app/
   CMD ["/app/noteflow-server"]
   ```
5. nginx 反向代理（静态资源 + API + WebSocket 升级）
6. **验证：** 分享页在无 JS 环境下渲染 Markdown HTML；`docker compose up` 访问正常

**产出：** SSR 分享页 + Docker 部署

---

### 阶段 8：高级功能与生态集成（B-36 ~ B-40）

**目标：** 文档目录大纲、内部链接图谱、只读分享链接、专注写作模式、AI 辅助写作。

---

#### Step B-36: 文档目录大纲

| 属性 | 内容 |
|------|------|
| **前置** | B-35 |
| **难度** | ⭐⭐ |
| **核心知识点** | Markdown 标题解析生成 ToC、IntersectionObserver 滚动高亮 |

**执行清单：**

1. 解析 Markdown 标题结构：
   ```rust
   fn extract_toc(md: &str) -> Vec<TocItem> {
       // 正则匹配 ^#{1,6}\s+(.+)$
       // 返回层级、标题文本、锚点 ID
   }
   ```
2. 右侧 ToC 面板（可折叠）：
   - 缩进显示标题层级（H1 → H2 → H3）
   - 点击跳转到对应位置（`scrollIntoView`）
3. 滚动高亮：使用 IntersectionObserver 监听各个标题元素
   - 当前可见的标题在 ToC 中高亮
   - 同时高亮所有父级标题
4. ToC 折叠层级（默认显示到 H3）
5. **验证：** 滚动文档时当前标题在 ToC 中高亮；点击 ToC 跳转正确

**产出：** 文档目录大纲

---

#### Step B-37: 文档内部链接图谱

| 属性 | 内容 |
|------|------|
| **前置** | B-36 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `[[wikilink]]` 语法解析、反向链接列表、关联图可视化 |

**执行清单：**

1. 解析 `[[文档名]]` WikiLink 语法：
   - 渲染为 `<a>` 链接，点击跳转到对应文档
   - 如果目标文档不存在，显示为虚线链接（点击创建）
2. 构建文档链接图：
   - 解析所有文档中的 `[[wikilink]]`
   - 建立有向图：源文档 → 目标文档
3. 反向链接面板（编辑器底部或右侧）：
   - 列出所有链接到当前文档的其他文档
   - 点击跳转
4. 关联图可视化（Canvas / SVG）：
   - 节点 = 文档，边 = 链接
   - 当前文档节点高亮
   - 节点大小按链接数量缩放
5. 链接图数据存储与更新（编辑时实时解析）
6. **验证：** 创建 `[[Other Doc]]` 后出现链接引用提示；反向链接面板正确

**产出：** 文档内部链接 + 关联图

---

#### Step B-38: 只读分享链接

| 属性 | 内容 |
|------|------|
| **前置** | B-37 |
| **难度** | ⭐⭐ |
| **核心知识点** | 加密 token 生成、过期时间、权限校验中间件、分享页 SSR |

**执行清单：**

1. 创建分享链接功能：
   - 文档菜单 → "生成分享链接"
   - 选项：过期时间（1天/7天/30天/永不过期）、密码保护（可选）
2. 分享 token 生成：
   ```rust
   // 生成随机 token + 可选密码哈希
   // 存储到 IndexedDB 或本地 KV
   ```
3. 分享页面（基于 B-35 的 SSR 页面）：
   - 无需登录即可访问
   - 只读模式（无编辑功能）
   - 过期 token → 显示"此链接已失效"
   - 密码保护 → 输入密码表单
4. 权限校验：
   - 检查 token 有效性 + 过期时间
   - 如果有密码 → 验证密码
5. **验证：** 未登录访问分享链接 → 展示只读内容；过期链接提示失效；密码保护生效

**产出：** 只读分享链接

---

#### Step B-39: 专注写作模式

| 属性 | 内容 |
|------|------|
| **前置** | B-38 |
| **难度** | ⭐⭐ |
| **核心知识点** | 全屏编辑器、打字机滚动、字数目标、暗色主题 |

**执行清单：**

1. 专注模式入口：命令面板 / 快捷键 `Ctrl+Shift+F` / 编辑器菜单
2. 专注模式特性：
   - 全屏：隐藏侧边栏、顶栏、Tab 栏
   - 编辑器居中（`max-width: 720px; margin: 0 auto`）
   - 打字机滚动：当前编辑行始终在屏幕中央
   - 背景使用柔和暗色（减少视觉干扰）
3. 字数统计 + 目标设置：
   - 实时字数/字符数显示
   - 设置每日写作目标（如 1000 字）
   - 进度条可视化
4. "禅模式"音效（可选）：打字音效
5. 按 `Esc` 退出专注模式
6. **验证：** 进入专注模式后隐藏导航；退出后恢复；字数统计正确

**产出：** 专注写作模式

---

#### Step B-40: AI 辅助写作集成

| 属性 | 内容 |
|------|------|
| **前置** | B-39 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | LLM API 调用（`gloo-net`）、流式补全建议、翻译/润色命令面板 |

**执行清单：**

1. AI 功能入口：
   - 编辑器工具栏：AI 按钮
   - 命令面板：AI 命令
   - 选中文本右键：AI 操作菜单
2. AI 操作类型：
   - **继续写作**：从光标位置续写
   - **润色**：优化选中文本的表达
   - **翻译**：翻译为英文/中文
   - **摘要**：生成当前文档的摘要
   - **解释**：解释选中的技术概念
3. API 调用（`gloo-net::http::Request`）：
   ```rust
   async fn call_ai(prompt: String, system_prompt: String) -> Result<String, Error> {
       // POST to OpenAI-compatible API
       // 支持 stream 模式（SSE 读取）
   }
   ```
4. 流式输出：AI 返回的文字逐字显示在预览区（打字机效果）
5. AI 结果操作：插入到文档 / 替换选中文本 / 复制 / 重新生成
6. 设置页面：API 地址、API Key、模型选择
7. **验证：** AI 补全请求在 < 2s 内返回首批 token；润色后的文本可插入文档

**产出：** AI 辅助写作

---

## 5. 验证标准

| 检查项 | 说明 |
|--------|------|
| **编译检查** | `trunk build` / `cargo leptos build` 零错误零警告（练习和答案两个项目均需通过） |
| **路由完整性** | 所有页面路由可达，404 兜底正常 |
| **数据持久化** | 刷新后文档内容不丢失（IndexedDB） |
| **错误边界** | 编辑器渲染失败时显示错误提示而非白屏 |
| **实时同步** | 两个窗口编辑同一文档，< 200ms 同步延迟 |
| **离线支持** | 断网编辑内容在重连后自动同步 |
| **状态机合法性** | 看板状态只能按合法路径转换 |
| **未授权访问** | 未登录重定向到登录页；无权限编辑器只读 |
| **响应式布局** | 移动端侧边栏折叠、编辑器自适应 |
| **构建产物体积** | WASM < 5MB |
| **PWA** | Service Worker 激活、离线启动可用 |
| **递进兼容** | Step N 的修改不破坏 Step N-1 的已有功能 |
| **答案完整性** | 答案项目 (`noteflow_answer/`) 不含任何 TODO/FIXME，独立 `trunk build` 零错误 |

---

## 6. 风险与缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| B-03 编辑器双向绑定 Bug | 中 | 高 | 先做单向绑定（textarea → 预览），确认无误后再做反向 |
| B-23/B-24 Y.js 集成数据丢失 | 高 | 高 | 关键路径：每步保存 Y.Doc 快照到 IndexedDB，后端做增量备份 |
| B-25 离线同步冲突 | 中 | 高 | Y.js CRDT 天然支持自动合并；复杂冲突用 diff 面板 |
| B-28 Service Worker 缓存更新 | 中 | 中 | 使用版本化缓存策略，新版本 SW 先 `skipWaiting` 再 `clients.claim` |
| WASM 体积超限 | 中 | 中 | `twiggy` 分析 + lazy loading + `wasm-opt` |
| `yrs` 与 Leptos 响应式不兼容 | 低 | 高 | 通过 `RwSignal` 桥接 Y.Doc 变更事件 |
| 编译失败循环 | 中 | 低 | 每步独立 commit，回滚成本低 |

---

## 7. Agent 启动指令

```
## 任务: 编写项目 B — NoteFlow 知识库（练习 + 答案双文件夹）

## 上下文
工作区: c:\code\testruetlearn\leptos-learn\projects\
练习项目: noteflow/          (含 TODO，供学员补全)
答案项目: noteflow_answer/   (完整可编译运行，无 TODO)
步数: 40 (B-01 到 B-40)
结构: Trunk CSR 项目（B-35 起补充 SSR 分享页）

## 特别注意
- 强依赖链: 每一步依赖前一步
- 每步需同步维护两个文件夹：练习（含 TODO）+ 答案（完整代码）
- B-03 编辑器 → B-05 持久化 → B-23 Y.js → B-24 实时同步 是整个项目的生命线
- Y.js 集成是最大技术难点，两步分开做（B-23 基础 → B-24 完善）
- B-35 需要额外引入 cargo-leptos SSR
- 答案项目必须在 40 步全部完成后能独立编译运行，零错误零警告，不含 TODO

## 执行流程
从 B-01 开始，串行到 B-40。

每步执行:
1. 阅读 project-noteflow-execution-plan.md 中该 Step 的描述和知识点
2. 在练习项目 (noteflow/) 中增量开发，保留 TODO
3. trunk build 验证练习（B-35 起用 cargo leptos build）
4. 将完整代码同步到答案项目 (noteflow_answer/)，去掉 TODO
5. trunk build 验证答案
6. 失败则修复（最多 5 次）
7. 通过后 git commit + 进入下一步
```

---

## 8. Workspace 注册

workspace `Cargo.toml` 中需预注册两个项目的路径：

```toml
[workspace]
members = [
    # ... 其他章节成员 ...
    
    # 终极项目 B — NoteFlow
    "projects/noteflow",
    "projects/noteflow_answer",
]
```

---

## 9. 进度追踪

使用 `progress.json` 追踪进度：

```json
{
  "project": "noteflow",
  "steps": [
    { "step": "B-01", "status": "pending", "commits": 1, "time": null },
    { "step": "B-02", "status": "pending", "commits": 1, "time": null }
  ],
  "total_steps": 40,
  "completed_steps": 0,
  "last_updated": "2026-07-28T00:00:00Z"
}
```
