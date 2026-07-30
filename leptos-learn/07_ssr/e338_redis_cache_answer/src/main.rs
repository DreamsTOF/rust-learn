// ============================================================
// Exercise 338 - Answer: Redis Cache-Aside with TTL
// ============================================================

use leptos::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry {
    value: String,
    expires_at: Instant,
}

struct Cache {
    store: HashMap<String, CacheEntry>,
    default_ttl: Duration,
    hits: u32,
    misses: u32,
}

impl Cache {
    fn new(default_ttl_secs: u64) -> Self {
        Self {
            store: HashMap::new(),
            default_ttl: Duration::from_secs(default_ttl_secs),
            hits: 0,
            misses: 0,
        }
    }

    fn get(&mut self, key: &str) -> Option<String> {
        match self.store.get(key) {
            Some(entry) if Instant::now() < entry.expires_at => {
                self.hits += 1;
                Some(entry.value.clone())
            }
            _ => {
                self.store.remove(key);
                self.misses += 1;
                None
            }
        }
    }

    fn set(&mut self, key: String, value: String, ttl_secs: Option<u64>) {
        let ttl = ttl_secs.map(Duration::from_secs).unwrap_or(self.default_ttl);
        self.store.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + ttl,
        });
    }

    fn invalidate(&mut self, key: &str) {
        self.store.remove(key);
    }

    fn stats(&self) -> (u32, u32, usize) {
        (self.hits, self.misses, self.store.len())
    }
}

// Simulate a "database" of key-value pairs
fn simulate_db_fetch(key: &str) -> String {
    format!("[DB] Value for '{}' loaded at {:?}", key, Instant::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
}

#[component]
fn Exercise() -> impl IntoView {
    let cache = Rc::new(RefCell::new(Cache::new(30)));
    let cache2 = cache.clone();
    let (key, set_key) = signal("user:1001".to_string());
    let (cached_value, set_cached_value) = signal::<Option<String>>(None);
    let (cache_source, set_cache_source) = signal("none".to_string());
    let (hits, set_hits) = signal(0u32);
    let (misses, set_misses) = signal(0u32);
    let (entries, set_entries) = signal(0usize);

    let refresh_stats = move |c: &mut Cache| {
        let (h, m, e) = c.stats();
        set_hits.set(h);
        set_misses.set(m);
        set_entries.set(e);
    };

    let fetch_data = move |_| {
        let mut c = cache.borrow_mut();
        let k = key.get_untracked();
        match c.get(&k) {
            Some(val) => {
                set_cached_value.set(Some(val));
                set_cache_source.set("cache (HIT)".to_string());
            }
            None => {
                // Cache miss — simulate DB load
                let db_val = simulate_db_fetch(&k);
                c.set(k.clone(), db_val.clone(), None);
                set_cached_value.set(Some(db_val));
                set_cache_source.set("database (MISS)".to_string());
            }
        }
        refresh_stats(&mut *c);
    };

    let invalidate = move |_| {
        let mut c = cache.borrow_mut();
        c.invalidate(&key.get_untracked());
        set_cached_value.set(None);
        set_cache_source.set("invalidated".to_string());
        refresh_stats(&mut *c);
    };

    view! {
        <div style="max-width: 480px; margin: 2rem auto; padding: 1.5rem; border: 1px solid #d0d7de; border-radius: 8px; font-family: system-ui, sans-serif;">
            <h2 style="margin: 0 0 0.25rem;">"Redis Cache-Aside Demo"</h2>
            <p style="color: #656d76; font-size: 0.85rem; margin: 0 0 1rem;">
                "Lazy loading with TTL (30s default)"
            </p>
            <hr style="margin: 0 0 1rem; border: none; border-top: 1px solid #d0d7de;" />

            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.25rem; font-size: 0.85rem; font-weight: 600;">
                    "Cache Key:"
                </label>
                <input type="text"
                    prop:value=key
                    on:input=move |ev| set_key.set(event_target_value(&ev))
                    style="width: 100%; padding: 0.5rem; border: 1px solid #d0d7de; border-radius: 4px; box-sizing: border-box;" />
            </div>

            <div style="display: flex; gap: 0.5rem; margin-bottom: 1rem;">
                <button on:click=fetch_data
                    style="padding: 0.5rem 1rem; background: #0969da; color: #fff; border: none; border-radius: 4px; cursor: pointer;">
                    "Fetch Data"
                </button>
                <button on:click=invalidate
                    style="padding: 0.5rem 1rem; background: #cb2431; color: #fff; border: none; border-radius: 4px; cursor: pointer;">
                    "Invalidate"
                </button>
            </div>

            <div style="background: #f6f8fa; padding: 0.75rem; border-radius: 4px; margin-bottom: 1rem;">
                <p style="margin: 0 0 0.25rem;">
                    <strong>"Source: "</strong> {cache_source}
                </p>
                <p style="margin: 0;">
                    <strong>"Value: "</strong>
                    {move || match cached_value.get() {
                        Some(v) => v,
                        None => "(empty)".to_string(),
                    }}
                </p>
            </div>

            <div style="display: flex; gap: 1.5rem; font-size: 0.85rem; color: #656d76;">
                <span>"🎯 Hits: " {hits}</span>
                <span>"❌ Misses: " {misses}</span>
                <span>"📦 Cached entries: " {entries}</span>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
