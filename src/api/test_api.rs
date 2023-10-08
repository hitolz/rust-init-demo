use actix_web::{
    get,
    web::{self},
    HttpResponse, Scope,
};

use crate::api::success;

///请求路由
pub fn routes() -> Scope {
    web::scope("/test").service(index)
}

/// 短信预览
#[get("")]
pub async fn index() -> HttpResponse {
    success(Some(1))
}
