#[derive(Queryable)]
struct User {
	id: i32,
	email: String,
	skills: Vec<String>,
	tasks: Vec<String>,
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
	email: &'a str,
	skills: Vec::new(),
	tasks: Vec::new(),
}
