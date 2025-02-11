mod my_io;
mod my_number;

fn main() {
    println!("Hello, world!");

    let message = my_io::read_entry();
    
    println!("Enterd message : {}", message);
    
    let number = my_number::conversion_utility::string_to_number(message);
    let ganjil = my_number::is_odd_number(number);

    println!("is odd number: {}", ganjil);
}
