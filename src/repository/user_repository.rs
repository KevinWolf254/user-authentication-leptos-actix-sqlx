use std::sync::Arc;

use leptos::{ServerFnError, server};
use crate::{entity::user::User, CountResult, PaginatedResult};
use actix_web::web::Data;
use leptos_actix::extract;
use sqlx::{Pool, Postgres};

#[server(FindById, "/api", "GetJson", "users")]
pub async fn find_by_id(user_id: i32) -> Result<User, ServerFnError> {
    let conn = extract::<Data<Pool<Postgres>>>().await?;
    let pool: Arc<Pool<Postgres>> = conn.into_inner();

    sqlx::query_as!(User, 
        r#"SELECT * FROM "DEMO"."USER" WHERE user_id = $1 "#, user_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(CreateUser, "/api", "Url", "users/create")]
pub async fn create(name: String, email_address: String, role: String) -> Result<User, ServerFnError> {
    let conn: Data<Pool<Postgres>> = extract().await?;
    let pool = conn.into_inner();

    sqlx::query_as!(User, 
        r#"INSERT INTO "DEMO"."USER" (name, email_address, role) VALUES ($1, $2, $3) RETURNING * "#, 
        name, email_address, role)
        .fetch_one(&*pool) 
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))

}

#[server(FindPaginated, "/api", "GetJson", "users/paginated")]
pub async fn find_paginated(page: i64, page_size: i64) -> Result<PaginatedResult<User>, ServerFnError> {
    let conn: Data<Pool<Postgres>> = extract().await?;
    let pool = conn.into_inner();

    let offset = (page - 1) * page_size;
    let users =   sqlx::query_as!(User, r#"SELECT * FROM "DEMO"."USER" ORDER BY USER_id DESC LIMIT $1 OFFSET $2"#, page_size, offset)
            .fetch_all(&*pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

     let total = sqlx::query_as!(CountResult, 
            r#"SELECT COUNT(*) FROM "DEMO"."USER""#)
            .fetch_one(&*pool)
            .await 
            .map_err(|e| ServerFnError::new(e.to_string()))?;


    let result = PaginatedResult {
        data: users,
        total: total.count.unwrap_or(0),
        page,
        page_size
    };

    Ok(result)
}