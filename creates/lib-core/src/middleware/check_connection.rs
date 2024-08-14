// use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};

// use crate::get_mysql_pool;

// pub async fn check_pool_connection(
//     req: Request<Body>,
//     next: Next,
// ) -> Result<impl IntoResponse, AppError> {
//     // requires the http crate to get the header name
//     let pool = get_mysql_pool();

//     match pool {
//         Some(_) => Ok(next.run(req).await),
//         None => {
//             return Err(AppError::InternalServerError);
//         }
//     }
// }
