#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use leptos::ServerFnError;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};

pub mod user_repository;

#[cfg(feature = "ssr")]
pub async fn get_db() -> Result<Arc<Pool<Postgres>>, ServerFnError>{

    use actix_web::web::Data;
    use leptos_actix::extract;

    let conn: Data<Pool<Postgres>> = extract().await?;
    let pool = conn.into_inner();
    Ok(pool)
}