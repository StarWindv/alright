use crate::{
    traits::{
        PromiseErr,
        Transform
    },
    Train
};
use std::any::type_name_of_val;
use std::collections::HashMap;


impl Default for Train {
    fn default() -> Self {
        Self {
            registry: HashMap::new()
        }
    }
}

#[allow(dead_code)]
impl Train {
    pub fn push<T: PromiseErr + Transform + Default + Clone >(&mut self, e: T) {
        let name = type_name_of_val(&e)
            .split("::")
            .last()
            .unwrap()
            .to_string()
            .replace(">", "");
        self.registry.insert(
            name, Box::new(T::default())
        );
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn PromiseErr>> {
        self.registry.get(name)
    }

    pub fn show(&self) -> String {
        format!("{:#?}", self.registry)
    }
}

//
// pub fn unpack(&mut self, prop: Property<impl Transform>) -> impl PromiseErr {
//     let name = prop.name;
//     let inner = (self.registry.get(&name).unwrap().deref());
// }
