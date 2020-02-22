mod cli;

use cli::Cli;
use cognito_user_reader_lib::{User, UserReader};
use console::style;
use console::Emoji;
use std::env;
use std::io::Write;
use structopt::StructOpt;

static THUMB: Emoji<'_, '_> = Emoji("\u{1F44D}", "");
static TREE: Emoji<'_, '_> = Emoji("\u{1F335}", "");

fn main() -> std::io::Result<()> {
    // get cli
    let cli: Cli = Cli::from_args();
    let current_dir = env::current_dir().unwrap();

    // prepare the reader and get the users
    let reader = UserReader::new(cli.pool_id.to_owned(), cli.region.to_owned());
    let options = cli.to_options();
    let users = reader.get_users(&options)?;

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

fn get_content(users: &[User], cli: &Cli) -> (String, i32) {
    let mut filtered_len = 0;
    let content = users
        .iter()
        .fold("createdAt, id, email, status".to_owned(), |acc, u| {
            let email = u.get_email();
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
