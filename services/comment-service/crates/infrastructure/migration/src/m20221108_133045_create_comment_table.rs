use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Comment::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Comment::Id)
              .uuid()
              .not_null()
              .primary_key()
              .extra("DEFAULT (gen_random_uuid())".to_owned())
          )
          .col(ColumnDef::new(Comment::ThreadId).uuid().not_null())
          .col(ColumnDef::new(Comment::User).string().not_null())
          .col(ColumnDef::new(Comment::Text).string().not_null())
          .col(
            ColumnDef::new(Comment::Score)
              .integer()
              .not_null()
              .default(0)
          )
          .to_owned()
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("comment_user_index")
          .table(Comment::Table)
          .col(Comment::User)
          .to_owned()
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("comment_thread_index")
          .table(Comment::Table)
          .col(Comment::ThreadId)
          .to_owned()
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Comment::Table).to_owned())
      .await
  }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Comment {
  Table,
  Id,
  ThreadId,
  User,
  Text,
  Score
}
