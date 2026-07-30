// ============================================================
// 练习 e341: Email SMTP — 使用 lettre 发送 Email
//
// 核心知识点:
//   - lettre::Message: 构建邮件
//   - lettre::SmtpTransport: SMTP 连接
//   - async send: 异步发送邮件
//
// 难度: ⭐⭐ (关键 TODOs 指引)
// ============================================================

use leptos::prelude::*;

// SMTP 配置（生产环境应从环境变量读取）
const SMTP_HOST: &str = "smtp.example.com";
const SMTP_PORT: u16 = 587;
const SMTP_USERNAME: &str = "user@example.com";
const SMTP_PASSWORD: &str = "app_password";

// ============================================================
// TODO 区域
//
// 按照下面提示完成每一处 TODO
// ⭐⭐ = 关键 TODOs，每处均有详细指引
// ============================================================

/// TODO(⭐): 构建纯文本邮件
///
/// 使用 lettre::Message::builder()
///   - from("发件人 <sender@example.com>".parse().unwrap())
///   - to("收件人 <recipient@example.com>".parse().unwrap())
///   - subject("邮件主题")
///   - .body("邮件正文".to_string())?
/// 返回 Result<lettre::Message, Box<dyn std::error::Error>>
fn build_text_email() -> Result<lettre::Message, Box<dyn std::error::Error>> {
    // TODO: 完成邮件构建
    todo!("构建一封纯文本邮件并返回")
}

/// TODO(⭐): 构建 HTML 邮件
///
/// 使用 lettre::Message::builder()
///   - 设置相同 from/to
///   - subject 为 "HTML 邮件"
///   - 使用 lettre::message::SinglePart + builder.header(ContentType::TEXT_HTML)
/// 返回 Result<lettre::Message, Box<dyn std::error::Error>>
fn build_html_email() -> Result<lettre::Message, Box<dyn std::error::Error>> {
    // TODO: 完成 HTML 邮件构建
    todo!("构建一封 HTML 格式的邮件并返回")
}

/// TODO(⭐): 发送邮件 — 创建 SMTP 连接并发送
///
///   1. 用 std::net::TcpStream::connect 连接 SMTP_HOST:SMTP_PORT
///   2. 用 lettre::transport::smtp::SmtpTransport::starttls 创建加密传输
///   3. .credentials() 设置 Username + Password
///   4. .build() 得到 transport
///   5. transport.send(&msg) 发送
///   6. 打印成功/失败日志
/// 返回 Result<(), Box<dyn std::error::Error>>
async fn send_email(msg: &lettre::Message) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: 完成 SMTP 发送逻辑
    todo!("使用 lettre 的 SmtpTransport 发送邮件")
}

/// TODO(⭐): 发送测试邮件（依次发送纯文本和 HTML 邮件）
async fn send_test_emails() {
    // TODO: 调用 build_text_email() 和 build_html_email()，
    //   然后调用 send_email() 发送每封邮件
    //   用 match 处理 Result，打印发送状态
    todo!("构建并发送测试邮件")
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 341: Email SMTP"</h1>
            <p>"点击按钮发送一封测试邮件"</p>
            <button on:click=move |_| {
                leptos::task::spawn_local(async {
                    send_test_emails().await;
                });
            }>"发送测试邮件"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
