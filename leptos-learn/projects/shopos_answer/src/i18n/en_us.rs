use std::collections::HashMap;
use std::sync::OnceLock;

static EN_US: OnceLock<HashMap<String, String>> = OnceLock::new();

fn get_map() -> &'static HashMap<String, String> {
    EN_US.get_or_init(|| {
        let mut m = HashMap::new();
        // App
        m.insert("app.name".into(), "ShopOS Admin".into());
        m.insert("app.tagline".into(), "Full-featured e-commerce management system".into());
        m.insert("app.copyright".into(), "© 2024 ShopOS Team".into());

        // Navigation
        m.insert("nav.dashboard".into(), "Dashboard".into());
        m.insert("nav.products".into(), "Products".into());
        m.insert("nav.product_list".into(), "Product List".into());
        m.insert("nav.categories".into(), "Categories".into());
        m.insert("nav.users".into(), "Users".into());
        m.insert("nav.user_list".into(), "User List".into());
        m.insert("nav.orders".into(), "Orders".into());
        m.insert("nav.order_list".into(), "Order List".into());
        m.insert("nav.operations".into(), "Operations".into());
        m.insert("nav.coupons".into(), "Coupons".into());
        m.insert("nav.returns".into(), "Returns".into());
        m.insert("nav.invoices".into(), "Invoices".into());
        m.insert("nav.analytics".into(), "Analytics".into());
        m.insert("nav.reports".into(), "Reports".into());
        m.insert("nav.reconciliation".into(), "Reconciliation".into());
        m.insert("nav.system".into(), "System".into());
        m.insert("nav.audit".into(), "Audit Log".into());
        m.insert("nav.settings".into(), "Settings".into());

        // Actions
        m.insert("action.create".into(), "Create".into());
        m.insert("action.edit".into(), "Edit".into());
        m.insert("action.delete".into(), "Delete".into());
        m.insert("action.save".into(), "Save".into());
        m.insert("action.cancel".into(), "Cancel".into());
        m.insert("action.confirm".into(), "Confirm".into());
        m.insert("action.search".into(), "Search".into());
        m.insert("action.reset".into(), "Reset".into());
        m.insert("action.export".into(), "Export".into());
        m.insert("action.import".into(), "Import".into());
        m.insert("action.batch_delete".into(), "Batch Delete".into());
        m.insert("action.batch_update".into(), "Batch Update".into());
        m.insert("action.view".into(), "View".into());
        m.insert("action.back".into(), "Back".into());
        m.insert("action.submit".into(), "Submit".into());
        m.insert("action.upload".into(), "Upload".into());
        m.insert("action.download".into(), "Download".into());
        m.insert("action.refresh".into(), "Refresh".into());
        m.insert("action.more".into(), "More".into());

        // Status
        m.insert("status.active".into(), "Active".into());
        m.insert("status.inactive".into(), "Inactive".into());
        m.insert("status.published".into(), "Published".into());
        m.insert("status.draft".into(), "Draft".into());
        m.insert("status.archived".into(), "Archived".into());
        m.insert("status.pending".into(), "Pending".into());
        m.insert("status.processing".into(), "Processing".into());
        m.insert("status.completed".into(), "Completed".into());
        m.insert("status.cancelled".into(), "Cancelled".into());
        m.insert("status.refunded".into(), "Refunded".into());

        // Product
        m.insert("product.name".into(), "Product Name".into());
        m.insert("product.category".into(), "Category".into());
        m.insert("product.price".into(), "Price".into());
        m.insert("product.stock".into(), "Stock".into());
        m.insert("product.description".into(), "Description".into());
        m.insert("product.image".into(), "Product Image".into());
        m.insert("product.status".into(), "Status".into());
        m.insert("product.sku".into(), "SKU".into());
        m.insert("product.batch_import".into(), "Batch Import".into());

        // Order
        m.insert("order.order_no".into(), "Order No.".into());
        m.insert("order.total_amount".into(), "Total Amount".into());
        m.insert("order.payment_method".into(), "Payment Method".into());
        m.insert("order.shipping_address".into(), "Shipping Address".into());
        m.insert("order.remark".into(), "Remark".into());

        // User
        m.insert("user.username".into(), "Username".into());
        m.insert("user.email".into(), "Email".into());
        m.insert("user.phone".into(), "Phone".into());
        m.insert("user.role".into(), "Role".into());
        m.insert("user.avatar".into(), "Avatar".into());

        // Login / Auth
        m.insert("auth.login".into(), "Login".into());
        m.insert("auth.register".into(), "Register".into());
        m.insert("auth.logout".into(), "Logout".into());
        m.insert("auth.forgot_password".into(), "Forgot Password".into());
        m.insert("auth.username_placeholder".into(), "Please enter username".into());
        m.insert("auth.password_placeholder".into(), "Please enter password".into());
        m.insert("auth.login_success".into(), "Login successful".into());
        m.insert("auth.login_failed".into(), "Login failed, please check username or password".into());

        // Validation
        m.insert("validation.required".into(), "This field is required".into());
        m.insert("validation.email".into(), "Please enter a valid email address".into());
        m.insert("validation.phone".into(), "Please enter a valid phone number".into());
        m.insert("validation.min_length".into(), "Minimum length is {0} characters".into());
        m.insert("validation.max_length".into(), "Maximum length is {0} characters".into());
        m.insert("validation.price_positive".into(), "Price must be greater than 0".into());

        // Messages
        m.insert("message.operation_success".into(), "Operation successful".into());
        m.insert("message.operation_failed".into(), "Operation failed".into());
        m.insert("message.confirm_delete".into(), "Are you sure you want to delete? This action cannot be undone.".into());
        m.insert("message.no_data".into(), "No data available".into());
        m.insert("message.loading".into(), "Loading...".into());
        m.insert("message.network_error".into(), "Network error, please try again later".into());

        m
    })
}

pub fn get(key: &str) -> String {
    get_map()
        .get(key)
        .cloned()
        .unwrap_or_else(|| key.to_string())
}
