use clap::Parser;

use cross_convert::img_manager::{convert_image, is_image_extension};
use cross_convert::start_app;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    ext: Option<String>,
}

fn main() {
    let args = Args::parse();

    // check if the provided path is a valid file
    if !std::path::Path::new(&args.path).is_file() {
        eprintln!("Invalid file path: {}", args.path);
        std::process::exit(1);
    }

    match args.ext {
        Some(ext) => {
            if !is_image_extension(&ext) {
                eprintln!("Unsupported format: {}", ext);
                std::process::exit(1);
            }
            match convert_image(args.path.clone(), &ext) {
                Ok(_) => {
                    println!("Image converted successfully");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error converting image: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => match start_app(args.path) {
            Ok(_) => {
                println!("Application exited successfully");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error starting application: {}", e);
                std::process::exit(1);
            }
        },
    }
}
