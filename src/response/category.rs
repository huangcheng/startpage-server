use serde::Serialize;

use crate::models::category;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
}

impl From<category::Category> for Category {
    fn from(category: category::Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
            description: category.description,
            icon: category.icon,
        }
    }
}
