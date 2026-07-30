use crate::state::AppState;
use leptos::prelude::*;

pub struct WebSocketManager {
    pub connected: RwSignal<bool>,
    pub messages: RwSignal<Vec<String>>,
}

impl WebSocketManager {
    pub fn new(state: &AppState, _url: &str) -> Self {
        let connected = RwSignal::new(false);
        let messages = RwSignal::new(Vec::new());

        // TODO: 练习 - 实现 WebSocket 连接管理
        // 提示: 创建 WebSocket 连接，设置 onopen/onclose/onmessage 回调，
        //       更新 connected 和 messages 信号
        // Simulate WebSocket connection
        state.is_online.set(true);
        connected.set(true);

        Self { connected, messages }
    }

    pub fn send(&self, _message: &str) {
        // Simulate sending
    }

    pub fn disconnect(&self) {
        self.connected.set(false);
    }
}
