use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Board::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Board::Id)
              .uuid()
              .not_null()
              .primary_key()
              .extra("DEFAULT (gen_random_uuid())".to_owned())
          )
          .col(ColumnDef::new(Board::Name).string().unique_key().not_null())
          .to_owned()
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("board_name_index")
          .table(Board::Table)
          .col(Board::Name)
          .take()
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(BoardModerator::Table)
          .if_not_exists()
          .col(ColumnDef::new(BoardModerator::BoardId).uuid().not_null())
          .col(ColumnDef::new(BoardModerator::UserId).uuid().not_null())
          .primary_key(
            Index::create()
              .col(BoardModerator::BoardId)
              .col(BoardModerator::UserId)
          )
          .take()
      )
      .await?;

    manager
      .create_foreign_key(
        ForeignKey::create()
          .name("board_moderator_board_id_fk")
          .from(BoardModerator::Table, BoardModerator::BoardId)
          .to(Board::Table, Board::Id)
          .on_delete(ForeignKeyAction::Cascade)
          .take()
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(BoardModerator::Table).take())
      .await?;

    manager
      .drop_table(Table::drop().table(Board::Table).take())
      .await
  }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Board {
  Table,
  Id,
  Name
}

#[derive(Iden)]
enum BoardModerator {
  Table,
  BoardId,
  UserId
}
