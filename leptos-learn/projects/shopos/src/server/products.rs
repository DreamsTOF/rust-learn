use leptos::prelude::*;

use crate::{CreateSkuRequest, Product, ProductListResponse, ProductRow, Sku};

#[server(ListProducts)]
pub async fn list_products(
    page: i64,
    page_size: i64,
    category_id: Option<i64>,
    keyword: Option<String>,
    status: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
) -> Result<ProductListResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Query products with pagination, category filter, keyword search, and price range
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(GetProductDetail)]
pub async fn get_product_detail(id: i64) -> Result<Product, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Fetch a single product with its SKUs by ID from database
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(CreateProduct)]
pub async fn create_product(data: String) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Parse product data and insert into products table, return new product ID
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateProduct)]
pub async fn update_product(id: i64, data: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Parse product data and update product record by ID
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(DeleteProduct)]
pub async fn delete_product(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Delete product and its associated SKUs by ID
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(CreateSku)]
pub async fn create_sku(product_id: i64, data: String) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Parse SKU data and insert into product_skus table
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(UpdateSku)]
pub async fn update_sku(id: i64, data: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Parse SKU data and update product_skus record by ID
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(DeleteSku)]
pub async fn delete_sku(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Delete SKU by ID from product_skus table
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(ImportProducts)]
pub async fn import_products(file_data: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // TODO: Implement this server function
        // Hint: Bulk import products from JSON/CSV data, return summary string
        unimplemented!()
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
