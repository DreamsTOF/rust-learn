use leptos::prelude::*;

use crate::Category;

#[server(GetCategoryTree)]
pub async fn get_category_tree() -> Result<Vec<Category>, ServerFnError> {
    // TODO: Implement this server function
    // Hint: query all categories from the database and recursively build a parent-child tree structure.
    //       Return Vec<Category> with children nested
    unimplemented!()
}

#[server(CreateCategory)]
pub async fn create_category(name: String, parent_id: Option<i64>) -> Result<i64, ServerFnError> {
    // TODO: Implement this server function
    // Hint: insert a new category with the given name and optional parent_id into the database.
    //       Return the last_insert_rowid() of the new category
    unimplemented!()
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
