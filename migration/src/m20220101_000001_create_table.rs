use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Energizz::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Energizz::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Energizz::Temperatura).float().not_null())
                    .col(ColumnDef::new(Energizz::Umidade).float().not_null())
                    .col(ColumnDef::new(Energizz::Pressao).float().not_null())
                    .col(ColumnDef::new(Energizz::PacienteId).text().not_null())
                    .col(ColumnDef::new(Energizz::PostingTime).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Energizz::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Energizz {
    Table,
    Id,
    Temperatura,
    Umidade,
    Pressao,
    PacienteId,
    PostingTime,
}
