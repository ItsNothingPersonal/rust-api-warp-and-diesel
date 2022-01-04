use super::api::{AddBook, UpdateStatus};
use super::respond;
use crate::{database::DBAccessManager, server::api::IdResponse};

pub async fn add_book(
    db_manager: DBAccessManager,
    new_book: AddBook,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling add book");

    let create_book_dto = new_book.to_dto();

    let id_response = db_manager
        .create_book(create_book_dto)
        .map(|book| IdResponse::new(book.id));

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn update_status(
    book_id: i64,
    db_manager: DBAccessManager,
    status_update: UpdateStatus,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling update status");

    let id_response = db_manager
        .update_book_status(book_id, status_update.status)
        .map(|_| IdResponse::new(book_id));

    respond(id_response, warp::http::StatusCode::OK)
}

pub async fn delete_book(
    book_id: i64,
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling delete book");

    let result = db_manager.delete_book(book_id);

    respond(result, warp::http::StatusCode::NO_CONTENT)
}

pub async fn list_books(db_manager: DBAccessManager) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling list books");

    let result = db_manager.list_books();

    respond(result, warp::http::StatusCode::OK)
}
