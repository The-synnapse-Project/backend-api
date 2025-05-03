use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};

pub(super) async fn setup_db(url: String, dbname: String) -> Result<DatabaseConnection, DbErr> {
    Database::connect(url).await
}

pub async fn run_migration(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}

// /// Checks if the database exists inside the rdbms
// fn check_db(db: DatabaseConnection, dbname: String) -> Result<bool, DbErr> {
//     match db.get_database_backend() {
//         sea_orm::DatabaseBackend::MySql => {
// 			db.execute(stmt)
// 		},
//         sea_orm::DatabaseBackend::Postgres => todo!(),
//         sea_orm::DatabaseBackend::Sqlite => todo!(),
//     }
// }
