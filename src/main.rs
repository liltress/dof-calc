use std::io::stdin;
use termion::terminal_size;

mod lib;

fn main() {
    let lens = lib::core::Lens::new(50., 2000., 1.4, lib::core::Format::FF135);
    let lens2 = lib::core::Lens::new(50., 2000., 1.2, lib::core::Format::FF135);
    let lens3 = lib::core::Lens::new(50., 2000., 0.95, lib::core::Format::FF135);
    let lens4 = lib::core::Lens::new(50., 2000., 16., lib::core::Format::FF135);
    println!("{}", &lens);
    println!("{}", &lens2);
    println!("{}", &lens3);
    println!("{}", &lens4);
    /*
            let mut input = String::new();
            stdin().read_line(&mut input).expect("failed to read line");
            println!("{}", &input)
    */
}
