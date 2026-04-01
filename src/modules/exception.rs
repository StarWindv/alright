use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::traits::{Transform, ExceptionUtils};
use crate::modules::property::Property;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Exception<T: Transform<T>> {
    /// 这里之所以没有再使用 inner 的方案
    /// 是因为那样不可避免的会导致爆栈
    /// 因为全是递归
    /// 所以最后想到了使用结构体指针的方法
    pub property : Box<Property<T>>,
    pub mutex_ptr: T/* 任意一个实现了 Down 特征的结构体指针 */
}

impl<T: Transform<T>> Error for Exception<T> {}

impl<T: Transform<T>> Display for Exception<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match serde_json::ser::to_string_pretty(&self.property) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => {
                let downgrade = format!(
                    "{}\n{}\n{}\n{}",
                    format!("name :  {}", self.property.name),
                    format!("cause:  {}", self.property.clone().cause),
                    format!("context:{}", self.property.context.join("\n - ")),
                    format!("other:  {:#?}", self.property.other),
                );
                write!(f, "{}", downgrade)
            }
        }
    }
}

impl<T: Transform<T> + Clone> ExceptionUtils<T> for Exception<T> {
    fn get_property(&self) -> Box<Property<T>> {
        self.property.clone()
    }

    fn set_property(&mut self, property: Box<Property<T>>) {
        self.property = property;
    }

    fn get_ptr(&self) -> T {
        self.mutex_ptr.clone()
    }

    fn set_ptr(&mut self, ptr: T) {
        self.mutex_ptr = ptr;
    }
}

impl<T: Transform<T>> Transform<T> for Exception<T> {}
