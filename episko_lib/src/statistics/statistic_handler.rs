//! # Statistic handling

use crate::database::DatabaseHandler;

use super::{Error, Statistic};

pub struct StatisticHandler;

impl StatisticHandler {
    /// Generate the newest statistics
    pub async fn generate_statistics(db: &DatabaseHandler) -> Result<Statistic, Error> {
        Ok(Statistic {
            projects_by_language: Statistic::projects_by_language(db).await?,
            projects_by_ide: Statistic::projects_by_ide(db).await?,
            projects_by_category: Statistic::projects_by_category(db).await?,
            projects_by_build_system: Statistic::projects_by_build_system(db).await?,
            number_of_projects: Statistic::number_of_projects(db).await?,
            number_of_languages: Statistic::number_of_languages(db).await?,
        })
    }
}
