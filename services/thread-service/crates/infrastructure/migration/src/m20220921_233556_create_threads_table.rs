use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Thread::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Thread::Id)
              .uuid()
              .primary_key()
              .not_null()
              .extra("DEFAULT (gen_random_uuid())".to_owned())
          )
          .col(ColumnDef::new(Thread::BoardId).uuid().not_null())
          .col(ColumnDef::new(Thread::UserId).uuid().not_null())
          .col(ColumnDef::new(Thread::Title).string().not_null())
          .col(ColumnDef::new(Thread::Text).string().not_null())
          .col(
            ColumnDef::new(Thread::CreatedAt)
              .timestamp_with_time_zone()
              .not_null()
              .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
          )
          .take()
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("thread_user_index")
          .table(Thread::Table)
          .col(Thread::UserId)
          .take()
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("thread_board_index")
          .table(Thread::Table)
          .col(Thread::BoardId)
          .take()
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("thread_created_at_index")
          .table(Thread::Table)
          .col(Thread::CreatedAt)
          .take()
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Thread::Table).take())
      .await
  }
}

#[derive(Iden)]
enum Thread {
  Table,
  Id,
  BoardId,
  UserId,
  Title,
  Text,
  CreatedAt
}
