use std::any::type_name;
use serde_json::Map;
use alright::modules::utils::get_name;
use crate::modules::types::erased_property::ErasedProperty;
use crate::Property;
use alright::traits::Transform;

impl Default for ErasedProperty {
    fn default() -> Self {
        Self {
            name: get_name(type_name::<Self>()),
            context: Vec::new(),
            cause: None,
            other: Map::new(),
        }
    }
}

fn into<T: Transform>(cause_link: Box<Property<T>>) -> ErasedProperty {
    ErasedProperty {
        name: cause_link.name,
        context: cause_link.context,
        cause: cause_link.cause.map(|c| Box::new(into(c.property))),
        other: cause_link.other,
    }
}

impl<T: Transform> Into<ErasedProperty> for Property<T> {
    fn into(self) -> ErasedProperty {
        into(Box::new(self))
    }
}

impl<T: Transform> Into<Box<ErasedProperty>> for Box<Property<T>> {
    fn into(self) -> Box<ErasedProperty> {
        Box::new(into(self))
    }
}
