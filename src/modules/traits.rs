use crate::modules::exception::Exception;
use crate::modules::property::Property;
use serde::Serialize;
use std::error::Error;

pub trait ExceptionUtils<T: Transform<T>> {
    fn get_property(&self) -> Box<Property<T>>;
    fn set_property(&mut self, property: Box<Property<T>>);
    fn get_ptr(&self) -> T;
    fn set_ptr(&mut self, ptr: T);
}

pub trait Transform<T>:
ExceptionUtils<T> + 'static
+ Error
+ Serialize
+ Clone

where T: Transform<T>
{
    // 我认为 down 代表的是 `到基层去`
    // Exception 在这里就是底层, 所以应该是 down
    // 而暂未实现的 up
    // 它代表的是向上, 也就是转型到高层抽象去
    fn down(this: T) -> Exception<T> {
        Exception {
            // 这里必须是 this 或者不是 self 的名字,
            // 不然编译过不去(rust 不认识 Self 和 T 的关系),
            // 所以手动传递一下对象也是代价的一部分
            property : this.get_property(),
            mutex_ptr: this.clone(),
        }
    }
}
