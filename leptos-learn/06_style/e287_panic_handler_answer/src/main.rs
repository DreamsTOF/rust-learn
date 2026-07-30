use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h2>"Panic 处理示例"</h2>
            <p>"点击下方按钮触发一个 panic"</p>
            <p>"打开浏览器控制台 (F12) 查看格式化的 panic 错误信息"</p>
            <button on:click=move |_| {
                panic!("用户触发的测试 Panic！错误详情：模拟异常");
            }>"触发 Panic"</button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Exercise);
}
