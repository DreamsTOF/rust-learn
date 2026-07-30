// ============================================================
// 练习 e338: Redis Cache — 缓存旁路模式与 TTL
//
// 核心知识点:
//   - 缓存旁路 (Cache-Aside / Lazy Loading) 模式
//   - TTL (Time-To-Live) 过期策略
//   - 缓存失效与主动更新
//   - 缓存命中率统计
//
// 难度: ⭐⭐⭐ (最小指引)
//
// 任务: 实现内存缓存模拟 Redis
//   1. 实现 CacheEntry 结构体 (value: String, expires_at: Instant)
//   2. 实现 Cache 结构体 (HashMap<String, CacheEntry>, 默认 TTL)
//   3. get(key) — 检查 TTL 过期，返回 Option<String>
//   4. set(key, value) — 写入缓存并设置过期时间
//   5. invalidate(key) — 按 key 删除缓存
//   6. 组件中展示缓存旁路流程:
//      "Fetch Data" → 查缓存 → miss → "从数据库加载" → 写入缓存 → 显示
// ============================================================

use leptos::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// TODO: 实现 CacheEntry 结构体
//   - value: String
//   - expires_at: Instant

// TODO: 实现 Cache 结构体
//   - data: HashMap<String, CacheEntry>
//   - default_ttl: Duration
//   - hits: u64
//   - misses: u64

// TODO: 为 Cache 实现:
//   fn new(default_ttl_secs: u64) -> Self
//   fn get(&mut self, key: &str) -> Option<&str> — 检查过期，更新 hits/misses
//   fn set(&mut self, key: String, value: String) — 写入缓存
//   fn invalidate(&mut self, key: &str) — 删除缓存
//   fn hit_rate(&self) -> f64 — 计算命中率

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 RwSignal<Cache> 管理缓存
    // TODO: 缓存条目显示信号 (Vec<(String, String)>)
    // TODO: 当前显示值信号 (Option<String>)
    // TODO: 操作日志信号 (Vec<String>)

    // TODO: 实现 fetch_data 回调实现缓存旁路
    //   - 调用 cache.get("user:1")
    //   - 如果命中: 直接显示缓存值
    //   - 如果未命中: "模拟数据库查询" → cache.set() → 显示
    //   - 记录到日志

    // TODO: 实现 invalidate_cache 回调
    //   - 调用 cache.invalidate("user:1")
    //   - 清空显示值
    //   - 记录到日志

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 700px; margin: 20px auto; padding: 0 16px;">
            <h2>"🔍 缓存旁路模式 (Cache-Aside)"</h2>
            <p style="color: #666; margin-bottom: 16px;">
                "Cache-Aside: 应用先查缓存，命中直接返回；未命中则加载数据库、写入缓存、返回结果。"
                "TTL 过期后缓存自动失效，下次查询重新加载。"
            </p>

            // TODO: 数据显示区域 (当前缓存值)
            // "当前值: " + value 或 "无缓存 (点击 Fetch Data 加载)"

            // TODO: "Fetch Data" 和 "Invalidate Cache" 按钮

            // TODO: 缓存条目列表
            // key | value | 状态 (valid/expired)

            // TODO: 缓存统计
            // Hits / Misses / Hit Rate
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
