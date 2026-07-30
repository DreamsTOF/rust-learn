use leptos::prelude::*;

use crate::Return;

#[server(RequestRefund)]
pub async fn request_refund(
    order_id: i64,
    reason: String,
    amount: f64,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Verify order exists and belongs to user, then insert refund request
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ListReturns)]
pub async fn list_returns(
    status: Option<String>,
    page: i64,
    page_size: i64,
) -> Result<Vec<Return>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Query returns with optional status filter and pagination
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ReviewReturn)]
pub async fn review_return(
    id: i64,
    approved: bool,
    remark: Option<String>,
) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Approve or reject a return request with admin remark
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ProcessRefund)]
pub async fn process_refund(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Mark an approved return as refunded by updating status
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
