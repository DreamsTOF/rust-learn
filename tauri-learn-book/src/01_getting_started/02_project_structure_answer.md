# 练习 02 答案讲解：项目结构

> **用法**：卡住时再看本页。每一处 diff 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/main.ts`（前端）。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 命令框架已给，`vec![]` 为空 | 12 行结构说明 |
| `src/main.ts` | 调用、渲染已写好 | 仅 `import { invoke }` 一行被注释 |

> **前端基础提示**：本练习的前端用 Vanilla TS。`querySelector`、`textContent`、`join` 等语法不熟的话，先读 [Vanilla TS 速成（给 Vue 开发者）](00_vanilla_ts_primer.md)。下文前端 TODO 处有 **Vanilla TS 注解**。

## lib.rs TODO 1：填充 12 行结构说明

### 练习版这里是什么

```rust
#[tauri::command]
fn project_layout() -> Vec<String> {
    vec![]   // ← 你的作业在这
}
```

### 答案版填了什么

```rust
#[tauri::command]
fn project_layout() -> Vec<String> {
    vec![
        "src/                  # 前端（Vite + TS + HTML）".into(),
        "  index.html          # 页面骨架，浏览器加载入口".into(),
        "  src/main.ts         # 前端逻辑：UI 与 invoke 调用".into(),
        "  src/styles.css      # 全局样式".into(),
        "src-tauri/            # 后端（独立 Rust crate）".into(),
        "  src/main.rs         # 平台入口：只有一行，调用 lib 的 run()".into(),
        "  src/lib.rs          # 核心：命令、状态、Builder 配置都在这".into(),
        "  tauri.conf.json     # 应用配置：identifier / 窗口 / 构建命令".into(),
        "  capabilities/       # 权限声明（core:default 最小权限）".into(),
        "  icons/              # 应用图标（icon.ico / icon.png）".into(),
        "  Cargo.toml          # Rust 依赖（tauri 等）".into(),
        "  build.rs            # 构建脚本，调用 tauri_build::build()".into(),
    ]
}
```

### 为什么

- **为什么返回 `Vec<String>` 而不是拼好的一个长字符串？** 因为后端只负责"提供数据"，排版是前端的事——前端 `lines.join("\n")` 一行代码就能按行显示。如果后端拼字符串，前端想改样式就得改后端。
- **12 行恰好覆盖两个世界**：`src/` 前端世界 4 行、`src-tauri/` Rust 世界 8 行。这也是整棵结构树的分界线。
- **练习版的注释就是答案本身**：TODO 注释里已经列出了每一行的参考内容（"参考结构：src/ ... src-tauri/ ..."），你要做的只是把注释"翻译"成 `"xxx".into()` 代码——这就是填空练习的规律：**TODO 注释 = 作业单，提示 = 答案的骨架**。
- `"xxx".into()`：字符串字面量到 `String` 的隐式转换，`String` 实现了 `From<&str>`。

### 回查文档

[《练习 02》第 1 节：双目录结构](02_project_structure.md#sec-02-dual-dirs)、[第 2 节：src-tauri/ 内部解剖](02_project_structure.md#sec-02-src-tauri)、[第 3 节：数组返回值](02_project_structure.md#sec-02-vec-return)。

## main.ts TODO：取消 import 注释

同练习 01——`invoke` 不 import 就是未定义变量。练习版的调用和渲染其实已经写好了：

```typescript
const lines = await invoke<string[]>("project_layout");
layoutEl!.textContent = lines.join("\n");
```

注意 `invoke<string[]>` 对应后端 `Vec<String>`——数组类型映射在练习 01 的类型对照表里见过。

> **Vanilla TS 注解**：`layoutEl!.textContent = lines.join("\n")` 拆开看：
>
> - `lines.join("\n")`：把后端返回的行数组用换行符拼成一个字符串（数组 → 单块文本）
> - `textContent`：把结果**当纯文字**显示——这里只是展示结构树文本，不需要标签结构，所以不用 `innerHTML`（如果用了，`#` 等字符和缩进会被当 HTML 解析出问题）
> - `layoutEl!` 的 `!`：非空断言，告诉 TS"`querySelector` 找到的元素肯定存在"。`layoutEl` 本身来自 `document.querySelector<HTMLPreElement>("#layout")`——按 id 找 `<pre>` 元素，找不到返回 `null`
>
> 详见 [更新内容：textContent vs innerHTML](00_vanilla_ts_primer.md#sec-ts-render)、[找元素：querySelector](00_vanilla_ts_primer.md#sec-ts-query)。

## 验收标准

```bash
cd 01_getting_started/e02_project_structure
cargo tauri dev
```

窗口以结构树形式显示 12 行目录说明，每行格式"路径 + `#` 说明"。

**破坏性验证**：

- 删掉其中一行 → 窗口少一行，其余照常显示（验证"后端给什么前端排什么"）
- 把 `project_layout` 注册从 `generate_handler!` 里删掉 → 页面显示"调用失败"（验证注册表的必要性）