use episko_lib::{config::ConfigHandler, database::DatabaseHandler};

/// !TODO!
pub struct AppState {
    pub db: DatabaseHandler,
    pub config_handler: ConfigHandler,
}
impl AppState {
    /// !TODO
    #[must_use]
    pub fn new(db: DatabaseHandler, config_handler: ConfigHandler) -> Self {
        Self { db, config_handler }
    }
}
