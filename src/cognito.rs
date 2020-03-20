#![allow(clippy::missing_errors_doc)]

use crate::emojis::*;
use crate::models::{User, UserInfo};
use chrono::prelude::*;
use console::style;
use std::process::Command;

pub struct UserReader {
    pub aws_pool_id: String,
    pub aws_region: String,
}

pub struct UserReaderOptions<'a> {
    pub attributes_to_get: &'a Option<Vec<String>>,
    pub limit_of_users: Option<u32>,
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
        let aws_region = aws_pool_id
            .split('_')
            .next()
            .expect("Impossible to get the region from the pool id")
            .to_owned();
        Self {
            aws_pool_id,
            aws_region,
        }
    }

    pub fn get_users(
        &self,
        options: &UserReaderOptions,
        show_messages: bool,
    ) -> Result<Vec<User>, std::io::Error> {
        let mut users: Vec<User> = Vec::new();
        let mut pending_users: u32 = 0;
        let mut pagination_token: Option<String> = None;
        let mut limit: Option<u32> = None;

        if let Some(max_users) = options.limit_of_users {
            if max_users <= 60 {
                limit = Some(max_users);
            } else {
                limit = Some(60);
                pending_users = max_users - 60;
            }
        }

        // loop until we get all the users that we want
        loop {
            match get_users_from_cognito_idp(
                &self.aws_pool_id,
                &self.aws_region,
                &pagination_token,
                limit,
                options.show_unconfirmed_users,
                options.attributes_to_get,
            ) {
                Ok(mut info) => {
                    if show_messages {
                        println!(
                            "{} {} {}",
                            ROCKET,
                            style(format!("We got a batch of {} users", info.users.len()))
                                .bold()
                                .green(),
                            ROCKET
                        );
                    }
                    pagination_token = info.pagination_token;
                    users.append(&mut info.users);
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
            }

            if pending_users == 0 && limit.is_some() {
                break;
            }

            if pending_users <= 60 {
                limit = Some(pending_users);
                pending_users = 0;
            } else {
                limit = Some(60);
                pending_users -= 60;
            }

            if pagination_token.is_none() {
                break;
            }
        }

        // order by creation date
        users.sort_by(|a, b| a.user_create_date.partial_cmp(&b.user_create_date).unwrap());

        // apply filters
        users = users
            .into_iter()
            .filter(|u| {
                if let Some(ref avoid) = options.filtered_user_ids {
                    let is_in = avoid.contains(&u.username);
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
                    let duration = u.creation_date().signed_duration_since(created_at);
                    return duration.num_days() >= 0;
                }
                true
            })
            .collect();
        // return the list
        Ok(users)
    }
}

fn get_users_from_cognito_idp(
    pool_id: &str,
    region: &str,
    pagination_token: &Option<String>,
    limit: Option<u32>,
    show_unconfirmed_users: bool,
    attributes_to_get: &Option<Vec<String>>,
) -> Result<UserInfo, String> {
    let mut cmd = Command::new("aws");
    cmd.arg("cognito-idp")
        .arg("list-users")
        .arg("--user-pool-id")
        .arg(pool_id)
        .arg("--region")
        .arg(region)
        .arg("--attributes-to-get")
        .arg("email");

    if let Some(attributes) = attributes_to_get {
        for attribute in attributes {
            cmd.arg(attribute);
        }
    }

    if !show_unconfirmed_users {
        cmd.arg("--filter").arg("cognito:user_status = 'CONFIRMED'");
    }

    if let Some(pt) = pagination_token {
        cmd.arg("--pagination-token").arg(pt);
    }

    if let Some(l) = limit {
        if l > 60 {
            return Err("The limit cannot be greater than 60".to_string());
        }
        cmd.arg("--limit").arg(l.to_string());
    }

    let result = cmd
        .output()
        .map_err(|e| format!("Error calling aws {}", e))?;

    if result.status.success() {
        serde_json::from_slice(&result.stdout)
            .map_err(|e| format!("Error deserializing users: {}", e))
    } else {
        Err(std::str::from_utf8(&result.stderr).unwrap().into())
    }
}
