use schema::users;

#[derive(Queryable,Debug,Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub skills: Option<Vec<String>>,
    pub tasks: Option<Vec<String>>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub skills: Vec<&'a str>,
    pub tasks: Vec<&'a str>,
}
