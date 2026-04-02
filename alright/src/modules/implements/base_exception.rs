use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::{
    modules::types::{
        property::Property,
        exception::BaseException
    },
    traits::{ExceptionUtils, Transform}
};

impl<T: Transform<T>> Error for BaseException<T> {}

impl<T: Transform<T>> Display for BaseException<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::ser::to_string_pretty(&self.property) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => {
                let downgrade = format!(
                    "{}\n{}\n{}\n{}",
                    format!("name :  {}", self.property.name),
                    format!(
                        "cause:  {:?}",
                        match &self.property.cause {
                            Some(cause) => write!(f, "cause: {}", cause),
                            None => write!(f, "cause: none"),
                        }
                    ),
                    format!("context:{}", self.property.context.join("\n - ")),
                    format!("other:  {:#?}", self.property.other),
                );
                write!(f, "{}", downgrade)
            }
        }
    }
}

impl<T: Transform<T> + Clone> ExceptionUtils<T> for BaseException<T> {
    fn get_property(&self) -> Box<Property<T>> {
        self.property.clone()
    }

    fn set_property(&mut self, property: Box<Property<T>>) {
        self.property = property;
    }

    fn get_ptr(&self) -> T {
        self.target_ptr.clone()
    }

    fn set_ptr(&mut self, ptr: T) {
        self.target_ptr = ptr;
    }
}

impl<T: Transform<T>> Transform<T> for BaseException<T> {}

impl<T: Transform<T>> From<T> for BaseException<T> {
    fn from(value: T) -> Self {
        T::down(value)
    }
}

impl<T: Transform<T>> BaseException<T> {

}
