use crate::traits::Transform;
use crate::{ErasedProperty, Property};

pub trait ExceptionUtils<T: Transform> {
    fn get_property (&self) -> Box<Property<T>>;
    fn set_property (&mut self, property: Box<Property<T>>);
    fn get_ptr (&self) -> T;
    fn set_ptr (&mut self, ptr: T);
}

pub trait ErasedExceptionUtils<T: Transform> {
    fn get_e_property (&self) -> Box<ErasedProperty>;
    fn set_e_property (&mut self, property: Box<ErasedProperty>);
}
