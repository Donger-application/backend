use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Role::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Role::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::IsActive).boolean().not_null())
                    .col(ColumnDef::new(User::RoleId).integer().not_null())
                    .col(ColumnDef::new(User::CreatedDate).date().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    IsActive,
    RoleId,
    CreatedDate,
}

#[derive(Iden)]
enum Role {
    Table,
    Id,
    Name,
}
