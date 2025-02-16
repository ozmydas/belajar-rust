#![allow(dead_code)]

const SOME_MESSAGE: &str = "hello rust";

mod service_layer {

    pub fn some_black_magic() {
        println!("{}", crate::messaging::SOME_MESSAGE)
    }
}

// pub fn say_hello() {
//     service_layer::some_black_magic();
// }

// re export / shortcut
pub use self::service_layer::some_black_magic as do_magic;