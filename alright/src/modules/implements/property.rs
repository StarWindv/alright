use serde_json::{Map, Value};
use crate::traits::Transform;
use crate::{
    modules::types::{
        exception::BaseException,
        property::Property,
    }
};


impl<T: Transform<T>> Default for Property<T> {
    fn default() -> Self {
        Self {
            name: String::new(),
            context: Vec::new(),
            cause: None,
            other: Map::new(),
        }
    }
}

fn to_string_vec(
    msg_list: &mut Vec<impl Into<String>>
) -> Vec<String>{
    msg_list
        .drain(..)
        .map(|item| item.into())
        .collect()
}

impl<T: Transform<T> + Default> Property<T> {
    pub fn record(
        &mut self,
        msg_list: &mut Vec<impl Into<String>>
    ) {
        self.context.append(&mut to_string_vec(msg_list));
    }

    pub fn add(
        &mut self,
        msg: impl Into<String>
    ) {
        self.context.push(msg.into());
    }

    pub fn update(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>
    ) -> Result<(), BaseException<T>> {
        let string_key: String = key.into();
        if self.other.contains_key(&string_key.clone()) {
            return Err(
                BaseException {
                    property: Box::new(Property {
                        name: "".to_string(),
                        context: vec![],
                        cause: None,
                        other: Default::default(),
                    }), target_ptr: T::default()
                }
            )
        }
        self.other.insert(string_key.into(), Value::String(value.into()));
        Ok(())
    }
}
