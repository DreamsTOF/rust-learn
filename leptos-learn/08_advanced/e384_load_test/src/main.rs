// ============================================================
// 练习 e384: 负载测试模拟 — 模拟大量用户操作的 UI 展示
//
// 核心知识点:
//   - set_interval 产生模拟并发负载
//   - 信号跟踪性能指标（操作数、响应时间、错误率）
//   - 负载测试结果的实时可视化
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Default)]
struct LoadMetrics {
    total_operations: u64,
    successful_ops: u64,
    failed_ops: u64,
    total_response_time_ms: f64,
    current_rps: f64,   // 每秒操作数
    current_avg_latency: f64,
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO 1: 创建信号存储负载测试指标
    // let metrics = RwSignal::new(LoadMetrics::default());
    // let is_running = RwSignal::new(false);
    // let batch_size = RwSignal::new(5u32);

    // TODO 2: 实现模拟操作的异步函数
    // - 操作计数增加
    // - 模拟随机响应时间（5-50ms）
    // - 模拟 5% 的错误率

    // TODO 3: 使用 set_interval 实现每秒负载生成器
    // - 每秒执行 batch_size 个操作
    // - 更新 current_rps 和 current_avg_latency

    // TODO 4: 实现"爆发模式"按钮 — 一次性执行大量操作（100次）

    view! {
        <div>
            <h2>"⚡ 负载测试模拟"</h2>

            <div>
                <h3>"控制面板"</h3>
                // TODO 5: 创建开始/停止按钮和爆发模式按钮
                // TODO 6: 显示当前批次大小设置
            </div>

            <div>
                <h3>"实时指标"</h3>
                // TODO 7: 显示以下指标:
                // - 总操作数
                // - 成功/失败次数
                // - 每秒操作数 (RPS)
                // - 平均延迟
                // - 错误率 (%)
            </div>

            <div>
                <h3>"响应时间分布 (简单条形图)"</h3>
                // TODO 8: 用 div 宽度模拟条形图展示响应时间分布
                // 分类: <10ms, 10-20ms, 20-50ms, >50ms
            </div>
        </div>
    }
}

 fn main() {
     mount_to_body(Exercise);
 }
