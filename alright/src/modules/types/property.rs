use serde::Serialize;
use serde_json::{Map, Value};

use crate::traits::Transform;
use crate::{
    modules::types::{
        exception::BaseException
    }
};

#[derive(Debug, Serialize, Clone)]
pub struct Property<T: Transform<T>> {
    pub name: String,
    pub context: Vec<String>,
    pub cause: Option<BaseException<T>>,
    pub other: Map<String, Value>
}
