use leptos::prelude::*;

// 练习: 动态表单字段
//
// 目标: 创建可动态添加/删除表单项的页面（如多个电话号码输入）
//
// 实现提示:
// 1. 使用 RwSignal::new(vec![String::new()]) 存储电话号码列表
// 2. add_phone: phones.update(|p| p.push(String::new()))
// 3. remove_phone: phones.update(|p| { if p.len() > 1 { p.remove(idx); } })
// 4. 在 view! 中使用 move || phones.get().iter().enumerate().map(...) 遍历
// 5. 每个输入框绑定 on:input 按索引更新对应的值
// 6. 每个电话项附带"删除"按钮，列表底部有"添加电话"按钮

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 RwSignal<Vec<String>>
    // let phones = RwSignal::new(vec![String::new()]);

    // TODO: 实现 add_phone
    // TODO: 实现 remove_phone

    view! {
        <div style="padding: 1rem;">
            <h2>"动态表单字段"</h2>
            <p>"请参考上方提示实现动态添加/删除表单项功能"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
