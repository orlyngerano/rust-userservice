use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "username: {} password: {} first_name: {} last_name: {}",
            self.username.clone().unwrap(),
            self.password.clone().unwrap(),
            self.first_name.clone().unwrap(),
            self.last_name.clone().unwrap(),
        )
    }
}
