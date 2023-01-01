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
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Persona::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Persona::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Persona::UserId).date_time().not_null())
                    .col(ColumnDef::new(Persona::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Persona::IconUrl).text().not_null())
                    .col(ColumnDef::new(Persona::Biography).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bot::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Bot::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Bot::UserId).date_time().not_null())
                    .col(ColumnDef::new(Bot::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Bot::IconUrl).text().not_null())
                    .col(ColumnDef::new(Bot::Biography).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Board::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Board::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Board::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Board::Content).text().not_null())
                    .col(ColumnDef::new(Board::RawContent).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Post::BoardId).string().not_null())
                    .col(ColumnDef::new(Post::UserId).string().not_null())
                    .col(ColumnDef::new(Post::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Post::Content).text().not_null())
                    .col(ColumnDef::new(Post::RawContent).text().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Thread::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Thread::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Thread::UserId).string().not_null())
                    .col(ColumnDef::new(Thread::PostId).string().not_null())
                    .col(ColumnDef::new(Thread::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Thread::Content).text().not_null())
                    .col(ColumnDef::new(Thread::RawContent).text().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Reply::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Reply::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Reply::UserId).string().not_null())
                    .col(ColumnDef::new(Reply::ThreadId).string().not_null())
                    .col(ColumnDef::new(Reply::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Reply::Content).text().not_null())
                    .col(ColumnDef::new(Reply::RawContent).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bot::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Persona::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Board::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Thread::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Reply::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Board {
    Table,
    Id,
    Content,
    RawContent,
    CreatedAt,
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    UserId,
    BoardId,
    Content,
    RawContent,
    CreatedAt,
}

#[derive(Iden)]
enum Thread {
    Table,
    Id,
    UserId,
    PostId,
    Content,
    RawContent,
    CreatedAt,
}

#[derive(Iden)]
enum Reply {
    Table,
    UserId,
    ThreadId,
    Id,
    Content,
    RawContent,
    CreatedAt,
}
#[derive(Iden)]
enum User {
    Table,
    Id,
    CreatedAt,
}
#[derive(Iden)]
enum Bot {
    Table,
    Id,
    UserId,
    IconUrl,
    Biography,
    CreatedAt,
}
#[derive(Iden)]
enum Persona {
    Table,
    IconUrl,
    Id,
    UserId,
    Biography,
    CreatedAt,
}
