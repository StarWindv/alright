use crate::modules::types::erased_exception::ErasedBaseException;
use crate::BaseException;
use alright::traits::Transform;

impl<T: Transform> From<BaseException<T>> for ErasedBaseException {
    fn from(value: BaseException<T>) -> Self {
        Self {
            property: value.property.into()
        }
    }
}
