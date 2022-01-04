pub mod connection_pool;
mod data_access;
pub mod schema;

use self::connection_pool::PgPool;
pub use self::data_access::DBAccessManager;
use crate::error::{error_variants::AppError, ErrorType};

use warp::{reject, Filter};

pub fn with_db_access_manager(
    pool: PgPool,
) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {
            match pool.get() {
                Ok(conn) => Ok(DBAccessManager::new(conn)),
                Err(err) => Err(reject::custom(AppError::new(
                    format!("Error getting connection from pool: {}", err).as_str(),
                    ErrorType::Internal,
                ))),
            }
        })
}
