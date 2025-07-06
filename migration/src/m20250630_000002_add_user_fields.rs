use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Password).string().not_null())
                    .add_column(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .add_column(ColumnDef::new(User::EmailConfirmed).boolean().not_null().default(false))
                    .add_column(ColumnDef::new(User::UserDisplayId).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Password)
                    .drop_column(User::Email)
                    .drop_column(User::EmailConfirmed)
                    .drop_column(User::UserDisplayId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Password,
    Email,
    EmailConfirmed,
    UserDisplayId,
} 