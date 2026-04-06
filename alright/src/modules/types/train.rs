use crate::traits::PromiseErr;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Train {
    pub(crate) registry: HashMap<String, Box<dyn PromiseErr>>
}

