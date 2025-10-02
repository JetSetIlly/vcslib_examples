#![allow(non_snake_case)]
#![allow(dead_code)]

// defines for VCS/2600 memory mapped registers
pub const VSYNC: u16 = 0x00;
pub const VBLANK: u16 = 0x01;
pub const WSYNC: u16 = 0x02;
pub const RSYNC: u16 = 0x03;
pub const NUSIZ0: u16 = 0x04;
pub const NUSIZ1: u16 = 0x05;
pub const COLUP0: u16 = 0x06;
pub const COLUP1: u16 = 0x07;
pub const COLUPF: u16 = 0x08;
pub const COLUBK: u16 = 0x09;
pub const CTRLPF: u16 = 0x0a;
pub const REFP0: u16 = 0x0b;
pub const REFP1: u16 = 0x0c;
pub const PF0: u16 = 0x0d;
pub const PF1: u16 = 0x0e;
pub const PF2: u16 = 0x0f;
pub const RESP0: u16 = 0x10;
pub const RESP1: u16 = 0x11;
pub const RESM0: u16 = 0x12;
pub const RESM1: u16 = 0x13;
pub const RESBL: u16 = 0x14;
pub const AUDC0: u16 = 0x15;
pub const AUDC1: u16 = 0x16;
pub const AUDF0: u16 = 0x17;
pub const AUDF1: u16 = 0x18;
pub const AUDV0: u16 = 0x19;
pub const AUDV1: u16 = 0x1a;
pub const GRP0: u16 = 0x1b;
pub const GRP1: u16 = 0x1c;
pub const ENAM0: u16 = 0x1d;
pub const ENAM1: u16 = 0x1e;
pub const ENABL: u16 = 0x1f;
pub const HMP0: u16 = 0x20;
pub const HMP1: u16 = 0x21;
pub const HMM0: u16 = 0x22;
pub const HMM1: u16 = 0x23;
pub const HMBL: u16 = 0x24;
pub const VDELP0: u16 = 0x25;
pub const VDELP1: u16 = 0x26;
pub const VDELBL: u16 = 0x27;
pub const RESMP0: u16 = 0x28;
pub const RESMP1: u16 = 0x29;
pub const HMOVE: u16 = 0x2a;
pub const HMCLR: u16 = 0x2b;
pub const CXCLR: u16 = 0x2c;

unsafe extern "C" {
    fn vcsJmp3();
    fn vcsNop2();
    fn vcsWrite5(zeropage: u16, data: u8);
    fn vcsWrite6(address: u16, data: u8);
    fn vcsLdx2(data: u8);
    fn vcsSta3(address: u16);
    fn vcsTxs2();
    fn vcsJmpToRam3(address: u16);
}

#[inline(always)]
pub fn Jmp3() {
    unsafe { vcsJmp3() }
}

#[inline(always)]
pub fn Nop2() {
    unsafe { vcsNop2() }
}

#[inline(always)]
pub fn Write5(zeropage: u16, data: u8) {
    unsafe { vcsWrite5(zeropage, data) }
}

#[inline(always)]
pub fn Write6(address: u16, data: u8) {
    unsafe { vcsWrite6(address, data) }
}

#[inline(always)]
pub fn Ldx2(data: u8) {
    unsafe { vcsLdx2(data) }
}

#[inline(always)]
pub fn Sta3(address: u16) {
    unsafe { vcsSta3(address) }
}

#[inline(always)]
pub fn Txs2() {
    unsafe { vcsTxs2() }
}

#[inline(always)]
pub fn JmpToRam3(address: u16) {
    unsafe { vcsJmpToRam3(address) }
}
