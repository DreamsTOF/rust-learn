use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    // 目标: 创建文件上传区域
    //
    // 1. 使用 RwSignal::new(Vec::<leptos::web_sys::File>::new()) 保存选择的文件
    // 2. 创建 <input type="file" multiple> 元素
    // 3. 在 on:change 事件中读取 FileList
    //    提示:
    //    - event_target::<leptos::web_sys::HtmlInputElement>(&ev).files()
    //    - file_list.length() 获取文件数量
    //    - file_list.item(i) 获取单个 File 对象
    // 4. 显示每个文件的 name 和 size (KB)
    //    - file.name() 返回文件名
    //    - file.size() 返回字节数 (f64)，除以 1024 得 KB

    view! {
        <div>
            <h2>"练习 260 — 文件上传"</h2>
            {}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
