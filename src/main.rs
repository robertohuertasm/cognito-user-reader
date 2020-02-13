#![allow(clippy::cast_possible_truncation)]
mod models;

// use chrono::prelude::*;
use console::style;
use console::Emoji;
use models::{Cli, User, UserInfo};
use std::env;
use std::io::Write;
use std::process::Command;
use structopt::StructOpt;

static ERROR: Emoji<'_, '_> = Emoji("\u{26d4} ", "");
static ROCKET: Emoji<'_, '_> = Emoji("\u{1F680}", "");
static THUMB: Emoji<'_, '_> = Emoji("\u{1F44D}", "");
static TREE: Emoji<'_, '_> = Emoji("\u{1F335}", "");

fn main() -> std::io::Result<()> {
    let cli: Cli = Cli::from_args();
    let current_dir = env::current_dir().unwrap();
    let mut pagination_token: Option<String> = None;
    let mut users: Vec<User> = Vec::new();
    let mut limit: Option<u32> = None;
    let mut pending_users: u32 = 0;

    if let Some(max_users) = cli.max_number_users {
        if max_users <= 60 {
            limit = Some(max_users);
        } else {
            limit = Some(60);
            pending_users = max_users - 60;
        }
    }

    // loop until we get all the users that we want
    loop {
        match get_users(
            &cli.pool_id,
            &cli.region,
            &pagination_token,
            limit,
            cli.show_unconfirmed,
        ) {
            Ok(mut info) => {
                println!(
                    "{} {} {}",
                    ROCKET,
                    style(format!("We got a batch of {} users", info.users.len()))
                        .bold()
                        .green(),
                    ROCKET
                );
                pagination_token = info.pagination_token;
                users.append(&mut info.users);
            }
            Err(e) => {
                println!(
                    "{} {} {}\n{}",
                    ERROR,
                    style("SOMETHING WENT WRONG!").bold().red(),
                    ERROR,
                    style(e).red(),
                );
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

    // get the list of users in form of a string and the filtered users count
    let (content_file, filtered_len) = get_content(&users, &cli);

    // inform the user about how many users where found and filtered
    println!(
        "{}",
        style(format!(
            "{} {} USERS FOUND and {} SHOWN!",
            THUMB,
            users.len(),
            filtered_len
        ))
        .bold()
        .blue()
    );

    // export the list of users into a file
    let path = current_dir.join("cognito_users.csv");
    let mut file = std::fs::File::create(path.clone())?;
    file.write_all(content_file.as_bytes())?;
    println!(
        "{} {}",
        TREE,
        style(format!("You can find your file in {:?}", path)).magenta()
    );

    Ok(())
}

#[allow(clippy::needless_lifetimes)]
fn filter_by_user_id<'a>(cli: &'a Cli) -> impl FnMut(&&'a User) -> bool {
    move |&u| {
        if let Some(ref avoid) = cli.filtered_user_ids {
            let is_in = avoid.contains(&u.username);
            return if cli.include_user_ids { is_in } else { !is_in };
        }
        true
    }
}

#[allow(clippy::needless_lifetimes)]
fn filter_by_user_emails<'a>(cli: &'a Cli) -> impl FnMut(&&'a User) -> bool {
    move |&u| {
        if let Some(ref avoid) = cli.filtered_user_emails {
            let is_in = avoid.contains(&get_email(u));
            return if cli.include_user_emails {
                is_in
            } else {
                !is_in
            };
        }
        true
    }
}

fn get_content(users: &[User], cli: &Cli) -> (String, i32) {
    let mut filtered_len = 0;
    let content = users
        .iter()
        .filter(filter_by_user_id(cli))
        .filter(filter_by_user_emails(cli))
        // .filter(|&u| {
        //     let limit = Utc.ymd(2020, 2, 10).and_hms(0, 0, 0);
        //     let duration = u.creation_date().signed_duration_since(limit);
        //     duration.num_days() >= 0
        // })
        .fold("createdAt, id, email, status".to_owned(), |acc, u| {
            let email = get_email(u);
            let creation_date = u.creation_date();
            if cli.print_screen {
                println!(
                    "{} | {} | {} | {}",
                    style(creation_date).red(),
                    style(&u.username).green(),
                    &email,
                    style(&u.user_status).yellow()
                );
            }
            filtered_len += 1;
            format!(
                "{}\n{}",
                acc,
                format!(
                    "{}, {}, {}, {}",
                    creation_date, u.username, &email, u.user_status
                )
            )
        });
    (content, filtered_len)
}

fn get_email(user: &User) -> String {
    user.attributes
        .first()
        .map_or_else(|| "None".to_owned(), |x| x.value.clone())
}

fn get_users(
    pool_id: &str,
    region: &str,
    pagination_token: &Option<String>,
    limit: Option<u32>,
    show_unconfirmed: bool,
) -> Result<UserInfo, String> {
    let mut cmd = Command::new("aws");
    cmd.arg("cognito-idp")
        .arg("list-users")
        .arg("--user-pool-id")
        .arg(pool_id)
        .arg("--attributes-to-get")
        .arg("email")
        .arg("--region")
        .arg(region);

    if !show_unconfirmed {
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
