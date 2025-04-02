use episko_lib::{config::ConfigHandler, database::DatabaseHandler};

/// State of the application.
pub struct AppState {
    pub db: DatabaseHandler,
    pub config_handler: ConfigHandler,
}
impl AppState {
    /// Create a new [`AppState`] instance.
    #[must_use]
    pub fn new(db: DatabaseHandler, config_handler: ConfigHandler) -> Self {
        Self { db, config_handler }
    }
}
