use leptos::prelude::*;

use crate::Category;

#[server(GetCategoryTree)]
pub async fn get_category_tree() -> Result<Vec<Category>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let rows = sqlx::query(
            "SELECT id, name, parent_id, sort_order FROM categories ORDER BY parent_id IS NOT NULL, sort_order, id",
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let all: Vec<Category> = rows
            .iter()
            .map(|row| Category {
                id: row.get::<i64, _>(0),
                name: row.get::<String, _>(1),
                parent_id: row.get::<Option<i64>, _>(2),
                sort_order: row.get::<i32, _>(3),
                children: Vec::new(),
            })
            .collect();

        // Build tree: find roots (parent_id IS NULL) and recursively assign children
        fn build_children(parents: &[Category], all: &[Category]) -> Vec<Category> {
            parents
                .iter()
                .map(|p| {
                    let mut node = p.clone();
                    let kids: Vec<Category> = all
                        .iter()
                        .filter(|c| c.parent_id == Some(p.id))
                        .cloned()
                        .collect();
                    node.children = build_children(&kids, all);
                    node
                })
                .collect()
        }

        let roots: Vec<Category> = all.iter().filter(|c| c.parent_id.is_none()).cloned().collect();
        Ok(build_children(&roots, &all))
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("Server only"))
    }
}

#[server(CreateCategory)]
pub async fn create_category(name: String, parent_id: Option<i64>) -> Result<i64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let result = sqlx::query("INSERT INTO categories (name, parent_id) VALUES (?, ?)")
            .bind(&name)
            .bind(parent_id)
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

#[server(UpdateCategory)]
pub async fn update_category(id: i64, name: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        let affected = sqlx::query("UPDATE categories SET name = ? WHERE id = ?")
            .bind(&name)
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

#[server(DeleteCategory)]
pub async fn delete_category(id: i64) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use sqlx::Row;

        let pool = use_context::<sqlx::SqlitePool>()
            .ok_or_else(|| ServerFnError::new("DB pool not found"))?;

        // Check for child categories
        let child_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE parent_id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        if child_count > 0 {
            return Err(ServerFnError::new("该分类下有子分类，无法删除"));
        }

        // Check for products referencing this category
        let product_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM products WHERE category_id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;

        if product_count > 0 {
            return Err(ServerFnError::new("该分类下有商品，无法删除"));
        }

        let affected = sqlx::query("DELETE FROM categories WHERE id = ?")
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
