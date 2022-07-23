use shiro51_util::error::{ErrorType, Result, RuntimeError};

use super::instruction::Instruction;
use crate::addr::Addr8;
use crate::cpu::{CPU, PC};
use crate::registers::{GPR, SFR};

pub fn insn_rl_a(
    cpu: &mut CPU,
    _insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    cpu.write(SFR::ACC.addr(), cpu.read(SFR::ACC.addr()) << 1);
    Ok(PC::ADVANCE)
}

pub fn insn_rr_a(
    cpu: &mut CPU,
    _insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    cpu.write(SFR::ACC.addr(), cpu.read(SFR::ACC.addr()) >> 1);
    Ok(PC::ADVANCE)
}

pub fn insn_orl_data_const(
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

    let addr = Addr8::new(arg0.unwrap());
    cpu.write(addr, cpu.read(addr) | arg1.unwrap());

    Ok(PC::ADVANCE)
}

pub fn insn_orl_a_rn(
    cpu: &mut CPU,
    insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let rn = insn.op() - Instruction::ORL_A_R0.op();
    let reg = cpu.read(GPR::from(rn).addr());
    let acc_addr = SFR::ACC.addr();

    cpu.write(acc_addr, cpu.read(acc_addr) | reg);

    Ok(PC::ADVANCE)
}

pub fn insn_anl_a_rn(
    cpu: &mut CPU,
    insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let rn = insn.op() - Instruction::ANL_A_R0.op();
    let reg = cpu.read(GPR::from(rn).addr());
    let acc_addr = SFR::ACC.addr();

    cpu.write(acc_addr, cpu.read(acc_addr) & reg);

    Ok(PC::ADVANCE)
}
