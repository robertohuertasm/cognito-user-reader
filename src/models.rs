use chrono::prelude::*;
use console::style;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(author, about)]
pub struct Cli {
    /// Pool id
    #[structopt()]
    pub pool_id: String,
    /// AWS region
    #[structopt(short, long, default_value = "eu-west-1")]
    pub region: String,
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
    pub max_number_users: Option<u32>,
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

#[allow(clippy::cast_possible_wrap)]
fn parse_date(src: &str) -> DateTime<Utc> {
    let parsed = src
        .split('-')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>();

    if parsed.len() != 3 {
        let error_msg = "Review the creation date paramater. Use YYYY-MM-DD. eg. 2020-02-23";
        println!("{}", style(error_msg).red());
        panic!(error_msg);
    }

    Utc.ymd(parsed[0] as i32, parsed[1], parsed[2])
        .and_hms(23, 59, 59)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    pub users: Vec<User>,
    pub pagination_token: Option<String>,
}

#[derive(Debug, Deserialize)]
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
    pub fn creation_date(&self) -> DateTime<Utc> {
        Utc.timestamp(self.user_create_date as i64, 0)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
