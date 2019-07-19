# dynamicteams

Installation steps:
1) Install rust & cargo from rust-lang.org
2) Run each of the following in terminal:
    cargo install diesel_cli --no-default-features --features postgres
    echo DATABASE_URL=postgres://postgres:testpw@localhost/dynamicteams > .env
    diesel setup
    diesel migration run
3) Setup should be complete. Run in terminal 'cargo test' to confirm. If all tests suceed, database setup was successful.
