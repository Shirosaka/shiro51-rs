mod lib;

#[cfg(test)]
mod tests;

use lib::cpu::CPU;
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .write_style(WriteStyle::Auto)
        .filter(None, LevelFilter::Debug)
        .init();

    let mut cpu: CPU = CPU::init();

    cpu.load_from_file("MONKAW.HEX");
    cpu.run();
}
