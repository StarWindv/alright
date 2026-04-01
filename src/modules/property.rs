use serde::Serialize;
use serde_json::{Map, Value};
use crate::exception::Exception;
use crate::traits::Transform;

// property.rs
#[derive(Debug, Serialize, Clone, Default)]
pub struct Property<T: Transform<T>> {   // 加上约束
    pub name: String,
    pub context: Vec<String>,
    pub cause: Exception<T>,
    pub other: Map<String, Value>
}
