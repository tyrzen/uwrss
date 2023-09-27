# uwrss

## About

This program is designed to fetch job listings from upwork based on a query and send the results to a specified
email address.

## Features

- Fetches job listings from upwork based on a query
- Sends job listings to a specified email address
- Configurable via command-line flags and environment variables
- Graceful shutdown on `Ctrl+C`

## Prerequisites

- Rust and Cargo installed
- An SMTP server for sending emails

## Installation

Clone the repository:

```shell
git clone https://github.com/tyrzen/uwrss.git
cd uwrss
```

## Configuration

You can configure the program using command-line flags or environment variables. If you prefer environment variables,
create a `.env` file in the root directory and populate it with your settings. Here's an example:

```env
SMTP_SERVER="smtp.googlemail.com"
SMTP_USERNAME="your.email@example.com"
SMTP_PASSWORD="yourpassword"
SMTP_PORT=465
RECIPIENT="recipient1.email@example.com, recipient2.email@example.com"
QUERY="title:((/"Project manager/") OR (/"project management/"))"
PAGING=15
INTERVAL=15
```

## Build

Compile the program:

```shell
cargo build --release
```

## Run

Run the program using command-line flags:

```shell
cargo run --release -- --interval 15 --query 'your-query' --smtp-server 'smtp.googlemail.com' --smtp-port 465 --smtp-username 'your.email@example.com' --smtp-password 'your app password' --recipient 'recipient.email@example.com'
```

After building the project, you can run the compiled binary directly from the `target/release` directory:

```shell
./target/release/uwrss --interval 15 --query 'your-query' --smtp-server 'smtp.googlemail.com' --smtp-port 465 --smtp-username 'your.email@example.com' --smtp-password 'yourpassword' --recipient 'recipient.email@example.com'
```