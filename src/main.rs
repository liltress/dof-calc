#![allow(special_module_name)]
use std::io;
use std::process::exit;

use crate::args::{Commands, DofCalcArgs};
use crate::lib::core::*;
use clap::Parser;
use serde_json::{from_str, to_string_pretty};

use std::fs::{exists, read_to_string, write};
use std::path::Path;

mod args;
mod lib;

fn main() {
    let args = DofCalcArgs::parse();
    dbg!(&args);

    if args.command == Some(Commands::ShowPresets) {
        println!("none yet!");
        exit(1);
    }

    let mut lens = match &args.command {
        Some(Commands::ShowPresets) => unreachable!(),

        None => Lens::from_specs(50., 1000., 16.),

        Some(Commands::File { path: p }) => {
            let l: Lens = from_str(&read_file(p)).expect("failed to parse json");
            l
        }
        Some(Commands::Preset { name: _ }) => Lens::from_specs(2., 2., 2.),
    };
    interpolate_args(&mut lens, &args);
    dbg!(&lens);

    if let Some(path) = &args.path_out {
        let json = to_string_pretty(&lens).unwrap();
        let _ = write_file(path, json, &args);
    }
}

fn read_file(path: &Path) -> String {
    read_to_string(path).expect("failed to read file")
}

fn write_file(path: &Path, contents: String, args: &DofCalcArgs) -> Result<(), &'static str> {
    if path.is_relative() {
        println!("This is a relative path, please use absolute path");
        return Err("relativepath");
    } else if path.ancestors().any(|a| !a.exists()) {
        println!("Directory this file would be written to does not exist");
        return Err("noparent");
    } else if path.exists() && !args.force {
        println!("Attempted to write file {:#?} but it already exists", &path);
        println!("Hint: if you want to overwrite it, use -f or --force");
        return Err("needforce");
    } else if path.exists() && !path.is_file() {
        println!("Attempted to write to {:#?} but it is not a file", &path);
        return Err("nonfile");
    }

    println!("writing to file!");
    write(path, contents).unwrap();
    Ok(())
}

fn interpolate_args(lens: &mut Lens, args: &DofCalcArgs) {
    if let Some(fl) = args.with_fl {
        lens.with_fl(fl);
    }
    if let Some(fd) = args.with_fd {
        lens.with_fl(fd);
    }
    if let Some(fstop) = args.with_fstop {
        lens.with_fl(fstop);
    }
    if let Some(name) = &args.with_name {
        lens.with_name(name.clone());
    }
    if let Some(desc) = &args.with_description {
        lens.with_description(desc.clone());
    }
    if let Some(art) = &args.with_artwork {
        lens.with_artwork(art.clone());
    }
}

#[test]
fn verify_cli() {
    use crate::args::DofCalcArgs;
    use clap::CommandFactory;
    DofCalcArgs::command().debug_assert()
}
