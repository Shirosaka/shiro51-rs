use bitflags::bitflags;
use log::debug;
use shiro51_util::bit::Bit;
use shiro51_util::error::Result;

use super::instruction::Instruction;
use crate::addr::Addr8;
use crate::cpu::{CPU, PC};
use crate::registers::{GPR, SFR};

bitflags! {
    pub struct ArithmeticOpFlags: u8 {
        const OVERFLOW = 1 << 0;
        const AUXILIARY_CARRY = 1 << 1;
        const CARRY = 1 << 2;
    }
}

pub fn add(lhs: &mut u8, rhs: u8) -> ArithmeticOpFlags {
    let mut flags = ArithmeticOpFlags::empty();
    let mut carry = false;
    let mut carry_hist = Bit::empty();

    debug!("=== ADD ===");

    for n in 0..=7 {
        let n_shl = 1 << n;
        let n_shl_prev = if n == 0 { 1 << 0 } else { 1 << (n - 1) };
        let lhs_n = *lhs & n_shl;
        let rhs_n = rhs & n_shl;

        debug!("pre {}:\n- n_shl = {:#010b}\n- n_shl_prev = {:#010b}\n- lhs = {:#010b}\n- lhs_n = {:#010b}\n- rhs = {:#010b}\n- rhs_n = {:#010b}\n- carry = {}", n, n_shl, n_shl_prev, *lhs, lhs_n, rhs, rhs_n, carry);
        // let last_carry = CarryHistory::from_bits(if n == 0 { 1 << 0 } else { 1 << (n - 1) }).unwrap();

        debug!("lhs_n & rhs_n == n_shl: {}", lhs_n & rhs_n == n_shl);
        debug!("(*lhs & n_shl) != 0 && carry: {}", (*lhs & n_shl) != 0 && carry);
        debug!("(*lhs & n_shl_prev) != 0 && carry: {}", (*lhs & n_shl_prev) != 0 && carry);
        debug!("(rhs & n_shl) != 0 && carry: {}", (rhs & n_shl) != 0 && carry);
        debug!("(rhs & n_shl_prev) != 0 && carry: {}", (rhs & n_shl_prev) != 0 && carry);

        if lhs_n & rhs_n == n_shl {
            debug!("carry-out");

            // if !carry_hist.contains(last_carry) {
            if !carry {
                debug!("!carry");
                *lhs &= !n_shl;
            } else {
                debug!("carry");
                *lhs |= n_shl;
            }

            carry = true;
            carry_hist.set(Bit::from_bits(1 << n).unwrap(), true);
        } else if ((*lhs & n_shl) != 0 && carry) || ((rhs & n_shl) != 0 && carry) {
            debug!("previous carry-out with continuation");
            *lhs &= !n_shl;
            carry = true;
            carry_hist.set(Bit::from_bits(1 << n).unwrap(), true);
        } else if ((*lhs & n_shl_prev) != 0 && carry) || ((rhs & n_shl_prev) != 0 && carry) {
            debug!("previous carry-out without continuation");
            *lhs |= n_shl;
            carry = false;
            carry_hist.set(Bit::from_bits(1 << n).unwrap(), false);
        } else {
            debug!("no carry-out");
            if carry && (lhs_n == 0 && rhs_n == 0) {
                *lhs |= n_shl;
            } else {
                *lhs |= rhs_n;
            }

            carry = false;
            carry_hist.set(Bit::from_bits(1 << n).unwrap(), false);
        }
        debug!("post {}:\n- n_shl = {:#010b}\n- n_shl_prev = {:#010b}\n- lhs = {:#010b}\n- lhs_n = {:#010b}\n- rhs = {:#010b}\n- rhs_n = {:#010b}\n- carry = {}", n, n_shl, n_shl_prev, *lhs, lhs_n, rhs, rhs_n, carry);
    }

    flags.set(ArithmeticOpFlags::CARRY, carry_hist.contains(Bit::B7));
    flags.set(ArithmeticOpFlags::AUXILIARY_CARRY, carry_hist.contains(Bit::B3));
    flags.set(
        ArithmeticOpFlags::OVERFLOW,
        carry_hist.contains(Bit::B6) && !carry_hist.contains(Bit::B7)
            || !carry_hist.contains(Bit::B6) && carry_hist.contains(Bit::B7),
    );

    debug!("signed: {}; unsigned: {}", *lhs as i8, *lhs);
    debug!("carry_hist: {:?}", carry_hist);
    debug!("flags: {:?}", flags);

    flags
}

#[allow(unused)]
fn subb(lhs: &mut u8, rhs: u8) -> ArithmeticOpFlags {
    let mut flags = ArithmeticOpFlags::empty();
    let mut borrow = false;
    let mut borrow_hist = Bit::empty();

    debug!("=== SUBB ===");

    for n in 0..=7 {
        // for when we need to search for a borrow, maybe while loop?
        for i in n..=7 {}
    }

    flags.set(ArithmeticOpFlags::CARRY, borrow_hist.contains(Bit::B7));
    flags.set(ArithmeticOpFlags::AUXILIARY_CARRY, borrow_hist.contains(Bit::B3));
    flags.set(
        ArithmeticOpFlags::OVERFLOW,
        borrow_hist.contains(Bit::B6) && !borrow_hist.contains(Bit::B7)
            || !borrow_hist.contains(Bit::B6) && borrow_hist.contains(Bit::B7),
    );

    debug!("signed: {}; unsigned: {}", *lhs as i8, *lhs);
    debug!("borrow_hist: {:?}", borrow_hist);
    debug!("flags: {:?}", flags);

    flags
}

// fn mul(lhs: &mut u8, rhs: u8) -> ArithmeticOpFlags {
//     ArithmeticOpFlags::empty()
// }

// fn div(lhs: &mut u8, rhs: u8) -> ArithmeticOpFlags {
//     ArithmeticOpFlags::empty()
// }

pub fn insn_add_a_const(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let mut acc = cpu.read(SFR::ACC.addr());
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();

    let res = add(&mut acc, arg0.unwrap());

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());
    Ok(PC::ADVANCE)
}

pub fn insn_add_a_addr(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let mut acc = cpu.read(SFR::ACC.addr());
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();

    let res = add(&mut acc, cpu.read(Addr8::new(arg0.unwrap())));

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());

    Ok(PC::ADVANCE)
}

pub fn insn_add_a_rn(
    cpu: &mut CPU,
    insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let rn = insn.op() - Instruction::ADD_A_R0.op();
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();
    let mut acc = cpu.read(SFR::ACC.addr());

    let res = add(&mut acc, cpu.read(GPR::from(rn).addr()));

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());

    Ok(PC::ADVANCE)
}

pub fn insn_add_a_rn_indirect(
    cpu: &mut CPU,
    insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let rn = insn.op() - Instruction::ADD_A_INDIRECT_R0.op();
    let mut acc = cpu.read(SFR::ACC.addr());
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();

    let res = add(&mut acc, cpu.read(GPR::from(rn).addr()));

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());
    Ok(PC::ADVANCE)
}

pub fn insn_addc_a_const(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let mut acc = cpu.read(SFR::ACC.addr());
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();

    let res = add(&mut acc, arg0.unwrap());

    acc += 1;

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());
    Ok(PC::ADVANCE)
}

pub fn insn_addc_a_addr(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let mut acc = cpu.read(SFR::ACC.addr());
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();

    let res = add(&mut acc, cpu.read(Addr8::new(arg0.unwrap())));

    acc += 1;

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());

    Ok(PC::ADVANCE)
}

pub fn insn_addc_a_rn(
    cpu: &mut CPU,
    insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let rn = insn.op() - Instruction::ADDC_A_R0.op();
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();
    let mut acc = cpu.read(SFR::ACC.addr());

    let res = add(&mut acc, cpu.read(GPR::from(rn).addr()));

    acc += 1;

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());

    Ok(PC::ADVANCE)
}

pub fn insn_addc_a_rn_indirect(
    cpu: &mut CPU,
    insn: Instruction,
    _arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let rn = insn.op() - Instruction::ADDC_A_INDIRECT_R0.op();
    let mut acc = cpu.read(SFR::ACC.addr());
    let mut psw = Bit::from_bits(cpu.read(SFR::PSW.addr())).unwrap();

    let res = add(&mut acc, cpu.read(GPR::from(rn).addr()));

    acc += 1;

    if res.contains(ArithmeticOpFlags::CARRY) {
        psw.set(Bit::B7, true);
    } else {
        psw.set(Bit::B7, false);
    }

    if res.contains(ArithmeticOpFlags::AUXILIARY_CARRY) {
        psw.set(Bit::B6, true);
    } else {
        psw.set(Bit::B6, false);
    }

    if res.contains(ArithmeticOpFlags::OVERFLOW) {
        psw.set(Bit::B2, true);
    } else {
        psw.set(Bit::B2, false);
    }

    cpu.write(SFR::ACC.addr(), acc);
    cpu.write(SFR::PSW.addr(), psw.bits());
    Ok(PC::ADVANCE)
}

pub fn insn_inc_data(
    cpu: &mut CPU,
    _insn: Instruction,
    arg0: Option<u8>,
    _arg1: Option<u8>,
) -> Result<PC> {
    let addr = Addr8::new(arg0.unwrap());
    cpu.write(addr, {
        let mut data = cpu.read(addr);

        if data == 255 {
            data = 0;
        } else {
            data += 1;
        }

        data
    });

    Ok(PC::ADVANCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add1() {
        let mut val1 = 0xc3;
        let val2 = 0xaa;

        let res = add(&mut val1, val2);

        assert_eq!(val1, 0x6d);
        assert!(!res.contains(ArithmeticOpFlags::AUXILIARY_CARRY));
        assert!(res.contains(ArithmeticOpFlags::CARRY));
        assert!(res.contains(ArithmeticOpFlags::OVERFLOW));
    }

    #[test]
    fn test_add2() {
        let mut val1 = 0x35;
        let val2 = 0x19;

        let res = add(&mut val1, val2);

        assert_eq!(val1, 78);
        assert!(!res.contains(ArithmeticOpFlags::AUXILIARY_CARRY));
        assert!(!res.contains(ArithmeticOpFlags::CARRY));
        assert!(!res.contains(ArithmeticOpFlags::OVERFLOW));
    }

    #[test]
    fn test_add3() {
        let mut val1 = 0x35;
        let val2 = 0x5b;

        let res = add(&mut val1, val2);

        assert_eq!(val1, 0x90);
        assert!(res.contains(ArithmeticOpFlags::AUXILIARY_CARRY));
        assert!(!res.contains(ArithmeticOpFlags::CARRY));
        assert!(res.contains(ArithmeticOpFlags::OVERFLOW));
    }

    #[test]
    fn test_add4() {
        let mut val1 = 0x35;
        let val2 = 0xd3;

        let res = add(&mut val1, val2);

        assert_eq!(val1, 8);
        assert!(!res.contains(ArithmeticOpFlags::AUXILIARY_CARRY));
        assert!(res.contains(ArithmeticOpFlags::CARRY));
        assert!(!res.contains(ArithmeticOpFlags::OVERFLOW));
    }

    #[test]
    fn test_subb() {
        let mut val1 = 0b11001010;
        let val2 = 0b10011011;

        let _res = subb(&mut val1, val2);

        assert_eq!(val1, 0b00101111);
    }
}
