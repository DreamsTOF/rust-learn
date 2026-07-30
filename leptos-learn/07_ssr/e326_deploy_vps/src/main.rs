// ============================================================
// 练习 e326: Deploy to VPS — nginx + systemd + TLS
//
// 核心知识点:
//   - nginx 反向代理配置
//   - systemd 服务单元文件
//   - Let's Encrypt / Certbot TLS 配置
//
// 难度: ⭐⭐ (关键 TODOs)
// ============================================================

use leptos::prelude::*;

// TODO: 定义一个 nginx 配置字符串
// ⭐⭐ 提示: 用多行字符串展示完整的 nginx server block
// - 监听 443 (SSL)
// - proxy_pass 到 http://127.0.0.1:3000
// - 静态文件路径、WebSocket 支持
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

// TODO: 定义一个 systemd service 配置字符串
// ⭐⭐ 提示: 描述一个 Rust/Lepotos 应用的 systemd 单元
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

fn main() {
    // TODO: 使用 mount_to_body 挂载 Exercise 组件
    mount_to_body(|| view! { <p>"练习 326 (deploy_vps)"</p> });
}
