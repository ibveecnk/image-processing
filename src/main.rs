//! Image processing tool

use photon_rs::{
    native::{open_image, save_image},
    PhotonImage,
};
use std::path::PathBuf;

use crate::{
    cli::{Args, ProcessingType},
    processors::{ascii, contrast, convolution, processor::Processor},
};

/// CLI arguments
mod cli;

/// Rgba pixel type
mod rgba;

/// Image processing
mod processors;

/// Change the file name of a `PathBuf`
fn change_file_name<'a>(path: &'a mut PathBuf, name: &'a str) -> &'a mut PathBuf {
    let old_ext = path
        .extension()
        .expect("Path should have an extension")
        .to_os_string();

    path.set_file_name(name);
    path.set_extension(&old_ext);
    path
}

/// Change the extension of a `PathBuf`
fn change_extension<'a>(path: &'a mut PathBuf, ext: &'a str) -> &'a mut PathBuf {
    path.set_extension(ext);
    path
}

#[allow(clippy::print_stdout, clippy::print_stderr)]
fn main() {
    let mut args: Args = cli::parse_args();

    let img = open_image(&args.input.to_string_lossy());

    match img {
        Ok(_) => println!("Image opened successfully"),
        Err(e) => {
            eprintln!("Error opening image: {e}");
            std::process::exit(1);
        }
    }

    let img = img.expect("Clap should have validated this");

    let processed_image: Option<PhotonImage> = match args.processing_type {
        ProcessingType::Contrast => Some(contrast::Contrast.process(&img)),
        ProcessingType::Ascii => {
            let modified_path = change_extension(change_file_name(&mut args.input, "ascii"), "txt");
            ascii::Ascii::write(&img, modified_path);
            None
        }
        ProcessingType::Convolution => Some(convolution::Convolution.process(&img)),
    };

    if processed_image.is_none() {
        println!("Image processing complete");
        std::process::exit(0);
    }

    let processed_image = processed_image.expect("Image should be processed");

    let mut output = args.input;
    let save_name = change_file_name(&mut output, "processed");

    match save_image(processed_image, &save_name.to_string_lossy()) {
        Ok(()) => println!("Image saved successfully"),
        Err(e) => {
            eprintln!("Error saving image: {e}");
            std::process::exit(1);
        }
    }

    println!("Image processing complete");
    std::process::exit(0);
}
