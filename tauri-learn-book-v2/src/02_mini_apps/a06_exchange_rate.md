# 练习 A06: 汇率查询

## 为什么要学这个

前面的应用都只跟本地打交道。但真实的桌面应用经常要**连外部网络**：查汇率、拉天气、同步数据。这一章要回答三个问题：

1. **Rust 后端怎么发 HTTP 请求？** —— reqwest（http 插件 re-export）怎么用？
2. **网络请求会怎样失败？** —— 超时、断网、响应格式不对，怎么让每一环的失败都可读？
3. **为什么要缓存？** —— 汇率一小时变不了多少，每次都重新请求值得吗？Store 插件怎么存？

学完你会：后端发请求 + 超时兜底 + 本地缓存，拼出一个"又快又稳"的联网应用。

---

## 从问题出发

练习 A06 要做的事：**查"今天 100 美元 = 多少人民币"，再次查询时秒回。**

**核心矛盾：** 汇率数据在**外部的 API 服务器**上，应用要跨网络拿数据。但网络请求有天然的不确定性：

1. **慢**——可能几秒甚至更久；
2. **会失败**——断网、服务器宕机、响应格式变了；
3. **贵**——每次都重新请求，浪费流量和等待。

所以本课后端命令做三件事：**发请求（reqwest）→ 加超时（timeout）→ 缓存结果（Store）**。

```text
前端 (React)                         Rust 进程
┌─────────────────────┐  invoke    ┌────────────────────────────────┐
│ 选货币 + 输入金额     │ ────────► │ get_rate                        │
│ 显示汇率与结果        │ ◄──────── │  ① 查缓存（1 小时内）→ 直接返回  │
│ 显示"缓存/网络"       │           │  ② reqwest 请求 API（10s 超时） │
└─────────────────────┘           │  ③ 解析 JSON → 写缓存 → 返回     │
                                  └────────────────────────────────┘
```

---

<a id="sec-a06-http"></a>
## 1. HTTP 请求 — reqwest

Tauri 的 http 插件在 Rust 端 **re-export 了 reqwest**（Rust 生态最流行的 HTTP 客户端）。所以：

```rust
// http 插件注册（练习版已配好）
tauri::Builder::default().plugin(tauri_plugin_http::init())

// 命令里直接用它 re-export 的 reqwest
let client = tauri_plugin_http::reqwest::Client::new();
let url = format!("https://api.frankfurter.app/latest?from={from}&to={to}");
let response = client.get(&url).send().await.map_err(|e| format!("请求失败：{e}"))?;
let text = response.text().await.map_err(|e| format!("读取响应失败：{e}"))?;
```

- `Client::new()`：创建客户端（线程安全，可复用）
- `client.get(url).send().await`：发 GET 请求，返回 `Response`
- `response.text().await`：把响应体读成字符串（后续再解析 JSON）

> **和 A02 的类比：** fs 插件 Rust 端直接基于 `std::fs`（不走权限）；http 插件 Rust 端直接是 reqwest（不走 scope）。**Rust 端用插件的"底层"是直通的，权限是给前端 JS 调用插件时用的。** 请求的目标地址由你的代码决定，Tauri 不拦。

<a id="sec-a06-timeout"></a>
## 2. 超时 — `tokio::time::timeout`

网络请求可能"挂死"——服务器不响应但连接不断，`await` 永远不返回。所以必须**限时**：

```rust
let response = tokio::time::timeout(Duration::from_secs(10), client.get(&url).send())
    .await
    .map_err(|_| "请求超时（10 秒）".to_string())?   // 超时：返回 Elapsed 错误
    .map_err(|e| format!("请求失败：{e}"))?;          // 失败：reqwest 错误
```

- `timeout(时长, future)`：给一个 future 限时，超时返回 `Err(Elapsed)`
- **两层错误**：超时是一层（`Elapsed`），请求本身失败又是一层（reqwest 的 `Error`）——所以链式 `map_err` 两次，分别翻译成人话
- `.await` 之后 `?`：任一层失败都提前返回

<a id="sec-a06-error"></a>
## 3. 错误处理 — 把每一步失败都翻译成人话

一个网络命令里，可能失败的点很多。本课的做法是**每一环都 `map_err` 成人话 + `?`**（沿用 A02 的套路）：

```rust
let text = response.text().await.map_err(|e| format!("读取响应失败：{e}"))?;
let json: serde_json::Value =
    serde_json::from_str(&text).map_err(|e| format!("解析响应失败：{e}"))?;
let rate = json
    .get("rates")
    .and_then(|r| r.get(&to))
    .and_then(|v| v.as_f64())
    .ok_or_else(|| format!("响应里没有 {to} 的汇率"))?;
```

- **网络层**（请求/超时/读响应）→ "请求失败/超时/读取响应失败"
- **解析层**（JSON 格式不对）→ "解析响应失败"
- **数据层**（结构对了但缺字段）→ "响应里没有 {to} 的汇率"

前端拿到 `Err` 就显示"查询失败: <原因>"——**用户看到的是人话，不是 panic 或乱码**。

<a id="sec-a06-cache"></a>
## 4. Store 缓存 — 1 小时内不再请求

汇率一小时变不了多少。所以把结果**缓存到本地**：命中就秒回，过期才重新请求。

### 接入 store 插件

```rust
tauri::Builder::default().plugin(tauri_plugin_store::Builder::default().build())

// 命令里通过 StoreExt trait 使用
use tauri_plugin_store::StoreExt;
let store = app.store("rate-cache.json").map_err(|e| format!("打开缓存失败：{e}"))?;
```

### 读写

```rust
// 读：key 是 "USD_CNY" 这种，值是 { rate, date, fetched_at }
let value = store.get(format!("{from}_{to}"));

// 写：set 改内存，save 落盘
store.set(
    format!("{from}_{to}"),
    serde_json::json!({ "rate": rate, "date": date, "fetched_at": now_ts() }),
);
store.save().map_err(|e| format!("写入缓存失败：{e}"))?;
```

- `app.store("rate-cache.json")`：打开（或创建）一个 Store——它本质是一个 JSON 文件
- `get(key) / set(key, value)`：键值对操作（和 JS 的 `localStorage` 心智一致）
- **`save()` 才落盘**——set 只改内存，不 save 重启就丢
- 缓存文件存在应用数据目录，由 store 插件管理

### 命中判断（TTL）

```rust
if let Some((rate, date, fetched_at)) = read_cache(&app, &from, &to) {
    if now_ts() - fetched_at < CACHE_TTL_SECS {   // 1 小时内 → 直接用
        return Ok(RateInfo { from, to, rate, date, from_cache: true });
    }
}
```

- 缓存里多存一个 `fetched_at`（抓取时间戳）
- `now_ts() - fetched_at < 3600`：没过期 → 返回缓存，标记 `from_cache: true`（前端显示"来自缓存"）
- 过期 → 落到网络请求分支，请求成功后**覆盖写回缓存**

> **为什么能判断"多久前抓的"？** `SystemTime::now()` 与 `UNIX_EPOCH` 的差得到秒级时间戳——这是最朴素的时间度量，不需要引入时间库。

<a id="sec-a06-json"></a>
## 5. 解析 JSON — serde_json

响应是 JSON 字符串，用 `serde_json` 解析成树状 `Value`，再按路径取值：

```json
{ "amount": 1.0, "base": "USD", "date": "2025-08-15", "rates": { "CNY": 7.2356 } }
```

```rust
let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("解析响应失败：{e}"))?;
let rate = json.get("rates").and_then(|r| r.get(&to)).and_then(|v| v.as_f64())...;
let date = json["date"].as_str().unwrap_or("").to_string();
```

- `json.get("rates")`：取 `rates` 对象；链式 `.get(&to)` 取具体货币；`.as_f64()` 转数字
- **用 `get` 链而不是直接索引**：路径缺失返回 `None` 而不是 panic，配合 `ok_or_else` 给友好错误
- `as_str().unwrap_or("")`：取不到就用空串兜底

---

## 练习指引

**作业范围：** 动 2 个文件，共 7 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | 缓存命中检查（TTL） |
| `src-tauri/src/lib.rs` | 2 | HTTP 请求 + 10 秒超时 |
| `src-tauri/src/lib.rs` | 3 | 解析 JSON 取 rate / date |
| `src-tauri/src/lib.rs` | 4 | 写缓存 + 返回 |
| `src-tauri/src/lib.rs` | 5 | 注册 http / store 插件 + 登记命令 |
| `src/App.tsx` | 1-2 | 导入 invoke + 调 `get_rate` 并展示 |

**怎么验证：**

```bash
cd 02_mini_apps/a06_exchange_rate
cargo tauri dev
```

选 USD → CNY，输入金额，点查询 → 显示汇率与结果，状态行"来自网络"。**再点一次** → 秒回，状态行"来自缓存（1 小时内）"。断网再查 → 显示"查询失败: 请求超时/请求失败"，不崩溃。

**故意踩坑看效果：** 把 `timeout` 删掉 → 断网时请求可能一直挂住；把缓存 TTL 改成 `0` → 每次都是"来自网络"（等于没缓存）。

---

## 知识点连起来看

```text
reqwest（http 插件）              ← 发请求：client.get(url).send()
        │
tokio::time::timeout(10s, ...)   ← 超时：限制"挂死"的请求
        │
map_err 链式翻译                  ← 错误：网络/解析/数据三层人话
        │
serde_json 解析                   ← 数据：get 链取 rate / date
        │
store 插件 get/set/save           ← 缓存：TTL 判断，过期才重请求
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 网络 | 怎么拿到外部数据 | reqwest、http 插件 re-export |
| 稳定性 | 请求挂死怎么办 | `timeout`、超时错误 |
| 错误 | 失败了用户看到什么 | 链式 `map_err`、`ok_or_else` |
| 缓存 | 能不能少请求几次 | store 插件、`fetched_at`、TTL |

**一通百通的核心：** 这一课补上"**应用与外界的连接**"。联网命令的骨架永远是：**请求 → 超时 → 解析 → 错误翻译 → 缓存**。超级项目 P20 云同步就是在这条骨架上加鉴权和冲突处理。

**递进关系：** 练习 A07（批量重命名）将解决"**长任务的反馈**"——几百个文件改名要跑很久，怎么用 Channel 把进度推给前端、让用户看见"做到哪了"。
