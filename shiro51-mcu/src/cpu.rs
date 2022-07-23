use std::time::{Duration, Instant};

use bit_field::BitField;
use log::{debug, error};
use shiro51_util::error::{ErrorType, Result, RuntimeError};

use crate::addr::{Addr16, Addr8, BitAddr};
use crate::emi::EMI;
use crate::instructions::instruction::{Instruction, InstructionTable};
use crate::registers::SFR;

pub(crate) const MEMORY_FLASH_SIZE: usize = 0xFFFF;
pub(crate) const MEMORY_RAM_SIZE: usize = 0xFF;

#[derive(Debug)]
#[repr(u8)]
pub enum PC {
    ADVANCE = 1 << 0,
    SKIP = 1 << 1,
    JUMP = 1 << 2,
}

/// The power management modes of the CPU. These modes can be modified by
/// directly setting the PCON SFR register to 1 (Idle Mode) or 2 (Stop Mode).
#[derive(Debug)]
#[repr(u8)]
pub enum PowerManagementMode {
    /// Indicates that no power management mode has been selected. This is the default for PCON = 0.
    None,
    /// Indicates that the Controller should enter Idle Mode. This will cause the CPU clock to be stopped,
    /// but all other clocks will continue execution.
    Idle,
    /// Indicates that the Controller should enter Stop Mode. This will cause all clocks to be stopped.
    /// This excludes the external Oscillator circuit.
    Stop,
}

impl std::fmt::Display for PowerManagementMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PowerManagementMode::None => "No Mode",
            PowerManagementMode::Idle => "Idle Mode",
            PowerManagementMode::Stop => "Stop Mode",
        })
    }
}

#[derive(Debug)]
pub struct CPU {
    pub(crate) pc: Addr16,
    pub(crate) flash: [u8; MEMORY_FLASH_SIZE],
    pub(crate) ram: [u8; MEMORY_RAM_SIZE],
    emi: EMI,
    instruction_table: InstructionTable,
    initialized: bool,
    /// The currently selected [`PowerManagementMode`]
    pmm: PowerManagementMode,
    elapsed_time: Duration,
    last_cycle_time: Instant,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            pc: Addr16::zero(),
            flash: [0u8; MEMORY_FLASH_SIZE],
            ram: [0u8; MEMORY_RAM_SIZE],
            emi: EMI::default(),
            instruction_table: InstructionTable::new(),
            initialized: false,
            pmm: PowerManagementMode::None,
            elapsed_time: Duration::ZERO,
            last_cycle_time: Instant::now(),
        }
    }
}

impl CPU {
    /// Initializes the CPU for a new execution run.
    pub fn init(&mut self, file: &'static str) {
        self.flash = [0u8; MEMORY_FLASH_SIZE];
        self.ram = [0u8; MEMORY_RAM_SIZE];

        let content = std::fs::read_to_string(file).unwrap();

        debug!("HEX FILE CONTENT:\n{}", content);

        let mut pc_offset = 0;
        let mut lines = content.lines();

        while let Some(line) = lines.next() {
            let decoded_hex = hex::decode(&line.replace(":", "")).unwrap();
            for byte in decoded_hex {
                self.flash[pc_offset] = byte;
                pc_offset += 1;
            }
        }

        self.write(SFR::ADC0CF.addr(), 0xF8);
        self.write(SFR::ADC0GTH.addr(), 0xFF);
        self.write(SFR::ADC0GTL.addr(), 0xFF);
        self.write(SFR::CPT0MD.addr(), 0x02);
        self.write(SFR::CPT1MD.addr(), 0x02);
        self.write(SFR::SP.addr(), 0x07);
        self.write(SFR::IT01CF.addr(), 0x01);
        self.write(SFR::PFE0CN.addr(), 0x20);

        self.initialized = true;
    }

    pub fn reset(&mut self) {
        self.initialized = false;
    }

    pub fn cycle(&mut self) -> Result<()> {
        if !self.initialized {
            error!("CPU is not initialized!");
            return Err(RuntimeError::new(ErrorType::UninitializedCPU));
        }

        let cur_pc = self.pc.as_usize();
        let insn = Instruction::from(self.flash[cur_pc]);

        debug!("Current Instruction: {:?}", insn);

        let insn_bytes = insn.bytes();

        let (arg0, arg1): (Option<u8>, Option<u8>) = match insn_bytes {
            1 => (Some(self.flash[cur_pc + 1]), None),
            2 => (Some(self.flash[cur_pc + 1]), Some(self.ram[cur_pc + 2])),
            _ => (None, None),
        };

        let handler = self.instruction_table.get_handler(&insn)?;

        match handler(self, insn, arg0, arg1)? {
            PC::ADVANCE => self.pc += insn_bytes,
            _ => (),
        };

        self.elapsed_time += Instant::now() - self.last_cycle_time;
        self.last_cycle_time = Instant::now();
        debug!("Elapsed Time: {:?}", self.elapsed_time);

        Ok(())
    }

    pub(crate) fn write(&mut self, addr: Addr8, val: u8) {
        debug!("[WRITE {:#04x}]: {:#04x}", addr.as_u8(), val);
        self.ram[addr.as_usize()] = val;
    }

    pub(crate) fn read(&self, addr: Addr8) -> u8 {
        let val = self.ram[addr.as_usize()];
        debug!("[READ {:#04x}]: {:#04x}", addr.as_u8(), val);
        val
    }

    pub(crate) fn read_bit(&self, addr: BitAddr) -> Result<bool> {
        if !addr.is_bit_addressable() {
            // self.halt(CpuHaltRequest::new(format!("bit is not bit addressable: {:?} ({:#04x})", addr, addr).as_str()));
            return Err(RuntimeError::new(ErrorType::InvalidBitAddr));
        }

        let bit = addr.bit();

        debug!("addr: {:#04x}; bit: {:#010b}", addr, bit);

        let value = self.read(addr.as_addr8()) & bit == bit;

        debug!("[BIT READ: {:#04x}.{:?}]: {:?}", addr, bit, value);

        Ok(value)
    }

    #[allow(unused)]
    pub(crate) fn write_bit(&mut self, addr: BitAddr, val: bool) -> Result<()> {
        if !addr.is_bit_addressable() {
            return Err(RuntimeError::new(ErrorType::InvalidBitAddr));
            // self.halt(CpuHaltRequest::new(format!("bit is not bit addressable: {:?} ({:#04x})", addr, addr).as_str()));
            // return;
        }

        let bit = addr.bit();

        debug!("addr: {:#04x}; bit: {:#010b}", addr, bit);

        let mut bits = self.read(addr.as_addr8());
        bits.set_bit(bit as usize, val);

        self.write(addr.as_addr8(), bits);

        debug!("[BIT WRITE: {:#04x}]: {:?}", addr, val);
        Ok(())
    }
}
