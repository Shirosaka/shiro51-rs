use crate::lib::{
    cpu::{PC, CPU},
    instruction::Instruction,
    memory::registers::{Register, SFR},
    ops::arithmetics::BitOps,
};

fn init_cpu(pc: usize) -> CPU {
    let mut cpu = CPU::init();

    cpu.set_pc(pc);

    cpu
}

#[test]
fn ajmp() {
    let mut cpu = init_cpu(0x0345);

    let res = cpu.run_instruction_test(Instruction::AJMP2, 0x23, 0);

    assert_eq!(cpu.get_pc(), 0x0123);
    assert_eq!(res, PC::Handled);
}

#[test]
fn ljmp() {
    let mut cpu = init_cpu(0x0000);

    cpu.run_instruction_test(Instruction::LJMP, 0x12, 0x34);

    assert_eq!(cpu.get_pc(), 0x1234);
}

#[test]
fn jb_bit_code() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().set_sfr_reg(SFR::P1, 0b11001010);
    cpu.get_memory().set_sfr_reg(SFR::ACC, 0b01010110);

    let p1addr: u8 = SFR::P1.into();
    let accaddr: u8 = SFR::ACC.into();

    cpu.run_instruction_test(Instruction::JB_BIT_CODE, p1addr + 2, 0x2);
    assert_eq!(cpu.get_pc(), 3);
    cpu.run_instruction_test(Instruction::JB_BIT_CODE, accaddr + 2, 0x2);
    assert_eq!(cpu.get_pc(), 8);
}

#[test]
fn jnb_bit_addr() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().set_sfr_reg(SFR::P1, 0b11001010);
    cpu.get_memory().set_sfr_reg(SFR::ACC, 0b01010110);

    let p1addr: u8 = SFR::P1.into();
    let accaddr: u8 = SFR::ACC.into();

    cpu.run_instruction_test(Instruction::JNB_BIT_CODE, p1addr + 3, 0x2);
    assert_eq!(cpu.get_pc(), 3);
    cpu.run_instruction_test(Instruction::JNB_BIT_CODE, accaddr + 3, 0x2);
    assert_eq!(cpu.get_pc(), 8);
}

#[test]
fn acall() {
    let mut cpu = init_cpu(0x0123);

    cpu.get_memory().set_sfr_reg(SFR::SP, 0x07);

    cpu.run_instruction_test(Instruction::ACALL4, 0x45, 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x09);
    assert_eq!(cpu.get_memory().read(0x08), 0x25);
    assert_eq!(cpu.get_memory().read(0x09), 0x01);
    assert_eq!(cpu.get_pc(), 0x0345);
}

#[test]
fn ret() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().set_sfr_reg(SFR::SP, 0x0b);
    cpu.get_memory().write(0x0a, 0x23);
    cpu.get_memory().write(0x0b, 0x01);

    cpu.run_instruction_test(Instruction::RET, 0, 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x09);
    assert_eq!(cpu.get_pc(), 0x0123);
}

#[test]
fn addc_a_r2() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().set_sfr_reg(SFR::ACC, 0xc3);
    cpu.get_memory().set_gpr_reg(Register::R2, 0xaa);
    cpu.get_memory().set_sfr_reg(SFR::PSW, 0x80);

    cpu.run_instruction_test(Instruction::ADDC_A_R2, 0, 0);

    let psw = cpu.get_memory().get_sfr_reg(SFR::PSW);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::ACC), 0x6e);
    assert_eq!(psw.is_bit_set(2), true);
    assert_eq!(psw.is_bit_set(7), true);
    assert_eq!(psw.is_bit_set(6), false);
}

#[test]
pub fn orl_data_const() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().write(0x4f, 0xdb);

    cpu.run_instruction_test(Instruction::ORL_DATA_CONST, 0x4f, 0xf2);

    assert_eq!(cpu.get_memory().read(0x4f), 0xdb | 0xf2);
}

#[test]
fn subb() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().set_sfr_reg(SFR::ACC, 0xc9);
    cpu.get_memory().set_gpr_reg(Register::R2, 0x54);
    cpu.get_memory().set_sfr_reg(SFR::PSW, 0x80);

    cpu.run_instruction_test(Instruction::SUBB_A_R2, 0, 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::ACC), 0x74);

    let psw = cpu.get_memory().get_sfr_reg(SFR::PSW);

    assert_eq!(psw.is_bit_set(2), true);
    assert_eq!(psw.is_bit_set(6), false);
    assert_eq!(psw.is_bit_set(7), false);
}

#[test]
fn sjmp() {
    let mut cpu = init_cpu(0x0100);

    cpu.run_instruction_test(Instruction::SJMP, 2, 0);

    assert_eq!(cpu.get_pc(), 0x0104);
}

#[test]
fn push_data() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().set_sfr_reg(SFR::SP, 0x09);
    cpu.get_memory().set_sfr_reg(SFR::DPH, 0x01);
    cpu.get_memory().set_sfr_reg(SFR::DPL, 0x23);

    cpu.run_instruction_test(Instruction::PUSH_DATA, SFR::DPL.into(), 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x0a);
    assert_eq!(cpu.get_memory().read(0x0a), 0x23);

    cpu.run_instruction_test(Instruction::PUSH_DATA, SFR::DPH.into(), 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x0b);
    assert_eq!(cpu.get_memory().read(0x0b), 0x01);
}

#[test]
fn pop_data() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().write(0x30, 0x20);
    cpu.get_memory().write(0x31, 0x23);
    cpu.get_memory().write(0x32, 0x01);

    cpu.get_memory().set_sfr_reg(SFR::SP, 0x32);

    cpu.run_instruction_test(Instruction::POP_DATA, SFR::DPH.into(), 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x31);
    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::DPH), 0x01);

    cpu.run_instruction_test(Instruction::POP_DATA, SFR::DPL.into(), 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x30);
    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::DPL), 0x23);

    cpu.run_instruction_test(Instruction::POP_DATA, SFR::SP.into(), 0);

    assert_eq!(cpu.get_memory().get_sfr_reg(SFR::SP), 0x20);
}

#[test]
fn djnz() {
    let mut cpu = init_cpu(0x0000);

    cpu.get_memory().write(0x40, 0x01);
    cpu.get_memory().write(0x50, 0x70);
    cpu.get_memory().write(0x60, 0x15);

    cpu.run_instruction_test(Instruction::DJNZ_DATA_CODE, 0x40, 1);
    assert_eq!(cpu.get_pc(), 2);
    cpu.run_instruction_test(Instruction::DJNZ_DATA_CODE, 0x50, 1);
    assert_eq!(cpu.get_pc(), 5);

    assert_eq!(cpu.get_memory().read(0x40), 0x00);
    assert_eq!(cpu.get_memory().read(0x50), 0x6f);
    assert_eq!(cpu.get_memory().read(0x60), 0x15);
}
