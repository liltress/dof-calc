#![allow(special_module_name)]
use args::DofCalcArgs;
use clap::Parser;

mod args;
mod lib;

fn main() {
    let args = args::DofCalcArgs::parse();
    let lens = lib::core::Lens::default();
    //let lorem_ipsum: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi ac arcu lorem. Sed aliquet nunc vehicula, congue massa sit amet, ullamcorper turpis. Maecenas cursus sapien quis tempus efficitur. Duis scelerisque velit sed bibendum aliquam. Nam sit amet venenatis purus. Mauris nisi erat, porta eget quam vitae, tempor interdum ex. Praesent mattis felis eu sem maximus, a sodales odio tincidunt. Maecenas at ultrices lectus. Etiam euismod, risus sit amet vestibulum placerat, erat eros rutrum orci, vitae vehicula ligula est sed ipsum. In vel euismod massa. Interdum et malesuada fames ac ante ipsum primis in faucibus. Vivamus faucibus tempus nulla, vel tempor dui molestie a. ";
    //dbg!(&args);
    match args.verbosity {
        Some(0) => {
            dialogue(args, lens);
        }
        Some(1) => {
            show_once(lens);
        }
        Some(2) => {
            todo!("scale only");
        }
        Some(3) => {
            todo!("minimum");
        }
        None => {
            show_once(lens);
        }
        _ => {
            show_once(lens);
        }
    }
}

fn dialogue(_args: DofCalcArgs, _lens: lib::core::Lens) {
    todo!("dialogue");
}

#[allow(dead_code)]
fn helpmenu(_args: DofCalcArgs) {
    todo!();
}

fn show_once(lens: lib::core::Lens) {
    [
        lib::menu::MenuItem::Bar,
        lib::menu::MenuItem::Blank,
        lib::menu::MenuItem::SpecList(&lens),
        lib::menu::MenuItem::Blank,
        lib::menu::MenuItem::Scale(&lens),
        lib::menu::MenuItem::Blank,
        lib::menu::MenuItem::Bar,
    ]
    .iter()
    .for_each(|mi| print!("{mi}"));
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    args::DofCalcArgs::command().debug_assert()
}
