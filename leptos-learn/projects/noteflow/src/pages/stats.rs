use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn StatsPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState not provided");

    let total_docs = move || {
        let docs = state.docs.get();
        docs.iter().filter(|d| !d.is_folder).count()
    };

    let total_words = move || {
        let docs = state.docs.get();
        docs.iter()
            .filter(|d| !d.is_folder)
            .map(|d| d.content.split_whitespace().count())
            .sum::<usize>()
    };

    let top_words = move || -> Vec<(String, usize)> {
        let docs = state.docs.get();
        let mut items: Vec<_> = docs
            .iter()
            .filter(|d| !d.is_folder)
            .map(|d| {
                let wc = d.content.split_whitespace().count();
                (d.title.clone(), wc)
            })
            .collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.truncate(10);
        items
    };

    let tag_distribution = move || -> Vec<(String, usize)> {
        // TODO: 练习 - 计算标签分布
        // 提示: 遍历所有文档的 tags，统计每个标签出现的次数，按次数降序排列
        let docs = state.docs.get();
        let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for doc in docs.iter() {
            for tag in &doc.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        let mut result: Vec<_> = tag_counts.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    };

    view! {
        <div class="stats-page">
            <h1>"统计分析"</h1>
            <div class="stats-grid">
                <div class="stat-card">
                    <h3>"总文档数"</h3>
                    <div class="stat-value">{total_docs}</div>
                </div>
                <div class="stat-card">
                    <h3>"总字数"</h3>
                    <div class="stat-value">{total_words}</div>
                </div>
                <div class="stat-card">
                    <h3>"今日编辑"</h3>
                    <div class="stat-value">"0"</div>
                </div>
            </div>
            <div class="stats-section">
                <h2>"标签分布"</h2>
                <div class="tag-distribution">
                    <For
                        each=move || tag_distribution()
                        key=|(name, _)| name.clone()
                        children=move |(name, count)| {
                            view! {
                                <div class="tag-stat">
                                    <span>{name.clone()}</span>
                                    <span class="tag-count">{count}</span>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
            <div class="stats-section">
                <h2>"字数排行 Top 10"</h2>
                <div class="word-rank">
                    <For
                        each=move || top_words()
                        key=|(title, _)| title.clone()
                        children=move |(title, count)| {
                            view! {
                                <div class="rank-item">
                                    <span class="rank-title">{title.clone()}</span>
                                    <span class="rank-count">{count} " 字"</span>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
