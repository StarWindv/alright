use crate::commonly::KeyError;
use alright::modules::utils::to_string_vec;
use alright::Property;
use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Clone)]
pub struct ErasedProperty {
    pub name: String,
    pub context: Vec<String>,
    pub cause: Option<Box<Self>>,
    pub other: Map<String, Value>,
}

impl ErasedProperty {
    pub fn record(&mut self, msg_list: &mut Vec<impl Into<String>>) {
        self.context.append(&mut to_string_vec(msg_list));
    }

    pub fn add(&mut self, msg: impl Into<String>) {
        self.context.push(msg.into());
    }

    pub fn context(&mut self, msg: impl Into<String>) {
        self.add(msg.into());
    }

    pub fn update(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), KeyError> {
        let string_key: String = key.into();
        if self.other.contains_key(&string_key.clone()) {
            let mut kerr = KeyError {
                property: Box::new(
                    Property::default()
                ) 
            };
            kerr.property.add(
                format!("Duplicate Key: {string_key}"
                )
            );
            return Err(
                kerr
            )
        }
        self.other
            .insert(string_key.into(), Value::String(value.into()));
        Ok(())
    }
}
