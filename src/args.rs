use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DofCalcArgs {
    /// Provide a TOML config for the lens
    #[arg(short, long, value_name = "FILE")]
    pub path: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// 0: Enter dialogue;
    /// 1: Print all lens info;
    /// 2: Print scale;
    /// 3: Print near, far focus and hyperfocal distances
    #[arg(short, long)]
    pub verbosity: Option<u8>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Provide your own lens in CLI. If a config is provided this will interpolate the values into it
    Override {
        /// lens name
        #[arg(short, long)]
        name: Option<String>,
        /// focal length [mm]
        #[arg(short, long, value_name = "FOCAL_LENGTH")]
        length: Option<f32>,
        /// aperture
        #[arg(short, long, value_name = "FSTOP")]
        aperture: Option<f32>,
        /// the distance [m] (from the sensor) the lens is focused to
        #[arg(short, long)]
        focus: Option<f32>,
    },
}
