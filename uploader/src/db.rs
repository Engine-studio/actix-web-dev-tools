use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::pending_files;
use crate::error::{
    Result,
    ErrorType,
    ApiError,
};

#[derive(Deserialize,Serialize,Clone,Debug,Queryable)]
pub struct PendingFile {
    id: i64,
    url: String,
    upload: chrono::NaiveDateTime,
}

impl PendingFile { 
    pub async fn new(
        path: String,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::insert_into(pending_files::table)
            .values(&(pending_files::url.eq(path)))
            .execute(conn)?;
        Ok(())
    }
    pub async fn verify(
        paths: Vec<String>,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::delete(pending_files::table
            .filter(pending_files::url.eq_any(paths)))
            .execute(conn)?;
        Ok(())
    }
    pub async fn was_not_verifyed(
        path: &str,
        conn: &PgConnection,
    ) -> Result<bool> {
        let row = pending_files::table
            .filter(pending_files::url.eq(path))
            .get_results::<Self>(conn)?;
        if let Some(row) = row.get(0) {
            Self::verify(vec![row.url.clone()], conn).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

