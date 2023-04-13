#[derive(async_graphql::SimpleObject)]
pub struct Profile {
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

#[derive(async_graphql::SimpleObject)]
pub struct User {
    pub email: String,
    pub profile: Profile,
}
