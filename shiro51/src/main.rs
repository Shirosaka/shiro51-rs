pub mod emulator;

use chrono::Local;
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use log::{error, info};

use crate::emulator::Emulator;

#[derive(Debug, Parser)]
#[clap(name = "shiro51")]
#[clap(about = "An emulator for the MCS-51 instruction set written in Rust.", long_about = None)]
#[clap(author, version)]
struct Cli {
    /// The optional file to load in on startup.
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Enable application debug messages.
    #[clap(short, long, action)]
    enable_debug: bool,

    /// Run the emulator without a graphical user interface.
    #[clap(short, long, action)]
    no_gui: bool,
}

fn setup_logger(cli: &Cli) -> Result<(), fern::InitError> {
    let mut colors = ColoredLevelConfig::new();

    colors.debug = Color::Blue;
    colors.trace = Color::BrightBlue;
    colors.info = Color::Green;
    colors.warn = Color::Yellow;
    colors.error = Color::Red;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {} > {}",
                Local::now().format("%X%.3f"),
                colors.color(record.level()),
                record.target().split("::").last().unwrap_or(record.target()),
                message
            ))
        })
        .level(match cli.enable_debug {
            true => {
                info!("Enabled debug messages.");
                log::LevelFilter::Debug
            },
            false => log::LevelFilter::Info,
        })
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    setup_logger(&cli)?;

    let mut emulator = Emulator::new(&cli);

    if cli.file.is_none() {}

    if cli.no_gui {
        info!("Mode: Headless");
        emulator.run();
    } else {
        info!("Mode: GUI");
        error!("Unimplemented!")
    }

    Ok(())
}
