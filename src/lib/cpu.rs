use core::panic;
use std::ops::Sub;
use log::{debug, error, info};

use super::{
    instruction::Instruction,
    memory::{
        memory::Memory,
        registers::{Register, SFR},
    },
    ops::arithmetics::BitOps,
};

#[derive(Debug, PartialEq)]
enum PC {
    /// Let the main loop know that it can advance the PC by the instruction byte size.
    Advance,
    /// Let the main loop know that the instruction has handled PC operations.
    Handled,
}

pub struct CPU {
    pc: usize,
    data: Memory,
    halt: bool,
}

impl CPU {
    pub fn init() -> Self {
        info!("Initializing CPU.");

        let cpu: CPU = CPU { pc: 0, data: Memory::init(), halt: false };

        info!("CPU initialized.");

        cpu
    }

    pub fn load_from_file(&mut self, file_name: &str) -> bool {
        self.data.load_from_file(file_name)
    }

    pub fn run(&mut self) {
        loop {
            if self.halt {
                break;
            }

            let insn = Instruction::try_from(self.data.read_flash(self.pc as u16)).unwrap();

            info!("Current Instruction: {:?}", insn);

            let insn_size = Instruction::get_num_bytes(&insn);

            let (arg0, arg1): (u8, u8) = match insn_size {
                1 => (0, 0),
                2 => (self.data.read_flash((self.pc + 1) as u16), 0),
                3 => (
                    self.data.read_flash((self.pc + 1) as u16),
                    self.data.read_flash((self.pc + 2) as u16),
                ),
                _ => {
                    self.halt("invalid instruction size", insn);
                    (0, 0)
                },
            };

            match self.run_instruction(insn, arg0, arg1) {
                PC::Advance => {
                    self.pc += insn_size;

                    if self.pc >= 0xFFFF {
                        self.pc -= 0xFFFF
                    }
                },
                PC::Handled => continue,
            }
        }
    }

    fn run_instruction(&mut self, insn: Instruction, arg0: u8, arg1: u8) -> PC {
        debug!("CPU::run_instruction({:?}, {:?}, {:?})", insn, arg0, arg1);

        let op: u8 = insn.into();

        match insn {
            // 0x00
            Instruction::NOP => PC::Advance,
            // 0x01, 0x21, 0x41, 0x61, 0x81, 0xa1, 0xc1, 0xe1
            Instruction::AJMP1
            | Instruction::AJMP2
            | Instruction::AJMP3
            | Instruction::AJMP4
            | Instruction::AJMP5
            | Instruction::AJMP6
            | Instruction::AJMP7
            | Instruction::AJMP8 => {
                self.pc += 2;

                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);
                self.pc &= 0xF800;
                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);
                self.pc |= ((op & 0xe0) as usize) << 3;
                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);
                self.pc |= arg0 as usize;
                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);

                PC::Handled
            },
            // 0x02
            Instruction::LJMP => {
                self.pc = (arg0 as usize) << 8;
                self.pc |= arg1 as usize;
                debug!("PC: {:#06x}", self.pc);
                PC::Handled
            },
            // 0x03
            Instruction::RR_A => {
                self.data.set_sfr_reg(SFR::ACC, self.data.get_sfr_reg(SFR::ACC) >> 1);
                PC::Advance
            },
            // 0x04
            Instruction::INC_A => {
                self.data.set_sfr_reg(SFR::ACC, self.data.get_sfr_reg(SFR::ACC) + 1);
                PC::Advance
            },
            // 0x05
            Instruction::INC_DATA => {
                self.data.write(arg0, self.data.read(arg0) + 1);
                PC::Advance
            },
            // 0x06
            Instruction::INC_INDIRECT_R0 => {
                let addr = self.data.get_gpr_reg(Register::R0);
                self.data.write(addr, self.data.read(addr) + 1);
                PC::Advance
            },
            // 0x07
            Instruction::INC_INDIRECT_R1 => {
                let addr = self.data.get_gpr_reg(Register::R1);
                self.data.write(addr, self.data.read(addr) + 1);
                PC::Advance
            },
            // 0x08..=0x0f
            Instruction::INC_R0
            | Instruction::INC_R1
            | Instruction::INC_R2
            | Instruction::INC_R3
            | Instruction::INC_R4
            | Instruction::INC_R5
            | Instruction::INC_R6
            | Instruction::INC_R7 => {
                let reg = Register::try_from(op - 0x08).unwrap();
                self.data.set_gpr_reg(reg, self.data.get_gpr_reg(reg) + 1);
                PC::Advance
            },
            // 0x11, 0x31, 0x51, 0x71, 0x91, 0xb1, 0xd1, 0xf1
            Instruction::ACALL1
            | Instruction::ACALL2
            | Instruction::ACALL3
            | Instruction::ACALL4
            | Instruction::ACALL5
            | Instruction::ACALL6
            | Instruction::ACALL7
            | Instruction::ACALL8 => {
                self.pc += 2;

                let mut sp = self.data.get_sfr_reg(SFR::SP);

                sp += 1;
                self.data.write(sp, (self.pc & 0x00FF) as u8);
                sp += 1;
                self.data.write(sp, ((self.pc & 0xFF00) >> 8) as u8);

                self.data.set_sfr_reg(SFR::SP, sp);

                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);
                self.pc &= 0xF800;
                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);
                self.pc |= ((op & 0xe0) as usize) << 3;
                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);
                self.pc |= arg0 as usize;
                debug!("PC: {:#06x} ({:#018b})", self.pc, self.pc);

                PC::Handled
            },
            // 0x20
            Instruction::JB_BIT_CODE => {
                self.pc += 3;

                let bit = self.data.get_bit(arg0).unwrap();

                if bit == 1 {
                    self.pc += arg1 as usize;
                }

                PC::Handled
            },
            // 0x22
            Instruction::RET => {
                let mut sp = self.data.get_sfr_reg(SFR::SP);

                self.pc = (self.data.read(sp) as usize) << 8;
                sp -= 1;
                self.pc |= self.data.read(sp) as usize;
                sp -= 1;

                debug!("PC: {:#06x}; SP: {:#04x}", self.pc, sp);

                self.data.set_sfr_reg(SFR::SP, sp);
                PC::Handled
            },
            // 0x23
            Instruction::RL_A => {
                self.data.set_sfr_reg(SFR::ACC, self.data.get_sfr_reg(SFR::ACC) << 1);
                PC::Advance
            },
            // 0x30
            Instruction::JNB_BIT_CODE => {
                self.pc += 3;

                let bit = self.data.get_bit(arg0).unwrap();

                if bit == 0 {
                    self.pc += arg1 as usize;
                }

                PC::Handled
            }
            // 0x38..=0x3f
            Instruction::ADDC_A_R0
            | Instruction::ADDC_A_R1
            | Instruction::ADDC_A_R2
            | Instruction::ADDC_A_R3
            | Instruction::ADDC_A_R4
            | Instruction::ADDC_A_R5
            | Instruction::ADDC_A_R6
            | Instruction::ADDC_A_R7 => {
                let acc = self.data.get_sfr_reg(SFR::ACC);
                let rn = self.data.get_gpr_reg(Register::try_from(op - 0x38).unwrap());

                self.addc(acc, rn);

                PC::Advance
            },
            // 0x43
            Instruction::ORL_DATA_CONST => {
                self.data.write(arg0, self.data.read(arg0) | arg1);
                PC::Advance
            },
            // 0x4d
            Instruction::ORL_A_R5 => {
                self.data.set_sfr_reg(
                    SFR::ACC,
                    self.data.get_sfr_reg(SFR::ACC) | self.data.get_gpr_reg(Register::R5),
                );
                PC::Advance
            },
            // 0x50
            Instruction::JNC => {
                self.pc += 2;
                let psw = self.data.get_sfr_reg(SFR::PSW);

                // if carry is not set
                if !psw.is_bit_set(7) {
                    self.pc += arg0 as usize;
                }

                PC::Handled
            }
            // 0x52
            Instruction::ANL_DATA_A => {
                let data = self.data.read(arg0);
                let acc = self.data.get_sfr_reg(SFR::ACC);

                self.data.write(arg0, data & acc);
                PC::Advance
            },
            // 0x5f
            Instruction::ANL_A_R7 => {
                self.data.set_sfr_reg(
                    SFR::ACC,
                    self.data.get_sfr_reg(SFR::ACC) & self.data.get_gpr_reg(Register::R7),
                );
                PC::Advance
            },
            // 0x78..=0x7f
            Instruction::MOV_R0_CONST
            | Instruction::MOV_R1_CONST
            | Instruction::MOV_R2_CONST
            | Instruction::MOV_R3_CONST
            | Instruction::MOV_R4_CONST
            | Instruction::MOV_R5_CONST
            | Instruction::MOV_R6_CONST
            | Instruction::MOV_R7_CONST => {
                self.data.set_gpr_reg(Register::try_from(op - 0x78).unwrap(), arg0);
                PC::Advance
            }
            // 0x80
            Instruction::SJMP => {
                self.pc += 2;

                if arg0.is_bit_set(7){
                    self.pc = usize::wrapping_add(self.pc, arg0.to_signed() as usize);
                } else {
                    self.pc += arg0 as usize;
                }

                PC::Handled
            }
            // 0x88..=0x8f
            Instruction::MOV_DATA_R0
            | Instruction::MOV_DATA_R1
            | Instruction::MOV_DATA_R2
            | Instruction::MOV_DATA_R3
            | Instruction::MOV_DATA_R4
            | Instruction::MOV_DATA_R5
            | Instruction::MOV_DATA_R6
            | Instruction::MOV_DATA_R7 => {
                self.data.write(arg0, self.data.get_gpr_reg(Register::try_from(op - 0x88).unwrap()));
                PC::Advance
            },
            // 0x95
            Instruction::SUBB_A_DATA => {
                let acc = self.data.get_sfr_reg(SFR::ACC);
                let data = self.data.read(arg0);

                self.subb(acc, data);

                PC::Advance
            },
            // 0x98..=0x9f
            Instruction::SUBB_A_R0
            | Instruction::SUBB_A_R1
            | Instruction::SUBB_A_R2
            | Instruction::SUBB_A_R3
            | Instruction::SUBB_A_R4
            | Instruction::SUBB_A_R5
            | Instruction::SUBB_A_R6
            | Instruction::SUBB_A_R7 => {
                let reg = self.data.get_gpr_reg(Register::try_from(op - 0x98).unwrap());
                let acc = self.data.get_sfr_reg(SFR::ACC);

                self.subb(acc, reg);

                PC::Advance
            },
            // 0xa3
            Instruction::INC_DPTR => {
                let mut dpl = self.data.get_sfr_reg(SFR::DPL);
                let mut dph = self.data.get_sfr_reg(SFR::DPH);

                let mut dptr = (dph as u16) << 8 | dpl as u16;

                dptr += 1;

                dpl = (dptr & 0xff) as u8;
                dph = ((dptr & 0xff00) >> 8) as u8;

                self.data.set_sfr_reg(SFR::DPL, dpl);
                self.data.set_sfr_reg(SFR::DPH, dph);

                PC::Advance
            }
            // 0xa8..=0xaf
            Instruction::MOV_R0_DATA
            | Instruction::MOV_R1_DATA
            | Instruction::MOV_R2_DATA
            | Instruction::MOV_R3_DATA
            | Instruction::MOV_R4_DATA
            | Instruction::MOV_R5_DATA
            | Instruction::MOV_R6_DATA
            | Instruction::MOV_R7_DATA => {
                self.data.set_gpr_reg(Register::try_from(op - 0xa8).unwrap(), self.data.read(arg0));

                PC::Advance
            },
            // 0xb5
            Instruction::CJNE_A_DATA_CODE => {
                self.pc += 3;
                let acc = self.data.get_sfr_reg(SFR::ACC);

                debug!(
                    "CJNE_A_DATA_CODE: ACC={:#04x}; DATA={:#04x}, CODE={:#04x}",
                    acc, arg0, arg1
                );

                if acc != arg0 {
                    self.pc += arg1 as usize;
                    debug!("CJNE_A_DATA_CODE: acc != arg0");
                }

                let mut psw = self.data.get_sfr_reg(SFR::PSW);
                debug!("PSW: {:#010b}", psw);

                if acc < arg0 {
                    psw.set_bit(7);
                } else {
                    psw.clear_bit(7);
                }
                debug!("PSW: {:#010b}", psw);

                self.data.set_sfr_reg(SFR::PSW, psw);
                PC::Handled
            },
            // 0xb8..= 0xbf
            Instruction::CJNE_R0_CONST_CODE
            | Instruction::CJNE_R1_CONST_CODE
            | Instruction::CJNE_R2_CONST_CODE
            | Instruction::CJNE_R3_CONST_CODE
            | Instruction::CJNE_R4_CONST_CODE
            | Instruction::CJNE_R5_CONST_CODE
            | Instruction::CJNE_R6_CONST_CODE
            | Instruction::CJNE_R7_CONST_CODE => {
                self.pc += 3;
                let data = self.data.get_gpr_reg(Register::try_from(op - 0xb8).unwrap());
                let mut psw = self.data.get_sfr_reg(SFR::PSW);

                if data != arg0 {
                    self.pc += arg1 as usize;
                }

                if data < arg0 {
                    psw.set_bit(7);
                } else {
                    psw.clear_bit(7);
                }

                self.data.set_sfr_reg(SFR::PSW, psw);

                PC::Handled
            }
            // 0xc0
            Instruction::PUSH_DATA => {
                let sp = self.data.get_sfr_reg(SFR::SP) + 1;

                self.data.set_sfr_reg(SFR::SP, sp);
                self.data.write(sp, self.data.read(arg0));
                PC::Advance
            },
            // 0xd0
            Instruction::POP_DATA => {
                let sp = self.data.get_sfr_reg(SFR::SP);
                let data = self.data.read(sp);

                self.data.set_sfr_reg(SFR::SP, sp - 1);
                self.data.write(arg0, data);
                PC::Advance
            },
            // 0xd5
            Instruction::DJNZ_DATA_CODE => {
                self.pc += 2;
                let mut data = self.data.read(arg0);

                if data > 0 {
                    data -= 1;
                    self.data.write(arg0, data);
                    self.pc += arg1 as usize;
                }

                PC::Handled
            },
            // 0xd8..=0xda
            Instruction::DJNZ_R0_CODE
            | Instruction::DJNZ_R1_CODE
            | Instruction::DJNZ_R2_CODE
            | Instruction::DJNZ_R3_CODE
            | Instruction::DJNZ_R4_CODE
            | Instruction::DJNZ_R5_CODE
            | Instruction::DJNZ_R6_CODE
            | Instruction::DJNZ_R7_CODE => {
                self.pc += 2;
                let reg = Register::try_from(op - 0xd8).unwrap();
                let mut data = self.data.get_gpr_reg(reg);

                if data > 0 {
                    data -= 1;
                    self.data.set_gpr_reg(reg, data);
                    self.pc += arg1 as usize;
                }
                PC::Handled
            },
            // 0xe0
            Instruction::MOVX_A_INDIRECT_DPTR => {
                let mut dptr = 0u16;
                let dph = self.data.get_sfr_reg(SFR::DPH);
                let dpl = self.data.get_sfr_reg(SFR::DPL);

                debug!("DPH: {:#04x}, DPL: {:#04x}", dph, dpl);

                dptr |= (dph as u16) << 8;
                dptr |= dpl as u16;

                debug!("DPTR: {:#06x}", dptr);

                let val = self.data.read_flash(dptr);

                debug!("Value: {:#04x}", val);

                self.data.set_sfr_reg(SFR::ACC, val);
                PC::Advance
            },
            _ => {
                self.halt("Unimplemented Instruction", insn);
                PC::Advance
            },
        }
    }

    fn addc(&mut self, lhs: u8, rhs: u8) {
        let mut psw = self.data.get_sfr_reg(SFR::PSW);

        let res = lhs.wrapping_add(rhs + psw.get_bit(7));
        self.data.set_sfr_reg(SFR::ACC, res);

        let bit6overflow = rhs.is_bit_set(6) && lhs.is_bit_set(6);
        let bit7overflow = rhs.is_bit_set(7) && lhs.is_bit_set(7);

        match (bit6overflow, bit7overflow) {
            (true, false) => psw.set_bit(7),
            (false, true) => psw.set_bit(2),
            _ => (),
        };

        self.data.set_sfr_reg(SFR::PSW, psw);
    }

    fn subb(&mut self, lhs: u8, rhs: u8) {
        let mut psw = self.data.get_sfr_reg(SFR::PSW);

        // lhs - (rhs + carry)
        let mut res = i32::sub(lhs as i32, (rhs + psw.get_bit(7)) as i32);

        if res < 0 {
            res += 256;
            psw.set_bit(7);
        } else {
            psw.clear_bit(7)
        }

        self.data.set_sfr_reg(SFR::ACC, (res & 0xFF) as u8);

        let signed_res = lhs.to_signed() - rhs.to_signed();

        if signed_res <= 127 && signed_res >= -128 {
            psw.clear_bit(2);
        } else {
            psw.set_bit(2);
        }

        let lhs_low_nibble = lhs & 0xf;
        let rhs_low_nibble = rhs & 0xf;

        if lhs_low_nibble < rhs_low_nibble {
            psw.set_bit(6);
        } else {
            psw.clear_bit(6);
        }

        self.data.set_sfr_reg(SFR::PSW, psw);
    }

    fn halt(&mut self, msg: &str, insn: Instruction) {
        let halt_msg = format!("HALT: {} at Instruction::{:?} (PC: {:#06x})", msg, insn, self.pc);
        let padded_lb = format!("{:=>width$}", "", width = halt_msg.len());

        error!("{}", padded_lb);
        error!("{}", halt_msg);
        error!("{}", padded_lb);

        self.halt = true;
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::{BitOps, Instruction, Register, CPU, PC, SFR};

    #[test]
    fn ajmp() {
        let mut cpu = CPU::init();

        cpu.pc = 0x0345;

        let res = cpu.run_instruction(Instruction::AJMP2, 0x23, 0);

        assert_eq!(cpu.pc, 0x0123);
        assert_eq!(res, PC::Handled);
    }

    #[test]
    fn ljmp() {
        let mut cpu = CPU::init();

        cpu.run_instruction(Instruction::LJMP, 0x12, 0x34);

        assert_eq!(cpu.pc, 0x1234);
    }

    #[test]
    fn jb_bit_code() {
        let mut cpu = CPU::init();

        cpu.data.set_sfr_reg(SFR::P1, 0b11001010);
        cpu.data.set_sfr_reg(SFR::ACC, 0b01010110);

        let p1addr: u8 = SFR::P1.into();
        let accaddr: u8 = SFR::ACC.into();

        cpu.run_instruction(Instruction::JB_BIT_CODE, p1addr + 2, 0x2);
        assert_eq!(cpu.pc, 3);
        cpu.run_instruction(Instruction::JB_BIT_CODE, accaddr + 2, 0x2);
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn jnb_bit_addr() {
        let mut cpu = CPU::init();

        cpu.data.set_sfr_reg(SFR::P1, 0b11001010);
        cpu.data.set_sfr_reg(SFR::ACC, 0b01010110);

        let p1addr: u8 = SFR::P1.into();
        let accaddr: u8 = SFR::ACC.into();

        cpu.run_instruction(Instruction::JNB_BIT_CODE, p1addr + 3, 0x2);
        assert_eq!(cpu.pc, 3);
        cpu.run_instruction(Instruction::JNB_BIT_CODE, accaddr + 3, 0x2);
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn acall() {
        let mut cpu = CPU::init();

        cpu.pc = 0x0123;
        cpu.data.set_sfr_reg(SFR::SP, 0x07);

        cpu.run_instruction(Instruction::ACALL4, 0x45, 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x09);
        assert_eq!(cpu.data.read(0x08), 0x25);
        assert_eq!(cpu.data.read(0x09), 0x01);
        assert_eq!(cpu.pc, 0x0345);
    }

    #[test]
    fn ret() {
        let mut cpu = CPU::init();

        cpu.data.set_sfr_reg(SFR::SP, 0x0b);
        cpu.data.write(0x0a, 0x23);
        cpu.data.write(0x0b, 0x01);

        cpu.run_instruction(Instruction::RET, 0, 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x09);
        assert_eq!(cpu.pc, 0x0123);
    }

    #[test]
    fn addc_a_r2() {
        let mut cpu = CPU::init();

        cpu.data.set_sfr_reg(SFR::ACC, 0xc3);
        cpu.data.set_gpr_reg(Register::R2, 0xaa);
        cpu.data.set_sfr_reg(SFR::PSW, 0x80);

        cpu.run_instruction(Instruction::ADDC_A_R2, 0, 0);

        let psw = cpu.data.get_sfr_reg(SFR::PSW);

        assert_eq!(cpu.data.get_sfr_reg(SFR::ACC), 0x6e);
        assert_eq!(psw.is_bit_set(2), true);
        assert_eq!(psw.is_bit_set(7), true);
        assert_eq!(psw.is_bit_set(6), false);
    }

    #[test]
    pub fn orl_data_const() {
        let mut cpu = CPU::init();

        cpu.data.write(0x4f, 0xdb);

        cpu.run_instruction(Instruction::ORL_DATA_CONST, 0x4f, 0xf2);

        assert_eq!(cpu.data.read(0x4f), 0xdb | 0xf2);
    }

    #[test]
    fn subb() {
        let mut cpu = CPU::init();

        cpu.data.set_sfr_reg(SFR::ACC, 0xc9);
        cpu.data.set_gpr_reg(Register::R2, 0x54);
        cpu.data.set_sfr_reg(SFR::PSW, 0x80);

        cpu.run_instruction(Instruction::SUBB_A_R2, 0, 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::ACC), 0x74);

        let psw = cpu.data.get_sfr_reg(SFR::PSW);

        assert_eq!(psw.is_bit_set(2), true);
        assert_eq!(psw.is_bit_set(6), false);
        assert_eq!(psw.is_bit_set(7), false);
    }

    #[test]
    fn sjmp() {
        let mut cpu = CPU::init();

        cpu.pc = 0x0100;
        
        cpu.run_instruction(Instruction::SJMP, 0xfe - 153, 0);

        assert_eq!(cpu.pc, 0x0099);
    }

    #[test]
    fn push_data() {
        let mut cpu = CPU::init();

        cpu.data.set_sfr_reg(SFR::SP, 0x09);
        cpu.data.set_sfr_reg(SFR::DPH, 0x01);
        cpu.data.set_sfr_reg(SFR::DPL, 0x23);

        cpu.run_instruction(Instruction::PUSH_DATA, SFR::DPL.into(), 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x0a);
        assert_eq!(cpu.data.read(0x0a), 0x23);

        cpu.run_instruction(Instruction::PUSH_DATA, SFR::DPH.into(), 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x0b);
        assert_eq!(cpu.data.read(0x0b), 0x01);
    }

    #[test]
    fn pop_data() {
        let mut cpu = CPU::init();

        cpu.data.write(0x30, 0x20);
        cpu.data.write(0x31, 0x23);
        cpu.data.write(0x32, 0x01);

        cpu.data.set_sfr_reg(SFR::SP, 0x32);

        cpu.run_instruction(Instruction::POP_DATA, SFR::DPH.into(), 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x31);
        assert_eq!(cpu.data.get_sfr_reg(SFR::DPH), 0x01);

        cpu.run_instruction(Instruction::POP_DATA, SFR::DPL.into(), 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x30);
        assert_eq!(cpu.data.get_sfr_reg(SFR::DPL), 0x23);

        cpu.run_instruction(Instruction::POP_DATA, SFR::SP.into(), 0);

        assert_eq!(cpu.data.get_sfr_reg(SFR::SP), 0x20);
    }

    #[test]
    fn djnz() {
        let mut cpu = CPU::init();

        cpu.data.write(0x40, 0x01);
        cpu.data.write(0x50, 0x70);
        cpu.data.write(0x60, 0x15);

        cpu.run_instruction(Instruction::DJNZ_DATA_CODE, 1, 1);
        assert_eq!(cpu.pc, 2);
        cpu.run_instruction(Instruction::DJNZ_DATA_CODE, 1, 1);
        assert_eq!(cpu.pc, 5);

        assert_eq!(cpu.data.read(0x40), 0x00);
        assert_eq!(cpu.data.read(0x50), 0x69);
        assert_eq!(cpu.data.read(0x60), 0x15);
    }
}
