use schema::users;
use schema::orgs;

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

#[derive(Queryable,Debug,Serialize)]
pub struct Org {
    pub id: i32,
    pub code: String,
    pub name: Option<String>,
    pub employees: Option<Vec<i32>>,
}

#[derive(Insertable)]
#[table_name="orgs"]
pub struct NewOrg<'b> {
    pub code: &'b str,
    pub name: &'b str,
    pub employees: Vec<i32>,
}
