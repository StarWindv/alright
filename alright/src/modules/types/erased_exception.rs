use crate::modules::types::erased_property::ErasedProperty;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct ErasedBaseException {
    pub property: Box<ErasedProperty>,
}


