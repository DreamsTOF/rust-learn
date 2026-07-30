use leptos::prelude::*;

use crate::Notification;

#[server(GetNotifications)]
pub async fn get_notifications(user_id: i64) -> Result<Vec<Notification>, ServerFnError> {
    // TODO: Implement this server function
    // Hint: Query the latest 50 notifications for the given user_id
    unimplemented!()
}

#[server(MarkNotificationRead)]
pub async fn mark_notification_read(id: i64) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: Mark a notification as read by updating is_read to 1 for the given notification id
    unimplemented!()
}
