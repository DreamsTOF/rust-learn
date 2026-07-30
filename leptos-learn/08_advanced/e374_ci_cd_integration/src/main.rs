// ============================================================
// 练习 e374: CI/CD 集成配置 — 显示 CI 状态面板
//
// 核心知识点:
//   - 用 enum 定义 CI 阶段状态
//   - Signal 驱动状态切换
//   - 条件渲染不同状态下的 UI
//   - 模拟 CI/CD Pipeline 流程
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

/// CI 阶段状态枚举
#[derive(Clone, Copy, Debug, PartialEq)]
enum CiStageStatus {
    Pending,
    Running,
    Success,
    Fail,
}

impl CiStageStatus {
    /// 返回状态对应的文本标签
    fn label(&self) -> &'static str {
        match self {
            CiStageStatus::Pending => "等待中",
            CiStageStatus::Running => "运行中",
            CiStageStatus::Success => "通过",
            CiStageStatus::Fail => "失败",
        }
    }

    /// 返回状态对应的颜色
    fn color(&self) -> &'static str {
        match self {
            CiStageStatus::Pending => "#95a5a6",
            CiStageStatus::Running => "#f39c12",
            CiStageStatus::Success => "#27ae60",
            CiStageStatus::Fail => "#e74c3c",
        }
    }

    /// 返回状态对应的图标
    fn icon(&self) -> &'static str {
        match self {
            CiStageStatus::Pending => "⏳",
            CiStageStatus::Running => "🔄",
            CiStageStatus::Success => "✅",
            CiStageStatus::Fail => "❌",
        }
    }
}

/// CI 阶段定义
#[derive(Clone, Debug)]
struct CiStage {
    name: &'static str,
    status: RwSignal<CiStageStatus>,
}

/// 单个 CI 阶段显示组件
#[component]
fn StageCard(stage: CiStage) -> impl IntoView {
    let status = stage.status;

    view! {
        <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 8px 0;
                    display: flex; align-items: center; justify-content: space-between;
                    background: white; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
            <div>
                <strong>{stage.name}</strong>
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
                <span style="font-size: 18px;">
                    {move || status.read().icon()}
                </span>
                <span style={move || format!("color: {}; font-weight: bold;", status.read().color())}>
                    {move || status.read().label()}
                </span>
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建各 CI 阶段的 RwSignal
    let build_status = RwSignal::new(CiStageStatus::Pending);
    let test_status = RwSignal::new(CiStageStatus::Pending);
    let lint_status = RwSignal::new(CiStageStatus::Pending);
    let deploy_status = RwSignal::new(CiStageStatus::Pending);

    let stages = vec![
        CiStage { name: "🔨 Build", status: build_status },
        CiStage { name: "🧪 Test", status: test_status },
        CiStage { name: "🔍 Lint", status: lint_status },
        CiStage { name: "🚀 Deploy", status: deploy_status },
    ];

    // TODO: 运行完整 Pipeline — 依次将各阶段从 Pending -> Running -> Success
    // 如果任一阶段失败，后续阶段应标记为 Fail
    let run_pipeline = move |_| {
        // 重置所有状态
        for stage in &stages {
            stage.status.set(CiStageStatus::Pending);
        }
    };

    // 快速设置各阶段状态（用于演示）
    let (selected_stage, set_selected_stage) = signal(0);
    let (selected_status, set_selected_status) = signal(0);
    let statuses = [
        CiStageStatus::Pending,
        CiStageStatus::Running,
        CiStageStatus::Success,
        CiStageStatus::Fail,
    ];

    let apply_status = move |_| {
        let idx = selected_stage.get();
        let st = selected_status.get();
        if idx < stages.len() && st < statuses.len() {
            stages[idx].status.set(statuses[st]);
        }
    };

    view! {
        <div style="padding: 20px; max-width: 800px; margin: 0 auto;">
            <h2>"CI/CD Pipeline 状态面板"</h2>

            <div style="margin: 16px 0;">
                <h3>"Pipeline 概览"</h3>
                <div>
                    {stages.into_iter().map(|stage| {
                        view! { <StageCard stage=stage/> }
                    }).collect_view()}
                </div>
            </div>

            <div style="background: #f8f9fa; padding: 16px; border-radius: 8px; margin: 16px 0;">
                <h3>"手动控制"</h3>
                <div style="display: flex; gap: 8px; align-items: center; flex-wrap: wrap;">
                    <select
                        on:change=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<usize>() {
                                set_selected_stage.set(v);
                            }
                        }
                        style="padding: 6px; border-radius: 4px;"
                    >
                        <option value="0">"Build"</option>
                        <option value="1">"Test"</option>
                        <option value="2">"Lint"</option>
                        <option value="3">"Deploy"</option>
                    </select>

                    <select
                        on:change=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<usize>() {
                                set_selected_status.set(v);
                            }
                        }
                        style="padding: 6px; border-radius: 4px;"
                    >
                        <option value="0">"Pending"</option>
                        <option value="1">"Running"</option>
                        <option value="2">"Success"</option>
                        <option value="3">"Fail"</option>
                    </select>

                    <button
                        on:click=apply_status
                        style="padding: 6px 16px; background: #3498db; color: white;
                               border: none; border-radius: 4px; cursor: pointer;"
                    >
                        "应用"
                    </button>

                    <button
                        on:click=move |_| {
                            for stage in &stages {
                                stage.status.set(CiStageStatus::Pending);
                            }
                        }
                        style="padding: 6px 16px; background: #95a5a6; color: white;
                               border: none; border-radius: 4px; cursor: pointer;"
                    >
                        "重置"
                    </button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
