use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::site::Site as SiteModel;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub description: String,
    pub icon: String,
    pub visit_count: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SiteWithCategory {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub description: String,
    pub icon: String,
    pub category: String,
    pub visit_count: i64,
}

impl From<SiteModel> for Site {
    fn from(site: SiteModel) -> Self {
        Self {
            id: site.id,
            name: site.name,
            url: site.url,
            description: site.description,
            icon: site.icon,
            visit_count: site.visit_count,
        }
    }
}
