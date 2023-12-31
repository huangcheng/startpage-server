use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCategory<'r> {
    pub name: Option<&'r str>,
    pub description: Option<&'r str>,
    pub icon: Option<&'r str>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory<'r> {
    pub name: &'r str,
    pub description: &'r str,
    pub icon: &'r str,
}

#[derive(Debug, Deserialize)]
pub struct SortCategory {
    pub active: i64,
    pub over: Option<i64>,
}
