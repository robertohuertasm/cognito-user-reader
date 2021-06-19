use chrono::prelude::*;

pub trait UserTypeExt {
    fn creation_date(&self) -> DateTime<Utc>;
    fn get_attribute(&self, name: &str) -> String;
    fn get_email(&self) -> String;
    fn attributes_values_to_string(&self, separator: &str) -> String;
    fn attributes_keys_to_string(&self, separator: &str) -> String;
}

impl UserTypeExt for rusoto_cognito_idp::UserType {
    fn get_email(&self) -> String {
        self.get_attribute("email")
    }

    fn get_attribute(&self, name: &str) -> String {
        self.attributes.as_ref().map_or_else(String::new, |attrs| {
            attrs
                .iter()
                .find(|a| a.name == name)
                .map_or_else(|| "None".to_owned(), |a| a.value.clone().unwrap())
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn creation_date(&self) -> DateTime<Utc> {
        Utc.timestamp(
            self.user_create_date
                .expect("No creation date for this user") as i64,
            0,
        )
    }

    fn attributes_values_to_string(&self, separator: &str) -> String {
        self.attributes.as_ref().map_or_else(String::new, |attrs| {
            attrs.iter().fold(String::new(), |acc, a| {
                let value = a.value.as_deref().unwrap_or("No Value");
                if acc.is_empty() {
                    value.to_string()
                } else {
                    format!("{}{}{}", acc, separator, value)
                }
            })
        })
    }

    fn attributes_keys_to_string(&self, separator: &str) -> String {
        self.attributes.as_ref().map_or_else(String::new, |attrs| {
            attrs.iter().fold(String::new(), |acc, a| {
                if acc.is_empty() {
                    a.name.to_string()
                } else {
                    format!("{}{}{}", acc, separator, a.name)
                }
            })
        })
    }
}
