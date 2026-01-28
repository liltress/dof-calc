use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DofCalcArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Saves the lens used in cli instance to specified path
    #[arg(long, short, value_name = "FILE OUT")]
    pub path_out: Option<PathBuf>,

    /// overrides values passed in through file or preset
    #[arg(long)]
    pub with_name: Option<String>,
    #[arg(long)]
    /// [mm]
    pub with_fl: Option<f32>,
    #[arg(long)]
    pub with_fstop: Option<f32>,
    /// [m] from sensor plane
    #[arg(long)]
    pub with_fd: Option<f32>,
    #[arg(long)]
    pub with_description: Option<String>,
    #[arg(long)]
    pub with_artwork: Option<String>,

    /// shows optional ui elements
    #[arg(long)]
    pub show_artwork: bool,
    #[arg(long)]
    pub show_misc: bool,
    #[arg(long)]
    pub show_description: bool,

    /// hides default ui elements
    #[arg(long)]
    pub hide_spec: bool,
    #[arg(long)]
    pub hide_scale: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// load lens from file File { path: PathBuf },
    File { path: PathBuf },
    /// Use a built-in lens
    Preset { name: String },
    /// Shows avaliable built-in lenses
    ShowPresets,
}
