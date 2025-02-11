mod models;

fn main() {
    println!("Hello, world!");

    let konsol = models::game::GamingConsole{
        name: String::from("PlayStation 5")
    };

    println!("Saya ingin : {:#?}", konsol);
    

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Mobil{
        brand: String,
        mode: String,
        year: u32
    }
    
    let year = 1986;

    let mut ae86 = Mobil{
        brand: String::from("Toyota"),
        mode: String::from("AE86"),
        year,
    };

    ae86.year = 1987;

    println!("That car is : {:#?}", ae86);


    let warna = models::color::Color(255,0,0);

    println!("That color is : {:?}", warna);
}
