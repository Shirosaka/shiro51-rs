mod lib;

use pretty_env_logger::env_logger::WriteStyle;
use log::LevelFilter;
use lib::cpu::CPU;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .write_style(WriteStyle::Auto)
        .filter(None, LevelFilter::Debug)
        .init();

    let mut cpu: CPU = CPU::init();

    cpu.load_from_file("MONKAW.HEX");
    cpu.run();
}
