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
