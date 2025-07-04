#![allow(dead_code)]

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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct UserSession {
  pub id : Option<i64>,
  pub name: Option<String>,
  pub logged_in: bool,
}

impl Default for UserSession {
  fn default() -> Self {
    UserSession {
      id: None,
      name: None,
      logged_in: false,
    }
  }
}

impl UserSession {
  pub fn new() -> Self {
    UserSession::default()
  }

  pub fn is_logged_in(&self) -> bool {
    self.logged_in
  }

  pub fn get_username(&self) -> Option<String> {
    self.name.clone()
  }

  pub fn login(&mut self, username: &str, id: i64) {
    self.name = Some(username.to_string());
    self.id = Some(id);
    self.logged_in = true;
  }

  pub fn logout(&mut self) {
    self.name = None;
    self.id = None;
    self.logged_in = false;
  }
}