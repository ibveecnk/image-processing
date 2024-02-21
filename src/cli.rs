use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

/// CLI arguments
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input image processing type
    #[arg(short, long)]
    pub processing_type: ProcessingType,

    /// Input image file
    #[arg(short, long)]
    pub input: PathBuf,
}

/// Image processing type enum
#[derive(Debug, Clone)]
pub enum ProcessingType {
    /// Contrast Processing
    Contrast,
    /// Ascii Processing
    Ascii,
    /// Convolution Processing
    Convolution,
}

impl FromStr for ProcessingType {
    type Err = String;

    #[allow(clippy::single_match_else)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "contrast" => Ok(Self::Contrast),
            "ascii" => Ok(Self::Ascii),
            "convolution" => Ok(Self::Convolution),
            _ => {
                let possible_values = ["contrast", "ascii", "convolution"];
                let err = format!("\npossible types are: [{}]", possible_values.join(", "));
                Err(err)
            }
        }
    }
}

impl Display for ProcessingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contrast => write!(f, "contrast"),
            Self::Ascii => write!(f, "ascii"),
            Self::Convolution => write!(f, "convolution"),
        }
    }
}

/// Validate CLI arguments
#[allow(clippy::print_stderr)]
fn validate_args(args: &Args) {
    if !args.input.exists() {
        eprintln!("Input file does not exist");
        std::process::exit(1);
    }

    if args.input.extension().is_none() {
        eprintln!("Input file must have an extension");
        std::process::exit(1);
    }
}

/// Ensure that the input path is an absolute path
fn ensure_absolute_path(args: &mut Args) {
    if !args.input.is_absolute() {
        args.input = std::env::current_dir()
            .expect("Current directory should exist")
            .join(&args.input);
    }
}

/// Parse CLI arguments and validate them
pub fn parse_args() -> Args {
    let mut args = Args::parse();
    ensure_absolute_path(&mut args);
    validate_args(&args);
    dbg!(&args);
    args
}
