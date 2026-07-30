// ============================================================
// Exercise 337 - Answer: Token Bucket Rate Limiting
// ============================================================

use leptos::prelude::*;
use std::cell::RefCell;
use std::time::Instant;

struct TokenBucket {
    capacity: u32,
    tokens: u32,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: u32, refill_rate: f64) -> Self {
        Self { capacity, tokens: capacity, refill_rate, last_refill: Instant::now() }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        let new_tokens = (elapsed * self.refill_rate) as u32;
        if new_tokens > 0 {
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = Instant::now();
        }
    }

    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let bucket = RefCell::new(TokenBucket::new(10, 2.0));
    let (tokens, set_tokens) = signal(10u32);
    let (allowed, set_allowed) = signal(0u32);
    let (denied, set_denied) = signal(0u32);
    let (last_result, set_last_result) = signal::<Option<bool>>(None);

    let send_request = move |_| {
        let mut b = bucket.borrow_mut();
        if b.try_consume() {
            set_tokens.set(b.tokens);
            set_allowed.update(|n| *n += 1);
            set_last_result.set(Some(true));
        } else {
            set_denied.update(|n| *n += 1);
            set_last_result.set(Some(false));
        }
    };

    let bar_fill = move || format!("{}%", tokens.get() * 10);
    let bar_color = move || if tokens.get() > 3 { "#2da44e" } else { "#cb2431" };

    view! {
        <div style="max-width: 480px; margin: 2rem auto; padding: 1.5rem; border: 1px solid #d0d7de; border-radius: 8px; font-family: system-ui, sans-serif;">
            <h2 style="margin: 0 0 0.25rem;">"Token Bucket Rate Limiter"</h2>
            <p style="color: #656d76; font-size: 0.85rem; margin: 0 0 1rem;">
                "Capacity: 10  |  Refill: 2 tokens/sec"
            </p>

            <div style="margin: 1rem 0;">
                <p style="margin: 0 0 0.25rem; font-size: 0.9rem;">
                    "Available Tokens: " {move || tokens.get()}
                </p>
                <div style="height: 20px; background: #eaeef2; border-radius: 6px; overflow: hidden;">
                    <div style=format!(
                        "height: 100%; width: {}; background: {}; transition: width 0.3s ease; border-radius: 6px;",
                        bar_fill(), bar_color(),
                    )></div>
                </div>
            </div>

            <div style="display: flex; gap: 1.5rem; margin: 1rem 0; font-size: 0.9rem;">
                <span>"✅ Allowed: " {allowed}</span>
                <span>"❌ Denied: " {denied}</span>
            </div>

            <button on:click=send_request
                style="padding: 0.75rem 1.5rem; background: #0969da; color: #fff; border: none; border-radius: 6px; cursor: pointer; font-size: 1rem;">
                "Send Request"
            </button>

            {move || last_result.get().map(|ok| {
                if ok {
                    view! { <p style="color: #2da44e; margin: 0.75rem 0 0;">"✓ Request allowed"</p> }.into_any()
                } else {
                    view! { <p style="color: #cb2431; margin: 0.75rem 0 0;">"✗ Rate limited — try again later"</p> }.into_any()
                }
            })}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
