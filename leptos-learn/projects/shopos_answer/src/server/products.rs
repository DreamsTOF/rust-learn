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
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let mut where_clauses: Vec<String> = Vec::new();
        let mut bind_idx = 0i64;

        // We'll use dynamic query building
        if category_id.is_some() {
            where_clauses.push(format!("p.category_id = ?{}", bind_idx + 1));
            bind_idx += 1;
        }
        if keyword.is_some() {
            where_clauses.push(format!("p.name LIKE ?{}", bind_idx + 1));
            bind_idx += 1;
        }
        if status.is_some() {
            where_clauses.push(format!("p.status = ?{}", bind_idx + 1));
            bind_idx += 1;
        }
        if min_price.is_some() {
            where_clauses.push(format!("p.price >= ?{}", bind_idx + 1));
            bind_idx += 1;
        }
        if max_price.is_some() {
            where_clauses.push(format!("p.price <= ?{}", bind_idx + 1));
            bind_idx += 1;
        }

        let where_sql = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };

        // Count query
        let count_sql = format!(
            "SELECT COUNT(*) FROM products p LEFT JOIN categories c ON p.category_id = c.id {}",
            where_sql
        );

        // Build the query with bindings for count
        let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
        if let Some(ref cat_id) = category_id {
            count_query = count_query.bind(cat_id);
        }
        if let Some(ref kw) = keyword {
            count_query = count_query.bind(format!("%{}%", kw));
        }
        if let Some(ref st) = status {
            count_query = count_query.bind(st);
        }
        if let Some(ref p) = min_price {
            count_query = count_query.bind(p);
        }
        if let Some(ref p) = max_price {
            count_query = count_query.bind(p);
        }

        let total: i64 = count_query
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Data query
        let offset = (page - 1) * page_size;
        let data_sql = format!(
            "SELECT p.id, p.name, c.name AS category_name, p.price, p.stock, p.status, p.image_urls \
             FROM products p LEFT JOIN categories c ON p.category_id = c.id {} \
             ORDER BY p.id DESC LIMIT ?{} OFFSET ?{}",
            where_sql,
            bind_idx + 1,
            bind_idx + 2
        );

        let mut data_query = sqlx::query(&data_sql);
        if let Some(ref cat_id) = category_id {
            data_query = data_query.bind(cat_id);
        }
        if let Some(ref kw) = keyword {
            data_query = data_query.bind(format!("%{}%", kw));
        }
        if let Some(ref st) = status {
            data_query = data_query.bind(st);
        }
        if let Some(ref p) = min_price {
            data_query = data_query.bind(p);
        }
        if let Some(ref p) = max_price {
            data_query = data_query.bind(p);
        }
        data_query = data_query.bind(page_size);
        data_query = data_query.bind(offset);

        let rows = data_query
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let items: Vec<ProductRow> = rows
            .iter()
            .map(|row| ProductRow {
                id: row.get::<i64, _>(0),
                name: row.get::<String, _>(1),
                category_name: row.get::<Option<String>, _>(2),
                price: row.get::<f64, _>(3),
                stock: row.get::<i32, _>(4),
                status: row.get::<String, _>(5),
                image_urls: row.get::<Option<String>, _>(6),
            })
            .collect();

        Ok(ProductListResponse { items, total })
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
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let row = sqlx::query(
            "SELECT id, name, description, category_id, price, stock, image_urls, status FROM products WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("商品不存在"))?;

        let sku_rows = sqlx::query(
            "SELECT id, product_id, sku_code, spec_name, spec_value, price, stock FROM product_skus WHERE product_id = ?",
        )
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let skus: Vec<Sku> = sku_rows
            .iter()
            .map(|r| Sku {
                id: r.get::<i64, _>(0),
                product_id: r.get::<i64, _>(1),
                sku_code: r.get::<String, _>(2),
                spec_name: r.get::<Option<String>, _>(3),
                spec_value: r.get::<Option<String>, _>(4),
                price: r.get::<Option<f64>, _>(5),
                stock: r.get::<i32, _>(6),
            })
            .collect();

        Ok(Product {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            category_id: row.get(3),
            price: row.get(4),
            stock: row.get(5),
            image_urls: row.get(6),
            status: row.get(7),
            skus,
        })
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
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let req: crate::CreateProductRequest = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid product data: {}", e)))?;

        let result = sqlx::query(
            "INSERT INTO products (name, description, category_id, price, stock, image_urls, status) VALUES (?, ?, ?, ?, 0, ?, ?)",
        )
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.category_id)
        .bind(req.price)
        .bind(&req.image_urls)
        .bind(&req.status)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(result.last_insert_rowid())
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
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let req: crate::CreateProductRequest = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid product data: {}", e)))?;

        let affected = sqlx::query(
            "UPDATE products SET name = ?, description = ?, category_id = ?, price = ?, image_urls = ?, status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.category_id)
        .bind(req.price)
        .bind(&req.image_urls)
        .bind(&req.status)
        .bind(id)
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

#[server(DeleteProduct)]
pub async fn delete_product(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Delete associated SKUs first
        sqlx::query("DELETE FROM product_skus WHERE product_id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let affected = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
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

#[server(CreateSku)]
pub async fn create_sku(product_id: i64, data: String) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let req: CreateSkuRequest = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid SKU data: {}", e)))?;

        let result = sqlx::query(
            "INSERT INTO product_skus (product_id, sku_code, spec_name, spec_value, price, stock) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(product_id)
        .bind(&req.sku_code)
        .bind(&req.spec_name)
        .bind(&req.spec_value)
        .bind(req.price)
        .bind(req.stock)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(result.last_insert_rowid())
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
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let req: CreateSkuRequest = serde_json::from_str(&data)
            .map_err(|e| ServerFnError::new(format!("Invalid SKU data: {}", e)))?;

        let affected = sqlx::query(
            "UPDATE product_skus SET sku_code = ?, spec_name = ?, spec_value = ?, price = ?, stock = ? WHERE id = ?",
        )
        .bind(&req.sku_code)
        .bind(&req.spec_name)
        .bind(&req.spec_value)
        .bind(req.price)
        .bind(req.stock)
        .bind(id)
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

#[server(DeleteSku)]
pub async fn delete_sku(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query("DELETE FROM product_skus WHERE id = ?")
            .bind(id)
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

#[server(ImportProducts)]
pub async fn import_products(file_data: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Try JSON array first, then CSV
        let trimmed = file_data.trim();

        let records: Vec<CreateSkuRequest> = if trimmed.starts_with('[') {
            // Parse as JSON array of products
            #[derive(serde::Deserialize)]
            struct ImportRow {
                name: String,
                #[serde(default)]
                description: Option<String>,
                #[serde(default)]
                category_id: Option<i64>,
                price: f64,
                #[serde(default)]
                image_urls: Option<String>,
                #[serde(default)]
                status: String,
            }

            let items: Vec<ImportRow> = serde_json::from_str(trimmed)
                .map_err(|e| ServerFnError::new(format!("JSON parse error: {}", e)))?;

            let mut imported = 0i64;
            for item in &items {
                let status = if item.status.is_empty() {
                    "draft".to_string()
                } else {
                    item.status.clone()
                };
                sqlx::query(
                    "INSERT INTO products (name, description, category_id, price, image_urls, status) VALUES (?, ?, ?, ?, ?, ?)",
                )
                .bind(&item.name)
                .bind(&item.description)
                .bind(item.category_id)
                .bind(item.price)
                .bind(&item.image_urls)
                .bind(&status)
                .execute(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
                imported += 1;
            }
            return Ok(format!("成功导入 {} 个商品", imported));
        } else {
            return Err(ServerFnError::new("不支持的数据格式，请使用JSON数组"));
        };
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}
