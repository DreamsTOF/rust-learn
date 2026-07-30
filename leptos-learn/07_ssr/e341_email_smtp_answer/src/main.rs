// ============================================================
// Exercise 341 - Email SMTP (Answer)
//
// Core: lettre SMTP, email building, async send
// ============================================================

use leptos::prelude::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::Message;
use std::net::TcpStream;

const SMTP_HOST: &str = "smtp.example.com";
const SMTP_PORT: u16 = 587;
const SMTP_USERNAME: &str = "user@example.com";
const SMTP_PASSWORD: &str = "app_password";

fn build_text_email() -> Result<Message, Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("发件人 <sender@example.com>".parse().unwrap())
        .to("收件人 <recipient@example.com>".parse().unwrap())
        .subject("邮件主题")
        .body("这是一封纯文本邮件正文。".to_string())?;
    Ok(email)
}

fn build_html_email() -> Result<Message, Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("发件人 <sender@example.com>".parse().unwrap())
        .to("收件人 <recipient@example.com>".parse().unwrap())
        .subject("HTML 邮件")
        .header(ContentType::TEXT_HTML)
        .body("<h1>HTML 邮件</h1><p>这是一封 <b>HTML</b> 格式的邮件。</p>".to_string())?;
    Ok(email)
}

async fn send_email(msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect((SMTP_HOST, SMTP_PORT))?;
    let transport = SmtpTransport::starttls(stream)
        .credentials(Credentials::new(
            SMTP_USERNAME.to_string(),
            SMTP_PASSWORD.to_string(),
        ))
        .build();

    transport.send(msg)?;
    println!("邮件发送成功: {}", msg.subject());
    Ok(())
}

async fn send_test_emails() {
    match build_text_email() {
        Ok(email) => {
            if let Err(e) = send_email(&email).await {
                eprintln!("纯文本邮件发送失败: {e}");
            }
        }
        Err(e) => eprintln!("构建纯文本邮件失败: {e}"),
    }

    match build_html_email() {
        Ok(email) => {
            if let Err(e) = send_email(&email).await {
                eprintln!("HTML 邮件发送失败: {e}");
            }
        }
        Err(e) => eprintln!("构建 HTML 邮件失败: {e}"),
    }
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
