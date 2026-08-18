# ShopOS — 全栈电商管理后台 执行规划

## 1. 项目概述

| 维度 | 说明 |
|------|------|
| **项目名称** | ShopOS — B2C 全栈电商管理后台 |
| **技术栈** | Leptos 0.9 (nightly) + Thaw UI + leptos_router + leptos-use + Server Functions + SQLite + Axum |
| **构建方式** | `cargo-leptos` (SSR) |
| **步数** | 40 步，分 8 个阶段 |
| **执行模式** | **单 agent 串行**（强依赖链，每步依赖前一步代码） |
| **目录位置** | 练习: `leptos-learn/projects/shopos/` — 答案: `leptos-learn/projects/shopos_answer/` |
| **预计耗时** | ~6-8h 墙钟时间 |

### 1.1 练习/答案双文件夹结构

每个终极项目分两个独立文件夹，遵循与章节练习题一致的规范：

```
leptos-learn/projects/
├── shopos/                # 练习项目（含 TODO，供学员逐步完成）
│   ├── Cargo.toml
│   ├── src/...
│   └── ...
└── shopos_answer/         # 参考答案项目（完整可编译运行，无 TODO）
    ├── Cargo.toml
    ├── src/...
    └── ...
```

**核心原则：**
- **练习文件夹**：Agent 递进式编写，每步增量添加代码。最终结果是一个**包含 TODO 引导的练习项目**，学员按照 TODO 提示逐步补全。
- **答案文件夹**：40 步全部完成后，生成一个**完整自洽的独立项目**。`trunk build` / `cargo leptos build` 零错误零警告，可直接编译运行，不含任何 TODO。答案项目是练习项目的"最终形态参照"。
- 两个文件夹的内容在 workspace `Cargo.toml` 中**分别注册**为独立 member。

---

## 2. 核心依赖链

```
A-01(脚手架) → A-02(布局) → A-03(导航) → A-04(Schema) → A-05(种子)
                                                          ↓
A-12(注册) ← A-11(详情) ← A-10(SKU) ← A-09(新增) ← A-08(搜索) ← A-07(列表) ← A-06(类目)
    ↓           ↓                                 ↑                  ↓
A-13(登录) → A-14(信息) → A-15(地址) → A-16(安全)  A-17(购物车) → A-18(购物车页)
                                                          ↓
A-28(仪表盘) ← A-27(发票) ← A-26(物流) ← A-25(退款) ← A-24(优惠券核销) ← A-23(优惠券管理) ← A-22(详情+状态) ← A-21(订单列表) ← A-20(创建) ← A-19(结算)
    ↓                                                                                               ↑
A-29(通知) → A-30(审计) → A-31(配置) → A-32(i18n) → A-33(暗黑) → A-34(测试) → A-35(部署)
    ↓
A-36(批量导入) → A-37(评价) → A-38(报表) → A-39(对账) → A-40(API文档)
```

**关键风险点：**
- A-04 Schema 设计错误 → 后续所有 CRUD 崩
- A-12/A-13 认证 → 后续所有受保护路由依赖
- A-20 事务 → 库存扣减错误会导致超卖

---

## 3. 编写执行流程

### 3.1 总体策略

**单 agent 串行执行，最终产出两份项目。** 

Agent 在两个文件夹中同步编写：
- **练习项目** (`projects/shopos/`)：递进式增量开发，每步代码中保留 TODO 注释引导学员补全。
- **答案项目** (`projects/shopos_answer/`)：同步写入完整可编译代码，不含任何 TODO，作为练习的最终参照。

每步在练习项目中先写入含 TODO 的骨架代码，编译通过后，再将该步对应的完整实现同步到答案文件夹。

### 3.2 每步执行标准流程

```
1. 阅读本规划中该 Step 的描述、前置依赖、核心知识点
2. 在练习项目 (shopos/) 中增量开发，代码中保留 TODO 注释
3. 编译验证练习: cd projects/shopos && cargo leptos build
4. 编译通过后，将该步的完整实现（去掉 TODO，补全答案）写入答案项目 (shopos_answer/)
5. 编译验证答案: cd projects/shopos_answer && cargo leptos build
6. 如编译失败：
   - 分析错误信息 → 修复代码
   - 重试编译，最多 5 次
7. 两个项目都编译通过后: git add + git commit
8. 进入下一步
```

> **注意：** 答案项目必须在 40 步全部完成后能作为一个完整项目独立编译运行，不得有未补全的 TODO 或缺失的模块。

### 3.3 分支策略

```
main
├── step-A-01  (脚手架)
├── step-A-02  (布局+主题)
├── step-A-03  (导航)
├── ...        (增量提交)
└── step-A-40  (最终)
```

每步独立 commit，依赖链上的问题可精确回滚到出错的步骤。

---

## 4. 逐阶段详细规划

---

### 阶段 1：项目骨架与数据库（A-01 ~ A-05）

**目标：** 搭建 SSR 项目骨架，设计数据库 Schema，创建种子数据。完成后项目可启动，侧边栏导航可用，数据库 6 张表就绪。

---

#### Step A-01: cargo-leptos 初始化 + 项目结构

| 属性 | 内容 |
|------|------|
| **前置** | 无 |
| **难度** | ⭐ |
| **核心知识点** | `cargo leptos new`、`lib.rs`/`main.rs`/`app.rs` 职责划分、Axum 集成 |

**执行清单：**

1. 运行 `cargo leptos new shopos` 在 `leptos-learn/projects/shopos/` 下创建项目
2. 确认项目结构：
   ```
   projects/shopos/
   ├── Cargo.toml
   ├── src/
   │   ├── main.rs    # server entry（启动 axum server）
   │   ├── lib.rs     # app 定义 + 路由配置
   │   └── app.rs     # 根组件
   ├── index.html
   └── style.css
   ```
3. 在 workspace `Cargo.toml` 的 `members` 中注册 `projects/shopos`
4. 配置 `[dependencies]` 引入：`leptos`、`leptos_router`、`leptos_axum`、`thaw`、`leptos-use`、`sqlx`、`serde`
5. 编辑 `lib.rs`：初始化路由骨架
6. 编辑 `app.rs`：创建根组件 `ShopOSApp`
7. **验证：** `cargo leptos build` 零错误零警告

**产出：** 可编译运行的 SSR 空项目

---

#### Step A-02: 路由布局 + Thaw UI 主题

| 属性 | 内容 |
|------|------|
| **前置** | A-01 |
| **难度** | ⭐ |
| **核心知识点** | `<Router/>` 嵌套布局、Thaw `<ConfigProvider/>` 主题定制、CSS 变量 |

**执行清单：**

1. 在 `lib.rs` 中配置路由布局：
   ```rust
   <Router>
       <Routes>
           <Route path="" view=Layout>
               // 子路由占位
           </Route>
       </Routes>
   </Router>
   ```
2. 创建 `src/layout.rs`：Layout 组件包裹 `<ConfigProvider/>`
3. 配置 Thaw UI 主题：
   - 主色 `--primary-color`
   - 字体大小、圆角等
   - 通过 `ConfigProvider` 的 prop 传入
4. 定义全局 CSS 变量（`style.css`）
5. **验证：** `cargo leptos build` 通过，启动后页面有 Thaw UI 样式

**产出：** 有 Thaw 主题的 Layout 骨架

---

#### Step A-03: 侧边栏导航 + 顶部栏

| 属性 | 内容 |
|------|------|
| **前置** | A-02 |
| **难度** | ⭐ |
| **核心知识点** | Thaw `<Layout/>` `<Sider/>` `<Header/>` `<Content/>`、`<Menu/>` 递归渲染 |

**执行清单：**

1. 创建 `src/components/nav/` 目录
2. 实现 `Sidebar` 组件：使用 Thaw `<Sider/>` + `<Menu/>`
   - 菜单项：商品管理、用户管理、订单管理、运营管理、数据分析、系统设置
   - 每个一级菜单下含二级子项
   - 菜单折叠/展开功能
3. 实现 `Topbar` 组件：使用 Thaw `<Header/>`
   - 显示当前用户头像/名称（先放占位）
   - 退出按钮（占位）
4. 在 Layout 中组合：`<Layout>` → `<Sider>` + `<Header>` + `<Content>`
5. 菜单点击通过 `<A/>` 跳转到对应路由
6. **验证：** 侧边栏点击展开收起，菜单项高亮激活态正确

**产出：** 完整的后台布局框架（侧边栏 + 顶栏 + 内容区）

---

#### Step A-04: 数据库 Schema 设计 + 迁移

| 属性 | 内容 |
|------|------|
| **前置** | A-03 |
| **难度** | ⭐⭐ |
| **核心知识点** | `sqlx migrate`、ER 设计、`sqlx::Pool` 共享 |

**执行清单：**

1. 创建 `migrations/` 目录，编写迁移 SQL
2. 设计 6 张数据表：

   **categories（商品类目）**
   ```sql
   CREATE TABLE categories (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       name TEXT NOT NULL,
       parent_id INTEGER REFERENCES categories(id),
       sort_order INTEGER DEFAULT 0,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

   **products（商品）**
   ```sql
   CREATE TABLE products (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       name TEXT NOT NULL,
       description TEXT,
       category_id INTEGER REFERENCES categories(id),
       price REAL NOT NULL,
       stock INTEGER DEFAULT 0,
       image_urls TEXT,  -- JSON array
       status TEXT DEFAULT 'draft',  -- draft/published/offline
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
       updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

   **product_skus（商品 SKU）**
   ```sql
   CREATE TABLE product_skus (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       product_id INTEGER NOT NULL REFERENCES products(id),
       sku_code TEXT UNIQUE NOT NULL,
       spec_name TEXT,   -- e.g. "颜色/尺寸"
       spec_value TEXT,  -- e.g. "红色/S"
       price REAL,
       stock INTEGER DEFAULT 0,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

   **users（用户）**
   ```sql
   CREATE TABLE users (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       username TEXT UNIQUE NOT NULL,
       email TEXT UNIQUE NOT NULL,
       password_hash TEXT NOT NULL,
       role TEXT DEFAULT 'user',  -- admin/user
       avatar_url TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

   **orders（订单）**
   ```sql
   CREATE TABLE orders (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       user_id INTEGER NOT NULL REFERENCES users(id),
       order_no TEXT UNIQUE NOT NULL,
       status TEXT DEFAULT 'pending_payment',
       total_amount REAL NOT NULL,
       address_id INTEGER REFERENCES addresses(id),
       coupon_id INTEGER REFERENCES coupons(id),
       discount_amount REAL DEFAULT 0,
       actual_amount REAL NOT NULL,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
       updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

   **order_items（订单明细）**
   ```sql
   CREATE TABLE order_items (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       order_id INTEGER NOT NULL REFERENCES orders(id),
       product_id INTEGER NOT NULL REFERENCES products(id),
       sku_code TEXT,
       product_name TEXT NOT NULL,
       price REAL NOT NULL,
       quantity INTEGER NOT NULL,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```

   **addresses（收货地址）**、**coupons（优惠券）**、**reviews（评价）**、**audit_logs（审计日志）** 等后续阶段按需创建

3. 配置 `sqlx::Pool<Sqlite>` 作为 Axum State
4. 在 `main.rs` 服务器启动时执行 `sqlx::migrate!().run(&pool).await`
5. **验证：** 启动后数据库文件创建成功，所有表存在，migration 日志输出正常

**产出：** 6 张核心表 + 数据库连接池

---

#### Step A-05: 数据库初始化 + 种子数据

| 属性 | 内容 |
|------|------|
| **前置** | A-04 |
| **难度** | ⭐⭐ |
| **核心知识点** | 启动时自动迁移、种子数据脚本、`Resource` 健康检查 |

**执行清单：**

1. 创建 `src/db/seed.rs`：种子数据模块
2. 插入种子数据：
   - 类目（约 5-10 条，含父子关系）
   - 示例商品（约 10-20 条，关联到类目）
   - 示例 SKU（每种商品 2-3 个 SKU）
   - 管理员用户（username: `admin`, 密码哈希存储）
3. 启动时检查数据是否存在，不存在则自动插入（幂等，重复运行不报错）
4. 创建数据库健康检查 Server Function：
   ```rust
   #[server]
   pub async fn db_health_check() -> Result<String, ServerFnError> {
       let pool = use_context::<Pool<Sqlite>>().unwrap();
       sqlx::query_scalar("SELECT COUNT(*) FROM products").fetch_one(&pool).await?;
       Ok("OK".into())
   }
   ```
5. **验证：** 启动后控制台输出 seeding 日志，数据库中有初始数据

**产出：** 可启动的后台系统，数据库中有初始数据

---

### 阶段 2：商品管理（A-06 ~ A-11）

**目标：** 实现完整的商品管理模块——类目、列表、搜索、新增、SKU、详情。完成后可完整地进行商品 CRUD。

---

#### Step A-06: 商品类目管理

| 属性 | 内容 |
|------|------|
| **前置** | A-05 |
| **难度** | ⭐⭐ |
| **核心知识点** | 无限级分类树、`#[server]` 递归查询、Thaw `<Tree/>` 展示 |

**执行清单：**

1. 创建 Server Functions（`src/server/categories.rs`）：
   - `get_category_tree()` — 递归查询所有类目，组装为树形 JSON
   - `create_category(name, parent_id)` — 新建类目
   - `update_category(id, name)` — 重命名
   - `delete_category(id)` — 删除（检查无子类目和无商品关联才可删）
2. 创建类目管理页面 `src/pages/categories.rs`：
   - 左侧：Thaw `<Tree/>` 展示类目树
   - 右侧：选中类目后的详情/编辑表单
   - 新建/编辑/删除按钮，操作后刷新树
3. 添加路由 `/admin/categories`
4. **验证：** 可创建多级类目，展开折叠正常，删除有子类目的节点应提示错误

**产出：** 类目 CRUD 完整功能

---

#### Step A-07: 商品列表页

| 属性 | 内容 |
|------|------|
| **前置** | A-06 |
| **难度** | ⭐⭐ |
| **核心知识点** | Thaw `<Table/>`、`Resource` 分页查询、`use_params_map` 页码同步 URL |

**执行清单：**

1. 创建 Server Function `list_products(page, page_size, category_id?, keyword?)`：
   - 分页查询（`LIMIT .. OFFSET`）
   - 返回 `{ items: Vec<ProductRow>, total: i64 }`
2. 创建商品列表页 `src/pages/products/list.rs`：
   - Thaw `<Table/>` 展示：商品图片缩略图、名称、类目、价格、库存、状态、操作
   - 分页器组件（`<Pagination/>`），页码同步到 URL query `?page=2`
   - 每行操作按钮：编辑、上架/下架、删除（带确认弹窗）
3. 添加路由 `/admin/products`
4. **验证：** 种子数据中商品正确展示，分页切换正常，URL 参数变化时列表重新加载

**产出：** 商品列表页

---

#### Step A-08: 商品搜索 + 多条件过滤

| 属性 | 内容 |
|------|------|
| **前置** | A-07 |
| **难度** | ⭐⭐ |
| **核心知识点** | `use_debounce` 防抖搜索、高级筛选面板、URL query 参数同步 |

**执行清单：**

1. 在商品列表页上方添加搜索栏：
   - 搜索输入框 + `use_debounce` 防抖 300ms
   - 类目下拉筛选（`<Select/>` 从 A-06 获取）
   - 价格区间筛选
   - 商品状态筛选（草稿/已上架/已下架）
2. 所有筛选条件同步到 URL query：`?q=xxx&category_id=1&status=published`
3. 扩展 `list_products` Server Function 以支持多条件查询
4. 搜索结果高亮匹配关键词（前端 `String.replace(keyword, '<mark>$&</mark>')`）
5. **验证：** 输入搜索词后防抖延迟搜索结果更新，切换筛选条件列表刷新，刷新页面筛选条件保留

**产出：** 带搜索和筛选的商品列表

---

#### Step A-09: 新增商品表单

| 属性 | 内容 |
|------|------|
| **前置** | A-08 |
| **难度** | ⭐⭐ |
| **核心知识点** | Thaw `<Form/>` `<Input/>` `<Select/>` `<DatePicker/>`、复杂表单验证 |

**执行清单：**

1. 创建商品表单组件 `src/components/product_form.rs`（新增和编辑共用）
2. 表单字段：
   - 商品名称（必填，`<Input/>`）
   - 所属类目（必填，`<TreeSelect/>`）
   - 描述（`<Textarea/>`，支持简单 Markdown）
   - 价格（必填，`<InputNumber/>`，> 0）
   - 图片上传（先做 URL 输入，后续替换）
   - 状态（`<Select/>`：草稿/上架）
3. 表单验证：
   - 字段级验证（即时提示）
   - 提交前整体验证（阻止提交 + 滚动到第一个错误字段）
4. Server Function `create_product(data)`：插入数据 + 返回新商品 ID
5. 提交成功后跳转到商品列表页
6. **验证：** 表单必填项为空时提交按钮禁用，验证消息正常显示，新增后列表出现新商品

**产出：** 商品新增功能

---

#### Step A-10: 编辑商品 + SKU 管理

| 属性 | 内容 |
|------|------|
| **前置** | A-09 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 多 SKU 动态表单（`<For/>` 增删行）、库存字段联动、`Action` 乐观更新 |

**执行清单：**

1. 编辑页面复用 A-09 的表单组件，通过路由参数 `:id` 加载已有数据
2. Server Function `get_product_detail(id)`：返回商品 + SKU 列表
3. SKU 管理面板（在编辑页面中）：
   - 动态表格：`<For/>` 渲染 SKU 行
   - 每行：SKU 编码、规格名称/值、独立价格、独立库存
   - 新增 SKU 行 / 删除 SKU 行按钮
   - 删除前检查是否有订单引用
4. Server Functions：
   - `update_product(id, data)`
   - `create_sku(product_id, data)`
   - `update_sku(id, data)`
   - `delete_sku(id)`
5. `Action` 乐观更新：提交时先更新 UI，失败时回滚
6. 库存总量 = 所有 SKU 库存之和（派生信号）
7. **验证：** 编辑后数据正确保存，SKU 增删正常，SKU 库存联动显示总库存

**产出：** 商品编辑 + SKU 管理

---

#### Step A-11: 商品详情页 + 图片画廊

| 属性 | 内容 |
|------|------|
| **前置** | A-10 |
| **难度** | ⭐⭐ |
| **核心知识点** | 路由参数 `:id`、动态 `<Title/>`、Thaw `<Image/>` `<Carousel/>` 画廊 |

**执行清单：**

1. 创建商品详情页 `src/pages/products/detail.rs`
2. 页面结构：
   - 面包屑：商品管理 > 商品名称
   - 图片画廊：Thaw `<Carousel/>` (轮播图) 展示商品图片
   - 基本信息卡片：名称、类目、价格、库存、状态、时间
   - SKU 列表表格
   - 编辑按钮（跳转到编辑页）
3. 动态 `<Title/>`：`[商品名称] - ShopOS`
4. 路由：`/admin/products/:id`
5. 商品不存在时（404）显示错误提示而非白屏
6. **验证：** 点击列表中的商品名跳转到详情页，面包屑正确，图片轮播正常

**产出：** 商品详情页

---

### 阶段 3：用户与认证系统（A-12 ~ A-16）

**目标：** 实现用户注册、登录、Session 管理、个人信息、地址管理、账号安全。完成后系统有完整的用户认证体系。

---

#### Step A-12: 用户注册 + 密码哈希

| 属性 | 内容 |
|------|------|
| **前置** | A-11 |
| **难度** | ⭐⭐ |
| **核心知识点** | Server Action 注册、`argon2` 密码哈希、唯一性校验 |

**执行清单：**

1. 添加 `argon2` 到 `Cargo.toml` 依赖
2. 创建注册页面 `src/pages/auth/register.rs`：
   - 表单：用户名、邮箱、密码、确认密码
   - 实时验证：用户名 / 邮箱格式、密码强度（8 位+含数字和字母）、两次密码一致
3. Server Function `register_user(username, email, password)`：
   - 检查用户名和邮箱唯一性
   - `argon2` 哈希密码
   - 插入 `users` 表
4. 注册成功 → 自动登录 → 跳转到首页
5. 错误处理：用户名已存在 / 邮箱已存在 → 表单字段级错误提示
6. **验证：** 注册后数据库有用户记录，密码以哈希存储；重复注册同一用户名报错

**产出：** 用户注册功能

---

#### Step A-13: 登录 + Session 管理

| 属性 | 内容 |
|------|------|
| **前置** | A-12 |
| **难度** | ⭐⭐ |
| **核心知识点** | `create_server_action`、Cookie Session、`axum_session` |

**执行清单：**

1. 添加 `axum_session` / `tower-sessions` 到依赖
2. 创建登录页面 `src/pages/auth/login.rs`：
   - 用户名/邮箱 + 密码表单
   - "记住我"复选框
3. Server Function `login_user(username, password)`：
   - 查询用户，`argon2` 验证密码
   - 创建 Session（`axum_session`）
   - 返回用户信息（不含密码）
4. Session 管理：
   - 在 `main.rs` Axum 路由中配置 Session 中间件
   - 创建 `require_auth` 中间件：检查 Session，未登录返回 401 并重定向
5. 提供 Context：`use_auth_user() -> Option<UserInfo>`
6. 登录成功 → 跳转到首页；登录失败 → 显示错误消息
7. **验证：** 登录后 Cookie 中有 Session ID，刷新页面保持登录态；A-12 注册的账号能登录

**产出：** 登录 + Session 管理

---

#### Step A-14: 用户信息页 + 编辑

| 属性 | 内容 |
|------|------|
| **前置** | A-13 |
| **难度** | ⭐⭐ |
| **核心知识点** | 受保护路由（未登录重定向）、`use_context` 获取当前用户 |

**执行清单：**

1. 创建个人中心页面 `src/pages/user/profile.rs`：
   - 显示头像（默认占位图）、用户名、邮箱、注册时间
   - 编辑按钮 → 切换到编辑模式
   - 编辑模式：修改用户名、邮箱、头像 URL
2. Server Functions：
   - `get_user_profile()` — 返回当前登录用户信息
   - `update_user_profile(data)` — 更新用户信息
3. 受保护路由：`/admin/profile` 和 `/admin/profile/edit` 需要登录
4. 未登录访问 → 重定向到 `/login`，登录后跳回原页面
5. 顶部栏显示当前用户名（替换 A-03 的占位）
6. **验证：** 登录后才可访问 profile，未登录被重定向到登录页

**产出：** 用户个人中心

---

#### Step A-15: 收货地址管理

| 属性 | 内容 |
|------|------|
| **前置** | A-14 |
| **难度** | ⭐⭐ |
| **核心知识点** | 多地址 CRUD、默认地址设置、Thaw `<Table/>` 行操作 |

**执行清单：**

1. 创建 addresses 表（在 A-04 基础上补 migration）：
   ```sql
   CREATE TABLE addresses (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       user_id INTEGER NOT NULL REFERENCES users(id),
       receiver_name TEXT NOT NULL,
       phone TEXT NOT NULL,
       province TEXT NOT NULL,
       city TEXT NOT NULL,
       district TEXT NOT NULL,
       detail TEXT NOT NULL,
       is_default INTEGER DEFAULT 0,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 创建地址管理页面 `src/pages/user/addresses.rs`：
   - 地址列表（Thaw `<Table/>`）
   - 新建/编辑/删除/设为默认
   - 同一用户只有一个默认地址（设置新默认时取消旧的）
3. Server Functions：CRUD 地址 + `set_default_address(id)`
4. **验证：** 新增、编辑、删除地址正常；设置默认后旧默认取消

**产出：** 收货地址管理

---

#### Step A-16: 密码修改 + 账号安全

| 属性 | 内容 |
|------|------|
| **前置** | A-15 |
| **难度** | ⭐⭐ |
| **核心知识点** | 旧密码校验、Session 失效处理、登录日志记录 |

**执行清单：**

1. 创建安全设置页面 `src/pages/user/security.rs`
2. 密码修改功能：
   - 表单：旧密码 + 新密码 + 确认新密码
   - Server Function `change_password(old, new)`：
     - 验证旧密码正确
     - 哈希新密码并更新
     - 清除所有现有 Session（强制重新登录）
3. 修改密码后自动退出，跳转登录页提示"密码已修改，请重新登录"
4. 登录日志记录（可选）：
   - 在每次登录时插入 `login_logs` 表
   - 安全页面展示最近登录记录（IP、时间、设备）
5. **验证：** 修改密码后旧 Session 失效，用新密码可登录，旧密码不可登录

**产出：** 账号安全功能

---

### 阶段 4：购物车与订单（A-17 ~ A-22）

**目标：** 实现购物车（本地持久化）、下单结算、订单创建（事务）、订单列表、订单详情（状态流转）。完成后可完成完整的购买流程。

---

#### Step A-17: 购物车（本地持久化）

| 属性 | 内容 |
|------|------|
| **前置** | A-16 |
| **难度** | ⭐⭐ |
| **核心知识点** | leptos-use `use_local_storage`、购物车 Signal 派生计算（总价/数量） |

**执行清单：**

1. 定义购物车数据结构：
   ```rust
   #[derive(Clone, Serialize, Deserialize)]
   struct CartItem {
       product_id: i64,
       sku_code: String,
       name: String,
       price: f64,
       quantity: i32,
       image_url: String,
   }
   ```
2. 创建 `use_cart()` Hook（在 `src/hooks/` 下）：
   - 使用 `use_local_storage::<Vec<CartItem>>` 持久化
   - `add_item(item)` — 添加（已存在则 +1）
   - `remove_item(sku_code)` — 删除
   - `update_quantity(sku_code, qty)` — 更新数量（qty < 1 则删除）
   - `clear_cart()` — 清空
   - 派生信号：`total_count`（总件数）、`total_price`（总价）
3. 在顶部栏添加购物车图标 + 角标数字（显示 `total_count`）
4. **验证：** 添加商品到购物车 → 刷新页面 → 数据仍在 localStorage → 购物车角标更新

**产出：** 购物车 Hook + 持久化

---

#### Step A-18: 购物车页 + 数量/删除

| 属性 | 内容 |
|------|------|
| **前置** | A-17 |
| **难度** | ⭐⭐ |
| **核心知识点** | Thaw `<Table/>` 编辑模式、`update` 原地改值、空购物车 fallback |

**执行清单：**

1. 创建购物车页面 `src/pages/cart/list.rs`
2. 页面结构：
   - 购物车表格：商品图片、名称、SKU、单价、数量（可编辑 +/-）、小计、删除
   - 全选/取消全选复选框
   - 底部：总价汇总 + "去结算"按钮
   - 空购物车时显示 fallback 提示"购物车还是空的哦～" + 去逛逛按钮
3. 数量编辑：Thaw `<InputNumber/>`，`min=1`，用 `update` 原地更新
4. 删除按钮带确认：`<Modal/>` 确认弹窗
5. 路由：`/cart`
6. **验证：** 修改数量后总价实时更新，删除后行消失，空购物车显示 fallback

**产出：** 购物车页面

---

#### Step A-19: 下单结算页

| 属性 | 内容 |
|------|------|
| **前置** | A-18 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 地址选择、商品确认、Thaw `<Steps/>` 分步表单、金额汇总 |

**执行清单：**

1. 创建结算页面 `src/pages/checkout/mod.rs`
2. 分步表单（Thaw `<Steps/>`）：
   - **第 1 步：** 选择收货地址（从 A-15 获取），或新增地址
   - **第 2 步：** 确认商品清单（从购物车派生，只读展示）
   - **第 3 步：** 支付确认（汇总：商品总额、运费、优惠金额、实付金额、"提交订单"按钮）
3. 地址选择：`<Radio/>` 列表，支持选中 + 新建
4. 运费计算：固定逻辑（先简化为满 99 包邮，否则 10 元）
5. 金额汇总使用派生信号计算
6. 路由：`/checkout`
7. **验证：** 三步表单依次填写，运费正确计算，未登录时无法进入结算页

**产出：** 下单结算流程

---

#### Step A-20: 订单创建（事务）

| 属性 | 内容 |
|------|------|
| **前置** | A-19 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 数据库事务、库存扣减 + 回滚、`ServerFnError` 自定义错误 |

**执行清单：**

1. Server Function `create_order(address_id, cart_items)`：
   ```rust
   // BEGIN TRANSACTION
   // 1. 生成订单号
   // 2. INSERT orders
   // 3. FOR each item:
   //    a. 检查库存充足
   //    b. 扣减 product_skus.stock
   //    c. INSERT order_items
   // 4. COMMIT
   // 5. 如果任何一步失败 → ROLLBACK
   ```
2. 并发安全：使用行级锁或 `UPDATE ... WHERE stock >= ?` 确保不超卖
3. 错误类型：
   - `InsufficientStock` → 提示 "xxx 库存不足，仅剩 N 件"
   - `ProductNotFound` → 提示 "商品已下架"
4. 成功后：
   - 清空购物车
   - 跳转到订单详情页
5. 乐观更新 + 错误回滚
6. **验证：** 创建订单后数据库库存正确扣减；模拟库存不足场景确认事务回滚

**产出：** 订单创建（含事务）

---

#### Step A-21: 订单列表 + 筛选

| 属性 | 内容 |
|------|------|
| **前置** | A-20 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 多状态查询、Thaw `<Tabs/>` |

**执行清单：**

1. 创建订单列表页 `src/pages/orders/list.rs`
2. Thaw `<Tabs/>` 按状态分页：
   - 全部 | 待付款 | 已付款 | 已发货 | 已完成 | 已取消
3. 每种 tab 对应一个 `Resource`（按 status 参数查询）
4. 订单卡片/表格：订单号、时间、商品缩略图、金额、状态标签、操作按钮
5. 操作按钮按状态不同：
   - 待付款：去付款、取消
   - 已发货：确认收货
   - 已完成：查看详情
6. Server Function `list_orders(user_id, status?, page)`：
   - 分页查询 + 状态过滤
   - 含 order_items 聚合
7. **验证：** 各状态 tab 切换正确，订单列表数据从数据库中读取

**产出：** 订单列表页

---

#### Step A-22: 订单详情 + 状态流转

| 属性 | 内容 |
|------|------|
| **前置** | A-21 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 状态机（`enum OrderState` + 合法转换）、操作按钮条件显示 |

**执行清单：**

1. 定义订单状态机：
   ```rust
   enum OrderState {
       PendingPayment,  // 待付款
       Paid,            // 已付款
       Shipped,         // 已发货
       Completed,       // 已完成
       Cancelled,       // 已取消
   }
   // 合法转换：
   // PendingPayment → Paid | Cancelled
   // Paid → Shipped | Cancelled
   // Shipped → Completed
   ```
2. 创建订单详情页 `src/pages/orders/detail.rs`：
   - 订单基本信息（订单号、时间、状态、金额）
   - 商品列表（缩略图 + 名称 + 价格 + 数量）
   - 收货地址
   - 操作按钮（仅显示当前状态允许的下一步操作）
3. Server Function `get_order_detail(id)` + `update_order_state(id, new_state)`
4. 状态流转时：
   - 校验当前状态 → 目标状态的合法性
   - 无效转换返回错误
   - 取消订单时恢复库存
5. 路由：`/admin/orders/:id`
6. **验证：** 各状态间只能按合法路径转换，非法操作按钮禁用或隐藏

**产出：** 订单详情 + 状态流转

---

### 阶段 5：运营功能（A-23 ~ A-27）

**目标：** 实现优惠券系统（管理端 + 用户端）、退货退款、物流追踪、发票管理。

---

#### Step A-23: 优惠券系统（管理员端）

| 属性 | 内容 |
|------|------|
| **前置** | A-22 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 优惠券 CRUD、有效期校验、使用条件、Thaw `<Form/>` 动态规则 |

**执行清单：**

1. 创建 coupons 表：
   ```sql
   CREATE TABLE coupons (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       code TEXT UNIQUE NOT NULL,
       name TEXT NOT NULL,
       discount_type TEXT NOT NULL,  -- fixed/reduce/percent
       discount_value REAL NOT NULL,
       min_amount REAL DEFAULT 0,
       max_discount REAL,
       total_count INTEGER,
       used_count INTEGER DEFAULT 0,
       start_time DATETIME NOT NULL,
       end_time DATETIME NOT NULL,
       status TEXT DEFAULT 'active',
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 创建优惠券管理页面（管理员）`src/pages/admin/coupons.rs`：
   - 列表 + 新增/编辑弹窗
   - 字段：优惠码、名称、类型（满减/打折/固定减）、面值、使用门槛、总量、有效期
3. Server Functions：CRUD 优惠券
4. **验证：** 管理员可创建/编辑/停用优惠券

**产出：** 优惠券管理后台

---

#### Step A-24: 优惠券核销（用户端）

| 属性 | 内容 |
|------|------|
| **前置** | A-23 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 结算页输入优惠码、服务端验证 + 折扣计算、`Action` 错误回显 |

**执行清单：**

1. 在结算页（A-19 第 2/3 步）添加优惠码输入框
2. Server Function `validate_coupon(code, order_amount)`：
   - 检查优惠码是否存在且有效
   - 检查是否在有效期内
   - 检查是否达到使用门槛（`min_amount`）
   - 检查是否超过使用次数（`used_count < total_count`）
   - 计算折扣金额（满减/打折/固定金额）
   - 返回折扣后的实付金额
3. 前端展示：折扣金额明细，原价 → 优惠 → 实付
4. 订单创建时关联优惠券 ID，增加 `used_count`
5. 错误反馈：
   - 优惠码不存在 → "无效的优惠码"
   - 已过期 → "优惠码已过期"
   - 已领完 → "优惠码已被抢光"
   - 未达门槛 → "订单金额不足 ¥X"
6. **验证：** 输入有效优惠码 → 实付金额正确扣减；过期优惠码提示错误

**产出：** 优惠券核销

---

#### Step A-25: 退货/退款流程

| 属性 | 内容 |
|------|------|
| **前置** | A-24 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 售后单创建、审批流转（发起→审核→退款→完成）、状态机扩展 |

**执行清单：**

1. 创建 returns 表（refunds/returns）：
   ```sql
   CREATE TABLE returns (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       order_id INTEGER NOT NULL REFERENCES orders(id),
       user_id INTEGER NOT NULL REFERENCES users(id),
       reason TEXT NOT NULL,
       status TEXT DEFAULT 'pending_review',
       -- pending_review / approved / rejected / refunded / completed
       refund_amount REAL NOT NULL,
       admin_remark TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
       updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 用户端：订单详情页"申请退款"按钮
3. 创建退款申请表单（退款原因、退款金额、上传凭证）
4. 管理员端：售后管理页 `src/pages/admin/returns.rs`
   - 售后单列表（按状态 tab）
   - 审核操作：通过/拒绝（拒绝需填写理由）
   - 执行退款："退款"按钮（退款后恢复库存 → 更新订单状态）
5. Server Functions：
   - `request_refund(order_id, reason, amount)`
   - `review_return(id, approved, remark)`
   - `process_refund(id)`
6. 单据已发货时：需要先拦截物流（提示"请先退回商品"）
7. **验证：** 完整的退款流程：申请 → 审核 → 退款 → 库存恢复

**产出：** 退货退款系统

---

#### Step A-26: 物流追踪

| 属性 | 内容 |
|------|------|
| **前置** | A-25 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 物流信息表设计、模拟物流进度、Thaw `<Timeline/>` 时间线 |

**执行清单：**

1. 创建 shipments 表：
   ```sql
   CREATE TABLE shipments (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       order_id INTEGER NOT NULL REFERENCES orders(id),
       tracking_number TEXT NOT NULL,
       carrier TEXT DEFAULT '顺丰速运',
       status TEXT DEFAULT 'pending',
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );

   CREATE TABLE shipment_tracks (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       shipment_id INTEGER NOT NULL REFERENCES shipments(id),
       location TEXT NOT NULL,
       description TEXT NOT NULL,
       track_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 管理员端发货功能：
   - 订单详情页"发货"按钮 → 填写物流单号 + 选择快递公司
   - 自动生成模拟物流轨迹（3-5 条，从揽收到签收的时间线）
3. 用户端物流展示：
   - 订单详情页内嵌物流追踪
   - Thaw `<Timeline/>` 组件展示物流轨迹
4. Server Functions：
   - `ship_order(order_id, tracking_number)`
   - `get_shipping_tracks(order_id)`
5. **验证：** 发货后订单状态变为"已发货"，物流时间线正常展示

**产出：** 物流追踪

---

#### Step A-27: 发票管理

| 属性 | 内容 |
|------|------|
| **前置** | A-26 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 发票申请、PDF 生成预览、下载链接 |

**执行清单：**

1. 创建 invoices 表：
   ```sql
   CREATE TABLE invoices (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       order_id INTEGER NOT NULL REFERENCES orders(id),
       user_id INTEGER NOT NULL REFERENCES users(id),
       invoice_type TEXT NOT NULL,  -- personal/company
       title TEXT NOT NULL,
       tax_number TEXT,
       amount REAL NOT NULL,
       status TEXT DEFAULT 'pending',
       file_url TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 用户端：订单详情页"申请发票"按钮
   - 发票类型：个人/企业（企业需税号）
   - 发票抬头
3. 管理员端：发票管理页
   - 列表 + 审核开具
   - 开具后生成下载链接
4. PDF 生成（服务端）：使用 `printpdf` 或简化的 HTML 模板渲染
5. **验证：** 申请发票 → 管理员开具 → 用户下载

**产出：** 发票管理

---

### 阶段 6：数据分析与通知（A-28 ~ A-31）

**目标：** 实现数据仪表盘、实时通知（SSE）、操作审计日志、系统配置。

---

#### Step A-28: 数据仪表盘

| 属性 | 内容 |
|------|------|
| **前置** | A-27 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 聚合查询（GROUP BY）、Thaw `<Statistic/>` `<Card/>` 指标卡、图表 |

**执行清单：**

1. 创建仪表盘页面 `src/pages/dashboard/mod.rs`（管理员首页）
2. 指标卡（Thaw `<Statistic/>` + `<Card/>`）：
   - 今日订单数、今日销售额
   - 本周订单数、本周销售额
   - 商品总数、用户总数
   - 待处理售后数
3. 折线图/柱状图（使用 `leptos_chart` 或 Canvas WASM 绑定）：
   - 近 7 天销售额趋势
   - 近 7 天订单量趋势
4. Server Function `get_dashboard_stats()`：聚合查询
   ```sql
   SELECT DATE(created_at) as day,
          COUNT(*) as orders,
          SUM(actual_amount) as revenue
   FROM orders
   WHERE created_at >= date('now', '-7 days')
   GROUP BY DATE(created_at)
   ORDER BY day
   ```
5. **验证：** 仪表盘数据来自真实数据库，指标卡数值正确

**产出：** 数据仪表盘

---

#### Step A-29: 实时通知（SSE）

| 属性 | 内容 |
|------|------|
| **前置** | A-28 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | Server-Sent Events、`EventSource`、Thaw `<Notification/>` 弹出通知 |

**执行清单：**

1. 创建 Axum SSE 路由 `/api/notifications/stream`
2. 在 `main.rs` 中配置 SSE 端点：
   ```rust
   pub async fn sse_handler(/* ... */) -> Sse<impl Stream<Item = Result<Event, ...>>> {
       // 使用 tokio::sync::broadcast channel
   }
   ```
3. 前端 `NotificationBell` 组件：
   - 连接 SSE（`EventSource`）
   - 新消息到达 → Thaw `<Notification/>` 弹出
   - 未读消息角标
4. 通知场景：
   - 新订单产生 → 通知管理员
   - 订单状态变更 → 通知用户
   - 退款申请 → 通知管理员
5. 创建 notifications 表存储历史消息
6. Server Function `get_notifications()` + `mark_read(id)`
7. **验证：** 新订单产生时管理员页面右上角弹出通知

**产出：** SSE 实时通知系统

---

#### Step A-30: 操作审计日志

| 属性 | 内容 |
|------|------|
| **前置** | A-29 |
| **难度** | ⭐⭐ |
| **核心知识点** | 中间件拦截记录操作、日志列表查询与筛选、Thaw `<Table/>` 展示 |

**执行清单：**

1. 创建 audit_logs 表：
   ```sql
   CREATE TABLE audit_logs (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       user_id INTEGER,
       action TEXT NOT NULL,
       resource TEXT NOT NULL,
       resource_id TEXT,
       detail TEXT,
       ip_address TEXT,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 创建审计日志中间件（Axum middleware）：
   - 拦截所有管理端 API 请求
   - 记录：用户 ID、操作类型（CREATE/UPDATE/DELETE）、资源类型、资源 ID、请求体摘要
3. 审计日志页面（管理员）：`src/pages/admin/audit.rs`
   - Thaw `<Table/>` + 筛选（按用户、操作类型、时间范围）
   - 分页 + 详情弹窗
4. **验证：** 创建/编辑/删除商品后审计日志有对应记录

**产出：** 操作审计日志

---

#### Step A-31: 系统配置

| 属性 | 内容 |
|------|------|
| **前置** | A-30 |
| **难度** | ⭐⭐ |
| **核心知识点** | KV 配置表读写、Thaw `<Form/>` 动态配置项渲染 |

**执行清单：**

1. 创建 settings 表：
   ```sql
   CREATE TABLE settings (
       key TEXT PRIMARY KEY,
       value TEXT NOT NULL,
       description TEXT,
       updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 系统配置页面 `src/pages/admin/settings.rs`：
   - 站点名称、Logo URL、公告栏内容
   - 运费规则（满减门槛、基础运费）
   - 上传限制、分页大小等
3. 动态表单：遍历 settings 表渲染不同的输入组件
4. Server Function `get_all_settings()` + `update_setting(key, value)`
5. 全局 Context 提供 `use_settings()` Hook
6. **验证：** 修改站点名称后全局生效

**产出：** 系统配置管理

---

### 阶段 7：工程化与部署（A-32 ~ A-35）

**目标：** 实现多语言、暗黑模式、测试、Docker 部署。

---

#### Step A-32: 多语言 i18n

| 属性 | 内容 |
|------|------|
| **前置** | A-31 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | leptos-use `use_i18n`、语言切换 Signal、翻译 Key 管理 |

**执行清单：**

1. 安装 leptos-use 的 i18n 模块
2. 创建翻译文件 `src/i18n/`：
   - `zh-CN.toml`（中文）
   - `en-US.toml`（英文）
3. 包装所有硬编码中文文本为 i18n key：
   - 菜单项、按钮文本、表单标签、提示消息、表格列名
4. 语言切换：顶部栏语言下拉选择（中文/英文）
5. 语言偏好持久化到 localStorage
6. **验证：** 切换语言后所有 UI 文本即时更新

**产出：** 中英双语支持

---

#### Step A-33: 暗黑模式 + 响应式布局

| 属性 | 内容 |
|------|------|
| **前置** | A-32 |
| **难度** | ⭐⭐ |
| **核心知识点** | CSS 变量切换、`use_media_query`、移动端侧边栏折叠 |

**执行清单：**

1. 暗黑模式：
   - 使用 CSS 变量体系（或 Thaw ConfigProvider 的 dark 模式）
   - `use_media_query("(prefers-color-scheme: dark)")` 自动检测
   - 顶部栏主题切换按钮（☀️/🌙）
   - 偏好持久化到 localStorage
2. 响应式布局：
   - 移动端（< 768px）侧边栏默认折叠，汉堡菜单展开
   - 表格横向滚动
   - 表单字段改为单列布局
3. 使用 `use_media_query` 检测断点
4. **验证：** 浏览器窗口缩放到移动端尺寸时布局自适应；暗黑模式切换正常

**产出：** 暗黑模式 + 移动端适配

---

#### Step A-34: 单元测试 + 集成测试

| 属性 | 内容 |
|------|------|
| **前置** | A-33 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | `#[cfg(test)]` Server Function 测试、`RouterTestHarness` 组件测试 |

**执行清单：**

1. 编写 Server Function 单元测试：
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       #[tokio::test]
       async fn test_create_product() { /* ... */ }
       #[tokio::test]
       async fn test_create_order_transaction() { /* ... */ }
   }
   ```
2. 使用测试数据库（`:memory:` 模式），自动迁移 + 种子
3. 关键测试用例：
   - 商品 CRUD 全流程
   - 订单创建 + 库存扣减 + 事务回滚
   - 用户注册 + 登录 + Session
   - 优惠券核销 + 边界条件
   - 订单状态机非法转换
4. 组件测试（`leptos::testing` 或 `wasm-bindgen-test`）：
   - 购物车添加/删除/数量变更
   - 表单验证逻辑
5. **验证：** `cargo test` 所有测试通过

**产出：** 测试覆盖核心业务逻辑

---

#### Step A-35: Docker 构建 + CI/CD

| 属性 | 内容 |
|------|------|
| **前置** | A-34 |
| **难度** | ⭐⭐ |
| **核心知识点** | 多阶段 Dockerfile、`.github/workflows` CI、nginx 反向代理 |

**执行清单：**

1. 编写 Dockerfile：
   ```dockerfile
   # Stage 1: Build
   FROM rust:nightly AS builder
   # ... 编译 cargo-leptos 项目

   # Stage 2: Runtime
   FROM debian:bookworm-slim
   COPY --from=builder /app/target/release/shopos /app/shopos
   # ...
   ```
2. 编写 `docker-compose.yml`（含 SQLite 数据卷持久化）
3. nginx 配置文件（反向代理 + 静态资源缓存 + HTTPS）
4. GitHub Actions 工作流（`.github/workflows/deploy.yml`）：
   - 检出代码
   - `cargo test`
   - `docker build`
   - 推送到镜像仓库
5. 构建产物体积检查（< 20MB）
6. **验证：** `docker compose up` 后 http://localhost 可访问；首页 < 1s 响应

**产出：** Docker 部署方案 + CI/CD

---

### 阶段 8：增强功能与数据价值（A-36 ~ A-40）

**目标：** 批量导入、评价系统、数据导出报表、支付对账、API 文档。

---

#### Step A-36: 商品批量导入

| 属性 | 内容 |
|------|------|
| **前置** | A-35 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | Excel/CSV 解析、`calamine` 读取、批量事务写入、进度条反馈 |

**执行清单：**

1. 添加 `calamine` 依赖（读取 Excel）
2. 创建批量导入页面（管理员）`src/pages/admin/products/import.rs`
3. 功能：
   - 拖拽/选择 CSV 或 Excel 文件
   - 预览前 10 行（Thaw `<Table/>`），支持列映射
   - "开始导入"按钮 → `Action` → 进度条反馈
4. Server Function `import_products(file_data)`：
   - 解析文件
   - 批量 INSERT（每批 100 条，使用事务）
   - 返回导入结果（成功 N 条，失败 M 条 + 原因）
5. **验证：** 导入 1 万条商品数据在 5 秒内完成

**产出：** 商品批量导入

---

#### Step A-37: 商品评价系统

| 属性 | 内容 |
|------|------|
| **前置** | A-36 |
| **难度** | ⭐⭐ |
| **核心知识点** | 评价 CRUD、评分星选组件、Thaw `<Rate/>`、评价列表分页 |

**执行清单：**

1. 创建 reviews 表：
   ```sql
   CREATE TABLE reviews (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       user_id INTEGER NOT NULL REFERENCES users(id),
       product_id INTEGER NOT NULL REFERENCES products(id),
       order_id INTEGER REFERENCES orders(id),
       rating INTEGER NOT NULL CHECK(rating BETWEEN 1 AND 5),
       content TEXT,
       images TEXT,  -- JSON array
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 用户端：
   - 订单完成后"评价"入口
   - 评价表单：Thaw `<Rate/>` 星级 + 文字 + 图片上传
3. 商品详情页展示评价列表（分页 + 平均评分）
4. Server Functions：评价 CRUD + `get_product_reviews(product_id, page)`
5. **验证：** 评价后商品详情页显示评分和评价内容

**产出：** 商品评价系统

---

#### Step A-38: 数据导出报表

| 属性 | 内容 |
|------|------|
| **前置** | A-37 |
| **难度** | ⭐⭐ |
| **核心知识点** | 聚合查询（销售/用户/商品）、CSV 流式下载、XLSX 格式支持 |

**执行清单：**

1. 创建报表页面（管理员）`src/pages/admin/reports.rs`
2. 报表类型：
   - 销售报表（按日期汇总）
   - 用户报表（注册趋势）
   - 商品报表（销量排行）
3. 日期范围选择器 + 筛选条件
4. Server Function `export_report(report_type, date_range, format)`：
   - 生成 CSV/XLSX 数据
   - 返回下载 URL 或 base64 内容
5. 浏览器触发下载（`<a download>` 或 blob URL）
6. CSV 文件用 Excel 打开不乱码（BOM + UTF-8）
7. **验证：** 导出 CSV 文件内容与页面数据一致

**产出：** 数据导出报表

---

#### Step A-39: 支付流水对账

| 属性 | 内容 |
|------|------|
| **前置** | A-38 |
| **难度** | ⭐⭐⭐ |
| **核心知识点** | 支付记录表设计、对账差异比对、`Action` 标记异常 |

**执行清单：**

1. 创建 payment_records 表：
   ```sql
   CREATE TABLE payment_records (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       order_id INTEGER NOT NULL REFERENCES orders(id),
       transaction_id TEXT,
       payment_method TEXT,
       amount REAL NOT NULL,
       status TEXT,
       paid_at DATETIME,
       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
   );
   ```
2. 对账页面（管理员）`src/pages/admin/reconciliation.rs`
3. 功能：
   - 支付流水列表（带筛选）
   - 对账比对：订单实付金额 vs 支付记录金额
   - 差异高亮标记（红色警告）
   - 异常标记 + 备注
4. Server Function `reconcile_payments()`：比对 orders.actual_amount 与 payment_records.amount
5. **验证：** 支付金额与订单金额不一致的被标记为异常

**产出：** 支付对账功能

---

#### Step A-40: API 文档自动生成

| 属性 | 内容 |
|------|------|
| **前置** | A-39 |
| **难度** | ⭐⭐ |
| **核心知识点** | `utoipa` / OpenAPI 集成、Swagger UI 路由挂接、接口调试页 |

**执行清单：**

1. 添加 `utoipa` + `utoipa-swagger-ui` 到依赖
2. 给所有 `#[server]` 函数添加 `#[derive(utoipa::ToSchema)]` 和文档注释
3. 在 Axum 路由中挂接 Swagger UI 页面：`/api/docs`
4. OpenAPI 装饰：
   ```rust
   /// 创建订单
   #[utoipa::path(
       post,
       path = "/api/orders",
       request_body = CreateOrderRequest,
       responses(...)
   )]
   #[server]
   pub async fn create_order(...) { ... }
   ```
5. 生成的 Swagger 页面展示所有 Server Function 端点
6. **验证：** 访问 `/api/docs` 看到完整的 API 文档，每个端点可展开查看请求/响应示例

**产出：** OpenAPI 文档 + Swagger UI

---

## 5. 验证标准

| 检查项 | 说明 |
|--------|------|
| **编译检查** | `cargo leptos build` 零错误零警告（练习和答案两个项目均需通过） |
| **路由完整性** | 所有页面路由可达，404 兜底正常 |
| **数据持久化** | 刷新后数据不丢失（SQLite） |
| **错误边界** | Server Function 失败时 UI 显示错误提示而非白屏 |
| **表单验证** | 必填字段为空时禁止提交并显示验证消息 |
| **状态机合法性** | 订单状态只能按合法路径转换 |
| **未授权访问** | 未登录状态重定向到登录页而非显示空白 |
| **响应式布局** | 移动端侧边栏折叠、表格水平滚动 |
| **构建产物体积** | Docker 镜像 < 20MB |
| **递进兼容** | Step N 的修改不破坏 Step N-1 的已有功能 |
| **答案完整性** | 答案项目 (`shopos_answer/`) 不含任何 TODO/FIXME，独立 `cargo leptos build` 零错误 |

---

## 6. 风险与缓解

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| A-04 Schema 设计错误 | 中 | 高 | 数据库迁移版本化，发现问题时新建 migration 修复而非回退已跑过的 |
| A-12/A-13 认证 Bug | 中 | 高 | 写出明确的 Session 测试用例 |
| A-20 事务回滚失败 | 中 | 高 | 在测试中覆盖库存不足和超卖场景 |
| 编译失败循环 | 中 | 低 | 每步独立 commit，回滚成本低 |
| Thaw UI 版本兼容 | 低 | 中 | 锁定 Thaw 版本号 |

---

## 7. Agent 启动指令

```
## 任务: 编写项目 A — ShopOS 电商后台（练习 + 答案双文件夹）

## 上下文
工作区: c:\code\testruetlearn\leptos-learn\projects\
练习项目: shopos/          (含 TODO，供学员补全)
答案项目: shopos_answer/   (完整可编译运行，无 TODO)
步数: 40 (A-01 到 A-40)
结构: cargo-leptos SSR 项目

## 特别注意
- 强依赖链: 每一步依赖前一步
- 每步需同步维护两个文件夹：练习（含 TODO）+ 答案（完整代码）
- Step A-04 Schema 设计错误 → 后续所有 CRUD 崩
- Step A-12/A-13 认证 → 后续所有受保护路由依赖
- Step A-20 事务 → 库存扣减错误会导致超卖
- 答案项目必须在 40 步全部完成后能独立编译运行，零错误零警告，不含 TODO

## 执行流程
从 A-01 开始，串行到 A-40。

每步执行:
1. 阅读 project-shopos-execution-plan.md 中该 Step 的描述和知识点
2. 在练习项目 (shopos/) 中增量开发，保留 TODO
3. cargo leptos build 验证练习
4. 将完整代码同步到答案项目 (shopos_answer/)，去掉 TODO
5. cargo leptos build 验证答案
6. 失败则修复（最多 5 次）
7. 通过后 git commit + 进入下一步
```

---

## 8. Workspace 注册

workspace `Cargo.toml` 中需预注册两个项目的路径：

```toml
[workspace]
members = [
    # ... 其他章节成员 ...
    
    # 终极项目 A — ShopOS
    "projects/shopos",
    "projects/shopos_answer",
    
    # 终极项目 B — NoteFlow
    "projects/noteflow",
    "projects/noteflow_answer",
]
```

---

## 9. 进度追踪

使用 `progress.json` 追踪进度：

```json
{
  "project": "shopos",
  "steps": [
    { "step": "A-01", "status": "pending", "commits": 1, "time": null },
    { "step": "A-02", "status": "pending", "commits": 1, "time": null }
  ],
  "total_steps": 40,
  "completed_steps": 0,
  "last_updated": "2026-07-28T00:00:00Z"
}
```
