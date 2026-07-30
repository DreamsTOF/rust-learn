// ============================================================
// 练习 e380: 构建分析工具 — 展示依赖树、编译时间、版本兼容性
//
// 核心知识点:
//   - Cargo 依赖树的可视化展示
//   - 编译阶段时间统计
//   - 依赖版本兼容性评估
//
// 难度: ⭐⭐ (需补全依赖数据和编译时间统计，约 50%)
// ============================================================

use leptos::prelude::*;

struct Dependency {
    name: &'static str,
    version: &'static str,
    size_kb: f64,
    compat_score: u8,
}

struct BuildPhase {
    name: &'static str,
    duration_secs: f64,
}

fn get_dependency_tree() -> Vec<Dependency> {
    vec![
        Dependency { name: "leptos", version: "0.8.0-nightly", size_kb: 120.0, compat_score: 100 },
        Dependency { name: "  ├─ leptos_macro", version: "0.8.0-nightly", size_kb: 25.0, compat_score: 100 },
        Dependency { name: "  ├─ leptos_dom", version: "0.8.0-nightly", size_kb: 85.0, compat_score: 100 },
        // TODO 1: 添加 web-sys、serde、wasm-bindgen 等传递依赖
        // 参考: web-sys 0.3.103 (200KB), serde 1.0.217 (40KB), wasm-bindgen 0.2.126 (50KB)
    ]
}

fn get_build_phases() -> Vec<BuildPhase> {
    vec![
        BuildPhase { name: "依赖下载与解析", duration_secs: 8.5 },
        // TODO 2: 添加更多编译阶段数据
        // 参考: leptos 编译 ~45s, web-sys 编译 ~62s, LTO ~30s
    ]
}

fn format_duration(secs: f64) -> String {
    if secs >= 60.0 {
        format!("{:.1} 分 {:.0} 秒", secs / 60.0, secs % 60.0)
    } else {
        format!("{:.1} 秒", secs)
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let deps = get_dependency_tree();
    let phases = get_build_phases();
    let total_time: f64 = phases.iter().map(|p| p.duration_secs).sum();

    view! {
        <div style="padding: 20px; max-width: 700px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"🔨 构建分析工具"</h2>

            // TODO 3: 显示依赖树
            // 使用 <pre> 展示每个依赖的名称、版本、体积和兼容性评分

            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 16px 0;">
                <h3>"⏱️ 编译时间统计"</h3>
                <p>"总耗时: " <strong>{format_duration(total_time)}</strong></p>
                {phases.into_iter().map(|p| {
                    let pct = p.duration_secs / total_time * 100.0;
                    view! {
                        <div style="margin: 6px 0;">
                            <div style="display: flex; justify-content: space-between; font-size: 13px;">
                                <span>{p.name}</span>
                                <span style="font-weight: bold;">{format_duration(p.duration_secs)}</span>
                            </div>
                            // TODO 4: 添加进度条表示各阶段耗时占比
                            // 提示: div 内嵌 div，外层为灰色背景条，内层为蓝色填充
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div style="border: 1px solid #1976d2; border-radius: 8px; padding: 16px; background: #e3f2fd;">
                <h3 style="color: #1565c0;">"💡 构建优化建议"</h3>
                <ul style="line-height: 1.8;">
                    <li>"使用 cargo check --timings 分析编译瓶颈。"</li>
                    // TODO 5: 添加更多优化建议
                    // 例如: 启用 sccache 缓存、将大型依赖移至独立 crate、CI 缓存等
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
