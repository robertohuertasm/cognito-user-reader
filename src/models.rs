use chrono::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub users: Vec<User>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub username: String,
    pub attributes: Vec<Attribute>,
    pub user_create_date: f32,
    pub user_last_modified_date: f32,
    pub enabled: bool,
    pub user_status: String,
}

impl User {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn creation_date(&self) -> DateTime<Utc> {
        Utc.timestamp(self.user_create_date as i64, 0)
    }

    #[must_use]
    pub fn get_email(&self) -> String {
        self.attributes
            .first()
            .map_or_else(|| "None".to_owned(), |x| x.value.clone())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
