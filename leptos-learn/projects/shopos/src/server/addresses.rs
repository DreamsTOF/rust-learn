use leptos::prelude::*;

use crate::Address;

#[server(ListAddresses)]
pub async fn list_addresses(user_id: i64) -> Result<Vec<Address>, ServerFnError> {
    // TODO: Implement this server function
    // Hint: query all addresses for the given user_id from the database,
    //       ordered by is_default DESC and id DESC, and return them as Vec<Address>
    unimplemented!()
}

#[server(CreateAddress)]
pub async fn create_address(
    receiver_name: String,
    phone: String,
    province: String,
    city: String,
    district: String,
    detail: String,
    is_default: bool,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    // TODO: Implement this server function
    // Hint: insert a new address record into the database for the given user_id.
    //       If is_default is true, first unset any existing default address for this user.
    //       Return the last_insert_rowid() of the new address
    unimplemented!()
}

#[server(UpdateAddress)]
pub async fn update_address(
    id: i64,
    data: String,
    user_id: i64,
) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: parse `data` as JSON into an Address struct, then update the corresponding address
    //       record in the database. If the new address is_default, first unset any existing default.
    //       Return true if any row was affected
    unimplemented!()
}

#[server(DeleteAddress)]
pub async fn delete_address(id: i64, user_id: i64) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: delete the address with the given id that belongs to the given user_id.
    //       Return true if any row was affected
    unimplemented!()
}

#[server(SetDefaultAddress)]
pub async fn set_default_address(id: i64, user_id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Unset all defaults for this user
        sqlx::query("UPDATE addresses SET is_default = 0 WHERE user_id = ?")
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Set the new default
        let affected = sqlx::query("UPDATE addresses SET is_default = 1 WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .rows_affected();

        Ok(affected > 0)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
