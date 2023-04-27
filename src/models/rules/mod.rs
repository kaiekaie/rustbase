use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Rules {
    ListRule,
    CreateRule,
    VieweRule,
    UpdateRule,
    DeleteRule,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListRule(pub String);
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateRule(pub String);
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViewRule(pub String);
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateRule(pub String);
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeleteRule(pub String);

trait SharedFunctionality {
    fn check_rule(&self, string: &String) -> bool;
}

impl SharedFunctionality for Rules {
    fn check_rule(&self, string: &String) -> bool {
        self::CreateRule(format!("ads"));
        true
    }
}
