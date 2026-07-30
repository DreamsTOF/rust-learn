use leptos::prelude::*;

use crate::Invoice;

#[server(ApplyInvoice)]
pub async fn apply_invoice(
    order_id: i64,
    invoice_type: String,
    title: String,
    tax_number: Option<String>,
    user_id: i64,
) -> Result<i64, ServerFnError> {
    // TODO: Implement this server function
    // Hint: Create an invoice for a given order, validate the order exists and no invoice already exists
    unimplemented!()
}

#[server(ListInvoices)]
pub async fn list_invoices(
    status: Option<String>,
    page: i64,
    page_size: i64,
) -> Result<Vec<Invoice>, ServerFnError> {
    // TODO: Implement this server function
    // Hint: List invoices with optional status filter, paginated by page/page_size
    unimplemented!()
}

#[server(ApproveInvoice)]
pub async fn approve_invoice(id: i64) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: Approve a pending invoice by updating its status to 'approved'
    unimplemented!()
}
