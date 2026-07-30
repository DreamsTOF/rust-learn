// ============================================================
// 练习 e327: Deploy to Vercel — serverless adapter
//
// 核心知识点:
//   - Vercel 配置 (vercel.json)
//   - serverless function 适配器
//   - 部署配置展示
//
// 难度: ⭐⭐ (关键 TODOs)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个 Vercel 配置字符串 (vercel.json)
// ⭐⭐ 提示: builds, routes, rewrites 规则
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

// TODO: 定义一个 serverless 函数适配器代码字符串
// ⭐⭐ 提示: leptos_serverless 或自定义 handler
const ADAPTER_CODE: &str = "\
use leptos::prelude::*;
use leptos_serverless::{vercel::VercelHandler, register_fn};

register_fn!();

#[tokio::main]
async fn handler(req: VercelRequest) -> VercelResponse {
    // SSR rendering entry point
    leptos_serverless::render().await
}";

fn main() {
    // TODO: 使用 mount_to_body 挂载 Exercise 组件
    mount_to_body(|| view! { <p>"练习 327 (deploy_vercel)"</p> });
}
