use leptos::prelude::*;

use crate::Setting;

#[server(GetAllSettings)]
pub async fn get_all_settings() -> Result<Vec<Setting>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Fetch all settings ordered by key
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateSetting)]
pub async fn update_setting(key: String, value: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Upsert a setting by key
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
