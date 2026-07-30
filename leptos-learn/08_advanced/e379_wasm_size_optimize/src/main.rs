// ============================================================
// 练习 e379: WASM 体积优化分析 — 计算包体并分析优化策略
//
// 核心知识点:
//   - WASM 各依赖模块的体积估算
//   - Tree Shaking、LTO、Code Splitting 等优化技术
//   - 优化前后效果对比
//
// 难度: ⭐⭐ (需补全模块体积数据和优化策略展示，约 50%)
// ============================================================

use leptos::prelude::*;

struct ModuleSize {
    name: &'static str,
    size_kb: f64,
    description: &'static str,
}

struct OptimizationTip {
    title: &'static str,
    description: &'static str,
    saving_kb: f64,
}

fn get_modules() -> Vec<ModuleSize> {
    vec![
        ModuleSize { name: "leptos (core)", size_kb: 120.0, description: "框架核心" },
        ModuleSize { name: "leptos_dom", size_kb: 85.0, description: "DOM 渲染" },
        ModuleSize { name: "serde / serde_json", size_kb: 65.0, description: "序列化" },
        // TODO 1: 添加 web-sys、wasm-bindgen runtime、application code 等模块
        // 参考: web-sys ~200KB, wasm-bindgen ~50KB, application ~30KB
    ]
}

fn get_optimization_tips() -> Vec<OptimizationTip> {
    vec![
        OptimizationTip {
            title: "Tree Shaking",
            description: "移除未调用函数，减少无用代码。",
            saving_kb: 80.0,
        },
        // TODO 2: 添加更多优化策略
        // 例如: 轻量替代依赖 (-55KB), Code Splitting (-100KB), LTO (-40KB)
    ]
}

fn total_before() -> f64 {
    get_modules().iter().map(|m| m.size_kb).sum()
}

fn total_after() -> f64 {
    // TODO 3: 计算优化后的总体积（优化前体积减去各优化策略节省的体积）
    total_before() // placeholder
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

            <div style="border: 1px solid #ddd; border-radius: 8px; overflow: hidden; margin: 16px 0;">
                <table style="width: 100%; border-collapse: collapse;">
                    <thead>
                        <tr style="background: #f5f5f5;">
                            <th style="padding: 10px; text-align: left; border-bottom: 2px solid #ddd;">"模块"</th>
                            <th style="padding: 10px; text-align: right; border-bottom: 2px solid #ddd;">"体积 (KB)"</th>
                            <th style="padding: 10px; border-bottom: 2px solid #ddd;">"说明"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {modules.into_iter().map(|m| {
                            view! {
                                <tr style="border-bottom: 1px solid #eee;">
                                    <td style="padding: 8px 10px;">{m.name}</td>
                                    <td style="padding: 8px 10px; text-align: right;">{format!("{:.0}", m.size_kb)}</td>
                                    <td style="padding: 8px 10px;">{m.description}</td>
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

            // TODO 4: 显示优化前后对比区块
            // 格式: 优化前 XXX KB → 优化后 XXX KB（节省 XXX KB）
            // 提示: 使用绿色背景卡片，三列对比布局

            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px;">
                <h3>"💡 优化策略"</h3>
                {tips.into_iter().map(|t| {
                    view! {
                        <div style="border: 1px solid #eee; border-radius: 6px; padding: 12px; margin: 8px 0;">
                            <strong>{t.title}</strong>
                            <span style="margin-left: 8px; color: #2e7d32;">
                                {format!("(-{:.0} KB)", t.saving_kb)}
                            </span>
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
