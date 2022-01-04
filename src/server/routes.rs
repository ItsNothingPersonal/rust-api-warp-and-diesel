use crate::database::{connection_pool::PgPool, with_db_access_manager};

use super::{
    api::{AddBook, UpdateStatus},
    handler, with_json_body,
};

use warp::Filter;

/// POST /books
pub fn add_book(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books") // Match /books path
        .and(warp::post()) // Match POST method
        .and(with_db_access_manager(pool)) // Add DBAccess Manager to params tuple
        .and(with_json_body::<AddBook>()) // Try to deserialize JSON body to AddBook
        .and_then(handler::add_book) // Pass the params touple to handler function
}

/// GET /books
pub fn list_books(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(handler::list_books)
}

/// GET /books/:id
pub fn get_book(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books" / i64)
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(handler::get_book_by_id)
}

/// PUT /books/:id
pub fn update_status(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books" / i64)
        .and(warp::put())
        .and(with_db_access_manager(pool))
        .and(with_json_body::<UpdateStatus>())
        .and_then(handler::update_status)
}

/// DELETE /books/:id
pub fn delete_book(
    pool: PgPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("books" / i64)
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(handler::delete_book)
}
