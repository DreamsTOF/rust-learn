// ============================================================
// 练习 A06: 汇率查询 —— 答案版
// 目标: HTTP 插件（reqwest）、异步 + 超时、Store 缓存
// ============================================================

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const CACHE_FILE: &str = "rate-cache.json";
/// 缓存有效期：1 小时
const CACHE_TTL_SECS: u64 = 3600;

#[derive(Serialize)]
pub struct RateInfo {
    from: String,
    to: String,
    rate: f64,
    date: String,
    from_cache: bool,
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// 读缓存：返回 (汇率, 日期, 抓取时间戳)
fn read_cache(app: &AppHandle, from: &str, to: &str) -> Option<(f64, String, u64)> {
    let store = app.store(CACHE_FILE).ok()?;
    let value = store.get(format!("{from}_{to}"))?;
    Some((
        value["rate"].as_f64()?,
        value["date"].as_str()?.to_string(),
        value["fetched_at"].as_u64()?,
    ))
}

/// 写缓存
fn write_cache(app: &AppHandle, from: &str, to: &str, rate: f64, date: &str) -> Result<(), String> {
    let store = app.store(CACHE_FILE).map_err(|e| format!("打开缓存失败：{e}"))?;
    store.set(
        format!("{from}_{to}"),
        serde_json::json!({ "rate": rate, "date": date, "fetched_at": now_ts() }),
    );
    store.save().map_err(|e| format!("写入缓存失败：{e}"))?;
    Ok(())
}

/// 查汇率：缓存命中（1 小时内）直接用；否则请求网络
#[tauri::command]
async fn get_rate(app: AppHandle, from: String, to: String) -> Result<RateInfo, String> {
    // 1. 缓存命中就直接返回
    if let Some((rate, date, fetched_at)) = read_cache(&app, &from, &to) {
        if now_ts() - fetched_at < CACHE_TTL_SECS {
            return Ok(RateInfo { from, to, rate, date, from_cache: true });
        }
    }

    // 2. 否则请求网络（10 秒超时）
    let url = format!("https://api.frankfurter.app/latest?from={from}&to={to}");
    let client = tauri_plugin_http::reqwest::Client::new();
    let response = tokio::time::timeout(Duration::from_secs(10), client.get(&url).send())
        .await
        .map_err(|_| "请求超时（10 秒）".to_string())?
        .map_err(|e| format!("请求失败：{e}"))?;
    let text = response.text().await.map_err(|e| format!("读取响应失败：{e}"))?;

    // 3. 解析 JSON
    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("解析响应失败：{e}"))?;
    let rate = json
        .get("rates")
        .and_then(|r| r.get(&to))
        .and_then(|v| v.as_f64())
        .ok_or_else(|| format!("响应里没有 {to} 的汇率"))?;
    let date = json["date"].as_str().unwrap_or("").to_string();

    // 4. 写缓存，标记"来自网络"
    write_cache(&app, &from, &to, rate, &date)?;
    Ok(RateInfo { from, to, rate, date, from_cache: false })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_rate])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
