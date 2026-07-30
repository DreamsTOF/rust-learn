// ============================================================
// 练习 e384: 负载测试模拟 — 模拟大量用户操作的 UI 展示
//
// 核心知识点:
//   - set_interval_with_handle 产生模拟并发负载
//   - 信号跟踪性能指标（操作数、响应时间、错误率）
//   - 负载测试结果的实时可视化
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[derive(Clone, Default)]
struct LoadMetrics {
    total_operations: u64,
    successful_ops: u64,
    failed_ops: u64,
    total_response_time_ms: f64,
    current_rps: f64,
    current_avg_latency: f64,
}

#[derive(Clone, Default)]
struct LatencyDistribution {
    under_10ms: u64,
    under_20ms: u64,
    under_50ms: u64,
    over_50ms: u64,
}

struct SimpleRng(u32);

impl SimpleRng {
    fn new() -> Self { Self(42) }
    fn next(&mut self, max: u32) -> u32 {
        self.0 = self.0.wrapping_mul(1103515245).wrapping_add(12345);
        (self.0 >> 16) % max
    }
    fn next_f64(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(1103515245).wrapping_add(12345);
        (self.0 >> 8) as f64 / 16777216.0
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let metrics = RwSignal::new(LoadMetrics::default());
    let latencies = RwSignal::new(LatencyDistribution::default());
    let is_running = RwSignal::new(false);
    let batch_size = RwSignal::new(5u32);
    let interval_handle: RwSignal<Option<IntervalHandle>> = RwSignal::new(None);

    // Execute a single simulated operation
    let exec_op = move |rng: &mut SimpleRng| {
        let rt = 5.0 + rng.next_f64() * 45.0;
        let err = rng.next(100) < 5;
        metrics.update(|m| {
            m.total_operations += 1;
            m.total_response_time_ms += rt;
            if err { m.failed_ops += 1; } else { m.successful_ops += 1; }
        });
        latencies.update(|d| {
            if rt < 10.0 { d.under_10ms += 1; }
            else if rt < 20.0 { d.under_20ms += 1; }
            else if rt < 50.0 { d.under_50ms += 1; }
            else { d.over_50ms += 1; }
        });
    };

    let exec_batch = move |count: u32| {
        let mut rng = SimpleRng::new();
        for _ in 0..count { exec_op(&mut rng); }
        let m = metrics.get();
        let avg = if m.total_operations > 0 { m.total_response_time_ms / m.total_operations as f64 } else { 0.0 };
        metrics.update(|m| { m.current_rps = m.total_operations as f64; m.current_avg_latency = avg; });
    };

    let start_load = move |_| {  // takes event arg for on:click
        if is_running.get() { return; }
        is_running.set(true);
        let bs = batch_size.get();
        let rng = std::cell::RefCell::new(SimpleRng::new());
        if let Ok(handle) = set_interval_with_handle(move || {
            let cnt = rng.borrow_mut().next(bs) + 1;
            exec_batch(cnt);
        }, std::time::Duration::from_secs(1)) {
            interval_handle.set(Some(handle));
        }
    };

    let stop_load = move || {
        if let Some(handle) = interval_handle.get() {
            handle.clear();
        }
        interval_handle.set(None);
        is_running.set(false);
    };

    let run_burst = move || exec_batch(100);

    let reset_all = move || {
        stop_load();
        metrics.set(LoadMetrics::default());
        latencies.set(LatencyDistribution::default());
    };

    on_cleanup(move || {
        if let Some(handle) = interval_handle.get() {
            handle.clear();
        }
    });

    let button_text = move || {
        if is_running.get() { "⏹ 停止负载" } else { "▶️ 开始负载" }
    };
    let load_action = move |_ev: leptos::ev::MouseEvent| {
        if is_running.get() { stop_load(); } else { start_load(_ev); }
    };

    view! {
        <div>
            <h2>"⚡ 负载测试模拟"</h2>

            <div>
                <h3>"控制面板"</h3>
                <div>
                    <label>"批次大小: " <input type="number" prop:value={move || batch_size.get().to_string()} on:input=move |ev| { if let Ok(v) = event_target_value(&ev).parse::<u32>() { batch_size.set(v.max(1).min(100)); } } /></label>
                </div>
                <div style="margin-top:8px">
                    <button on:click=load_action>{move || button_text()}</button>
                    <button on:click=move |_| run_burst()>"🔥 爆发模式 (100次)"</button>
                    <button on:click=move |_| reset_all()>"🔄 重置"</button>
                </div>
            </div>

            <div>
                <h3>"实时指标"</h3>
                {move || {
                    let m = metrics.get();
                    let err_rate = if m.total_operations > 0 { (m.failed_ops as f64 / m.total_operations as f64) * 100.0 } else { 0.0 };
                    view! {
                        <table>
                            <tr><td>"总操作数:"</td><td>{m.total_operations}</td></tr>
                            <tr><td>"成功:"</td><td>{m.successful_ops}</td></tr>
                            <tr><td>"失败:"</td><td>{m.failed_ops}</td></tr>
                            <tr><td>"RPS:"</td><td>{format!("{:.1}", m.current_rps)}</td></tr>
                            <tr><td>"平均延迟:"</td><td>{format!("{:.1} ms", m.current_avg_latency)}</td></tr>
                            <tr><td>"错误率:"</td><td>{format!("{:.1}%", err_rate)}</td></tr>
                        </table>
                    }
                }}
            </div>

            <div>
                <h3>"响应时间分布"</h3>
                {move || {
                    let d = latencies.get();
                    let total = d.under_10ms + d.under_20ms + d.under_50ms + d.over_50ms;
                    let max_c = d.under_10ms.max(d.under_20ms).max(d.under_50ms).max(d.over_50ms).max(1);
                    let pct = |c: u64| -> String {
                        if total > 0 { format!("{}", c * 100 / total) } else { "0".to_string() }
                    };
                    let bw = |c: u64| -> String {
                        format!("width:{}%;height:100%", (c as f64 / max_c as f64) * 100.0)
                    };
                    view! {
                        <div>
                            <div>"<10ms: " {pct(d.under_10ms)} "% "
                                <span style="display:inline-block;width:200px;height:16px;background:#eee;vertical-align:middle">
                                    <span style={bw(d.under_10ms) + ";background:#4caf50;display:inline-block"}></span>
                                </span>
                            </div>
                            <div>"10-20ms: " {pct(d.under_20ms)} "% "
                                <span style="display:inline-block;width:200px;height:16px;background:#eee;vertical-align:middle">
                                    <span style={bw(d.under_20ms) + ";background:#ff9800;display:inline-block"}></span>
                                </span>
                            </div>
                            <div>"20-50ms: " {pct(d.under_50ms)} "% "
                                <span style="display:inline-block;width:200px;height:16px;background:#eee;vertical-align:middle">
                                    <span style={bw(d.under_50ms) + ";background:#f44336;display:inline-block"}></span>
                                </span>
                            </div>
                            <div>">50ms: " {pct(d.over_50ms)} "% "
                                <span style="display:inline-block;width:200px;height:16px;background:#eee;vertical-align:middle">
                                    <span style={bw(d.over_50ms) + ";background:#9c27b0;display:inline-block"}></span>
                                </span>
                            </div>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
