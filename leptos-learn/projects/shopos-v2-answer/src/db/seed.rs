use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::SqlitePool;

pub async fn run_seed(pool: &SqlitePool) {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

    if count.0 > 0 {
        tracing::info!("Seed data already exists, skipping");
        return;
    }

    tracing::info!("Seeding database...");

    // ── Categories ──
    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (1, '电子产品', 'electronics', '各类电子数码产品', NULL, 1, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (2, '服装鞋帽', 'clothing', '时尚服装与鞋帽配饰', NULL, 2, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (3, '食品饮料', 'food', '美味食品与饮品', NULL, 3, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (4, '家居用品', 'home', '家居生活用品', NULL, 4, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (5, '图书音像', 'books', '图书与音像制品', NULL, 5, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (6, '运动户外', 'sports', '运动器材与户外装备', NULL, 6, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (7, '美妆个护', 'beauty', '美妆护肤与个人护理', NULL, 7, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO categories (id, name, slug, description, parent_id, sort_order, created_at, updated_at)
         VALUES (8, '母婴用品', 'baby', '母婴用品与玩具', NULL, 8, datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // ── Products ──
    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (1, 'iPhone 15 Pro Max', '苹果最新旗舰手机，A17 Pro芯片，钛金属设计', 1, 9999.00, 100, 'https://picsum.photos/seed/p1/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (2, 'MacBook Air M3', '轻薄便携，M3芯片强劲性能', 1, 8999.00, 50, 'https://picsum.photos/seed/p2/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (3, '纯棉T恤', '100%纯棉，舒适透气', 2, 99.00, 500, 'https://picsum.photos/seed/p3/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (4, '运动跑鞋', '轻便缓震，适合日常跑步', 2, 399.00, 200, 'https://picsum.photos/seed/p4/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (5, '有机绿茶', '高山有机绿茶，清香回甘', 3, 68.00, 1000, 'https://picsum.photos/seed/p5/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (6, '进口咖啡豆', '哥伦比亚阿拉比卡咖啡豆，深度烘焙', 3, 128.00, 300, 'https://picsum.photos/seed/p6/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (7, '记忆棉枕头', '人体工学设计，改善睡眠质量', 4, 199.00, 150, 'https://picsum.photos/seed/p7/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (8, '不锈钢保温杯', '真空双层，24小时保温', 4, 89.00, 800, 'https://picsum.photos/seed/p8/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (9, 'Rust编程入门', '系统学习Rust语言的基础与实战', 5, 79.00, 200, 'https://picsum.photos/seed/p9/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (10, '瑜伽垫', '加厚防滑，环保TPE材质', 6, 149.00, 400, 'https://picsum.photos/seed/p10/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (11, '保湿面霜', '深层补水，持久保湿', 7, 169.00, 250, 'https://picsum.photos/seed/p11/400/400', 'published', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO products (id, name, description, category_id, price, stock, image_url, status, created_at, updated_at)
         VALUES (12, '婴儿纸尿裤', '超薄透气，整夜干爽', 8, 99.00, 600, 'https://picsum.photos/seed/p12/400/400', 'draft', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // ── SKUs ──
    // Product 1 - iPhone
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (1, 1, 'IP15PM-BK-256', 'iPhone 15 Pro Max 黑色 256GB', 9999.00, 30, '{\"color\": \"黑色\", \"storage\": \"256GB\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (2, 1, 'IP15PM-WH-256', 'iPhone 15 Pro Max 白色 256GB', 9999.00, 40, '{\"color\": \"白色\", \"storage\": \"256GB\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (3, 1, 'IP15PM-BK-512', 'iPhone 15 Pro Max 黑色 512GB', 11999.00, 20, '{\"color\": \"黑色\", \"storage\": \"512GB\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 2 - MacBook
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (4, 2, 'MBA-M3-SV-8-256', 'MacBook Air M3 银色 8+256', 8999.00, 20, '{\"color\": \"银色\", \"memory\": \"8GB\", \"storage\": \"256GB\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (5, 2, 'MBA-M3-SV-16-512', 'MacBook Air M3 银色 16+512', 10999.00, 15, '{\"color\": \"银色\", \"memory\": \"16GB\", \"storage\": \"512GB\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 3 - T-Shirt
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (6, 3, 'COTTON-TS-M', '纯棉T恤 M码', 99.00, 200, '{\"size\": \"M\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (7, 3, 'COTTON-TS-L', '纯棉T恤 L码', 99.00, 200, '{\"size\": \"L\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (8, 3, 'COTTON-TS-XL', '纯棉T恤 XL码', 99.00, 100, '{\"size\": \"XL\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 4 - Running Shoes
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (9, 4, 'RUN-SHOE-42', '运动跑鞋 42码', 399.00, 80, '{\"size\": \"42\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (10, 4, 'RUN-SHOE-43', '运动跑鞋 43码', 399.00, 70, '{\"size\": \"43\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 5 - Green Tea
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (11, 5, 'GREEN-TEA-250', '有机绿茶 250g装', 68.00, 500, '{\"spec\": \"250g\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (12, 5, 'GREEN-TEA-500', '有机绿茶 500g装', 118.00, 500, '{\"spec\": \"500g\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 7 - Pillow
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (13, 7, 'MEM-PILLOW-L', '记忆棉枕头 大号', 199.00, 80, '{\"size\": \"大号\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (14, 7, 'MEM-PILLOW-XL', '记忆棉枕头 加大号', 249.00, 70, '{\"size\": \"加大号\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 9 - Book
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (15, 9, 'RUST-BOOK-PB', 'Rust编程入门 平装版', 79.00, 150, '{\"binding\": \"平装\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (16, 9, 'RUST-BOOK-HB', 'Rust编程入门 精装版', 119.00, 50, '{\"binding\": \"精装\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 10 - Yoga Mat
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (17, 10, 'YOGA-MAT-6MM', '瑜伽垫 6mm厚', 149.00, 200, '{\"thickness\": \"6mm\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (18, 10, 'YOGA-MAT-10MM', '瑜伽垫 10mm厚', 199.00, 200, '{\"thickness\": \"10mm\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 11 - Cream
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (19, 11, 'MOIST-CREAM-50', '保湿面霜 50ml', 169.00, 150, '{\"spec\": \"50ml\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (20, 11, 'MOIST-CREAM-100', '保湿面霜 100ml', 269.00, 100, '{\"spec\": \"100ml\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // Product 12 - Diapers
    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (21, 12, 'DIAPER-M', '婴儿纸尿裤 M码 40片', 99.00, 300, '{\"size\": \"M\", \"count\": \"40\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO skus (id, product_id, sku_code, name, price, stock, attrs, created_at, updated_at)
         VALUES (22, 12, 'DIAPER-L', '婴儿纸尿裤 L码 36片', 109.00, 300, '{\"size\": \"L\", \"count\": \"36\"}', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // ── Admin User ──
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(b"admin123", &salt)
        .unwrap()
        .to_string();

    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, avatar_url, status, created_at, updated_at)
         VALUES (1, 'admin', 'admin@shopos.com', ?, 'admin', NULL, 'active', datetime('now'), datetime('now'))",
    )
    .bind(&password_hash)
    .execute(pool)
    .await
    .unwrap();

    // ── Settings ──
    sqlx::query(
        "INSERT INTO settings (id, `key`, value, description, created_at, updated_at)
         VALUES (1, 'site_name', 'ShopOS', '站点名称', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO settings (id, `key`, value, description, created_at, updated_at)
         VALUES (2, 'site_description', 'ShopOS 电商管理后台', '站点描述', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO settings (id, `key`, value, description, created_at, updated_at)
         VALUES (3, 'order_auto_cancel_minutes', '30', '未支付订单自动取消时间(分钟)', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO settings (id, `key`, value, description, created_at, updated_at)
         VALUES (4, 'default_shipping_fee', '10', '默认运费', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO settings (id, `key`, value, description, created_at, updated_at)
         VALUES (5, 'free_shipping_threshold', '99', '免运费门槛', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    // ── Coupons ──
    sqlx::query(
        "INSERT INTO coupons (id, code, name, discount_type, discount_value, min_amount, max_amount, total_count, used_count, valid_from, valid_to, status, created_at, updated_at)
         VALUES (1, 'WELCOME10', '新用户专享', 'percentage', 10.00, 100.00, 50.00, 1000, 0, datetime('now'), datetime('now', '+30 days'), 'active', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO coupons (id, code, name, discount_type, discount_value, min_amount, max_amount, total_count, used_count, valid_from, valid_to, status, created_at, updated_at)
         VALUES (2, 'SUMMER50', '夏日大促', 'fixed', 50.00, 200.00, 50.00, 500, 0, datetime('now'), datetime('now', '+60 days'), 'active', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO coupons (id, code, name, discount_type, discount_value, min_amount, max_amount, total_count, used_count, valid_from, valid_to, status, created_at, updated_at)
         VALUES (3, 'VIP20', 'VIP会员折扣', 'percentage', 20.00, 0.00, 200.00, 200, 0, datetime('now'), datetime('now', '+90 days'), 'active', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO coupons (id, code, name, discount_type, discount_value, min_amount, max_amount, total_count, used_count, valid_from, valid_to, status, created_at, updated_at)
         VALUES (4, 'FREESHIP', '免运费', 'fixed', 0.00, 50.00, 0.00, 500, 0, datetime('now'), datetime('now', '+15 days'), 'active', datetime('now'), datetime('now'))",
    )
    .execute(pool)
    .await
    .unwrap();

    tracing::info!("Seed data inserted successfully");
}
