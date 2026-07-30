use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub user: Option<UserInfo>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    pub key: String,
    pub label: String,
    pub icon: String,
    pub path: Option<String>,
    pub children: Option<Vec<NavItem>>,
}

pub fn get_sidebar_menu() -> Vec<NavItem> {
    vec![
        NavItem {
            key: "dashboard".into(),
            label: "仪表盘".into(),
            icon: "dashboard".into(),
            path: Some("/admin/dashboard".into()),
            children: None,
        },
        NavItem {
            key: "products".into(),
            label: "商品管理".into(),
            icon: "products".into(),
            path: None,
            children: Some(vec![
                NavItem {
                    key: "product-list".into(),
                    label: "商品列表".into(),
                    icon: "".into(),
                    path: Some("/admin/products".into()),
                    children: None,
                },
                NavItem {
                    key: "categories".into(),
                    label: "商品类目".into(),
                    icon: "".into(),
                    path: Some("/admin/categories".into()),
                    children: None,
                },
            ]),
        },
        NavItem {
            key: "users".into(),
            label: "用户管理".into(),
            icon: "users".into(),
            path: None,
            children: Some(vec![
                NavItem {
                    key: "user-list".into(),
                    label: "用户列表".into(),
                    icon: "".into(),
                    path: Some("/admin/users".into()),
                    children: None,
                },
            ]),
        },
        NavItem {
            key: "orders".into(),
            label: "订单管理".into(),
            icon: "orders".into(),
            path: None,
            children: Some(vec![
                NavItem {
                    key: "order-list".into(),
                    label: "订单列表".into(),
                    icon: "".into(),
                    path: Some("/admin/orders".into()),
                    children: None,
                },
            ]),
        },
        NavItem {
            key: "operations".into(),
            label: "运营管理".into(),
            icon: "operations".into(),
            path: None,
            children: Some(vec![
                NavItem {
                    key: "coupons".into(),
                    label: "优惠券管理".into(),
                    icon: "".into(),
                    path: Some("/admin/coupons".into()),
                    children: None,
                },
                NavItem {
                    key: "returns".into(),
                    label: "售后管理".into(),
                    icon: "".into(),
                    path: Some("/admin/returns".into()),
                    children: None,
                },
                NavItem {
                    key: "invoices".into(),
                    label: "发票管理".into(),
                    icon: "".into(),
                    path: Some("/admin/invoices".into()),
                    children: None,
                },
            ]),
        },
        NavItem {
            key: "analytics".into(),
            label: "数据分析".into(),
            icon: "analytics".into(),
            path: None,
            children: Some(vec![
                NavItem {
                    key: "reports".into(),
                    label: "数据报表".into(),
                    icon: "".into(),
                    path: Some("/admin/reports".into()),
                    children: None,
                },
                NavItem {
                    key: "reconciliation".into(),
                    label: "支付对账".into(),
                    icon: "".into(),
                    path: Some("/admin/reconciliation".into()),
                    children: None,
                },
            ]),
        },
        NavItem {
            key: "system".into(),
            label: "系统设置".into(),
            icon: "system".into(),
            path: None,
            children: Some(vec![
                NavItem {
                    key: "audit".into(),
                    label: "审计日志".into(),
                    icon: "".into(),
                    path: Some("/admin/audit".into()),
                    children: None,
                },
                NavItem {
                    key: "settings".into(),
                    label: "系统配置".into(),
                    icon: "".into(),
                    path: Some("/admin/settings".into()),
                    children: None,
                },
            ]),
        },
    ]
}
