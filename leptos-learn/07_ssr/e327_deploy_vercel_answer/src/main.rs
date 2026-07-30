// ============================================================
// Exercise e327 — Answer: Deploy to Vercel
//
// Core: Vercel config, serverless adapters
// ============================================================

use leptos::prelude::*;

const VERCEL_CONFIG: &str = "\
{
    \"builds\": [
        {
            \"src\": \"**/*.rs\",
            \"use\": \"@vercel/rust\"
        }
    ],
    \"routes\": [
        { \"src\": \"/api/(.*)\", \"dest\": \"/api/$1\" },
        { \"src\": \"/(.*)\", \"dest\": \"/$1\" }
    ]
}";

const ADAPTER_CODE: &str = "\
use leptos::prelude::*;
use leptos_serverless::{vercel::VercelHandler, register_fn};

register_fn!();

#[tokio::main]
async fn handler(req: VercelRequest) -> VercelResponse {
    leptos_serverless::render().await
}";

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"Vercel 部署配置"</h1>
            <section>
                <h2>"vercel.json"</h2>
                <pre>{VERCEL_CONFIG}</pre>
            </section>
            <section>
                <h2>"Serverless 适配器代码"</h2>
                <pre>{ADAPTER_CODE}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
