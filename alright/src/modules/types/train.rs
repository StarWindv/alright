use alright::traits::{PromiseErr, Transform};
use std::any::type_name_of_val;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Train {
    registry: HashMap<String, Box<dyn PromiseErr>>
}

impl Train {
    pub fn push(&mut self, e: impl PromiseErr + 'static + Transform) {
        let name = type_name_of_val(&e)
            .split("::")
            .last()
            .unwrap()
            .to_string()
            .replace(">", "");
        self.registry.insert(
            name, Box::new(e)
        );
    }
    //
    // pub fn unpack(&mut self, prop: Property<impl Transform>) -> impl PromiseErr {
    //     let name = prop.name;
    //     let inner = (self.registry.get(&name).unwrap().deref());
    // }
}
