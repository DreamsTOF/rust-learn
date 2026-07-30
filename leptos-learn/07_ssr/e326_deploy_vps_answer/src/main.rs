// ============================================================
// Exercise e326 — Answer: Deploy to VPS
//
// Core: nginx reverse proxy, systemd service, TLS
// ============================================================

use leptos::prelude::*;

const NGINX_CONF: &str = "\
server {
    listen 443 ssl http2;
    server_name example.com;

    ssl_certificate     /etc/letsencrypt/live/example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/example.com/privkey.pem;

    root /var/www/example.com/public;
    try_files $uri /index.html;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}";

const SYSTEMD_UNIT: &str = "\
[Unit]
Description=Leptos SSR App
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/leptos-app
ExecStart=/opt/leptos-app/target/release/leptos-app
Restart=on-failure
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target";

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"VPS 部署配置"</h1>
            <section>
                <h2>"nginx 反向代理"</h2>
                <pre>{NGINX_CONF}</pre>
            </section>
            <section>
                <h2>"systemd 服务单元"</h2>
                <pre>{SYSTEMD_UNIT}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
