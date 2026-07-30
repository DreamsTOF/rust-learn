use std::collections::HashMap;
use std::sync::OnceLock;

static ZH_CN: OnceLock<HashMap<String, String>> = OnceLock::new();

fn get_map() -> &'static HashMap<String, String> {
    ZH_CN.get_or_init(|| {
        let mut m = HashMap::new();
        // App
        m.insert("app.name".into(), "ShopOS 电商管理后台".into());
        m.insert("app.tagline".into(), "全功能电商管理系统".into());
        m.insert("app.copyright".into(), "© 2024 ShopOS Team".into());

        // Navigation
        m.insert("nav.dashboard".into(), "仪表盘".into());
        m.insert("nav.products".into(), "商品管理".into());
        m.insert("nav.product_list".into(), "商品列表".into());
        m.insert("nav.categories".into(), "商品类目".into());
        m.insert("nav.users".into(), "用户管理".into());
        m.insert("nav.user_list".into(), "用户列表".into());
        m.insert("nav.orders".into(), "订单管理".into());
        m.insert("nav.order_list".into(), "订单列表".into());
        m.insert("nav.operations".into(), "运营管理".into());
        m.insert("nav.coupons".into(), "优惠券管理".into());
        m.insert("nav.returns".into(), "售后管理".into());
        m.insert("nav.invoices".into(), "发票管理".into());
        m.insert("nav.analytics".into(), "数据分析".into());
        m.insert("nav.reports".into(), "数据报表".into());
        m.insert("nav.reconciliation".into(), "支付对账".into());
        m.insert("nav.system".into(), "系统设置".into());
        m.insert("nav.audit".into(), "审计日志".into());
        m.insert("nav.settings".into(), "系统配置".into());

        // Actions
        m.insert("action.create".into(), "新增".into());
        m.insert("action.edit".into(), "编辑".into());
        m.insert("action.delete".into(), "删除".into());
        m.insert("action.save".into(), "保存".into());
        m.insert("action.cancel".into(), "取消".into());
        m.insert("action.confirm".into(), "确认".into());
        m.insert("action.search".into(), "搜索".into());
        m.insert("action.reset".into(), "重置".into());
        m.insert("action.export".into(), "导出".into());
        m.insert("action.import".into(), "导入".into());
        m.insert("action.batch_delete".into(), "批量删除".into());
        m.insert("action.batch_update".into(), "批量更新".into());
        m.insert("action.view".into(), "查看".into());
        m.insert("action.back".into(), "返回".into());
        m.insert("action.submit".into(), "提交".into());
        m.insert("action.upload".into(), "上传".into());
        m.insert("action.download".into(), "下载".into());
        m.insert("action.refresh".into(), "刷新".into());
        m.insert("action.more".into(), "更多".into());

        // Status
        m.insert("status.active".into(), "启用".into());
        m.insert("status.inactive".into(), "停用".into());
        m.insert("status.published".into(), "已上架".into());
        m.insert("status.draft".into(), "草稿".into());
        m.insert("status.archived".into(), "已归档".into());
        m.insert("status.pending".into(), "待处理".into());
        m.insert("status.processing".into(), "处理中".into());
        m.insert("status.completed".into(), "已完成".into());
        m.insert("status.cancelled".into(), "已取消".into());
        m.insert("status.refunded".into(), "已退款".into());

        // Product
        m.insert("product.name".into(), "商品名称".into());
        m.insert("product.category".into(), "商品分类".into());
        m.insert("product.price".into(), "价格".into());
        m.insert("product.stock".into(), "库存".into());
        m.insert("product.description".into(), "商品描述".into());
        m.insert("product.image".into(), "商品图片".into());
        m.insert("product.status".into(), "商品状态".into());
        m.insert("product.sku".into(), "SKU".into());
        m.insert("product.batch_import".into(), "批量导入".into());

        // Order
        m.insert("order.order_no".into(), "订单编号".into());
        m.insert("order.total_amount".into(), "订单金额".into());
        m.insert("order.payment_method".into(), "支付方式".into());
        m.insert("order.shipping_address".into(), "收货地址".into());
        m.insert("order.remark".into(), "订单备注".into());

        // User
        m.insert("user.username".into(), "用户名".into());
        m.insert("user.email".into(), "邮箱".into());
        m.insert("user.phone".into(), "手机号".into());
        m.insert("user.role".into(), "角色".into());
        m.insert("user.avatar".into(), "头像".into());

        // Login / Auth
        m.insert("auth.login".into(), "登录".into());
        m.insert("auth.register".into(), "注册".into());
        m.insert("auth.logout".into(), "退出登录".into());
        m.insert("auth.forgot_password".into(), "忘记密码".into());
        m.insert("auth.username_placeholder".into(), "请输入用户名".into());
        m.insert("auth.password_placeholder".into(), "请输入密码".into());
        m.insert("auth.login_success".into(), "登录成功".into());
        m.insert("auth.login_failed".into(), "登录失败，请检查用户名或密码".into());

        // Validation
        m.insert("validation.required".into(), "此项为必填项".into());
        m.insert("validation.email".into(), "请输入有效的邮箱地址".into());
        m.insert("validation.phone".into(), "请输入有效的手机号".into());
        m.insert("validation.min_length".into(), "长度不能少于{0}个字符".into());
        m.insert("validation.max_length".into(), "长度不能超过{0}个字符".into());
        m.insert("validation.price_positive".into(), "价格必须大于0".into());

        // Messages
        m.insert("message.operation_success".into(), "操作成功".into());
        m.insert("message.operation_failed".into(), "操作失败".into());
        m.insert("message.confirm_delete".into(), "确定要删除吗？此操作不可撤销。".into());
        m.insert("message.no_data".into(), "暂无数据".into());
        m.insert("message.loading".into(), "加载中...".into());
        m.insert("message.network_error".into(), "网络错误，请稍后重试".into());

        m
    })
}

pub fn get(key: &str) -> String {
    get_map()
        .get(key)
        .cloned()
        .unwrap_or_else(|| key.to_string())
}
