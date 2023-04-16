use sea_orm::Iden;
use sea_orm_migration::prelude::*;
/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Email,
    #[iden = "password_hash"]
    PasswordHash,
    ProfileId,
}

#[derive(Iden)]
pub enum Profile {
    Table,
    Id,
    #[iden = "first_name"]
    FirstName,
    #[iden = "last_name"]
    LastName,
}

