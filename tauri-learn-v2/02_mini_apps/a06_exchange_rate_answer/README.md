# 练习 A06: 汇率查询（答案版）

**目标：** 查询"今天 100 美元 = 多少人民币"。后端用 **reqwest**（http 插件 re-export）请求汇率 API，**10 秒超时**兜底；结果用 **Store 插件**缓存 1 小时，再次查询不重复请求。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - `read_cache` 命中判断（`now_ts() - fetched_at < CACHE_TTL_SECS`）
  - `tokio::time::timeout(10s, client.get(url).send())` + `response.text()`
  - `json.get("rates").and_then(...).and_then(as_f64)` 取 rate
  - `write_cache(...)` + 返回 `from_cache: false`
  - `.plugin(tauri_plugin_http::init())` + `.plugin(tauri_plugin_store::Builder::default().build())` + 登记命令
- `src/App.tsx`
  - `invoke<RateInfo>("get_rate", { from, to })`，按 `from_cache` 显示"来自缓存/来自网络"

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a06_exchange_rate_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1433
- identifier: com.taurilearn.a06a
