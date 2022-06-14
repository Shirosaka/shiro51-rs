use std::{
    fs::File,
    io::{BufReader, Read},
};

use super::{
    super::ops::arithmetics::BitOps,
    registers::{Register, SFR},
};

use log::debug;

pub struct Memory {
    flash: [u8; 0xffff],
    mem: [u8; 0xff],
}

impl Memory {
    pub fn init() -> Self {
        let mut data_memory = Memory { flash: [0; 0xffff], mem: [0; 0xff] };

        data_memory.set_sfr_reg(SFR::ACC, 0x00);
        data_memory.set_sfr_reg(SFR::SP, 0x07);

        data_memory
    }

    pub fn load_from_file(&mut self, file_name: &str) -> bool {
        let mut reader = BufReader::new(
            File::open(file_name).expect(format!("Failed to open: {:?}", file_name).as_str()),
        );

        let mut string: String = String::new();

        reader.read_to_string(&mut string).unwrap();

        debug!("HEX FILE CONTENTS:\n{}", string);

        let mut pc_offset = 0;
        let mut lines = string.lines();

        while let Some(line) = lines.next() {
            let decoded_hex = hex::decode(&line.replace(":", "")).unwrap();
            for u8 in decoded_hex {
                self.flash[pc_offset] = u8;
                pc_offset += 1;
            }
        }

        true
    }

    fn get_bit_internal(&self, addr: u8, bit: u8) -> Option<u8> {
        if !addr.is_bit_addressable() {
            return None;
        }

        let data = self.read(addr).get_bit(bit);
        debug!("Reading bit {} at address {}: {}", bit, addr, data);
        Some(data)
    }

    pub fn get_bit(&self, bit_addr: u8) -> Option<u8> {
        let bit = bit_addr % 8;

        if bit_addr <= 127 {
            return self.get_bit_internal(bit_addr / 8 + 32, bit);
        }

        self.get_bit_internal(bit_addr - bit, bit)
    }

    pub fn cur_reg_bank(&self) -> u8 {
        let psw = self.get_sfr_reg(SFR::PSW);
        let rs0 = psw.get_bit(3);
        let rs1 = psw.get_bit(4);

        (rs1 << 1) | rs0
    }

    pub fn read_flash(&self, addr: u16) -> u8 {
        let val = self.flash[addr as usize];

        debug!("[FLASH READ: {:#06x}]: {:#04x}", addr, val);

        val
    }

    // leaving this if needed
    // pub fn write_flash(&mut self, addr: u16, val: u8) {
    //     debug!("[FLASH WRITE: {:#06x}]: {:#04x}", addr, val);

    //     self.mem[addr as usize] = val;
    // }

    pub fn read(&self, addr: u8) -> u8 {
        let val = self.mem[addr as usize];

        debug!("[READ: {:#04x}]: {:#04x}", addr, val);

        val
    }

    pub fn write(&mut self, addr: u8, val: u8) {
        debug!("[WRITE: {:#04x}]: {:#04x}", addr, val);

        self.mem[addr as usize] = val;
    }

    pub fn get_sfr_reg(&self, sfr: SFR) -> u8 {
        debug!("[SFR READ]: {:?}", sfr);
        self.read(sfr.into())
    }

    pub fn set_sfr_reg(&mut self, sfr: SFR, val: u8) {
        debug!("[SFR WRITE]: {:?}", sfr);
        self.write(sfr.into(), val);
    }

    pub fn get_gpr_reg(&self, reg: Register) -> u8 {
        debug!("[GPR READ]: {:?} at REG BANK {}", reg, self.cur_reg_bank());
        let reg_addr: u8 = reg.into();
        self.read(reg_addr * self.cur_reg_bank())
    }

    pub fn set_gpr_reg(&mut self, reg: Register, val: u8) {
        debug!("[GPR WRITE]: {:?} at REG BANK {}", reg, self.cur_reg_bank());
        let reg_addr: u8 = reg.into();
        self.write(reg_addr * self.cur_reg_bank(), val);
    }
}
