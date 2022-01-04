use std::io::Write;

use crate::database::schema::books;

use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, not_none, AsExpression, FromSqlRow, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct BookDTO {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

/// Struct for creating Books
#[derive(Debug, Clone, Insertable)]
#[table_name = "books"]
pub struct CreateBookDTO {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum BookStatus {
    WantToRead,
    Reading,
    Finished,
    Rereading,
}

impl ToSql<Text, Pg> for BookStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            BookStatus::WantToRead => out.write_all(b"WANT_TO_READ")?,
            BookStatus::Reading => out.write_all(b"READING")?,
            BookStatus::Finished => out.write_all(b"FINISHED")?,
            BookStatus::Rereading => out.write_all(b"REREADING")?,
        }

        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for BookStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"WANT_TO_READ" => Ok(BookStatus::WantToRead),
            b"READING" => Ok(BookStatus::Reading),
            b"FINISHED" => Ok(BookStatus::Finished),
            b"REREADING" => Ok(BookStatus::Reading),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
