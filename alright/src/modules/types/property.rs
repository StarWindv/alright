use serde::Serialize;
use serde_json::{Map, Value};

use crate::modules::types::exception::BaseException;
use crate::modules::traits::transform::Transform;

#[derive(Debug, Serialize, Clone)]
pub struct Property<T: Transform> {
    pub name: String,
    pub context: Vec<String>,
    pub cause: Option<BaseException<T>>,
    pub other: Map<String, Value>, // and something what you need
}
