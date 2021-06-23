#![allow(clippy::missing_errors_doc)]

use crate::{
    emojis::{ERROR, ROCKET},
    UserTypeExt,
};

use chrono::prelude::*;
use console::style;
use rusoto_cognito_idp::{
    CognitoIdentityProvider, CognitoIdentityProviderClient, ListUsersRequest, ListUsersResponse,
    UserType,
};
use rusoto_core::Region;
use std::str::FromStr;

pub struct UserReader {
    pub aws_pool_id: String,
    pub aws_region: String,
    cognito_provider: CognitoIdentityProviderClient,
}

pub struct UserReaderOptions<'a> {
    pub attributes_to_get: &'a Option<Vec<String>>,
    pub limit_of_users: Option<i64>,
    pub show_unconfirmed_users: bool,
    pub filtered_user_ids: &'a Option<Vec<String>>,
    pub include_user_ids: bool,
    pub filtered_user_emails: &'a Option<Vec<String>>,
    pub include_user_emails: bool,
    pub created_at: Option<DateTime<Utc>>,
}

impl UserReader {
    #[must_use]
    pub fn new(aws_pool_id: String) -> Self {
        let raw_aws_region = Self::extract_region(&aws_pool_id);
        let aws_region: Region =
            Region::from_str(&raw_aws_region).expect("Wrong format for this pool id.");
        let cognito_provider = CognitoIdentityProviderClient::new(aws_region);

        Self {
            aws_pool_id,
            aws_region: raw_aws_region,
            cognito_provider,
        }
    }

    #[must_use]
    pub fn extract_region(pool_id: &str) -> String {
        pool_id
            .split('_')
            .next()
            .expect("Impossible to get the region from the pool id")
            .to_owned()
    }

    pub async fn get_users(
        &self,
        options: UserReaderOptions<'_>,
        show_messages: bool,
    ) -> Vec<UserType> {
        let mut users: Vec<UserType> = Vec::new();
        let mut pending_users: i64 = 0;
        let mut limit: Option<i64> = None;

        if let Some(max_users) = options.limit_of_users {
            println!("------ max users {}", max_users);
            if max_users <= 60 {
                limit = Some(max_users);
            } else {
                limit = Some(60);
                pending_users = max_users - 60;
            }
        }

        let mut req = ListUsersRequest {
            user_pool_id: self.aws_pool_id.clone(),
            attributes_to_get: options.attributes_to_get.clone(),
            filter: if options.show_unconfirmed_users {
                None
            } else {
                Some("cognito:user_status = 'CONFIRMED'".to_string())
            },
            limit,
            pagination_token: None,
        };

        // loop until we get all the users that we want
        loop {
            match self.cognito_provider.list_users(req.clone()).await {
                Ok(ListUsersResponse {
                    pagination_token,
                    users: Some(mut response_users),
                }) => {
                    if show_messages {
                        println!(
                            "{} {} {}",
                            ROCKET,
                            style(format!("We got a batch of {} users", response_users.len()))
                                .bold()
                                .green(),
                            ROCKET
                        );
                    }
                    req.pagination_token = pagination_token;
                    users.append(&mut response_users);
                }
                Err(e) => {
                    if show_messages {
                        println!(
                            "{} {} {}\n{}",
                            ERROR,
                            style("SOMETHING WENT WRONG!").bold().red(),
                            ERROR,
                            style(e).red(),
                        );
                    }
                }
                Ok(_x) => (),
            }

            if req.limit.is_some() {
                if pending_users == 0 {
                    break;
                } else if pending_users <= 60 {
                    req.limit = Some(pending_users);
                    pending_users = 0;
                } else {
                    req.limit = Some(60);
                    pending_users -= 60;
                }
            }

            if req.pagination_token.is_none() {
                break;
            }
        }

        Self::order_users(users, &options)
    }

    fn order_users(mut users: Vec<UserType>, options: &UserReaderOptions<'_>) -> Vec<UserType> {
        // order by creation date
        users.sort_by(|a, b| {
            a.user_create_date
                .partial_cmp(&b.user_create_date)
                .unwrap_or(std::cmp::Ordering::Less)
        });

        // apply filters
        users
            .into_iter()
            .filter(|u| {
                if let Some(ref avoid) = options.filtered_user_ids {
                    let username = u.username.as_deref().unwrap_or("");
                    let is_in = avoid.contains(&username.to_string());
                    return if options.include_user_ids {
                        is_in
                    } else {
                        !is_in
                    };
                }
                true
            })
            .filter(|u| {
                if let Some(ref avoid) = options.filtered_user_emails {
                    let is_in = avoid
                        .iter()
                        .any(|e| e.to_lowercase() == (&u.get_email()).to_lowercase());
                    return if options.include_user_emails {
                        is_in
                    } else {
                        !is_in
                    };
                }
                true
            })
            .filter(|u| {
                if let Some(created_at) = options.created_at {
                    let duration = &u.creation_date().signed_duration_since(created_at);
                    return duration.num_days() >= 0;
                }
                true
            })
            .collect()
    }
}

#[must_use]
pub fn users_to_csv(users: &[UserType], print_screen: bool) -> (String, i32) {
    let mut filtered_len = 0;

    let content = users.iter().fold(String::new(), |acc, u| {
        let creation_date = u.creation_date();
        let username = u.username.as_deref().unwrap_or("No username");
        let user_status = u.user_status.as_deref().unwrap_or("No user status");
        if print_screen {
            println!(
                "{} | {} | {} | {}",
                style(creation_date).red(),
                style(username).green(),
                style(user_status).yellow(),
                u.attributes_values_to_string(" | "),
            );
        }
        filtered_len += 1;
        format!(
            "{}\n{}",
            if acc.is_empty() {
                format!(
                    "createdAt,username,status,{}",
                    u.attributes_keys_to_string(",")
                )
            } else {
                acc
            },
            format!(
                "{},{},{},{}",
                creation_date,
                username,
                user_status,
                u.attributes_values_to_string(","),
            )
        )
    });
    (content, filtered_len)
}
