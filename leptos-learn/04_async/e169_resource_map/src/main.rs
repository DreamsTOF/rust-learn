// ============================================================
// 练习 e169: .map() — Resource 数据转换 (resource_map)
//
// 核心知识点:
//   - LocalResource::map() 同步转换资源值
//   - Option<&T> → Option<U> 的便捷转换
//   - 避免手动 match Some/None
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

async fn fetch_temperature(city: &str) -> f64 {
    // 模拟 API: 返回城市温度
    match city {
        "北京" => 26.5,
        "上海" => 30.2,
        "广州" => 33.8,
        _ => 20.0,
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (city, set_city) = signal("北京".to_string());

    // TODO: 创建 LocalResource，依赖 city 信号
    // 提示: 在 fetcher 闭包中读取 city.get()
    let temp_resource = LocalResource::new(move || {
        let city = city.get();
        async move { fetch_temperature(&city).await }
    });

    view! {
        <div>
            <h2>"Resource.map() 示例"</h2>
            <select on:change:target=move |ev| set_city.set(ev.target().value())>
                <option value="北京">"北京"</option>
                <option value="上海">"上海"</option>
                <option value="广州">"广州"</option>
            </select>

            <p>
                "温度: "
                // TODO: 使用 .map() 将 f64 转为字符串
                // 提示: temp_resource.map(|t| format!("{:.1}°C", t))
                {move || temp_resource
                    .map(|t| format!("{:.1}°C", t))
                    .unwrap_or_else(|| "加载中...".to_string())
                }
            </p>

            // TODO: 对于 Result 类型的 Resource，可以使用 and_then
            // 此处 temperature 是 f64 而非 Result，所以直接使用 .map()
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ============================================================
// 参考答案 (思考后再看!)
// ============================================================
// <details>
// <summary>点击展开答案</summary>
//
// ### 代码
// ```rust
// use leptos::prelude::*;
//
// async fn fetch_temperature(city: &str) -> f64 {
//     match city {
//         "北京" => 26.5,
//         "上海" => 30.2,
//         "广州" => 33.8,
//         _ => 20.0,
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (city, set_city) = signal("北京".to_string());
//
//     let temp_resource = LocalResource::new(move || {
//         let city = city.get();
//         async move { fetch_temperature(&city).await }
//     });
//
//     view! {
//         <div>
//             <h2>"Resource.map() 示例"</h2>
//             <select on:change:target=move |ev| set_city.set(ev.target().value())>
//                 <option value="北京">"北京"</option>
//                 <option value="上海">"上海"</option>
//                 <option value="广州">"广州"</option>
//             </select>
//             <p>
//                 "温度: "
//                 {move || temp_resource
//                     .map(|t| format!("{:.1}°C", t))
//                     .unwrap_or_else(|| "加载中...".to_string())
//                 }
//             </p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
// ```
//
// ### 知识点
// - LocalResource::map(f) 在资源有值时应用 f: impl FnOnce(&T) -> U
// - 返回 Option<U>，无值时返回 None
// - 配合 unwrap_or_else 提供默认显示
// - 对于 Result<T, E> 类型，还可使用 and_then 直接提取 Ok 分支
//
// </details>
