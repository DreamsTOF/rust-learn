// Make optional SSR-only crates available for name resolution behind cfg
#[cfg(feature = "ssr")]
extern crate sqlx;
#[cfg(feature = "ssr")]
extern crate argon2;
#[cfg(feature = "ssr")]
extern crate tracing;
#[cfg(feature = "ssr")]
extern crate chrono;
#[cfg(feature = "ssr")]
extern crate uuid;

pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod db;
pub mod error;
pub mod hooks;
pub mod i18n;
pub mod layout;
pub mod pages;
pub mod server;
#[cfg(feature = "ssr")]
pub mod state;
pub mod types;
