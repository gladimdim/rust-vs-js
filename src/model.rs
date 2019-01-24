use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct ModelNode {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Reports {
    pub reports: Vec<ModelNode>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    pub items: Reports,
}
