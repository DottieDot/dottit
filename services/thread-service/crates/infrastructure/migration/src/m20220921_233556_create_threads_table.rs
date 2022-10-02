use sea_orm_migration::{
  prelude::*,
  sea_orm::{ConnectionTrait, Statement}
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = r#"
      CREATE TABLE thread
      (
          id     UUID         DEFAULT (gen_random_uuid()) NOT NULL,
          board  VARCHAR(255)                             NOT NULL,
          "user" VARCHAR(255)                             NOT NULL,
          title  VARCHAR(255)                             NOT NULL,
          text   TEXT                                     NULL,
          media  TEXT                                     NULL,
          score  int          DEFAULT 0                   NOT NULL,
          CONSTRAINT threads_pk
              PRIMARY KEY (id)
      );"#;
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = "DROP TABLE `thread`";
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }
}
