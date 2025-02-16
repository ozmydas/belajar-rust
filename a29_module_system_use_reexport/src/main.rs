use std::env::{current_dir, args};

mod messaging;

fn main() {


    messaging::do_magic();
    
    println!("***************************************************");


    let package_path = current_dir().unwrap();
    println!("package_path: {:?}", package_path);

    for i in 1..=args().len() {
        let each_arg = args().nth(i);
        if each_arg != None {
            println!("arg{}: {:?}", i, each_arg.unwrap());
        }
    }
}