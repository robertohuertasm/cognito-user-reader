# Cognito User Reader

[![Actions Status](https://github.com/robertohuertasm/cognito-user-reader/workflows/Rust/badge.svg)](https://github.com/robertohuertasm/cognito-user-reader/actions)

This small utility will fetch all the users and emails from a Cognito User Pool of your choice and print a nice file for you.

## Requirements

You need to have previously uninstalled [awscli](https://aws.amazon.com/cli/).

```python
pip install awscli
```

## Usage

First of all, you have to use `assure-role` in order to access `aws`:

```bash
assure-role prod
#or
assure-role stg
```

Then, just execute: 

```bash
cur <pool_id> [-r eu-west-1] [-p] [-s] [-i "id1" "id2"] [-e "a@email.com" "b@email.com"] [-n] [-m]
```

You will see a new `cognito_users.csv` file in your working directory with all your user's emails.

If you want to learn more about the options of this `cli` just execute `cur -h`.

### Options

* `-r`: AWS region
* `-p`: Prints the result to the terminal
* `-s`: Shows also the unconfirmed users
* `-i`: Array of user ids to be filtered
* `-e`: Array of user emails to be filtered
* `-n`: Inverts the userId filter
* `-m`: Inverts the user email filter
