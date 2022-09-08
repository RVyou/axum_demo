use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Page {
    pub count: u64,
    pub limit: u64,
    pub page: u64,
}