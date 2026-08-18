# 练习 A06: 汇率查询（练习版）

**目标：** 查询"今天 100 美元 = 多少人民币"。后端用 **reqwest**（http 插件 re-export）请求汇率 API，**10 秒超时**兜底；结果用 **Store 插件**缓存 1 小时，再次查询不重复请求。

**新增知识：** http 插件（Rust 端 reqwest）、`tokio::time::timeout` 超时、store 插件（`app.store().get/set/save`）、JSON 解析（serde_json）。

**TODO（共 7 处）：**

- `src-tauri/src/lib.rs`（5 处）
  - 步骤 1：缓存命中检查
  - 步骤 2：HTTP 请求 + 超时
  - 步骤 3：解析 JSON 取 rate / date
  - 步骤 4：写缓存并返回
  - 步骤 5：注册 http/store 插件 + 登记命令
- `src/App.tsx`（2 处）
  - 步骤 1：导入 `invoke`
  - 步骤 2：调 `get_rate` 并展示结果

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1432
- identifier: com.taurilearn.a06
