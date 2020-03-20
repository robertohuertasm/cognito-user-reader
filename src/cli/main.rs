#![allow(clippy::non_ascii_literal)]

mod cli;

use cli::Cli;
use cognito_user_reader::{User, UserReader, ROCKET, THUMB, TREE};
use console::style;
use std::env;
use std::io::Write;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    // get cli
    let cli: Cli = Cli::from_args();
    let current_dir = env::current_dir().unwrap();

    println!(
        "
 ██████╗██╗   ██╗██████╗      ██████╗██╗     ██╗
██╔════╝██║   ██║██╔══██╗    ██╔════╝██║     ██║
██║     ██║   ██║██████╔╝    ██║     ██║     ██║
██║     ██║   ██║██╔══██╗    ██║     ██║     ██║
╚██████╗╚██████╔╝██║  ██║    ╚██████╗███████╗██║
 ╚═════╝ ╚═════╝ ╚═╝  ╚═╝     ╚═════╝╚══════╝╚═╝  \n\n{}  Let's get some users!\n",
        ROCKET
    );

    // prepare the reader and get the users
    let reader = UserReader::new(cli.pool_id.to_owned());
    let options = cli.to_options();
    let users = reader.get_users(&options, true)?;

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

    let content = users.iter().fold(String::new(), |acc, u| {
        let creation_date = u.creation_date();
        if cli.print_screen {
            println!(
                "{} | {} | {} | {}",
                style(creation_date).red(),
                style(&u.username).green(),
                style(&u.user_status).yellow(),
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
                u.username,
                u.user_status,
                u.attributes_values_to_string(","),
            )
        )
    });
    (content, filtered_len)
}
