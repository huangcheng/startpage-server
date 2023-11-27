use serde::Serialize;

use crate::models::site;

#[derive(Debug, Serialize)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub description: String,
    pub icon: String,
}

impl From<site::Site> for Site {
    fn from(site: site::Site) -> Self {
        Self {
            id: site.id,
            name: site.name,
            url: site.url,
            description: site.description,
            icon: site.icon,
        }
    }
}
