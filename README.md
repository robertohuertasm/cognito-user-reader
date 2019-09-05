# Cognito User Reader

[![Build Status](https://travis-ci.org/robertohuertasm/cognito-user-reader.svg?branch=master)](https://travis-ci.org/robertohuertasm/cognito-user-reader) [![Crates.io](https://img.shields.io/crates/v/cognito-user-reader.svg)](https://crates.io/crates/cognito-user-reader)

This small utility will fetch all the users and emails from a Cognito User Pool of your choice and print a nice file for you.

## Installation

You can compile it yourself:

```sh
cargo install cognito-user-reader
```

or you can download the executable from [GitHub releases](https://github.com/robertohuertasm/cognito-user-reader/releases) and add it to your path.

## Requirements

You need to have previously uninstalled [awscli](https://aws.amazon.com/cli/).

```python
pip install awscli
```

## Usage

Depending on how you have configured `AWS`, you may need to use something similar to `assume-role` before using `cur` in order to access the data:

```bash
assume-role your-env
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
