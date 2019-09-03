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
    /// If you set this to true, you will only display the filtered user ids
    #[structopt(short = "n", long)]
    pub include_user_ids: bool,
    /// List of user emails that you don't want to display
    #[structopt(short = "e", long)]
    pub filtered_user_emails: Option<Vec<String>>,
    /// If you set this to true, you will only display the filtered user emails
    #[structopt(short = "m", long)]
    pub include_user_emails: bool,
    /// Set this to true to output the result to the terminal
    #[structopt(short, long)]
    pub print_screen: bool,
    /// Set this to true to show the unconfirmed accounts
    #[structopt(short, long)]
    pub show_unconfirmed: bool,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
