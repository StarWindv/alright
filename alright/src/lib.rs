pub mod modules;
pub use modules::*;

#[macro_export]
macro_rules! exceptions {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(::alright_derive::Exception, Debug, Clone, ::serde::Serialize)]
            pub struct $name {
                pub property: Box<Property<$name>>,
            }

            impl ::std::default::Default for $name {
                fn default() -> Self {
                    Self {
                        property: ::std::boxed::Box::new(
                            ::alright::modules::types::property::Property
                            {
                                name: stringify!($name).to_string(),
                                ..Default::default()
                            }
                        ),
                    }
                }
            }
        )*
    };
}
