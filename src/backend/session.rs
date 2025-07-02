#[cfg(feature="server")]
use axum_session::{Key, SessionConfig, SessionStore, SessionSqlitePool};
//#[cfg(feature="server")]
//use axum_session_sqlx::SessionSqlitePool;
#[cfg(feature="server")]
use super::db::get_db;



#[cfg(feature="server")]
pub async fn session() -> SessionStore<SessionSqlitePool> {
  let pool = get_db().await;
  let config = SessionConfig::new().with_table_name("session_table").with_key(Key::generate());

  SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config).await.unwrap()
}