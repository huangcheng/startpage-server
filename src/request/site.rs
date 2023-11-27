use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct CreateSite<'r> {
    pub name: &'r str,
    pub url: &'r str,
    pub description: &'r str,
    pub icon: &'r str,
}
