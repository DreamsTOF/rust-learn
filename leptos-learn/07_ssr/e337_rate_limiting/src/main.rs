// ============================================================
// 练习 e337: Rate Limiting — 令牌桶算法限流
//
// 核心知识点:
//   - 令牌桶算法 (Token Bucket) 原理
//   - 限流中间件在 SSR 中的位置
//   - 突发流量处理与平滑限流
//
// 难度: ⭐⭐⭐ (最小指引)
//
// 任务: 实现令牌桶限流模拟
//   1. 实现 TokenBucket 结构体 (capacity, tokens, refill_rate, last_refill)
//   2. refill() 方法按时间补充令牌
//   3. try_consume() 返回是否允许请求
//   4. 显示当前令牌数、请求结果 (allowed / denied)
//   5. 统计总请求数和限流次数
// ============================================================

use leptos::prelude::*;
use std::time::Instant;

// TODO: 实现 TokenBucket 结构体
//   - capacity: 桶容量
//   - tokens: 当前可用令牌 (f64)
//   - refill_rate: 每秒补充令牌数
//   - last_refill: 上次补充时间 (Instant)

// TODO: 为 TokenBucket 实现:
//   fn new(capacity: f64, refill_rate: f64) -> Self
//   fn refill(&mut self) — 根据时间差补充令牌
//   fn try_consume(&mut self) -> bool — 消费一个令牌，成功返回 true

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 管理令牌桶 (RwSignal<TokenBucket>)
    // TODO: 统计数据信号: total_requests, allowed, denied
    // TODO: 上一次请求结果信号 (Option<&'static str>)

    // TODO: 实现 send_request 回调
    //   - 对桶调用 refill()
    //   - 尝试 try_consume()
    //   - 更新统计数据和结果

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 700px; margin: 20px auto; padding: 0 16px;">
            <h2>"🪣 令牌桶限流模拟"</h2>
            <p style="color: #666; margin-bottom: 16px;">
                "令牌桶算法: 固定容量桶按固定速率补充令牌，每次请求消耗一个令牌。"
                "无令牌时请求被限流，允许一定程度的突发流量。"
            </p>

            // TODO: 显示当前令牌数 (进度条)
            // 背景灰条，前景绿色填充，显示 "tokens / capacity"

            // TODO: "Send Request" 按钮触发请求

            // TODO: 显示上一次请求结果 (allowed ✅ / rate limited ❌)

            // TODO: 统计卡片
            // 总请求数 | 放行 | 拒绝
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
