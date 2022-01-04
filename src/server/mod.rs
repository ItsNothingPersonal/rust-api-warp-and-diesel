mod api;
mod handler;
pub mod model;
mod routes;

use self::routes::{add_book, delete_book, list_books, update_status};
use crate::{config, database::connection_pool::PgPool, error::error_variants::AppError};

use serde::{de::DeserializeOwned, Serialize};
use warp::Filter;

fn respond<T: Serialize>(
    result: Result<T, AppError>,
    status: warp::http::StatusCode,
) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            status,
        )),
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * config::MAX_REQUEST_SIZE_KB()).and(warp::body::json())
}

pub fn api_filters(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v1" / ..) // Add path prefix /api/v1 to all our routes
        .and(
            add_book(pool.clone())
                .or(update_status(pool.clone()))
                .or(delete_book(pool.clone()))
                .or(list_books(pool)),
        )
}
