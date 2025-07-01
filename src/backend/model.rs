use serde::{Serialize, Deserialize};

#[cfg(feature="server")]
#[derive(sqlx::FromRow)]
pub struct UserSql {
  pub id : i64,
  pub username: String,
  pub password: String
}

#[cfg(feature="server")]
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Robot {
  pub id: i64,
  pub name: String,
  pub owner: i64,
  pub online: bool,
}
