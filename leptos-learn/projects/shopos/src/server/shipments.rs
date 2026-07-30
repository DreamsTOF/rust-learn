use leptos::prelude::*;

use crate::Shipment;

#[server(ShipOrder)]
pub async fn ship_order(
    order_id: i64,
    tracking_number: String,
    carrier: String,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Verify order is paid, update status to shipped, and create shipment record
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(GetShippingInfo)]
pub async fn get_shipping_info(order_id: i64) -> Result<Option<Shipment>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Fetch shipment record by order_id from shipments table
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
