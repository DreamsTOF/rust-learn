// ============================================================
// 练习 e149: use_geolocation — 浏览器地理位置 API
//
// 目标: 封装浏览器 Geolocation API 为响应式 Hook
//
// 难度: ⭐⭐⭐
// 核心知识点: navigator.geolocation, watchPosition, Closure
//
// TODO: 按照注释提示补全代码
// ============================================================

use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// 通过自定义 wasm_bindgen 绑定访问 Geolocation API
#[wasm_bindgen]
extern "C" {
    type Geo;
    type GeoCoords;
    type GeoPosition;
    type GeoError;

    #[wasm_bindgen(method, getter = coords)]
    fn geo_coords(this: &GeoPosition) -> GeoCoords;
    #[wasm_bindgen(method, getter = latitude)]
    fn geo_lat(this: &GeoCoords) -> f64;
    #[wasm_bindgen(method, getter = longitude)]
    fn geo_lng(this: &GeoCoords) -> f64;
    #[wasm_bindgen(method, getter = message)]
    fn geo_msg(this: &GeoError) -> String;

    #[wasm_bindgen(method, js_name = watchPosition)]
    fn watch_position(this: &Geo, success: &JsValue, error: &JsValue);

    #[wasm_bindgen(js_namespace = ["navigator"])]
    fn geolocation() -> Option<Geo>;
}

/// Hook: 获取实时地理位置
fn use_geolocation() -> (ReadSignal<Option<f64>>, ReadSignal<Option<f64>>, ReadSignal<Option<String>>) {
    let (lat, set_lat) = signal(None::<f64>);
    let (lng, set_lng) = signal(None::<f64>);
    let (error, set_error) = signal(None::<String>);

    if let Some(geo) = geolocation() {
        let success = Closure::<dyn FnMut(JsValue)>::new(move |pos: JsValue| {
            let pos: GeoPosition = pos.unchecked_into();
            let c = pos.geo_coords();
            set_lat.set(Some(c.geo_lat()));
            set_lng.set(Some(c.geo_lng()));
        });
        let fail = Closure::<dyn FnMut(JsValue)>::new(move |err: JsValue| {
            let err: GeoError = err.unchecked_into();
            set_error.set(Some(err.geo_msg()));
        });
        geo.watch_position(success.as_ref().unchecked_ref(), fail.as_ref().unchecked_ref());
        success.forget();
        fail.forget();
    } else {
        set_error.set(Some("浏览器不支持地理位置".into()));
    }

    (lat, lng, error)
}

#[component]
fn Exercise() -> impl IntoView {
    let (lat, lng, error) = use_geolocation();

    view! {
        <div>
            <h2>"e149: use_geolocation"</h2>
            {move || {
                if let Some(err) = error.get() {
                    view! { <p style="color:red">"错误: " {err}</p> }.into_any()
                } else {
                    let lat_str = lat.get().map(|v| format!("{:.6}", v));
                    let lng_str = lng.get().map(|v| format!("{:.6}", v));
                    view! {
                        <div>
                            <p>"纬度: " {lat_str.unwrap_or_else(|| "获取中...".into())}</p>
                            <p>"经度: " {lng_str.unwrap_or_else(|| "获取中...".into())}</p>
                            <p style="font-size:0.8em;color:#888">"位置信息基于浏览器定位，精度因设备而异"</p>
                        </div>
                    }.into_any()
                }
            }}
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
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
//
// #[wasm_bindgen]
// extern "C" {
//     type Geo;
//     type GeoCoords;
//     type GeoPosition;
//     type GeoError;
//     #[wasm_bindgen(method, getter = coords)]
//     fn geo_coords(this: &GeoPosition) -> GeoCoords;
//     #[wasm_bindgen(method, getter = latitude)]
//     fn geo_lat(this: &GeoCoords) -> f64;
//     #[wasm_bindgen(method, getter = longitude)]
//     fn geo_lng(this: &GeoCoords) -> f64;
//     #[wasm_bindgen(method, getter = message)]
//     fn geo_msg(this: &GeoError) -> String;
//     #[wasm_bindgen(method, js_name = watchPosition)]
//     fn watch_position(this: &Geo, success: &JsValue, error: &JsValue);
//     #[wasm_bindgen(js_namespace = ["navigator"])]
//     fn geolocation() -> Option<Geo>;
// }
//
// fn use_geolocation() -> (ReadSignal<Option<f64>>, ReadSignal<Option<f64>>, ReadSignal<Option<String>>) {
//     let (lat, set_lat) = signal(None);
//     let (lng, set_lng) = signal(None);
//     let (error, set_error) = signal(None);
//     if let Some(geo) = geolocation() {
//         let success = Closure::<dyn FnMut(JsValue)>::new(move |pos: JsValue| {
//             let pos: GeoPosition = pos.unchecked_into();
//             let c = pos.geo_coords();
//             set_lat.set(Some(c.geo_lat()));
//             set_lng.set(Some(c.geo_lng()));
//         });
//         let fail = Closure::<dyn FnMut(JsValue)>::new(move |err: JsValue| {
//             let err: GeoError = err.unchecked_into();
//             set_error.set(Some(err.geo_msg()));
//         });
//         geo.watch_position(success.as_ref().unchecked_ref(), fail.as_ref().unchecked_ref());
//         success.forget();
//         fail.forget();
//     }
//     (lat, lng, error)
// }
// ```
//
// ### 知识点
// - `navigator.geolocation()` 获取 Geolocation 对象
// - `watchPosition` 持续追踪位置变化（优于 getCurrentPosition 的一次性获取）
// - `Closure` 将 Rust 闭包转为 JS 函数指针，`forget()` 防止被 GC
// - `unchecked_into::<T>()` 将 raw JsValue 转为自定义类型
// - 返回三个 ReadSignal 分别表示纬度/经度/错误信息
//
// </details>
