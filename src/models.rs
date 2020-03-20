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
    pub fn get_attribute(&self, name: &str) -> String {
        self.attributes
            .iter()
            .find(|a| a.name == name)
            .map_or_else(|| "None".to_owned(), |a| a.value.clone())
    }

    #[must_use]
    pub fn get_email(&self) -> String {
        self.get_attribute("email")
    }

    #[must_use]
    pub fn attributes_values_to_string(&self, separator: &str) -> String {
        self.attributes.iter().fold(String::new(), |acc, a| {
            if acc.is_empty() {
                a.value.to_string()
            } else {
                format!("{}{}{}", acc, separator, a.value)
            }
        })
    }

    #[must_use]
    pub fn attributes_keys_to_string(&self, separator: &str) -> String {
        self.attributes.iter().fold(String::new(), |acc, a| {
            if acc.is_empty() {
                a.name.to_string()
            } else {
                format!("{}{}{}", acc, separator, a.name)
            }
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
