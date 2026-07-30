// ============================================================
// Exercise e335 — Answer: SSR Performance Optimization
//
// Core: Memory/CPU optimization, caching strategies,
//       streaming rendering, code splitting, benchmarks
// ============================================================

use leptos::prelude::*;

const BOTTLENECKS: &str = "\
# SSR 常见性能瓶颈

## 1. 渲染 CPU 瓶颈
   问题: 每次请求都完整执行组件树渲染
   影响: 高并发下 CPU 100%，响应时间飙升
   解决: 响应缓存 + 片段缓存 + 流式渲染

## 2. 内存分配瓶颈
   问题: 每次请求分配大量临时对象（VNode 等）
   影响: GC 压力大，Full GC 导致 STW 延迟
   解决: 对象池、复用缓冲区、减少分配

## 3. 响应体积瓶颈
   问题: 完整 HTML 在内存中序列化
   影响: 占用带宽、增加 TTFB
   解决: 流式输出、压缩、去除多余空白";

const CACHING_STRATEGIES: &str = "\
# 缓存策略

## 响应缓存 (Response Cache)
   适用: 不依赖用户状态的公共页面（首页、关于）
   策略: Cache-Control: public, max-age=60
   收益: 减少 80% 渲染负载

## 片段缓存 (Fragment Cache)
   适用: 页面中的静态组件（Header/Footer）
   策略: 缓存组件渲染输出，按 props 键值索引
   收益: 减少 40% 重复渲染

## 数据缓存 (Data Cache)
   适用: 数据库查询结果
   策略: Redis / in-memory cache with TTL
   收益: 减少 90% 数据库查询";

const BENCHMARK_DATA: &str = "\
# 基准测试对比 (100 concurrent VUs, 60s)

┌─────────────────────┬──────────┬──────────┬─────────┐
│        指标         │  优化前   │  优化后   │  提升   │
├─────────────────────┼──────────┼──────────┼─────────┤
│ RPS (请求/秒)       │    1,200 │    4,800 │   +300% │
│ p50 延迟            │    45ms  │    12ms  │   -73%  │
│ p95 延迟            │   180ms  │    35ms  │   -81%  │
│ p99 延迟            │   420ms  │    85ms  │   -80%  │
│ 内存/请求           │   128KB  │    32KB  │   -75%  │
│ GC 暂停             │   15ms   │     2ms  │   -87%  │
└─────────────────────┴──────────┴──────────┴─────────┘

# 优化措施
1. 启用响应缓存 (public pages)
2. 实现组件片段缓存
3. 数据查询加 Redis 缓存
4. 流式 SSR 输出替代完整缓冲
5. 代码分割减少初始包体积";

const STREAMING_RENDERING: &str = "\
# 流式渲染策略

## 按序流式渲染 (In-Order Streaming)
   原理: 按组件树深度优先顺序输出 HTML
   优点: 浏览器渐进式解析，TTFB 极低
   适用: SEO 优先的页面

## 无序流式渲染 (Out-of-Order Streaming)
   原理: 先输出 shell，异步数据加载完成后注入
   优点: 首屏 FCP 最快
   适用: 数据依赖复杂的页面

## Suspense 集成
   原理: <Suspense> 包裹异步数据加载区域
   行为: 先发送 fallback，数据到达后补发内容
   收益: 首字节时间减少 60%+";

#[component]
fn SectionBlock(title: &'static str, content: &'static str) -> impl IntoView {
    view! {
        <div style="margin-bottom: 20px;">
            <h3 style="margin-bottom: 8px; color: #333;">{title}</h3>
            <pre style="background:#1e1e1e;color:#d4d4d4;padding:12px;border-radius:6px;font-size:13px;overflow-x:auto;margin:0;line-height:1.5;">
                {content}
            </pre>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="font-family:system-ui,sans-serif;max-width:900px;margin:20px auto;padding:0 16px;">
            <h2>"🚀 SSR 性能优化"</h2>
            <p style="color:#666;margin-bottom:20px;">
                "分析 SSR 渲染瓶颈，应用缓存策略、流式渲染和代码分割，全面提升服务器端渲染性能。"
            </p>

            <SectionBlock title="🔍 常见性能瓶颈" content={BOTTLENECKS} />
            <SectionBlock title="💾 缓存策略" content={CACHING_STRATEGIES} />
            <SectionBlock title="📈 基准测试对比" content={BENCHMARK_DATA} />
            <SectionBlock title="🌊 流式渲染" content={STREAMING_RENDERING} />

            <div style="margin-top:20px;padding:12px;background:#f0f8ff;border-radius:6px;font-size:13px;color:#333;">
                <strong>"💡 性能优化要点: "</strong>
                "优先识别瓶颈（用 profiling），针对性优化而非盲目缓存。"
                "响应缓存 + 片段缓存 + 数据缓存分层使用。"
                "流式渲染显著降低 TTFB，是 SSR 性能的关键优化手段。"
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
