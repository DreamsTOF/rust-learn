use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
    pub children: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub price: f64,
    pub stock: i32,
    pub image_urls: Option<String>,
    pub status: String,
    pub skus: Vec<Sku>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRow {
    pub id: i64,
    pub name: String,
    pub category_name: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub status: String,
    pub image_urls: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub items: Vec<ProductRow>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub price: f64,
    pub image_urls: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sku {
    pub id: i64,
    pub product_id: i64,
    pub sku_code: String,
    pub spec_name: Option<String>,
    pub spec_value: Option<String>,
    pub price: Option<f64>,
    pub stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkuRequest {
    pub sku_code: String,
    pub spec_name: Option<String>,
    pub spec_value: Option<String>,
    pub price: Option<f64>,
    pub stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: i64,
    pub user_id: i64,
    pub receiver_name: String,
    pub phone: String,
    pub province: String,
    pub city: String,
    pub district: String,
    pub detail: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub order_no: String,
    pub status: String,
    pub total_amount: f64,
    pub discount_amount: f64,
    pub actual_amount: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetail {
    pub id: i64,
    pub user_id: i64,
    pub order_no: String,
    pub status: String,
    pub total_amount: f64,
    pub discount_amount: f64,
    pub actual_amount: f64,
    pub created_at: String,
    // Address info
    pub receiver_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub detail: Option<String>,
    // Items
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub sku_code: Option<String>,
    pub product_name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListResponse {
    pub items: Vec<Order>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub discount_type: String,
    pub discount_value: f64,
    pub min_amount: f64,
    pub max_discount: Option<f64>,
    pub total_count: Option<i32>,
    pub used_count: i32,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return {
    pub id: i64,
    pub order_id: i64,
    pub user_id: i64,
    pub reason: String,
    pub status: String,
    pub refund_amount: f64,
    pub admin_remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub id: i64,
    pub order_id: i64,
    pub tracking_number: String,
    pub carrier: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: i64,
    pub order_id: i64,
    pub user_id: i64,
    pub invoice_type: String,
    pub title: String,
    pub tax_number: Option<String>,
    pub amount: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub today_orders: i64,
    pub today_revenue: f64,
    pub week_orders: i64,
    pub week_revenue: f64,
    pub total_products: i64,
    pub total_users: i64,
    pub pending_returns: i64,
    pub chart_data: Vec<ChartPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPoint {
    pub day: String,
    pub orders: i64,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub resource: String,
    pub resource_id: Option<String>,
    pub detail: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub order_id: Option<i64>,
    pub rating: i32,
    pub content: Option<String>,
    pub images: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRecord {
    pub id: i64,
    pub order_id: i64,
    pub transaction_id: Option<String>,
    pub payment_method: Option<String>,
    pub amount: f64,
    pub status: Option<String>,
    pub paid_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}
