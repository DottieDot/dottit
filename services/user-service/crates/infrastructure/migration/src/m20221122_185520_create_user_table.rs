use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(User::Id)
              .uuid()
              .not_null()
              .primary_key()
              .extra("DEFAULT (gen_random_uuid())".to_owned())
          )
          .col(
            ColumnDef::new(User::Username)
              .char_len(24)
              .unique_key()
              .not_null()
          )
          .col(ColumnDef::new(User::PasswordHash).char_len(100).not_null())
          .to_owned()
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(User::Table).to_owned())
      .await
  }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
  Table,
  Id,
  Username,
  PasswordHash
}
