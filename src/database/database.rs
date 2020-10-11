use crate::model::project;

#[derive(Debug, Clone, PartialEq)]
pub struct Database {
    pub project: project::Project,
}
