use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email_address: String,
    pub enabled: bool,
    pub email_confirmed: bool,
    pub role: String,
    pub created_at: DateTime<Utc>,
}