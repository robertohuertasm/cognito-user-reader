mod models;

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

    loop {
        match get_users(&cli.pool_id, &cli.region, &pagination_token) {
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
                users.append(&mut info.users)
            }
            Err(e) => {
                println!(
                    "{} {} {}\n{}",
                    ERROR,
                    style("SOMETHING WENT WRONG!").bold().red(),
                    ERROR,
                    style(e).red(),
                );
                return Ok(());
            }
        }

        if pagination_token.is_none() {
            break;
        }
    }

    let mut filtered_len = 0;

    let content_file = users
        .iter()
        .filter(|&u| {
            if cli.show_unconfirmed {
                true
            } else {
                u.user_status != "UNCONFIRMED"
            }
        })
        .filter(|&u| {
            if let Some(ref avoid) = cli.filtered_user_ids {
                let is_in = avoid.contains(&u.username);
                return if cli.include_user_ids { is_in } else { !is_in };
            }
            true
        })
        .filter(|&u| {
            if let Some(ref avoid) = cli.filtered_user_emails {
                let is_in = avoid.contains(&get_email(u));
                return if cli.include_user_emails {
                    is_in
                } else {
                    !is_in
                };
            }
            true
        })
        .fold("id, email, status".to_owned(), |acc, u| {
            let email = get_email(u);
            if cli.print_screen {
                println!(
                    "{} | {} | {}",
                    style(&u.username).green(),
                    &email,
                    style(&u.user_status).yellow()
                );
            }
            filtered_len += 1;
            format!(
                "{}\n{}",
                acc,
                format!("{}, {}, {}", u.username, &email, u.user_status)
            )
        });

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

fn get_email(user: &User) -> String {
    user.attributes
        .first()
        .and_then(|x| Some(x.value.clone()))
        .unwrap_or_else(|| "None".to_owned())
}

fn get_users(
    pool_id: &str,
    region: &str,
    pagination_token: &Option<String>,
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

    if let Some(pt) = pagination_token {
        cmd.arg("--pagination-token").arg(pt);
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
