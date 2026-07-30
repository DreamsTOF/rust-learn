// ============================================================
// 参考答案 e379: WASM 体积优化分析
//
// 展示 WASM 各模块估算体积、分析优化策略、对比优化效果
// ============================================================

use leptos::prelude::*;

/// 依赖模块及其估算体积（KB）
struct ModuleSize {
    name: &'static str,
    size_kb: f64,
    description: &'static str,
}

/// 优化建议
struct OptimizationTip {
    title: &'static str,
    description: &'static str,
    saving_kb: f64,
}

fn get_modules() -> Vec<ModuleSize> {
    vec![
        ModuleSize { name: "leptos (core)", size_kb: 120.0, description: "框架核心 — 响应式系统、模板宏" },
        ModuleSize { name: "leptos_dom", size_kb: 85.0, description: "DOM 操作与渲染" },
        ModuleSize { name: "serde / serde_json", size_kb: 65.0, description: "序列化/反序列化" },
        ModuleSize { name: "wasm-bindgen runtime", size_kb: 50.0, description: "JS-Rust 互操作运行时" },
        ModuleSize { name: "web-sys", size_kb: 200.0, description: "浏览器 API 绑定（树摇前）" },
        ModuleSize { name: "application code", size_kb: 30.0, description: "用户业务逻辑" },
        ModuleSize { name: "allocator (wee_alloc)", size_kb: 8.0, description: "内存分配器" },
        ModuleSize { name: "panic handler", size_kb: 15.0, description: "错误处理与栈回溯" },
    ]
}

fn get_optimization_tips() -> Vec<OptimizationTip> {
    vec![
        OptimizationTip {
            title: "Tree Shaking",
            description: "Rust 编译器的 dead code elimination + wasm-opt 的 --dce 标记可移除未调用函数。",
            saving_kb: 80.0,
        },
        OptimizationTip {
            title: "轻量替代依赖",
            description: "用 miniserde 替代 serde_json（-45KB），或用 wee_alloc 替代默认分配器。",
            saving_kb: 55.0,
        },
        OptimizationTip {
            title: "Code Splitting / 懒加载",
            description: "将非首屏功能拆分为独立 WASM 块，按需加载。",
            saving_kb: 100.0,
        },
        OptimizationTip {
            title: "LTO + 优化等级",
            description: "Cargo.toml 设置 lto = true, opt-level = 'z' 或 's' 以优化体积。",
            saving_kb: 40.0,
        },
        OptimizationTip {
            title: "精简 panic 处理",
            description: "使用 console_error_panic_hook 替代完整 panic 处理，或设置 panic = 'abort'。",
            saving_kb: 25.0,
        },
    ]
}

fn total_before() -> f64 {
    get_modules().iter().map(|m| m.size_kb).sum()
}

fn total_after() -> f64 {
    total_before() - get_optimization_tips().iter().map(|t| t.saving_kb).sum::<f64>()
}

#[component]
fn Exercise() -> impl IntoView {
    let modules = get_modules();
    let tips = get_optimization_tips();
    let before = total_before();
    let after = total_after();

    view! {
        <div style="padding: 20px; max-width: 700px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"📦 WASM 体积优化分析"</h2>
            <p style="color: #666;">
                "以下数据为典型 Leptos CSR 应用的 WASM 各模块估算体积（KB）。"
                "实际值因依赖版本与编译配置而异。"
            </p>

            <div style="border: 1px solid #ddd; border-radius: 8px; overflow: hidden; margin: 16px 0;">
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f5f5f5;">
                            <th style="padding: 10px; text-align: left; border-bottom: 2px solid #ddd;">"模块"</th>
                            <th style="padding: 10px; text-align: right; border-bottom: 2px solid #ddd;">"体积 (KB)"</th>
                            <th style="padding: 10px; text-align: left; border-bottom: 2px solid #ddd;">"说明"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {modules.into_iter().map(|m| {
                            let bar_color = if m.size_kb > 100.0 { "#e53935" }
                                else if m.size_kb > 50.0 { "#fb8c00" }
                                else { "#43a047" };
                            let bar_width = format!("{}%", (m.size_kb / 200.0 * 100.0).min(100.0));
                            view! {
                                <tr style="border-bottom: 1px solid #eee;">
                                    <td style="padding: 8px 10px;">{m.name}</td>
                                    <td style="padding: 8px 10px; text-align: right; font-weight: bold;">
                                        {format!("{:.0}", m.size_kb)}
                                    </td>
                                    <td style="padding: 8px 10px;">
                                        <div style="display: flex; align-items: center; gap: 8px;">
                                            <div style="flex: 1; height: 8px; background: #e0e0e0; border-radius: 4px;">
                                                <div style=move || format!("height: 8px; width: {}; background: {}; border-radius: 4px;",
                                                    bar_width, bar_color)></div>
                                            </div>
                                            <span style="font-size: 12px; color: #666;">{m.description}</span>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                    <tfoot>
                        <tr style="background: #fafafa; font-weight: bold;">
                            <td style="padding: 10px;">"合计"</td>
                            <td style="padding: 10px; text-align: right;">{format!("{:.0}", before)}</td>
                            <td style="padding: 10px;"></td>
                        </tr>
                    </tfoot>
                </table>
            </div>

            <div style="border: 1px solid #4caf50; border-radius: 8px; padding: 16px; margin: 16px 0;
                        background: #e8f5e9;">
                <h3 style="color: #2e7d32; margin: 0 0 12px 0;">"⚡ 优化前后对比"</h3>
                <div style="display: flex; gap: 24px; align-items: center;">
                    <div style="text-align: center; flex: 1;">
                        <div style="font-size: 28px; font-weight: bold; color: #c62828;">{format!("{:.0} KB", before)}</div>
                        <div style="color: #666;">"优化前"</div>
                    </div>
                    <div style="font-size: 24px; color: #4caf50;">"→"</div>
                    <div style="text-align: center; flex: 1;">
                        <div style="font-size: 28px; font-weight: bold; color: #2e7d32;">{format!("{:.0} KB", after)}</div>
                        <div style="color: #666;">"优化后"</div>
                    </div>
                    <div style="text-align: center; flex: 1;">
                        <div style="font-size: 28px; font-weight: bold; color: #e65100;">
                            {format!("-{:.0} KB", before - after)}
                        </div>
                        <div style="color: #666;">"节省"</div>
                    </div>
                </div>
            </div>

            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px;">
                <h3 style="margin: 0 0 12px 0;">"💡 优化策略"</h3>
                {tips.into_iter().map(|t| {
                    let saving_pct = format!("{:.0}%", (t.saving_kb / before * 100.0));
                    view! {
                        <div style="border: 1px solid #eee; border-radius: 6px; padding: 12px; margin: 8px 0;
                                    background: #fafafa;">
                            <div style="display: flex; justify-content: space-between; align-items: center;">
                                <strong>{t.title}</strong>
                                <span style="background: #e8f5e9; color: #2e7d32; padding: 2px 8px; border-radius: 4px;
                                           font-size: 12px; font-weight: bold;">
                                    {format!("-{:.0} KB ({})", t.saving_kb, saving_pct)}
                                </span>
                            </div>
                            <p style="margin: 6px 0 0; color: #555; font-size: 14px;">{t.description}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
