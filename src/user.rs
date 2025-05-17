use serde::{Deserialize, Serialize};


#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub i32);

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub password: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id : i64,
    pub anonymous: bool,
    pub username: String
}

impl Default for UserResponse {
    fn default() -> Self {
        Self {
            id: 0,
            username: String::from("default_user"),
            anonymous: true,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct UserEntry {
    pub username: String,
    pub password: String,
}

#[derive(PartialEq, Clone)]
pub struct UserList {
    pub list: Vec<User>,
}

impl UserList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn push(&mut self, todo: User) {
        self.list.push(todo);
    }
    
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl From<Vec<User>> for UserList {
    fn from(value: Vec<User>) -> Self {
        UserList { list: value }
    }
}


impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("default_user"),
            password: String::new(),
        }
    }
}