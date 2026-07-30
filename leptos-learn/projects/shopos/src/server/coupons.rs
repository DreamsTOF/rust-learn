use leptos::prelude::*;

use crate::Coupon;

#[server(ListCoupons)]
pub async fn list_coupons(page: i64, page_size: i64) -> Result<Vec<Coupon>, ServerFnError> {
    // TODO: Implement this server function
    // Hint: query coupons from the database with pagination (page, page_size),
    //       ordered by id DESC, and return them as Vec<Coupon>
    unimplemented!()
}

#[server(CreateCoupon)]
pub async fn create_coupon(data: String) -> Result<i64, ServerFnError> {
    // TODO: Implement this server function
    // Hint: parse `data` as JSON into a Coupon struct, then insert a new coupon record
    //       into the database. Return the last_insert_rowid() of the new coupon
    unimplemented!()
}

#[server(UpdateCoupon)]
pub async fn update_coupon(id: i64, data: String) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: parse `data` as JSON into a Coupon struct, then update the coupon with the given id.
    //       Return true if any row was affected
    unimplemented!()
}

#[server(DeleteCoupon)]
pub async fn delete_coupon(id: i64) -> Result<bool, ServerFnError> {
    // TODO: Implement this server function
    // Hint: delete the coupon with the given id from the database.
    //       Return true if any row was affected
    unimplemented!()
}

#[server(ValidateCoupon)]
pub async fn validate_coupon(
    code: String,
    order_amount: f64,
) -> Result<String, ServerFnError> {
    // TODO: Implement this server function
    // Hint: look up the coupon by code, check it is active and within the valid time range,
    //       verify the order amount meets min_amount and total_count hasn't been exceeded,
    //       calculate the discount, and return a JSON string with the validation result
    unimplemented!()
}
