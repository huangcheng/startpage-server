use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct CreateSite<'r> {
    pub name: &'r str,
    pub url: &'r str,
    pub description: &'r str,
    pub icon: &'r str,
    pub category: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSite<'r> {
    pub name: Option<&'r str>,
    pub url: Option<&'r str>,
    pub description: Option<&'r str>,
    pub icon: Option<&'r str>,
    pub category: Option<i64>,
}
