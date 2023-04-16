use sea_orm_migration::prelude::*;
use crate::table_enums::{Profile, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

const PROFILE_USER_FK_NAME: &str = "FK_user_profile";
const USER_EMAIL_INDEX_NAME: &str = "idx-user_email";
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Profile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Profile::FirstName).string())
                    .col(ColumnDef::new(Profile::LastName).string())
                    .to_owned(),
            )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::PasswordHash).string())
                    .col(ColumnDef::new(User::ProfileId).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(Index::create()
                .if_not_exists()
                .name(USER_EMAIL_INDEX_NAME)
                .table(User::Table)
                .col(User::Email)
                .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(PROFILE_USER_FK_NAME)
                    .from(User::Table, User::ProfileId)
                    .to(Profile::Table, Profile::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(PROFILE_USER_FK_NAME)
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
