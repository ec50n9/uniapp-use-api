use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommonResponse<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: Serialize> CommonResponse<T> {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, "success".to_string(), Some(data))
    }

    pub fn error(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}

impl<T: Serialize> Responder for CommonResponse<T> {
  type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
