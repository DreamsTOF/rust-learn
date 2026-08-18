# 练习 01 答案讲解：环境准备与项目创建

> **用法**：卡住时再看本页。每一处 diff 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/main.ts`（前端），其余全是脚手架。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | `EnvCheck` 结构体已预填 | `#[tauri::command]` 标注、5 项检查项、注册 |
| `src/main.ts` | 接口与占位代码已给 | import、真实 invoke、完整渲染模板、className |

> **前端基础提示**：本练习的前端用 Vanilla TS（原生 TS 操作 DOM，无框架）。如果 `querySelector`、`innerHTML`、`map().join()` 这些语法不熟，先读 [Vanilla TS 速成（给 Vue 开发者）](00_vanilla_ts_primer.md)——用 Vue 语法一一对照讲解。下文每个前端 TODO 处都有对应的 **Vanilla TS 注解**。

## lib.rs TODO 1：`serde::Serialize` 派生（已预填）

### 练习版这里是什么

```rust
#[derive(serde::Serialize)]
struct EnvCheck {
    name: String,
    ok: bool,
    detail: String,
}
```

### 为什么需要它（这一处不用填，但要理解）

命令的返回值要穿越进程边界，而进程间只能传字节，所以必须序列化。`String`、`bool` 等基础类型自带序列化能力；**自定义结构体要自己声明"怎么序列化"**——`#[derive(serde::Serialize)]` 让编译器自动生成序列化代码。

没有它 → 编译报错：`Vec<EnvCheck>` 无法穿越进程边界。

**序列化规则**：结构体字段名就是 JSON 的 key：

```
Rust: EnvCheck { name, ok, detail }
        │ serde 序列化（字段名 → JSON key）
        ▼
IPC:  {"name":"Rust 工具链","ok":true,"detail":"cargo 1.8x+ / rustc stable"}
        │ 反序列化（JSON key → TS 属性）
        ▼
TS:   { name: "...", ok: true, detail: "..." }
```

### 回查文档

[《练习 01》第 2 节：返回值：第一次出现结构体](01_hello_world.md#sec-01-return-struct)。

## lib.rs TODO 2：添加 `#[tauri::command]` 标注

### 练习版这里是什么

```rust
fn check_environment() -> Vec<EnvCheck> {
    vec![]
}
```

### 答案版填了什么

```rust
#[tauri::command]
pub fn check_environment() -> Vec<EnvCheck> {
    vec![ /* 5 项，见 TODO 3 */ ]
}
```

### 为什么

`#[tauri::command]` 是一个**属性宏**：Rust 语言本身没有"命令"这个概念，这个宏在编译期把普通函数"升级"成框架可以识别和调用的单元——自动生成参数反序列化、返回值序列化、错误转换和注册信息。

**不加会发生什么**：函数编译通过，但前端 `invoke("check_environment")` 时后端查无此命令，报"命令未找到"。

### 回查文档

[《练习 01》第 2 节：命令与普通函数的区别](01_hello_world.md#sec-01-command-vs-fn)、[为什么用宏？](01_hello_world.md#sec-01-command-macro)。

## lib.rs TODO 3：补充 5 项检查项

### 练习版这里是什么

```rust
fn check_environment() -> Vec<EnvCheck> {
    vec![]   // ← 你的作业在这
}
```

### 答案版填了什么

```rust
fn check_environment() -> Vec<EnvCheck> {
    vec![
        EnvCheck {
            name: "Rust 工具链".into(),
            ok: true,
            detail: "cargo 1.8x+ / rustc stable".into(),
        },
        EnvCheck {
            name: "Node.js 与 pnpm".into(),
            ok: true,
            detail: "Node 18+ / pnpm 9+".into(),
        },
        EnvCheck {
            name: "WebView2 Runtime".into(),
            ok: true,
            detail: "Windows 11 自带，Windows 10 需安装".into(),
        },
        EnvCheck {
            name: "Tauri CLI".into(),
            ok: true,
            detail: "cargo tauri 2.x（或 pnpm dlx tauri）".into(),
        },
        EnvCheck {
            name: "Rust 目标链".into(),
            ok: true,
            detail: "x86_64-pc-windows-msvc".into(),
        },
    ]
}
```

### 为什么

- **TODO 注释就是作业单**：注释里"建议 5 项"和每项格式（`EnvCheck { name: "...".into(), ok: true, detail: "...".into() }`）已经把答案骨架给全了，你只需要按格式填充内容——这是全部练习的通用规律。
- **`.into()` 是什么**：字符串字面量 `&str` 到 `String` 的隐式转换（`String` 实现了 `From<&str>`）。结构体字段声明的是 `String`，字面量是 `&str`，用 `.into()` 转换。
- **为什么 `ok` 硬编码 `true`**：真实项目应该读取 `rustc --version`、`node --version` 做动态判断，本练习以教学为目的直接给结论。你可以在"破坏性验证"里把它改成 `false` 看效果。

### 回查文档

[《练习 01》第 2 节：返回值：第一次出现结构体](01_hello_world.md#sec-01-return-struct)。

## lib.rs TODO 4：注册命令

### 练习版这里是什么

```rust
tauri::Builder::default()
    // TODO: 注册 check_environment 命令，让前端可以 invoke
    // 提示: .invoke_handler(tauri::generate_handler![check_environment])
    .run(tauri::generate_context!())
    .expect("启动 Tauri 应用失败");
```

### 答案版填了什么

```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_environment])
    .run(tauri::generate_context!())
    .expect("启动 Tauri 应用失败");
```

### 为什么

- **`generate_handler![...]`** 把列出的命令打包成一张"命令注册表"（宏生成注册代码）
- **`.invoke_handler(...)`** 把这张表挂到 Builder 上
- 前端发来的每条调用请求，都在这张表里**按名字查找**——命令写了 `#[tauri::command]` 但忘了注册，前端照样报"命令未找到"。这是每个练习都会埋一次的坑。

### 回查文档

[《练习 01》第 2 节：注册：generate_handler! 与 invoke_handler](01_hello_world.md#sec-01-register)。

## main.ts TODO 1：取消 import 注释

### 练习版这里是什么

```typescript
// import { invoke } from "@tauri-apps/api/core";
```

### 答案版填了什么

```typescript
import { invoke } from "@tauri-apps/api/core";
```

### 为什么

`invoke` 是前端调用后端的唯一入口（调用层）。不 import，运行时 `invoke` 就是未定义变量，`render()` 抛 ReferenceError——页面直接显示"调用失败"。

### 回查文档

[《练习 01》第 3 节：invoke() — 前端如何调用后端？](01_hello_world.md#sec-01-invoke)。

## main.ts TODO 2：调用真实命令

### 练习版这里是什么

```typescript
const checks: EnvCheck[] = [];   // 占位：保持可编译，页面无内容
```

### 答案版填了什么

```typescript
const checks = await invoke<EnvCheck[]>("check_environment");
```

### 为什么

- **`invoke` 返回 Promise，必须 `await`**：IPC 是进程间消息传递，跟 `fetch` 同一个模型——消息要序列化、跨进程传输、后端执行、结果传回来。同步等待会卡死 WebView 的单线程事件循环。
- **`<EnvCheck[]>` 泛型**：只是 TS 侧的**类型声明**，运行时不会校验。字段名写错，显示的就是 `undefined`——前后端联调最常见的错位来源。
- **`"check_environment"` 是命令名（字符串）**：后端注册表按它查找函数，必须和 `generate_handler![...]` 里的一致。

> **Vanilla TS 注解**：`await` 是"等结果回来"。`invoke` 返回 Promise（和 `fetch` 一样），**不 `await` 的话 `checks` 拿到的是 Promise 对象而不是数据**。`<EnvCheck[]>` 是泛型，告诉 TS"返回值是 EnvCheck 数组"。详见 [异步：async/await 与错误兜底](00_vanilla_ts_primer.md#sec-ts-async)、[类型：interface 与泛型](00_vanilla_ts_primer.md#sec-ts-types)。

### 回查文档

[《练习 01》第 3 节：为什么必须异步？](01_hello_world.md#sec-01-invoke-async)、[类型对应](01_hello_world.md#sec-01-type-map)。

## main.ts TODO 3：完善渲染模板

### 练习版这里是什么

```typescript
listEl!.innerHTML = checks.map((c) => `<li>${c.name}</li>`).join("");
```

### 答案版填了什么

```typescript
listEl!.innerHTML = checks
  .map(
    (c) =>
      `<li class="${c.ok ? "ok" : "warn"}">
        <span class="badge">${c.ok ? "✓" : "!"}</span>
        <strong>${c.name}</strong>
        <span class="detail">${c.detail}</span>
      </li>`
  )
  .join("");
```

### 为什么

- **练习版只显示名字，答案版显示完整信息**：`ok` 决定样式类和徽标（✓ / !），`detail` 决定说明文字——这就是"命令返回结构体而不是拼字符串"的意义：数据有了结构，前端才能分别使用。
- 模板字符串里的 `${c.ok ? "ok" : "warn"}` 三元表达式是全部诀窍。

> **Vanilla TS 注解**：这一行是 Vue `v-for` 的手写版，拆开看：
>
> ```typescript
> checks.map((c) => `<li class="${c.ok ? "ok" : "warn"}">...</li>`).join("")
> ```
>
> 1. **模板字符串**（反引号 `` ` ``）：`${变量}` 把值插进字符串
> 2. **`.map((c) => ...)`**：把每个检查项 `c` 变成一段 `<li>...</li>` 字符串（数组 → 新数组）
> 3. **三元 `c.ok ? "ok" : "warn"`**：一行 `if/else`，`ok` 为 true 用 `"ok"` 否则 `"warn"`
> 4. **`.join("")`**：把字符串数组拼成一个字符串——**不 join 会带逗号**，这是新手最常见的错
>
> Vue 里 `v-for` + `:class` + `{{ }}` 干的事，在这里全由这一行手写完成。详见 [渲染列表：map + join + 模板字符串](00_vanilla_ts_primer.md#sec-ts-list)。

### 回查文档

[《练习 01》第 3 节：答案版 main.ts 解读](01_hello_world.md#sec-01-answer-main-ts)。

## main.ts TODO 4：就绪判断与样式

### 练习版这里是什么

```typescript
const ready = checks.length > 0 && checks.every((c) => c.ok);
statusEl!.textContent = ready ? "环境就绪，可以开始练习 🎉" : "存在未满足项，请先处理";
```

### 答案版填了什么

```typescript
const ready = checks.every((c) => c.ok);
statusEl!.textContent = ready ? "环境就绪，可以开始练习 🎉" : "存在未满足项，请先处理";
statusEl!.className = ready ? "status ok" : "status err";
```

### 为什么

- **`checks.every((c) => c.ok)`**：全部 `ok` 才显示"环境就绪"。练习版的 `checks.length > 0 &&` 是防御占位阶段的写法（空列表时显示"未满足"而非"就绪"），填完后两种写法都成立，答案版用更直接的 `every`。
- **`statusEl!.className = ...` 是练习版缺失的一行**：页面 CSS 里 `.status.ok` 是绿色、`.status.err` 是红色。不切 className，状态行永远是默认样式——UI 反馈失效。**改完记得补上**。

> **Vanilla TS 注解**：`textContent` 是"把字符串当纯文字放进去"（对比 `innerHTML` 的"当 HTML 解析"）——这里显示的是状态文字，用 `textContent` 就够。`className` 是**整体替换** class 属性（Vue 的 `:class` 在此的等价物）。`render().catch(...)` 是"顶层调用 + 失败兜底"：后端报错时显示错误而不是白屏。详见 [更新内容：textContent vs innerHTML](00_vanilla_ts_primer.md#sec-ts-render)、[样式类](00_vanilla_ts_primer.md#sec-ts-class)、[异步与错误兜底](00_vanilla_ts_primer.md#sec-ts-async)。

### 回查文档

[《练习 01》第 3 节：答案版 main.ts 解读](01_hello_world.md#sec-01-answer-main-ts)。

## 验收标准

```bash
cd 01_getting_started/e01_hello_world
cargo tauri dev
```

窗口出现 5 项带 ✓ 的环境检查清单，状态行显示"环境就绪，可以开始练习 🎉"。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把某一项 `ok: true` 改成 `ok: false` → 该项显示"!"和警告样式，状态行变为"存在未满足项"
- 把命令名改成 `"check_environment_xxx"` → 页面显示"调用失败"
- 把注册从 `generate_handler!` 里删掉 → 同样的"调用失败"（验证注册的必要性）