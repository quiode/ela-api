/// this module interacts with the database
///
/// it provides functions to save to the database and functions to read from it
pub mod database_interactions {
    use rocket_db_pools::{sqlx, Database};

    #[derive(Database)]
    #[database("db")]
    pub struct Db(sqlx::SqlitePool);

    /// module that provides functions to read from the database
    pub mod read {}

    /// module that provides functions to write to the database
    pub mod write {
        use rocket::serde::uuid::Uuid;
        use rocket_db_pools::Connection;

        use super::Db;

        pub async fn save_ping(uuid: Uuid, mut db: Connection<Db>) {}
    }

    /// module for creating the database
    mod setup {
        use rocket_db_pools::Connection;

        use super::Db;

        fn correct_setup(mut db: Connection<Db>) {}
    }
}
