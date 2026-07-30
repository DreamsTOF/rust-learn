use leptos::ev;
use leptos::prelude::*;
use leptos::web_sys;

fn main() {
    mount_to_body(|| {
        let (width, set_width) = signal(0.0);
        let (height, set_height) = signal(0.0);

        if let Some(win) = web_sys::window() {
            set_width.set(win.inner_width().unwrap().as_f64().unwrap());
            set_height.set(win.inner_height().unwrap().as_f64().unwrap());
        }

        window_event_listener(ev::resize, move |_| {
            if let Some(win) = web_sys::window() {
                set_width.set(win.inner_width().unwrap().as_f64().unwrap());
                set_height.set(win.inner_height().unwrap().as_f64().unwrap());
            }
        });

        view! {
            <p>"练习 135 (window_event_listener)"</p>
            <div style="font-size: 18px; line-height: 1.8;">
                <p>"当前窗口尺寸："</p>
                <p>
                    "宽度："
                    <strong>{move || format!("{:.0}px", width.get())}</strong>
                </p>
                <p>
                    "高度："
                    <strong>{move || format!("{:.0}px", height.get())}</strong>
                </p>
            </div>
            <p style="color: #666; font-size: 13px;">
                "提示：调整浏览器窗口大小，数值会实时更新"
            </p>
        }
    });
}
