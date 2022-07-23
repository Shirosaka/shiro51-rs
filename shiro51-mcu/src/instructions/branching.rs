use log::debug;
use shiro51_util::error::{ErrorType, Result, RuntimeError};

use super::instruction::Instruction;
use crate::addr::{addr16, Addr8, BitAddr};
use crate::cpu::{CPU, PC};
use crate::registers::SFR;

pub fn insn_acall(
    cpu: &mut CPU,
    insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    if arg0.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg0Missing));
    }

    cpu.pc += 2;

    let mut sp_addr = Addr8::new(cpu.read(SFR::SP.addr()));

    sp_addr += 1;
    cpu.write(sp_addr, (cpu.pc & 0x00FF).as_u16() as u8);
    sp_addr += 1;
    cpu.write(sp_addr, ((cpu.pc & 0xFF00) >> 8).as_u16() as u8);

    cpu.write(SFR::SP.addr(), sp_addr.as_u8());

    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    cpu.pc &= 0xF800;
    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    cpu.pc |= ((insn.op() & 0xe0) as u16) << 3;
    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    cpu.pc |= arg0.unwrap() as u16;
    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);

    Ok(PC::JUMP)
}

pub fn insn_lcall(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    arg1: Option<u8>,
) -> Result<PC> {
    if arg0.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg0Missing));
    }

    if arg1.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg1Missing));
    }

    cpu.pc += 3;

    let mut sp_addr = Addr8::new(cpu.read(SFR::SP.addr()));

    sp_addr += 1;
    cpu.write(sp_addr, (cpu.pc & 0x00FF).as_u16() as u8);
    sp_addr += 1;
    cpu.write(sp_addr, ((cpu.pc & 0xFF00) >> 8).as_u16() as u8);

    cpu.write(SFR::SP.addr(), sp_addr.as_u8());

    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    cpu.pc = addr16(Addr8::new(arg0.unwrap()), Addr8::new(arg1.unwrap()));
    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);

    Ok(PC::JUMP)
}

pub fn insn_ajmp(
    cpu: &mut CPU,
    insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    if arg0.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg0Missing));
    }

    cpu.pc += 2;

    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    cpu.pc &= 0xF800;
    cpu.pc |= ((insn.op() & 0xe0) as u16) << 3;
    cpu.pc |= arg0.unwrap() as u16;
    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    Ok(PC::JUMP)
}

pub fn insn_ljmp(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    arg1: Option<u8>,
) -> Result<PC> {
    if arg0.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg0Missing));
    }

    if arg1.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg1Missing));
    }

    cpu.pc = addr16(Addr8::new(arg0.unwrap()), Addr8::new(arg1.unwrap()));
    debug!("PC: {:#06x} ({:#018b})", cpu.pc, cpu.pc);
    Ok(PC::JUMP)
}

pub fn insn_jb_bit_code(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    arg1: Option<u8>,
) -> Result<PC> {
    if arg0.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg0Missing));
    }

    if arg1.is_none() {
        return Err(RuntimeError::new(ErrorType::InstructionArg1Missing));
    }

    cpu.pc += 3;

    let bit = cpu.read_bit(BitAddr::try_new(arg0.unwrap())?)?;


    if bit {
        cpu.pc += arg1.unwrap() as u16;
    }

    Ok(PC::JUMP)
}

pub fn insn_nop(
    _cpu: &mut CPU,
    _insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    Ok(PC::ADVANCE)
}

// #[cfg(test)]
// mod branching_tests {
//     use super::*;

//     #[test]
//     fn test_acall() {
//         todo!()
//     }

//     #[test]
//     fn test_lcall() {
//         let mut cpu = CPU::init();

//         cpu.pc = 0x0123;

//         let res = insn_lcall(&mut cpu, Instruction::LCALL, 0x12, 0x34).unwrap();

//         assert_eq!(res, PC::HANDLED);
//         assert_eq!(cpu.pc, 0x1234);
//         assert_eq!(cpu.read_sfr(SFR::SP), 0x09);
//         assert_eq!(cpu.read(0x08), 0x26);
//         assert_eq!(cpu.read(0x09), 0x01);
//     }
// }
