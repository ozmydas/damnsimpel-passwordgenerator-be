use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloParam{
    pub name: Option<String>,
}