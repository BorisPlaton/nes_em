use crate::cpu::bus::CPUBusOperation;
use crate::cpu::cpu::CPU;
use crate::cpu::error::UnknownOpCode;
use crate::cpu::opcode::{AddressingMode, OPCODES, OpCode};

const NON_READABLE_ADDRESSES: [u16; 11] = [
    0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x4014, 0x4016, 0x4017,
];

pub fn trace(cpu: &mut CPU) -> String {
    let program_counter = cpu.program_counter.get();
    let raw_opcode = cpu.bus.read(program_counter);
    let opcode = OPCODES
        .get(&raw_opcode)
        .ok_or(UnknownOpCode(raw_opcode))
        .unwrap();

    let mut hex_dump = vec![raw_opcode];

    let (mem_addr, stored_value) = match opcode.mode {
        AddressingMode::Immediate
        | AddressingMode::Accumulator
        | AddressingMode::Implied
        | AddressingMode::Relative => (0, 0),
        _ => {
            let (_, addr) = cpu.get_operand_address(&opcode.mode, program_counter + 1);

            if !NON_READABLE_ADDRESSES.contains(&addr) {
                (addr, cpu.bus.read(addr))
            } else {
                (addr, 0u8)
            }
        }
    };

    let tmp = match opcode.mode.operand_bytes() {
        0 => match opcode.mode {
            AddressingMode::Accumulator => "A ".to_string(),
            _ => "".to_string(),
        },
        1 => {
            let address: u8 = cpu.bus.read(program_counter + 1);
            hex_dump.push(address);

            match opcode.mode {
                AddressingMode::Immediate => format!("#${:02x}", address),
                AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                AddressingMode::ZeroPageX => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::ZeroPageY => format!(
                    "${:02x},Y @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::IndexedIndirectX => format!(
                    "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    address,
                    address.wrapping_add(cpu.register_x.get()),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::IndirectIndexedY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    mem_addr.wrapping_sub(cpu.register_y.get() as u16),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::Accumulator
                | AddressingMode::Implied
                | AddressingMode::Relative => {
                    let address: usize =
                        (program_counter as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", address)
                }
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {}",
                    opcode.mode, opcode.opcode
                ),
            }
        }
        2 => {
            let address_lo = cpu.bus.read(program_counter + 1);
            let address_hi = cpu.bus.read(program_counter + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.bus.read(program_counter + 1);

            match (&opcode.opcode, &opcode.mode) {
                (_, AddressingMode::Indirect) => {
                    let jmp_addr = if address & 0x00FF == 0x00FF {
                        let lo: u8 = cpu.bus.read(address);
                        let hi: u8 = cpu.bus.read(address & 0xFF00);
                        (hi as u16) << 8 | (lo as u16)
                    } else {
                        cpu.bus.read(address)
                    };
                    format!("(${address:04x}) = {jmp_addr:04x}")
                }
                (
                    _,
                    AddressingMode::Accumulator
                    | AddressingMode::Implied
                    | AddressingMode::Relative,
                ) => {
                    format!("${:04x}", address)
                }
                (OpCode::JMP | OpCode::JSR, AddressingMode::Absolute) => {
                    format!("${:04x}", mem_addr)
                }
                (_, AddressingMode::Absolute) => {
                    format!("${:04x} = {:02x}", mem_addr, stored_value)
                }
                (_, AddressingMode::AbsoluteX) => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                (_, AddressingMode::AbsoluteY) => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {}",
                    opcode.mode, opcode.opcode
                ),
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!(
        "{:04x}  {:8} {: >4} {}",
        program_counter,
        hex_str,
        match raw_opcode {
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => "*NOP".to_string(),
            0xEB => "*SBC".to_string(),
            _ => opcode.opcode.to_string(),
        },
        tmp
    )
    .trim()
    .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{:3},{:3} CYC:{}",
        asm_str,
        cpu.accumulator.get(),
        cpu.register_x.get(),
        cpu.register_y.get(),
        cpu.status.get(),
        cpu.stack.get_pointer(),
        cpu.bus.ppu.scanline,
        cpu.bus.ppu.cycles,
        cpu.bus.cycles,
    )
    .to_ascii_uppercase()
}
