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
    pub mod write {}
}
