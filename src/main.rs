#![allow(special_module_name)]
use std::process::exit;

use crate::args::{Commands, DofCalcArgs};
use crate::lib::core::*;
use clap::Parser;
use serde_json::to_string_pretty;

use std::path::{Path, PathBuf};

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
        Some(Commands::File { path: _ }) => Lens::from_specs(1., 1., 1.),
        Some(Commands::Preset { name: _ }) => Lens::from_specs(2., 2., 2.),
    };
    lens.with_name("nifty 50".to_string())
        .with_description("ol' reliable".to_string());
    interpolate_args(&mut lens, &args);
    dbg!(&lens);

    if let Some(path) = &args.path_out {
        let json = to_string_pretty(&lens).unwrap();
        write_file(path);
    }
}

fn write_file(path: &Path) {
    todo!();
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
