use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCategory<'r> {
    pub name: Option<&'r str>,
    pub description: Option<&'r str>,
    pub icon: Option<&'r str>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory<'r> {
    pub name: &'r str,
    pub description: &'r str,
    pub icon: &'r str,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SortCategory {
    pub active: i64,
    pub over: Option<i64>,
    pub parent_id: Option<i64>,
}
