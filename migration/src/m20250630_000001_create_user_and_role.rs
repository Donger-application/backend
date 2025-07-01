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
                    .col(ColumnDef::new(User::Balance).integer().not_null())
                    .col(ColumnDef::new(User::IsActive).boolean().not_null())
                    .col(ColumnDef::new(User::RoleId).integer().not_null())
                    .col(ColumnDef::new(User::GroupId).integer().not_null())
                    .col(ColumnDef::new(User::CreatedDate).date().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(User::Table, User::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Group::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .col(ColumnDef::new(Group::IsPublic).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Product::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::GroupId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Product::Table, Product::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Stock::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Stock::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Stock::Price).big_integer().not_null())
                    .col(ColumnDef::new(Stock::Consumed).boolean().not_null())
                    .col(ColumnDef::new(Stock::ProductId).integer().not_null().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Stock::Table, Stock::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Meal::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Meal::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Meal::ProductId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Meal::Table, Meal::ProductId)
                            .to(Product::Table, Product::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Invoice::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Invoice::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Invoice::Price).big_integer().not_null())
                    .col(ColumnDef::new(Invoice::IsDeleted).boolean().not_null())
                    .col(ColumnDef::new(Invoice::DeletedBy).integer().not_null())
                    .col(ColumnDef::new(Invoice::CreatedDate).date_time().not_null())
                    .col(ColumnDef::new(Invoice::LastModificationDate).date_time().not_null())
                    .col(ColumnDef::new(Invoice::MealId).integer().not_null().unique_key())
                    .col(ColumnDef::new(Invoice::GroupId).integer().not_null().unique_key())
                    .col(ColumnDef::new(Invoice::SupplierId).integer().not_null().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoice::Table, Invoice::MealId)
                            .to(Meal::Table, Meal::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoice::Table, Invoice::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Invoice::Table, Invoice::SupplierId)
                            .to(Supplier::Table, Supplier::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(InvoiceDetails::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(InvoiceDetails::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(InvoiceDetails::InvoiceId).integer().not_null())
                    .col(ColumnDef::new(InvoiceDetails::StockId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(InvoiceDetails::Table, InvoiceDetails::InvoiceId)
                            .to(Invoice::Table, Invoice::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(InvoiceDetails::Table, InvoiceDetails::StockId)
                            .to(Stock::Table, Stock::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Order::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(Order::IsDeleted).boolean().not_null())
                    .col(ColumnDef::new(Order::DeletedBy).integer().not_null())
                    .col(ColumnDef::new(Order::CreatedDate).date_time().not_null())
                    .col(ColumnDef::new(Order::LastModificationDate).date_time().not_null())
                    .col(ColumnDef::new(Order::GroupId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Order::Table, Order::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderCustomer::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OrderCustomer::OrderId).integer().not_null())
                    .col(ColumnDef::new(OrderCustomer::CustomerId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(OrderCustomer::OrderId)
                            .col(OrderCustomer::CustomerId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderCustomer::Table, OrderCustomer::OrderId)
                            .to(Order::Table, Order::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderCustomer::Table, OrderCustomer::CustomerId)
                            .to(Customer::Table, Customer::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrderDetails::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OrderDetails::Id).integer().not_null().primary_key())
                    .col(ColumnDef::new(OrderDetails::OrderId).integer().not_null())
                    .col(ColumnDef::new(OrderDetails::StockId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderDetails::Table, OrderDetails::OrderId)
                            .to(Order::Table, Order::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderDetails::Table, OrderDetails::StockId)
                            .to(Stock::Table, Stock::Id),
                    )
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
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Stock::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Meal::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Invoice::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(InvoiceDetails::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderCustomer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderDetails::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    Balance,
    IsActive,
    RoleId,
    GroupId,
    CreatedDate,
}

#[derive(Iden)]
enum Role {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Group {
    Table,
    Id,
    Name,
    IsPublic,
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
    Name,
    GroupId,
}

#[derive(Iden)]
enum Stock {
    Table,
    Id,
    Price,
    Consumed,
    ProductId,
}

#[derive(Iden)]
enum Meal {
    Table,
    Id,
    ProductId,
}

#[derive(Iden)]
enum Supplier {
    Table,
    Id,
    Balance,
    UserId,
}

#[derive(Iden)]
enum Invoice {
    Table,
    Id,
    Price,
    IsDeleted,
    DeletedBy,
    CreatedDate,
    LastModificationDate,
    MealId,
    GroupId,
    SupplierId,
}

#[derive(Iden)]
enum InvoiceDetails {
    Table,
    Id,
    InvoiceId,
    StockId,
}

#[derive(Iden)]
enum Order {
    Table,
    Id,
    IsDeleted,
    DeletedBy,
    CreatedDate,
    LastModificationDate,
    GroupId,
}

#[derive(Iden)]
enum OrderCustomer {
    Table,
    OrderId,
    CustomerId,
}

#[derive(Iden)]
enum Customer {
    Table,
    Id,
    // Add other fields as needed
}

#[derive(Iden)]
enum OrderDetails {
    Table,
    Id,
    OrderId,
    StockId,
}
