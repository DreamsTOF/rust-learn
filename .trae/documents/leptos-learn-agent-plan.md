# Leptos 练习项目 — Agent 编写规划

## 1. 概述

约 **465 项**练习题（385 道独立题 + 80 步项目）全部由 AI Agent 自动编写。本文档定义 agent 的编写流水线、并发策略、验证门禁和质量标准。

### 核心原则

| 原则                 | 说明                                              |
| -------------------- | ------------------------------------------------- |
| **模板驱动**   | 每道题从模板 crate 生成，避免重复脚手架           |
| **并行最大化** | 无依赖的章节并发编写，有依赖的章节串行编排        |
| **验证前置**   | 每道题写完即`trunk build`，不过关不留到下个阶段 |
| **幂等生成**   | 同一个 agent 重复运行同一道题应产生相同代码       |

---

## 2. 编写流水线总览

```
┌──────────────────────────────────────────────────────────────────────┐
│                        Phase 0: 脚手架                               │
│  workspace + rust-toolchain + 模板 + 00_preface + 预注册所有 member   │
│  耗时: ~2h  │  执行: 1 agent                                        │
└──────────────────────┬───────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────────────────────┐
│                    Phase 1: 章节编写（章内 batch 并发）               │
│                                                                      │
│  每章拆成 N 个 batch，每个 batch 由 1 个 agent 独立编写                │
│  同一章内的所有 batch 同时启动                                        │
│  不同章之间也同时启动                                                │
│                                                                      │
│  例: Ch2 (75题) ─→ 5 agents × 15题                                   │
│   ┌─ Agent 2a (e21-e35) ─┐                                           │
│   ├─ Agent 2b (e36-e50) ─┤                                           │
│   ├─ Agent 2c (e51-e65) ─┤── 同时启动                                │
│   ├─ Agent 2d (e66-e80) ─┤                                           │
│   └─ Agent 2e (e81-e95) ─┘                                           │
│                                                                      │
│  Phase 1 总量: ~25 agents 同时运行                                   │
│  耗时: ~2-3h（墙钟时间）                                              │
└──────────────────────┬───────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────────────────────┐
│              Phase 2: SSR + 高级（同样 batch 并发）                   │
│  Ch7 (55题 SSR): 4 agents × ~14题                                    │
│  Ch8 (40题):      4 agents × 10题                                    │
│  建议 Ch8 在 Ch7 后启动（部分概念依赖）                               │
│  耗时: ~2-3h                                                         │
└──────────────────────┬───────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────────────────────┐
│              Phase 3: 终极项目（串行递进，两个项目并行）               │
│  ┌─────────────────────────────────┐  ┌───────────────────────────┐  │
│  │  ShopOS: 1 agent 串行 A-01→A-40 │  │  NoteFlow: 1 agent       │  │
│  │  每步依赖前一步代码               │  │  串行 B-01→B-40          │  │
│  └─────────────────────────────────┘  └───────────────────────────┘  │
│  两个项目可并行 │  耗时: 各 ~6-8h                                   │
└──────────────────────┬───────────────────────────────────────────────┘
                       ↓
┌──────────────────────────────────────────────────────────────────────┐
│              Phase 4: 逐章全量验证                                    │
│                                                                      │
│  验证策略：每章独立验证，章章独立                                     │
│  第 N 章验证通过 → 第 N 章锁定，不再修改                              │
│  所有章都通过 → 整个项目正确                                          │
│                                                                      │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐          │
│  │  Ver Ch1 │   │  Ver Ch2 │   │  Ver Ch3 │   │  Ver ... │          │
│  │  并行验证 │   │  并行验证 │   │  并行验证 │   │  并行验证 │          │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘          │
│                                                                      │
│  耗时: ~2h（所有章并行验证）                                          │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 3. 练习 crate 模板

### 3.1 标准 CSR 模板（第 1-6 章、第 8 章）

每个练习 crate 由脚手架脚本 `scripts/new-exercise.ps1` 生成：

```
01_basics/eNN_exercise_name/
├── Cargo.toml          # workspace dependency 引用
├── index.html          # Trunk 入口
└── src/
    ├── main.rs          # 练习代码（含 TODO + 参考答案）
    └── lib.rs           # (可选) 共享工具
```

**main.rs 模板结构：**

```rust
// ============================================================
// 练习 N: 题目名称
//
// 目标: 一句话描述本练习要掌握的知识点
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // === 步骤 1 ——————————————————————————————————————————
    // TODO: 创建一个计数信号，初始值为 0
    // 提示: 使用 signal!() 或 signal()

    // === 步骤 2 ——————————————————————————————————————————
    // TODO: 在视图中显示当前计数值

    view! {
        <div>
            // 你的代码在这里
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

```

### 3.2 SSR 模板（第 7 章）

使用 `cargo-leptos` 项目结构，同样每个题目创建两个文件夹：

```
07_ssr/
├── eNNN_exercise_name/          # 练习项目
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   └── app.rs
│   └── index.html
└── eNNN_exercise_name_answer/   # 参考答案
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── lib.rs
    │   └── app.rs
    └── index.html
```

### 3.3 难度引导模板

根据难度等级，TODO 引导程度不同（练习与参考答案各一份）：

| 难度              | TODO 密度         | 练习代码完成度 | 参考答案完整度 |
| ----------------- | ----------------- | -------------- | -------------- |
| ⭐ (eN+0, eN+1)   | 每行都有详细 TODO | 只需填空       | 完整可编译     |
| ⭐⭐ (eN+2, eN+3) | 关键位置有 TODO   | 补全约 50%     | 完整可编译     |
| ⭐⭐⭐ (eN+4)     | 仅描述目标        | 仅骨架         | 完整可编译     |

---

## 4. 并发策略

### 4.1 核心模型：章内 Batch 并发

每个练习 crate 之间**完全独立**（无共享代码、无编译时依赖），因此并发粒度可以下探到 **batch 级别**——同一章内的多个 batch 同时由不同 agent 编写。

```
章节 (N 题)
├── Batch 1 (eX ~ eY)  ← Agent A
├── Batch 2 (eY+1 ~ eZ) ← Agent B    ← 同时启动
├── Batch 3 ...         ← Agent C
└── ...
```

**为什么这可行：**

- 每个练习是一个独立的 Cargo crate，不共享 `src/` 下的代码
- workspace Cargo.toml 的 `members` 列表在 Phase 0 预注册所有路径，不存在写冲突
- 唯一共享的文件是 `workspace Cargo.toml`，但 `members` 可以一次性声明所有路径

### 4.2 Batch 划分规则

```
每章题数 ÷ 5 = agent 数

固定规则:
- 每个 agent 固定负责 **5 道题**（即 10 个文件夹：5 练习 + 5 答案）
- 一个 batch 内的题尽量来自同一子节（知识点连贯）
- 章节题数不能被 5 整除时，余数题由最后一个 agent 额外承担
```

### 4.3 各章节 Batch 分配矩阵

每个 agent 固定负责 **5 道题**（10 个文件夹）。每 batch 内的题号按连续 5 题划分。

#### Phase 1: 第 1-6 章（全部同时启动）

| 章节 | 题量 | Agent 数 | Batch 划分（每 batch 5 题） |
| :--: | :--: | :------: | :------------------------- |
| Ch1 |  20  |    4    | 01-05 / 06-10 / 11-15 / 16-20 |
| Ch2 |  75  |   15    | 21-25 / 26-30 / 31-35 / 36-40 / 41-45 / 46-50 / 51-55 / 56-60 / 61-65 / 66-70 / 71-75 / 76-80 / 81-85 / 86-90 / 91-95 |
| Ch3 |  60  |   12    | 96-100 / 101-105 / 106-110 / 111-115 / 116-120 / 121-125 / 126-130 / 131-135 / 136-140 / 141-145 / 146-150 / 151-155 |
| Ch4 |  45  |    9    | 156-160 / 161-165 / 166-170 / 171-175 / 176-180 / 181-185 / 186-190 / 191-195 / 196-200 |
| Ch5 |  50  |   10    | 201-205 / 206-210 / 211-215 / 216-220 / 221-225 / 226-230 / 231-235 / 236-240 / 241-245 / 246-250 |
| Ch6 |  40  |    8    | 251-255 / 256-260 / 261-265 / 266-270 / 271-275 / 276-280 / 281-285 / 286-290 |

**合计：58 agents 同时运行**

```
时间线 →
Ch1: [A1a][A1b][A1c][A1d]                                                         ← 4 agents
Ch2: [A2a][A2b][A2c][A2d][A2e][A2f][A2g][A2h][A2i][A2j][A2k][A2l][A2m][A2n][A2o]  ← 15 agents
Ch3: [A3a][A3b][A3c][A3d][A3e][A3f][A3g][A3h][A3i][A3j][A3k][A3l]                 ← 12 agents
Ch4: [A4a][A4b][A4c][A4d][A4e][A4f][A4g][A4h][A4i]                                ← 9 agents
Ch5: [A5a][A5b][A5c][A5d][A5e][A5f][A5g][A5h][A5i][A5j]                           ← 10 agents
Ch6: [A6a][A6b][A6c][A6d][A6e][A6f][A6g][A6h]                                      ← 8 agents
     └────────── 全部同时启动 ──────────→ ~1-2h
```

#### Phase 2: 第 7-8 章

| 章节 | 题量 | Agent 数 | Batch 划分（每 batch 5 题） |
| :--: | :--: | :------: | :------------------------- |
| Ch7 |  55  |   11    | 291-295 / 296-300 / 301-305 / 306-310 / 311-315 / 316-320 / 321-325 / 326-330 / 331-335 / 336-340 / 341-345 |
| Ch8 |  40  |    8    | 346-350 / 351-355 / 356-360 / 361-365 / 366-370 / 371-375 / 376-380 / 381-385 |

**合计：19 agents**

**Ch8 启动时机：** Ch8 部分练习涉及 SSR 概念（WebSocket、Server Function 等），建议在 Ch7 完成后启动。若 agent 已熟悉相关概念，也可与 Ch7 同时启动（风险：Ch8 题目可能引用 Ch7 尚未验证的 API，但编译层面无依赖）。

#### Phase 3: 终极项目

|   项目   | 步数 | Agent 数 | 说明               |
| :------: | :--: | :------: | ------------------ |
|  ShopOS  |  40  |    1    | 强依赖链，必须串行 |
| NoteFlow |  40  |    1    | 强依赖链，必须串行 |

两个项目可相互并行。

### 4.4 总并发量

| Phase |    并行 agents    | 墙钟时间 |
| :---: | :---------------: | :------: |
|   0   |         1         |   ~2h   |
|   1   |        58        |  ~1-2h  |
|   2   |       19         |  ~1-2h  |
|   3   |         2         |  ~6-8h  |
|   4   | 10 (逐章并行验证) |   ~2h   |

**墙钟总时间：~12-16h**（相比单 agent 串行的 ~80h，加速比 ~6x）

---

## 5. Agent 编写指令规范

### 5.1 统一的 Agent Prompt 模板

每个编写 agent 收到以下指令结构：

```
## 任务: 编写第 N 章 — 《章节名称》，Batch eX-eY

### 上下文
- 工作区路径: c:\code\testruetlearn\leptos-learn\
- 负责题号: eX 到 eY（共 5 题）
- 需要创建 10 个文件夹（5 练习 + 5 答案）
- 依赖 crate: [leptos, leptos_router, thaw, ...]

### 编写流程（对每道题循环执行）:

1. **创建练习文件夹**:
   `powershell scripts/new-exercise.ps1 -Chapter "NN_name" -Number NN -Name "exercise_name" -Type "exercise"`
2. **填充练习 main.rs**: 根据难度模板仅写 TODO，不含答案
3. **编译验证练习**: `cd chapter/eNN_name && trunk build`
4. **创建答案文件夹**:
   `powershell scripts/new-exercise.ps1 -Chapter "NN_name" -Number NN -Name "exercise_name" -Type "answer"`
5. **填充答案 main.rs**: 写完整可编译代码（无 TODO）
6. **编译验证答案**: `cd chapter/eNN_name_answer && trunk build`
7. 如编译报错，修复后重复；最多 3 次
8. 通过后继续下一题

### 模板变量（每道题替换）:
- {NUMBER}: 练习编号
- {NAME}: 练习英文名 (kebab-case)
- {TITLE}: 练习中文标题
- {DIFFICULTY}: ⭐/⭐⭐/⭐⭐⭐
- {CONCEPTS}: 核心知识点列表
- {DESCRIPTION}: 练习描述
- {TODO_CONTENT}: 根据难度级别的 TODO 指引
- {ANSWER_CODE}: 参考答案完整代码

### 质量要求:
- 练习和答案都必须通过 trunk build (CSR) 或 cargo leptos build (SSR)
- 练习文件只含 TODO，**不包含**答案代码
- 答案文件是完整可编译代码，**不包含** TODO
- 每题两个文件夹均单独验证，不跳过编译步骤
```

### 5.2 脚手架脚本

`scripts/new-exercise.ps1` 接收参数：

```powershell
# 用法
.\scripts\new-exercise.ps1 -Chapter "01_basics" -Number 21 -Name "signal_create"

# 功能
# 1. 在 chapter_dir 下创建 e{Number}_{Name}/ 目录
# 2. 生成 Cargo.toml（自动填充 [package].name = "e{Number}_{Name}"）
# 3. 生成 index.html（Trunk 标准入口）
# 4. 生成 src/main.rs（从模板复制，填充变量占位符）
# 5. 在 workspace Cargo.toml 的 members 数组中追加路径
```

---

## 6. 验证门禁（QA Gates）

验证策略：**逐章独立验证，章章无关。第 N 章通过即锁定，不因后续章节的修改而回退。**

### 6.1 每道题验证（G1-G3，agent 自检）

```
┌─────────────────────────────────────────┐
│  G1: cargo check / trunk build 通过       │ ← 编译零错误零警告
├─────────────────────────────────────────┤
│  G2: 参考答案完整性                       │ ← 答案 crate 存在且可编译
├─────────────────────────────────────────┤
│  G3: 端口不冲突                           │ ← 相邻题不使用相同 trunk port
└──────────┬──────────────────────────────┘
           ↓ 全部通过 → 下一题
           ↓ 不通过 → 修复后重试 (最多 3 次)
```

### 6.2 每章验证（G4-G7，独立验证 agent 执行）

每章所有 batch 写完后，**立即**启动一个验证 agent 执行章级检查。不等待其他章节完成。

```
┌─────────────────────────────────────────┐
│  G4: 编号连续性                           │ ← 无跳号、无重号（脚本扫描）
├─────────────────────────────────────────┤
│  G5: 整章 trunk build                    │ ← 章内所有 crate 并行编译
├─────────────────────────────────────────┤
│  G6: 难度分布检查                         │ ← 每 5 题一组符合 ⭐/⭐⭐/⭐⭐⭐ 比例
├─────────────────────────────────────────┤
│  G7: 章节内容完整性                       │ ← 每题都有 main.rs + 组件导出
└─────────────────────────────────────────┘
```

**逐章验证状态表：**

```
验证状态                   下一动作
─────────                 ────────
Ch1: ✅ 通过               锁定，不做任何修改
Ch2: ✅ 通过               锁定
Ch3: ❌ G5 失败            Ch3 编写 agent 修复
Ch4: ⏳ 编写中             等待
Ch5: ⏳ 编写中             等待
...
```

整章验证通过后，该章节的所有 crate 被标记为 ✅，后续任何修改都不允许触及已通过的章节文件。这保证了「前 N 章正确」的递进可靠性。

### 6.3 全量验证（G8-G12，所有章都通过后执行）

仅当所有章节都通过各自验证后，执行一次全面检查：

```
┌─────────────────────────────────────────┐
│  G8: 全量编译                             │ ← workspace 级 cargo build
├─────────────────────────────────────────┤
│  G9: 00_preface 导航完整性                │ ← 导航页链接到所有练习
├─────────────────────────────────────────┤
│  G10: 代码风格一致性                       │ ← cargo fmt 检查
├─────────────────────────────────────────┤
│  G11: 无死代码 / 警告                      │ ← cargo clippy
├─────────────────────────────────────────┤
│  G12: 产物体积预算                         │ ← CSR < 5MB, SSR < 20MB
└─────────────────────────────────────────┘
```

---

## 7. 分阶段任务分配

### Phase 0: 脚手架（1 agent, ~2h）

| 任务 | 描述                                                     | 产出                                 |
| ---- | -------------------------------------------------------- | ------------------------------------ |
| 0.1  | 创建`leptos-learn/` workspace + 预注册所有 member 路径 | `Cargo.toml` (workspace)           |
| 0.2  | 写入`rust-toolchain.toml`                              | nightly 锁定                         |
| 0.3  | 创建`scripts/new-exercise.ps1`                         | 脚手架脚本                           |
| 0.4  | 创建习题难度标记和知识点 JSON（供 agent 读取）           | `exercise-data.json`               |
| 0.5  | 创建`00_preface/` 导航首页                             | 所有练习的索引页                     |
| 0.6  | 创建模板文件                                             | `template_csr/`, `template_ssr/` |
| 0.7  | 验证:`trunk build` 通过                                | 导航页可运行                         |

**注意：** workspace Cargo.toml 的 `members` 列表在 Phase 0 **一次性预注册所有 770 个 crates 的路径**（385 练习 + 385 答案）。这样 Phase 1 的 agent 只管写代码，不需要修改 Cargo.toml，彻底消除文件写冲突。

```toml
[workspace]
members = [
    "00_preface",
    # 第 1 章 — 练习
    "01_basics/e01_hello_world",
    "01_basics/e02_html_elements",
    # ... 预注册所有练习路径
    "01_basics/e20_builder_advanced",
    # 第 1 章 — 参考答案
    "01_basics/e01_hello_world_answer",
    "01_basics/e02_html_elements_answer",
    # ... 预注册所有答案路径
    "01_basics/e20_builder_advanced_answer",
    # 第 2 章 — 练习
    "02_signals/e21_signal_create",
    "02_signals/e22_signal_get",
    # ...
    # 第 2 章 — 参考答案
    "02_signals/e21_signal_create_answer",
    "02_signals/e22_signal_get_answer",
    # ...
]
```

### Phase 1: 第 1-6 章（58 agents 同时启动, ~1-2h）

每个 agent 负责连续 **5 道题**（10 个文件夹），执行以下循环：

```
for each exercise in batch:
    1. 从 exercise-data.json 读取题目信息
    2. 运行脚手架创建练习目录 + 填充 TODO main.rs
    3. trunk build 验证练习
    4. 运行脚手架创建答案目录 + 填充完整 main.rs
    5. trunk build 验证答案
    6. 通过 → 下一题；失败 × 3 → 记入 .known_failures
```

Agent 按章节和知识点分段命名。详细 Batch 划分见 [4.3 分配矩阵](#43-各章节-batch-分配矩阵)。

**Ch1 — 4 agents（每 agent 5 题）：**

| Agent | 题号 | 知识点分段 |
| :---: | :--: | :--------- |
| A1a | e01-e05 | 基础元素 |
| A1b | e06-e10 | 组件入门 |
| A1c | e11-e15 | 表达式与条件 |
| A1d | e16-e20 | 构建器与调试 |

**Ch2 — 15 agents（每 agent 5 题）：**

| Agent | 题号 | 子节 |
| :---: | :--: | :--- |
| A2a | e21-e25 | 2.1 信号创建 |
| A2b | e26-e30 | 2.1 信号读取 |
| A2c | e31-e35 | 2.1 信号更新 |
| A2d | e36-e40 | 2.1 信号高级 |
| A2e-A2i | e41-e65 | 2.2 派生信号与 Memo |
| A2j-A2l | e66-e80 | 2.3 Effect 与生命周期 |
| A2m-A2o | e81-e95 | 2.4 条件与列表渲染 |

**Ch3 — 12 agents（每 agent 5 题）：**

| Agent | 题号 | 子节 |
| :---: | :--: | :--- |
| A3a-A3d | e96-e115 | 3.1 Props 与通信 |
| A3e-A3f | e116-e125 | 3.2 Context |
| A3g-A3i | e126-e140 | 3.3 DOM 操作 + NodeRef |
| A3j-A3l | e141-e155 | 3.4 自定义 Hooks |

**Ch4 — 9 agents（每 agent 5 题）：**

| Agent | 题号 | 子节 |
| :---: | :--: | :--- |
| A4a-A4d | e156-e175 | 4.1 Resource |
| A4e   | e176-e180 | 4.2 Suspense |
| A4f   | e181-e185 | 4.2 Transition |
| A4g-A4h | e186-e195 | 4.3 Action |
| A4i   | e196-e200 | 4.4 定时与延时 |

**Ch5 — 10 agents（每 agent 5 题）：**

| Agent | 题号 | 知识点分段 |
| :---: | :--: | :--------- |
| A5a-A5b | e201-e210 | 路由基础 + 嵌套 |
| A5c-A5d | e211-e220 | 参数 + 导航 |
| A5e-A5f | e221-e230 | 布局 + SEO + Meta |
| A5g-A5h | e231-e240 | 高级路由 |
| A5i-A5j | e241-e250 | 路由测试与边界 |

**Ch6 — 8 agents（每 agent 5 题）：**

| Agent | 题号 | 子节 |
| :---: | :--: | :--- |
| A6a-A6c | e251-e265 | 6.1 表单与输入 |
| A6d-A6e | e266-e275 | 6.1 验证与复杂输入 |
| A6f-A6g | e276-e285 | 6.2 样式 |
| A6h   | e286-e290 | 6.3 开发体验 |

**Phase 1 启动命令示例（每个 agent 一条）：**

```
# 启动 Ch2 batch A2a 的 agent
## 任务: 编写 Ch2 batch A2a — 信号创建 (e21-e25, 共 5 题, 10 个文件夹)
## 上下文:
##   - 工作区: c:\code\testruetlearn\leptos-learn\
##   - 章节: 02_signals
##   - 题号范围: e21-e25
##   - 模板: CSR 填空（⭐ 难度每行 TODO，⭐⭐ 补全 50%）
## 执行: 对 e21 到 e25 逐个运行脚手架(练习) → 填充 TODO → trunk build → 脚手架(答案) → 填充答案 → trunk build
## 注意事项:
##   - workspace 已预注册所有 member，无需修改 Cargo.toml
##   - 练习路径: 02_signals/e21_xxx/，答案路径: 02_signals/e21_xxx_answer/
##   - 只创建和修改 02_signals/ 下的文件
##   - 不要碰其他章节的目录
```

### Phase 2: 第 7-8 章（19 agents, ~1-2h）

**Ch7 — 11 agents（SSR，cargo-leptos 模板，每 agent 5 题）：**

| Agent | 题号 | 知识点分段 |
| :---: | :--: | :--------- |
| A7a | e291-e295 | SSR 基础 + Server Functions |
| A7b | e296-e300 | Server Functions 进阶 |
| A7c-A7d | e301-e310 | 认证 + Session |
| A7e-A7f | e311-e320 | 中间件 + CORS |
| A7g-A7h | e321-e330 | 流式渲染 + Hydration |
| A7i-A7j | e331-e340 | 部署 + Docker |
| A7k | e341-e345 | CI/CD + 运维 |

**Ch8 — 8 agents（CSR，trunk 模板，每 agent 5 题）：**

> 启动时机：建议在 Ch7 验证通过后启动。

| Agent | 题号 | 知识点分段 |
| :---: | :--: | :--------- |
| A8a-A8b | e346-e355 | 状态管理模式 |
| A8c-A8d | e356-e365 | 工程化 + 测试 |
| A8e-A8f | e366-e375 | 高级 UI 组件 |
| A8g-A8h | e376-e385 | 浏览器集成 + 监控 |

### Phase 3: 终极项目（2 agent 并行, 各 ~6-8h）

| Agent |      项目      | 步数 |          结构          | 构建命令               |
| ----- | :-------------: | :--: | :--------------------: | ---------------------- |
| A9    | ShopOS 电商后台 |  40  | `cargo-leptos` (SSR) | `cargo leptos build` |
| A10   | NoteFlow 知识库 |  40  |      Trunk (CSR)      | `trunk build`        |

**递进式编写策略——每个 Step 的流程：**

```
1. 从上一步的代码分支创建新分支 (git checkout -b step-A-NN)
2. 增量添加本 step 所需的代码/组件/路由/数据库迁移
3. 编译验证
4. 如果编译失败，修复后重复
5. 合并回主分支 (git merge --squash)
6. 进入下一 step
```

**验证方式：** 每步独立 `trunk build` / `cargo leptos build`，不通过不进下一步。

### Phase 4: 逐章验证 + 全量验证

**4a: 逐章验证（每章写完立即执行，所有章节并行）**

每章所有 batch 写完后，立即启动一个验证 agent。所有章的验证可同时进行。

|   章节   | 验证内容                              |   预期耗时   |
| :------: | ------------------------------------- | :----------: |
|   Ch1   | trunk build 全部 20 题 + 编号扫描     |    ~10min    |
|   Ch2   | trunk build 全部 75 题 + 编号扫描     |    ~20min    |
|   Ch3   | trunk build 全部 60 题 + 编号扫描     |    ~15min    |
| Ch4-Ch8 | 同上                                  | 各 ~10-20min |
|  ShopOS  | cargo leptos build + 种子数据初始化   |    ~15min    |
| NoteFlow | trunk build + Service Worker 注册检查 |    ~15min    |

**4b: 全量检查（逐章验证全部通过后执行，1 agent, ~1h）**

| 任务                        |     耗时     | 命令                         |
| --------------------------- | :-----------: | ---------------------------- |
| workspace 级`cargo build` |    ~20min    | `cargo build --workspace`  |
| `cargo clippy` 全量       |    ~15min    | `cargo clippy --workspace` |
| 00_preface 导航链接完整性   |     ~5min     | 脚本扫描                     |
| 浏览器预览抽样（10 题）     |    ~20min    | 手动                         |
| **合计**              | **~1h** |                              |

---

## 8. 目录结构产出约定

### 8.1 工作区注册

```toml
[workspace]
members = [
    "00_preface",
    # 第 1 章 — 练习
    "01_basics/e01_hello_world",
    "01_basics/e02_html_elements",
    # ... 练习路径
    "01_basics/e20_builder_advanced",
    # 第 1 章 — 参考答案
    "01_basics/e01_hello_world_answer",
    "01_basics/e02_html_elements_answer",
    # ... 答案路径
    "01_basics/e20_builder_advanced_answer",
    # 后续章节同理，每个题目注册两个路径
    ...
]
```

脚手架脚本 `new-exercise.ps1` 自动维护 `members` 列表，每个题目注册两条路径（练习 + 答案）。

### 8.2 命名规范

每个题目创建两个目录：

```
e{NN}_{english_name}/            # 练习目录
  ├── Cargo.toml                  # name = "eNN_english_name"
  ├── index.html
  └── src/
      └── main.rs                 # 仅含 TODO，无答案

e{NN}_{english_name}_answer/      # 参考答案目录
  ├── Cargo.toml                  # name = "eNN_english_name_answer"
  ├── index.html
  └── src/
      └── main.rs                 # 完整可编译代码，无 TODO
```

- `eNN` 固定 3 位数字（`e001`, `e021`, `e385`）
- `english_name` 小写 kebab-case
- 答案目录名 = 练习目录名 + `_answer` 后缀
- 第 7 章 SSR 练习使用 `eNNN`（无前导零）

### 8.3 端口分配约定

`trunk serve` 端口范围分配：练习从 3000 起，参考答案从 4000 起（即练习端口 + 1000）。

|    章节    | 练习端口范围 | 答案端口范围 | 说明 |
| :--------: | :----------: | :----------: | :--- |
| 00_preface |     3000     |      —       | 导航首页（固定，无答案） |
|    Ch1    |  3001-3020  |  4001-4020  | 每题含练习 + 答案两个端口 |
|    Ch2    |  3021-3095  |  4021-4095  |  |
|    Ch3    |  3096-3155  |  4096-4155  |  |
|    Ch4    |  3156-3200  |  4156-4200  |  |
|    Ch5    |  3201-3250  |  4201-4250  |  |
|    Ch6    |  3251-3290  |  4251-4290  |  |
|    Ch7    | 自动分配 | 自动分配 | `cargo leptos serve` 自动分配 |
|    Ch8    |  3301-3340  |  4301-4340  |  |

---

## 9. 错误处理与重试策略

### 9.1 编译错误处理

```
┌─ trunk build 失败 ─┐
│                    │
│  分析错误信息       │
│  修复代码           │
│  重试 trunk build  │
│                    │
│  第 3 次仍失败 →    │
│  跳过并记录到       │
│  .known_failures    │
└────────────────────┘
```

### 9.2 常见失败模式及修复

| 失败模式                    | 原因             | 修复                                         |
| --------------------------- | ---------------- | -------------------------------------------- |
| `cannot find crate`       | workspace 未注册 | 追加到`[workspace].members`                |
| `use of undeclared crate` | 依赖缺失         | 在`Cargo.toml` 添加 `workspace = true`   |
| `expected function`       | nightly 语法     | 确认`rust-toolchain.toml` 存在             |
| `trunk` 命令未找到        | 未安装           | `cargo install trunk`                      |
| WASM 目标未安装             | 工具链           | `rustup target add wasm32-unknown-unknown` |

### 9.3 失败记录格式

`.known_failures` 文件：

```json
[
  {
    "exercise": "02_signals/e45_memo_vs_closure",
    "error": "type mismatch: expected i32, found ()",
    "attempts": 3,
    "status": "skipped",
    "timestamp": "2026-07-26T10:30:00Z"
  }
]
```

失败练习在 Phase 4 全量验证阶段集中修复。

---

## 10. 进度追踪

### 10.1 进度文件

每个 agent 在编写过程中更新 `progress.json`：

```json
{
  "chapters": {
    "01_basics": { "total": 20, "written": 15, "verified": 14, "failed": 1 },
    "02_signals": { "total": 75, "written": 0, "verified": 0, "failed": 0 }
  },
  "projects": {
    "shopos": { "total": 40, "written": 0, "verified": 0 },
    "noteflow": { "total": 40, "written": 0, "verified": 0 }
  },
  "last_updated": "2026-07-26T10:00:00Z"
}
```

### 10.2 Agent 完成确认清单

每个 agent 完成章节后输出：

```
Agent Ch2 完成报告
────────────────────
章节: 02_signals
题量: 75/75
编译通过: 73
编译失败: 2 (e45, e63 — 见 .known_failures)
耗时: 3h 45min
```

---

## 11. 实际执行指令样本

### 11.1 启动一个 batch agent（5 题，10 个文件夹）

```
## 任务: 编写第 2 章 batch A2a — 信号创建 (e21-e25, 5 题)

## 上下文
工作区: c:\code\testruetlearn\leptos-learn\
章节: 02_signals
题号: e21-e25（共 5 题，需创建 10 个文件夹）

## 模板
使用 scripts/new-exercise.ps1 创建每个练习和答案 crate。

## 执行流程
对 e21 到 e25 每道题:

1. 从表格读取题目信息（编号、名称、难度、知识点）
2. 创建练习 crate:
   powershell scripts/new-exercise.ps1 -Chapter "02_signals" -Number NN -Name "english_name" -Type "exercise"
3. 编辑练习 src/main.rs:
   - 组件名: ExerciseNN
   - 难度 ⭐: 每行 TODO → 填空
   - 难度 ⭐⭐: 关键 TODO → 补全 50%
   - 难度 ⭐⭐⭐: 仅目标描述
   - 不含答案代码
4. trunk build 验证练习
5. 创建答案 crate:
   powershell scripts/new-exercise.ps1 -Chapter "02_signals" -Number NN -Name "english_name" -Type "answer"
6. 编辑答案 src/main.rs: 写完整可编译代码（无 TODO）
7. trunk build 验证答案
8. 通过后提交，进入下一题

## 特别注意
- 本 batch 是 e21-e25，只涉及 2.1 信号创建章节
- 每道题必须创建两个文件夹（练习 + 答案），分别编译验证
- 练习只含 TODO，答案不含 TODO
- workspace 已预注册所有 member，无需修改 Cargo.toml
- 只创建和修改 02_signals/ 下的文件
```

### 11.2 启动项目 agent

```
## 任务: 编写项目 A — ShopOS 电商后台

## 上下文
工作区: c:\code\testruetlearn\leptos-learn\projects\shopos\
步数: 40 (A-01 到 A-40)
结构: cargo-leptos SSR 项目

## 特别注意
- 强依赖链: 每一步依赖前一步
- Step A-04 Schema 设计错误 → 后续所有 CRUD 崩
- Step A-12/A-13 认证 → 后续所有受保护路由依赖
- Step A-20 事务 → 库存扣减错误会导致超卖

## 执行流程
从 A-01 开始，串行到 A-40。

每步执行:
1. 阅读 leptos-learn-plan.md 中该 Step 的描述和知识点
2. 在现有代码基础上增量开发
3. cargo leptos build 验证
4. 失败则修复（最多 5 次）
5. 通过后进入下一步
```

---

## 12. 时间线估计

```
Phase 0                          Phase 1 (Ch1-6: 58 agents 同时跑)
   │                                  │
   ▼         2h                       ▼      ~1-2h
   ├───────────────────────────────────├────────────────
   │                                  │
   │  脚手架                          │ ░░░ 58 agents ░░░
   │  预注册 members                   │ Ch1  ██
   │  exercise-data.json              │ Ch2  ██████████████
   │                                  │ Ch3  ████████████
   │                                  │ Ch4  ████████
   │                                  │ Ch5  █████████
   │                                  │ Ch6  ███████
   └──────────────────────────────────┴────────────────

   Phase 2 (Ch7-8: 19 agents)        Phase 3 (项目并行)
         │                                │
         ▼      ~1-2h                     ▼     ~6-8h
   ├───────────────────────────────────────────────
         │                                │
   Ch7 ██████████████████           ShopOS  ████████████████████████████
   Ch8 ██████████████               NoteFlow ████████████████████████████

   Phase 4 (逐章验证并行 + 全量)
         │
         ▼      ~2h
   ├───────────────
   Ver Ch1 ██
   Ver Ch2 ████
   Ver Ch3 ███
   ...      (并行)
   全量检查 ████

   墙钟总时间: ~12-16h
   相比单 agent 串行 ~80h，加速比 ~6x
```

---

## 13. 关键风险与缓解

| 风险                              |  概率  | 影响 | 缓解措施                                                                               |
| --------------------------------- | :----: | :--: | -------------------------------------------------------------------------------------- |
| Agent 输出不一致（风格/质量）     |   高   |  中  | 统一模板 + 脚手架脚本 + 参考答案结构规范                                               |
| 编译失败循环（单题重试超过 3 次） |   中   |  低  | 跳过并记入`.known_failures`，Phase 4 统一修复                                        |
| 项目递进中前一步破坏后一步        |   高   |  高  | 每步独立 git commit，回滚成本低                                                        |
| 多个 agent 同时修改同一文件       | 已消除 |  —  | Phase 0 预注册所有 workspace members；每 agent 仅写自己 batch 的目录，无共享文件写冲突 |
| Batch 边界题号重叠或遗漏          |   中   |  高  | Phase 0 的 exercise-data.json 明确定义每个 batch 的起止号，验证 agent 脚本扫描连续性   |
| Leptos API 版本更新导致模板失效   |   低   |  高  | rust-toolchain.toml 锁定 nightly 日期                                                  |
| trunk build 时间过长              |   中   |  低  | 使用`--release` 仅用于最终验证，开发阶段用 debug                                     |

---

## 附录 A：new-exercise.ps1 脚手架脚本规范

该脚本通过 `-Type` 参数区分**练习**与**参考答案**两种模式。

```powershell
# scripts/new-exercise.ps1
param(
    [Parameter(Mandatory)] [string]$Chapter,       # e.g. "01_basics"
    [Parameter(Mandatory)] [int]$Number,           # e.g. 21
    [Parameter(Mandatory)] [string]$Name,          # e.g. "signal_create"
    [Parameter(Mandatory)] [string]$Type,          # "exercise" 或 "answer"
    [string]$Difficulty = "⭐",                    # ⭐ / ⭐⭐ / ⭐⭐⭐
    [string]$Template = "csr"                      # csr / ssr
)

# 根据 Type 确定目录名和包名
$suffix = if ($Type -eq "answer") { "_answer" } else { "" }
$dirName = "e$('{0:D2}' -f $Number)_$Name$suffix"
$pkgName = "e$('{0:D2}' -f $Number)_${Name}${suffix}"
$exerciseDir = "leptos-learn/$Chapter/$dirName"

# 1. 创建目录结构
New-Item -ItemType Directory -Path "$exerciseDir/src" -Force

# 2. 生成 Cargo.toml
@"
[package]
name = "$pkgName"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos.workspace = true
"@ | Out-File -FilePath "$exerciseDir/Cargo.toml" -Encoding UTF8

# 3. 生成 index.html
$title = if ($Type -eq "answer") { "练习 $Number — 参考答案" } else { "练习 $Number" }
@"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="utf-8"/>
    <title>$title</title>
</head>
<body></body>
</html>
"@ | Out-File -FilePath "$exerciseDir/index.html" -Encoding UTF8

# 4. 从模板生成 main.rs
# - Type=exercise: 根据难度选择 TODO 模板（仅骨架，无答案）
# - Type=answer:   使用完整代码模板（含完整实现，无 TODO）
# ...（模板填充逻辑）

# 5. 注册 workspace member（Phase 0 已预注册，此步可跳过或幂等追加）
if (-not (Select-String -Path "leptos-learn/Cargo.toml" -Pattern "\`"$exerciseDir\`"" -Quiet)) {
    # 幂等追加
}

Write-Host "✓ Created $exerciseDir ($Type)"
```

**脚本功能清单：**

- [X] 支持 `-Type exercise|answer` 两种模式
- [X] 练习模式：生成含 TODO 的骨架代码
- [X] 答案模式：生成完整可编译代码
- [X] 自动设置目录名和包名（答案加 `_answer` 后缀）
- [X] 注册 workspace member（幂等追加）
- [X] 支持 CSR/SSR 两种模板

---

## 附录 B：参考答案 crate 结构规范

参考答案是一个**完全独立、可编译的 crate**（不是练习文件中的注释块）。

### B.1 文件结构

```
e{NN}_{name}_answer/
  ├── Cargo.toml        # package.name = "eNN_name_answer"
  ├── index.html         # <title>练习 NN — 参考答案</title>
  └── src/
      └── main.rs        # 完整可运行代码
```

### B.2 main.rs 规范

```rust
// ============================================================
// 练习 N: 题目名称 — 参考答案
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // --- 完整实现，无 TODO ------------------------------------
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>当前值: {count}</p>
            <button on:click=move |_| set_count.update(|c| *c += 1)>
                增加
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
```

**规范要求：**

1. **完整可编译** — `trunk build` 必须零错误零警告
2. **无 TODO** — 参考答案不包含任何 `TODO:` 或 `FIXME:` 标记
3. **与练习同名但加 `_answer` 后缀** — 便于用户对照
4. **代码可直接运行** — 用户能启动 `trunk serve` 查看效果
5. **注释清晰** — 保留关键步骤注释，方便理解思路

---

## 附录 C：项目递进式编写工作流

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│ Step N-1 │ ──→ │  Step N  │ ──→ │ Step N+1 │ ──→ │ Step N+2 │
│ (已验证)  │     │  编写中   │     │ (未开始)  │     │ (未开始)  │
└──────────┘     └────┬─────┘     └──────────┘     └──────────┘
                      │
               ┌──────┴──────┐
               │  build 失败  │ ←── 回退到 Step N-1
               │  (修复)      │     修复后重试
               └──────┬──────┘
                      │
               ┌──────┴──────┐
               │  build 通过  │
               │  git commit  │
               │  进入 N+1    │
               └─────────────┘
```

每个 Step 独立提交，方便回滚。项目 git 分支策略：

```
main
├── step-A-01  (脚手架)
├── step-A-02  (布局+主题)
├── step-A-03  (导航)
├── ...        (增量提交)
└── step-A-40  (最终)
```
