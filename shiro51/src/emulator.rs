use shiro51_mcu::cpu::CPU;

use crate::Cli;

#[allow(unused)]
pub struct Emulator {
    cpu: CPU,
    headless: bool,
}

impl Emulator {
    pub(crate) fn new(cli: &Cli) -> Self {
        Emulator {
            cpu: CPU::default(),
            headless: cli.no_gui,
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.cpu.cycle().unwrap();
        }
    }
}
