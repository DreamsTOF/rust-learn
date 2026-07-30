use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
export function get_file_info(input) {
    const files = input.files;
    if (!files || files.length === 0) return [];
    const result = [];
    for (let i = 0; i < files.length; i++) {
        result.push(files[i].name + "\x00" + files[i].size);
    }
    return result;
}
"#)]
extern "C" {
    fn get_file_info(input: &leptos::web_sys::HtmlInputElement) -> Vec<String>;
}

#[component]
fn Exercise() -> impl IntoView {
    let files_info = RwSignal::new(Vec::<(String, u64)>::new());

    view! {
        <div>
            <h2>"练习 260 — 文件上传"</h2>
            <input type="file" multiple on:change=move|ev| {
                let input = event_target::<leptos::web_sys::HtmlInputElement>(&ev);
                let items = get_file_info(&input);
                let mut info = Vec::new();
                for item in items {
                    let parts: Vec<&str> = item.split('\0').collect();
                    if parts.len() == 2 {
                        let name = parts[0].to_string();
                        let size = parts[1].parse::<u64>().unwrap_or(0);
                        info.push((name, size));
                    }
                }
                files_info.set(info);
            } />
            <div>
                <p>"已选择 " {move || files_info.get().len()} " 个文件:"</p>
                <ul>
                    {move || files_info.get().iter().map(|(name, size)| {
                        let size_kb = *size / 1024;
                        view! {
                            <li>
                                {name.clone()} " — " {size_kb} " KB"
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
