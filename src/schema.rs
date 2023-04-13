use crate::gql::types::{User, Profile};

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn user(&self) -> User {
        User {
            email: "test@dev.com".to_string(),
            profile: Profile {
                first_name: Some("dev".to_string()),
                last_name: Some("tech".to_string())
            }
        }
    }
}
