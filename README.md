# Cognito User Reader

[![ActionsStatus](https://github.com/robertohuertasm/cognito-user-reader/workflows/Build/badge.svg)](https://github.com/robertohuertasm/cognito-user-reader/actions) [![Crates.io](https://img.shields.io/crates/v/cognito-user-reader.svg)](https://crates.io/crates/cognito-user-reader)

This small utility will fetch all the users and emails from a Cognito User Pool of your choice and print a nice file for you.

## Installation

You can compile it yourself:

```sh
cargo install cognito-user-reader 
```

or you can download the executable from [GitHub releases](https://github.com/robertohuertasm/cognito-user-reader/releases) and add it to your path.

## Usage

Depending on how you have configured `AWS`, you may need to set your credentials in your terminal.

Then, just execute:

```bash
cur <pool_id> [-p] [-s] [-a custom:company] [-i "id1" "id2"] [-e "a@email.com" "b@email.com"] [-n] [-m] [-x 20] [-c 2020-02-10]
```

You will see a new `cognito_users.csv` file in your working directory with all your user's emails.

If you want to learn more about the options of this `cli` just execute `cur -h`.

### Options

* `-a`: Array of attributes that you want to get for your users. Email is always included.
* `-p`: Prints the result to the terminal
* `-s`: Shows also the unconfirmed users
* `-i`: Array of user ids to be filtered
* `-e`: Array of user emails to be filtered
* `-n`: Inverts the userId filter
* `-m`: Inverts the user email filter
* `-x`: Max number of users to retrieve
* `-c`: Only shows users created from this date
