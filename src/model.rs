use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct ModelNode {
    pub long_description: String,
    pub Material: String,
    pub Demo: String,
    pub Customer: String,
    pub CustomerOrder: String,
    pub Field: String,
    pub Something: String,
    pub Nexthing: String,
    pub materials: String,
    pub versions: String,
    pub name: String,
    pub Android: String,
    pub iOS: String,
    pub reference: String,
    pub Order: String,
    pub Routing: String,
    pub Descriptoin: String,
    pub Country: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    pub items: Vec<ModelNode>,
}
