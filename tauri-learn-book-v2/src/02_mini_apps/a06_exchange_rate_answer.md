# 练习 A06 答案讲解：汇率查询

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 7 处 TODO。`read_cache` / `write_cache` 辅助函数已给。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 缓存读写、`RateInfo` 已给 | TTL 判断、HTTP+超时、JSON 解析、写缓存返回、插件注册 |
| `src/App.tsx` | 界面已给 | `invoke` 导入 + 调 `get_rate` |

## lib.rs TODO 1：缓存命中检查

### 练习版这里是什么

```rust
#[tauri::command]
async fn get_rate(app: AppHandle, from: String, to: String) -> Result<RateInfo, String> {
    // TODO: 缓存命中检查
    // TODO: HTTP 请求...
    // ...
}
```

### 答案版填了什么

```rust
    if let Some((rate, date, fetched_at)) = read_cache(&app, &from, &to) {
        if now_ts() - fetched_at < CACHE_TTL_SECS {
            return Ok(RateInfo { from, to, rate, date, from_cache: true });
        }
    }
```

### 为什么

- `read_cache` 返回 `Option<(rate, date, fetched_at)>`：没缓存（或格式不对）就是 `None`，走网络分支
- `now_ts() - fetched_at < 3600`：未过期才用；过期也落到网络分支（会覆盖写回）
- `from_cache: true` 让前端显示"来自缓存"

### 回查文档

[第 4 节：Store 缓存与 TTL](a06_exchange_rate.md#sec-a06-cache)。

## lib.rs TODO 2：HTTP 请求 + 超时

### 答案版填了什么

```rust
    let url = format!("https://api.frankfurter.app/latest?from={from}&to={to}");
    let client = tauri_plugin_http::reqwest::Client::new();
    let response = tokio::time::timeout(Duration::from_secs(10), client.get(&url).send())
        .await
        .map_err(|_| "请求超时（10 秒）".to_string())?
        .map_err(|e| format!("请求失败：{e}"))?;
    let text = response.text().await.map_err(|e| format!("读取响应失败：{e}"))?;
```

### 为什么

- `tauri_plugin_http::reqwest::Client`：http 插件 Rust 端 re-export reqwest，`Client::new()` 创建客户端
- `timeout(10s, future)`：**超时错误（`Elapsed`）和请求错误（reqwest `Error`）是两种错误**，链式 `map_err` 分别翻译
- `.await` 后 `?`：任一失败提前返回，前端收到人话错误

### 回查文档

[第 1 节：HTTP 请求](a06_exchange_rate.md#sec-a06-http)、[第 2 节：超时](a06_exchange_rate.md#sec-a06-timeout)。

## lib.rs TODO 3：解析 JSON

### 答案版填了什么

```rust
    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("解析响应失败：{e}"))?;
    let rate = json
        .get("rates")
        .and_then(|r| r.get(&to))
        .and_then(|v| v.as_f64())
        .ok_or_else(|| format!("响应里没有 {to} 的汇率"))?;
    let date = json["date"].as_str().unwrap_or("").to_string();
```

### 为什么

- `get` 链式取值，任何一环缺失返回 `None` → `ok_or_else` 给"响应里没有 {to} 的汇率"
- 用 `get` 而不是 `json["rates"][&to]` 直接索引：索引缺失会 panic，`get` 不会
- `as_str().unwrap_or("")`：日期取不到就空串兜底（不阻断整个命令）

### 回查文档

[第 5 节：解析 JSON](a06_exchange_rate.md#sec-a06-json)。

## lib.rs TODO 4：写缓存并返回

### 答案版填了什么

```rust
    write_cache(&app, &from, &to, rate, &date)?;
    Ok(RateInfo { from, to, rate, date, from_cache: false })
```

### 为什么

- `write_cache`（已给）：`set` 改内存 + `save` 落盘，存 `{ rate, date, fetched_at }`
- 返回 `from_cache: false`——前端显示"来自网络"

### 回查文档

[第 4 节：写缓存](a06_exchange_rate.md#sec-a06-cache)。

## lib.rs TODO 5：注册插件 + 登记命令

```rust
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_rate])
```

- http 插件让 Rust 端能拿到 reqwest；store 插件提供 `app.store(...)`
- 命令必须登记，前端按名字找（老规矩）

## App.tsx TODO 1-2：invoke + 展示

### 答案版填了什么

```typescript
import { invoke } from "@tauri-apps/api/core";
// ...
const info = await invoke<RateInfo>("get_rate", { from, to });
setInfo(info);
setStatus(info.from_cache ? "来自缓存（1 小时内）" : "来自网络");
```

### 为什么

- `{ from, to }` 的 key 等于 Rust 参数名
- `info.from_cache` 决定状态行文案——后端把"是否缓存"作为字段告诉前端，前端只负责显示

## 验收标准

```bash
cd 02_mini_apps/a06_exchange_rate
cargo tauri dev
```

USD → CNY 查询 → 显示"1 USD = 7.xxxx CNY"和换算结果，状态行"来自网络"；再点一次 → 秒回，"来自缓存（1 小时内）"。断开网络再查 → "查询失败: 请求超时/请求失败"，应用不崩。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把 `timeout` 删掉 → 断网时请求可能一直挂住（验证超时的意义）
- 把 `CACHE_TTL_SECS` 改成 `0` → 每次都是"来自网络"（验证 TTL 的作用）
- 把 `map_err(|_| "请求超时")` 改成 `unwrap_or` 之类的处理 → 观察错误信息变乱（验证错误翻译）
- 把 `get("rates")` 改成 `["rates"]` 直接索引 → API 响应缺字段时 panic（验证 get 链的安全）

## 升级挑战（可选）

- 加"刷新"按钮：强制忽略缓存（给 `get_rate` 加个 `force: bool` 参数）
- 加多个币种批量查询：一次请求拿 `rates` 整张表，本地算任意两币的交叉汇率
