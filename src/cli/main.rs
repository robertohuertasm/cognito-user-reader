#![allow(clippy::non_ascii_literal)]

mod cli;

use cli::Cli;
use cognito_user_reader::{users_to_csv, UserReader, ROCKET, THUMB, TREE};
use console::style;
use std::env;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let reader = UserReader::new(cli.pool_id.clone());
    let options = cli.to_options();
    let users = reader.get_users(options, true).await;

    // get the list of users in form of a string and the filtered users count
    let (content_file, filtered_len) = users_to_csv(&users, cli.print_screen);

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
    let mut file = File::create(path.clone()).await?;
    file.write_all(content_file.as_bytes()).await?;
    println!(
        "{} {}",
        TREE,
        style(format!("You can find your file in {:?}", path)).magenta()
    );

    Ok(())
}
