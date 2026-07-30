use leptos::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

fn storage() -> Option<web_sys::Storage> {
    let window = web_sys::window()?;
    window.local_storage().ok()?
}

pub fn use_local_storage<T>(key: &str) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + Serialize + DeserializeOwned + Default + Send + Sync + 'static,
{
    let (value, set_value) = signal(T::default());

    // Load from storage if available
    if let Some(storage) = storage() {
        if let Ok(Some(json)) = storage.get_item(key) {
            if let Ok(val) = serde_json::from_str::<T>(&json) {
                set_value.set(val);
            }
        }
    }

    // Auto-save on change
    let key = key.to_string();
    Effect::new(move |_| {
        let val = value.get();
        if let Ok(json) = serde_json::to_string(&val) {
            if let Some(storage) = storage() {
                let _ = storage.set_item(&key, &json);
            }
        }
    });

    (value, set_value)
}
