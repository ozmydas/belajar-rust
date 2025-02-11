use std::io;

pub fn read_entry() -> String {
    let mut message = String::new();

    println!("Enter something!");

    let reader_res = io::stdin().read_line(&mut message);

    if reader_res.is_err() {
        println!("Error : {:?}", reader_res.err())
    }

    return message.trim().to_string()
}
