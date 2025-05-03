use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(pk_auto(Person::Id))
                    .col(string_uniq(Person::Email).unique_key())
                    .col(string(Person::Name))
                    .col(timestamp(Person::BirthDate))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Register::Table)
                    .if_not_exists()
                    .col(pk_auto(Register::Id))
                    .col(
                        ColumnDef::new(Register::PersonId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Register::Time).timestamp().not_null())
                    .col(
                        ColumnDef::new(Register::Action)
                            .custom("action_enum")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_register_person")
                            .from(Register::Table, Register::PersonId)
                            .to(Person::Table, Person::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permissions::PersonId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Permissions::ControlPanel)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Permissions::SeeOwnHistory)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Permissions::SeeOthersHistory)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Permissions::AdminPanel)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Permissions::EditPermissions)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_permissions_person")
                            .from(Permissions::Table, Permissions::PersonId)
                            .to(Person::Table, Person::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Register::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await?;

        // Drop the custom enum type
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE IF EXISTS action_enum")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Person {
    Id,
    Name,
    BirthDate,
    Email,
}

#[derive(DeriveIden)]
enum Register {
    Id,
    PersonId,
    Time,
    Action,
}

#[derive(DeriveIden)]
enum Permissions {
    PersonId,
    ControlPanel,
    SeeOwnHistory,
    SeeOthersHistory,
    AdminPanel,
    EditPermissions,
}
