#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod vcslib;

#[unsafe(no_mangle)]
pub extern "C" fn elf_main(args: *mut u32) -> i32 {
    let argSystemType: u32 = unsafe {*args}; 

    if argSystemType <= 2 {
        startWaitSpinner2600();
    } else if argSystemType >= 3 && argSystemType <= 4 {
        startWaitSpinner7800();
    }

    loop {}
}

const WAIT_SPINNER_GRAPHICS: [u8; 32] = [
    0, 0b00011000, 0b00000100, 0b00010010, 0b10111010, 0b10010000, 0b01000000, 0b00110000,
    0, 0b00111000, 0b01000100, 0b00010000, 0b00111000, 0b00010000, 0b01000100, 0b00111000,
    0, 0b00110000, 0b01000000, 0b10010000, 0b10111010, 0b00010010, 0b00000100, 0b00011000,
    0, 0b00000000, 0b01000100, 0b10010010, 0b10111010, 0b10010010, 0b01000100, 0b00000000];

const WAIT26BIN: [u8; 92] = [
0xf0, 0xa2, 0xff, 0x9a, 0xa9, 0x0e, 0x85, 0x02, 0x85, 0x00, 0x4a, 0xd0, 0xf9, 0xa2, 0x25, 0x20,
0xf6, 0x00, 0x86, 0x01, 0xa2, 0x16, 0x20, 0xf6, 0x00, 0xe6, 0xa0, 0xa5, 0xa0, 0x0a, 0xa2, 0x18,
0xc9, 0x18, 0xb0, 0x03, 0x29, 0x18, 0xaa, 0x86, 0xce, 0xa2, 0x87, 0xa0, 0x06, 0xb5, 0x00, 0x85,
0x02, 0x85, 0x0e, 0x88, 0x10, 0xf9, 0xca, 0xe0, 0x80, 0x10, 0xf0, 0xa2, 0x7f, 0x20, 0xf6, 0x00,
0xa9, 0x02, 0x85, 0x01, 0xa2, 0x10, 0x20, 0xf6, 0x00, 0xad, 0xfe, 0x1f, 0x4d, 0xff, 0x1f, 0xc9,
0xff, 0xd0, 0xb1, 0x4c, 0x00, 0xf0, 0x85, 0x02, 0xca, 0xd0, 0xfb, 0x60];

const WAIT26BIN_ARG_START_WAIT: u16 = 0x00a1;
const WAIT26BIN_ARG_START_SPINNER: u16 = 0x0080;

fn startWaitSpinner2600() {
    vcslib::Jmp3();

	for i in 0..128 {
		vcslib::Write5(i, 0);
	}

	vcslib::Jmp3();
	vcslib::Sta3(vcslib::WSYNC);
	vcslib::Ldx2(0xff);
	vcslib::Txs2();
	vcslib::Sta3(vcslib::RESP1);
	vcslib::Write5(vcslib::COLUPF, 0x0a);
	vcslib::Write5(vcslib::COLUBK, 0x82);
	vcslib::Write5(vcslib::COLUP0, 0x82);
	vcslib::Write5(vcslib::GRP0, 0xff);
	vcslib::Write5(vcslib::NUSIZ0, 0x07);
	vcslib::Write5(vcslib::GRP1, 0xff);
	vcslib::Write5(vcslib::COLUP1, 0);
	vcslib::Write5(vcslib::HMP1, 0x30);
	vcslib::Nop2();
	vcslib::Sta3(vcslib::RESP0);

	vcslib::Sta3(vcslib::WSYNC);
	vcslib::Sta3(vcslib::HMOVE);

	for i in 0..32 {
		vcslib::Write5(WAIT26BIN_ARG_START_SPINNER + i, WAIT_SPINNER_GRAPHICS[i as usize]);
	}

	vcslib::Jmp3();

	for i in 0..WAIT26BIN.len() {
		vcslib::Write5(0xa0 + (i as u16), WAIT26BIN[i as usize]);
	}

	vcslib::JmpToRam3(WAIT26BIN_ARG_START_WAIT);
}

const WAIT78BIN_ARG_DISPLAY_LIST: u16 = 0x2400;
const WAIT78BIN_ARG_DISPLAY_LIST_LIST: u16 = 0x2300;
const WAIT78BIN_ARG_GRAPHICS: u16 = 0x2500;
const WAIT78BIN_ARG_KERNEL: u16 = 0x2200;

const WAIT78BIN: [u8; 121] = [
0xd8, 0xa9, 0x54, 0x85, 0x20, 0xa9, 0x23, 0x85, 0x2c, 0xa9, 0x00, 0x85, 0x30, 0x85, 0x24, 0xa5,
0x28, 0x10, 0xfa, 0xad, 0xfe, 0x1f, 0x4d, 0xff, 0x1f, 0xc9, 0xff, 0xd0, 0x07, 0xa9, 0x60, 0x85,
0x3c, 0x4c, 0x00, 0xf0, 0xa9, 0x0a, 0x85, 0x21, 0x18, 0xad, 0x00, 0x18, 0x69, 0x01, 0x8d, 0x00,
0x18, 0x29, 0xf0, 0x85, 0x20, 0xa9, 0x40, 0x85, 0x3c, 0xee, 0x78, 0x22, 0xad, 0x78, 0x22, 0x0a,
0xa2, 0x18, 0xc9, 0x18, 0xb0, 0x03, 0x29, 0x18, 0xaa, 0x8a, 0x0a, 0x0a, 0x0a, 0x18, 0x8d, 0x00,
0x24, 0x69, 0x08, 0x8d, 0x07, 0x24, 0x69, 0x08, 0x8d, 0x0e, 0x24, 0x69, 0x08, 0x8d, 0x15, 0x24,
0x69, 0x08, 0x8d, 0x1c, 0x24, 0x69, 0x08, 0x8d, 0x23, 0x24, 0x69, 0x08, 0x8d, 0x2a, 0x24, 0x85,
0x24, 0xa5, 0x28, 0x30, 0xfa, 0x4c, 0x0d, 0x22, 0xf0];

fn startWaitSpinner7800() {
    vcslib::Jmp3();

	// to make spinner quad wide each bit becomes a byte in 160A mode
	// 32 bytes becomes a full 256 byte page of graphics
    let mut baseAddress = WAIT78BIN_ARG_GRAPHICS;
    for i in 0..4 {
        for j in 0..8 {
            for k in 0..8 {
                if (WAIT_SPINNER_GRAPHICS[i * 8 + (7 - j)] >> (7 - k)) & 0x01 == 0x01 {
                    vcslib::Write6(baseAddress, 0x55);
                } else {
                    vcslib::Write6(baseAddress, 0x00);
                }
                baseAddress += 1;
            }
        }
    }

    vcslib::Jmp3();

    // generate the DLL
    baseAddress = WAIT78BIN_ARG_DISPLAY_LIST_LIST;      // offset
    baseAddress = writePadding(baseAddress);            // top padding
    baseAddress = writePadding(baseAddress);
    for i in 0..7 {
        for j in 0..6 {
            vcslib::Write6(baseAddress, 0);                                             // zone height = 1
            vcslib::Write6(baseAddress+1, (WAIT78BIN_ARG_DISPLAY_LIST >> 8) as u8);     // high
            vcslib::Write6(baseAddress+2, i * 7);                                       // low 
            baseAddress += 3;
        }
    }

    // bottom padding
    for i in 0..19 {
        baseAddress = writePadding(baseAddress);
    }

    vcslib::Jmp3();

    // generate display lists
    baseAddress = WAIT78BIN_ARG_DISPLAY_LIST;
    for i in 0..7 {
        // 5 byte header
        vcslib::Write6(baseAddress, i * 8);                                     // lowvector
        vcslib::Write6(baseAddress+1, 0x40);                                    // 160A direct
        vcslib::Write6(baseAddress+2, (WAIT78BIN_ARG_GRAPHICS >> 8) as u8);     // high
        vcslib::Write6(baseAddress+3, 0x18);                                    // palette=P0, width=8bytes
        vcslib::Write6(baseAddress+4, 16);                                      // position

        // end DL
        vcslib::Write6(baseAddress+5, 0);
        vcslib::Write6(baseAddress+6, 0);

        baseAddress += 7;
    }

    vcslib::Jmp3();

    // copy kernel
    for i in 0..WAIT78BIN.len() {
        vcslib::Write6(WAIT78BIN_ARG_KERNEL+(i as u16), WAIT78BIN[i as usize]);
    }

    vcslib::JmpToRam3(WAIT78BIN_ARG_KERNEL);
}

fn writePadding(address: u16) -> u16 {
    vcslib::Write6(address, 0x0f);
    vcslib::Write6(address+1, (WAIT78BIN_ARG_DISPLAY_LIST >> 8) as u8);
    vcslib::Write6(address+2, 0x05);
    return address + 3;
}
