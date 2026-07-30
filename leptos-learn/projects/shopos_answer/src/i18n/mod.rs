pub mod en_us;
pub mod zh_cn;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub data: HashMap<String, String>,
}

pub fn use_i18n() -> (ReadSignal<String>, impl Fn(String)) {
    let initial = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
        .and_then(|s| s.get_item("shopos_locale").ok().flatten())
        .unwrap_or_else(|| "zh-CN".to_string());

    let (locale, set_locale) = signal(initial);

    let save_locale = move |l: String| {
        set_locale.set(l.clone());
        if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()).flatten() {
            let _ = storage.set_item("shopos_locale", &l);
        }
    };

    (locale, save_locale)
}

pub fn t(key: &str, locale: &str) -> String {
    match locale {
        "en-US" => en_us::get(key),
        _ => zh_cn::get(key),
    }
}
