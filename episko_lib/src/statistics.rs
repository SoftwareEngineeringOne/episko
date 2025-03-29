//! # Statistics about all projects

use std::collections::HashMap;

pub mod statistic_handler;

/// Statistic structure containing different metrics about all projects.
#[derive(Debug)]
pub struct Statistic {
    pub projects_by_language: HashMap<String, u32>,
    pub projects_by_ide: HashMap<String, u32>,
    pub projects_by_category: HashMap<String, u32>,
    pub projects_by_build_system: HashMap<String, u32>,
    pub number_of_projects: u32,
}
