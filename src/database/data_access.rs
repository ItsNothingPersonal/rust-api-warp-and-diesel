use crate::error::error_variants::AppError;
use crate::error::ErrorType;
use crate::server::model::{BookDTO, BookStatus, CreateBookDTO};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn create_book(&self, dto: CreateBookDTO) -> Result<BookDTO, AppError> {
        use super::schema::books;

        diesel::insert_into(books::table) // insert into books table
            .values(&dto) // use values from CreateBookDTO
            .get_result(&self.connection) // execute query
            // if error occured map it to AppError
            .map_err(|err| AppError::from_diesel_err(err, "while creating book"))
    }

    pub fn list_books(&self) -> Result<Vec<BookDTO>, AppError> {
        use super::schema::books::dsl::*;

        books
            .load(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while listing books"))
    }

    pub fn update_book_status(
        &self,
        book_id: i64,
        new_status: BookStatus,
    ) -> Result<usize, AppError> {
        use super::schema::books::dsl::*;

        let updated = diesel::update(books)
            .filter(id.eq(book_id))
            .set(status.eq(new_status))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while updating book status"))?;

        if updated == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound));
        }
        Ok(updated)
    }

    pub fn delete_book(&self, book_id: i64) -> Result<usize, AppError> {
        use super::schema::books::dsl::*;

        let deleted = diesel::delete(books.filter(id.eq(book_id)))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while deleting book"))?;

        if deleted == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound));
        }
        Ok(deleted)
    }

    pub fn get_book_by_id(&self, book_id: i64) -> Result<BookDTO, AppError> {
        use super::schema::books::dsl::*;

        let book = books
            .filter(id.eq(book_id))
            .first::<BookDTO>(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(
                    err,
                    format!("while looking for book with id {}", book_id).as_str(),
                )
            })?;

        Ok(book)
    }
}
