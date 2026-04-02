use serde::Serialize;

use crate::{
    modules::types::property::Property,
    traits::Transform
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct BaseException<T: Transform<T>> {
    /// 这里之所以没有再使用 inner 的方案
    /// 是因为那样不可避免的会导致爆栈
    /// 因为全是递归
    /// 所以最后想到了使用结构体指针的方法
    pub property : Box<Property<T>>,
    pub target_ptr: T
}
