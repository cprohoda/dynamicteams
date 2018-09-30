use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn connect() -> PgConnection {
	dotenv.ok();

	let database_url = env::var("DATABASE_URL")?;
	PgConnection::establish(&database_url)?
}