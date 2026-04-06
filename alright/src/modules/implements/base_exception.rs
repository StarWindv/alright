use crate::modules::{
    traits::{
        exception_utils::ExceptionUtils,
        template_display::TemplateDisplay,
        transform::Transform,
    },
    types::{
        exception::BaseException,
        property::Property,
    }
};
use std::error::Error;
use std::fmt::{
    Display,
    Formatter
};

impl<T: Transform> Error for BaseException<T> {}
impl<T: Transform> Transform for BaseException<T> {}
impl<T: Transform> TemplateDisplay<T> for BaseException<T> {}

impl<T: Transform> Display for BaseException<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.template_fmt(f)
    }
}

impl<T: Transform + Clone> ExceptionUtils<T> for BaseException<T> {
    fn get_property (&self) -> Box<Property<T>> {
        self.property.clone()
    }

    fn set_property (&mut self, property: Box<Property<T>>) {
        self.property = property;
    }

    fn get_ptr (&self) -> T {
        self.target_ptr.clone()
    }

    fn set_ptr (&mut self, ptr: T) {
        self.target_ptr = ptr;
    }
}


impl<T: Transform + ExceptionUtils<T>> From<T> for BaseException<T> {
    fn from(value: T) -> Self {
        T::down(&value)
    }
}

impl<T: Transform> BaseException<T> {
    // TODO: Builder
}
