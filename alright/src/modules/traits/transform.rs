use std::fmt::Debug;
use serde::Serialize;
use crate::traits::ExceptionUtils;
use crate::BaseException;
use crate::Property;

pub trait Transform: Debug + 'static + Clone + Serialize {
    fn down(&self) -> BaseException<Self>
        where
            Self: Transform + ExceptionUtils<Self> + Clone,
    {
        let mut inner = self.clone();
        inner.set_property(Box::new(Property::<Self> {
            name: "".to_string(),
            ..Default::default()
        }));
        BaseException {
            property: self.get_property(),
            target_ptr: inner,
        }
    }

    fn up(this: BaseException<Self>) -> Self
        where
            Self: ExceptionUtils<Self> + 'static + std::error::Error + Serialize + Clone,
    {
        let mut result: Self = this.get_ptr();
        result.set_property(this.property.clone());
        result
    }
}
