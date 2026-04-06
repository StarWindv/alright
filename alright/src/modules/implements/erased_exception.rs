use crate::modules::types::erased_exception::ErasedBaseException;
use crate::BaseException;
use alright::traits::{ErasedExceptionUtils, Transform};
use alright::ErasedProperty;

impl<T: Transform> From<BaseException<T>> for ErasedBaseException {
    fn from(value: BaseException<T>) -> Self {
        Self {
            property: value.property.into()
        }
    }
}

impl<T: Transform + Clone> ErasedExceptionUtils<T> for ErasedBaseException {
    fn get_e_property (&self) -> Box<ErasedProperty> {
        self.property.clone().into()
    }

    fn set_e_property (&mut self, property: Box<ErasedProperty>) {
        self.property = property;
    }
}
