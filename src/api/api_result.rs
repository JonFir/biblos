use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use core::str;
use serde::{Serialize, Serializer};
use std::borrow::Cow;

#[derive(Serialize, Debug)]
pub struct ApiResponce<Result>
where
    Result: Into<Json<Result>>,
{
    #[serde(serialize_with = "status_code_serlialize")]
    code: StatusCode,
    result: Option<Result>,
    error: Option<Cow<'static, str>>,
}

fn status_code_serlialize<S>(value: &StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(value.as_u16())
}

impl<Result> ApiResponce<Result> {
    pub fn success(code: StatusCode, result: Result) -> Self {
        ApiResponce {
            code,
            result: Some(result),
            error: None,
        }
    }

    pub fn ok(result: Result) -> Self {
        ApiResponce {
            code: StatusCode::OK,
            result: Some(result),
            error: None,
        }
    }

    pub fn error<S>(code: StatusCode, error: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        ApiResponce {
            code,
            result: None,
            error: Some(error.into()),
        }
    }
}

impl<T> IntoResponse for ApiResponce<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.code, Json(self)).into_response()
    }
}

pub struct AppError(anyhow::Error);
pub type AppResult<V> = Result<ApiResponce<V>, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ApiResponce::<()>::error(StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string())
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
