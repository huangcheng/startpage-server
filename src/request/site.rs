use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct CreateCategory<'r> {
    pub name: &'r str,
    pub description: &'r str,
}
