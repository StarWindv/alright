use alright::{
    commonly::{
        JustAException,
        ValueError,
    },
    traits::AlrightBox
};
use alright_little_try::{
    define_your_error::MyError,
    quick_define_error::AnotherMyError
};


fn test1() -> AlrightBox {
    let je = JustAException {
        property: Box::new(Default::default()),
    };
    let ve = ValueError {
        property: Box::new(Default::default()),
    };
    let me = MyError {
        property: Box::new(Default::default()),
    };
    let ame = AnotherMyError {
        property: Box::new(Default::default()),
    };
    let _cloned = ame.clone();
    
    let num = rand::random_range(1..=4);
    println!("Case: {num}");
    if num == 1 {
        Box::new(je)
    } else if num == 2 {
        Box::new(ve)
    } else if num == 3 {
        Box::new(me)
    } else {
        Box::new(ame)
    }
}

fn main() {
    let some_e: Result<AlrightBox, AlrightBox> = Err::<AlrightBox, _>(test1());
    let inner = some_e.unwrap_err();
    println!("NAME: {:?}", inner);
}
