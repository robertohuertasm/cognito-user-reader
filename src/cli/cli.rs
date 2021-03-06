use chrono::prelude::*;
use cognito_user_reader::UserReaderOptions;
use console::style;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(
    name("🌇  Cognito User Reader CLI - CUR"),
    author("💻  Roberto Huertas <roberto.huertas@outlook.com>"),
    long_about("🧰  Utility to retrieve all users in a specific AWS user pool.\n🦀  Humbly written with Rust. 🧡 \n🔗  https://github.com/robertohuertasm/cognito-user-reader

 ██████╗██╗   ██╗██████╗      ██████╗██╗     ██╗
██╔════╝██║   ██║██╔══██╗    ██╔════╝██║     ██║
██║     ██║   ██║██████╔╝    ██║     ██║     ██║
██║     ██║   ██║██╔══██╗    ██║     ██║     ██║
╚██████╗╚██████╔╝██║  ██║    ╚██████╗███████╗██║
 ╚═════╝ ╚═════╝ ╚═╝  ╚═╝     ╚═════╝╚══════╝╚═╝  ")
)]
#[allow(clippy::struct_excessive_bools, clippy::empty_line_after_outer_attr)]
pub struct Cli {
    /// Pool id
    #[structopt()]
    pub pool_id: String,
    /// List of attributes you want to display. Email will always be included.
    #[structopt(short = "a", long)]
    pub attributes_to_get: Option<Vec<String>>,
    /// List of user ids that you don't want to display
    #[structopt(short = "i", long)]
    pub filtered_user_ids: Option<Vec<String>>,
    /// Only the filtered user ids will be displayed
    #[structopt(short = "n", long)]
    pub include_user_ids: bool,
    /// List of user emails that you don't want to display
    #[structopt(short = "e", long)]
    pub filtered_user_emails: Option<Vec<String>>,
    /// Only the filtered emails will be displayed
    #[structopt(short = "m", long)]
    pub include_user_emails: bool,
    /// Max number of users returned
    #[structopt(short = "x", long)]
    pub max_number_users: Option<i64>,
    /// Output the result to the terminal
    #[structopt(short, long)]
    pub print_screen: bool,
    /// Show the unconfirmed accounts, too
    #[structopt(short, long)]
    pub show_unconfirmed: bool,
    /// Date to filter the users by creation date (yyyy-mm-dd)
    #[structopt(short, long, parse(from_str = parse_date))]
    pub created_at: Option<DateTime<Utc>>,
}

impl Cli {
    pub const fn to_options(&self) -> UserReaderOptions {
        UserReaderOptions {
            attributes_to_get: &self.attributes_to_get,
            limit_of_users: self.max_number_users,
            show_unconfirmed_users: self.show_unconfirmed,
            filtered_user_ids: &self.filtered_user_ids,
            include_user_ids: self.include_user_ids,
            filtered_user_emails: &self.filtered_user_emails,
            include_user_emails: self.include_user_emails,
            created_at: self.created_at,
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
fn parse_date(src: &str) -> DateTime<Utc> {
    let parsed = src
        .split('-')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();

    if parsed.len() != 3 {
        let error_msg = "Review the creation date paramater. Use YYYY-MM-DD. eg. 2020-02-23";
        println!("{}", style(error_msg).red());
        panic!("{}", error_msg);
    }

    Utc.ymd(parsed[0] as i32, parsed[1], parsed[2])
        .and_hms(23, 59, 59)
}
