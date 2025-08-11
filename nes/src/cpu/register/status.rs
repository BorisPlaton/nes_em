use bitflags::bitflags;

// Status flags
// https://www.nesdev.org/wiki/Status_flags
//
// 7654 3210 bit
// ---- ----
// NV1B DIZC
// |||| ||||
// |||| |||+- Carry
// |||| ||+-- Zero
// |||| |+--- Interrupt Disable
// |||| +---- Decimal
// |||+------ (No CPU effect; see: the B flag)
// ||+------- (No CPU effect; always pushed as 1)
// |+-------- Overflow
// +--------- Negative
bitflags! {
    #[derive(Clone)]
    pub struct ProcessorStatus: u8 {
        const CARRY_FLAG = 0b0000_0001;
        const ZERO_FLAG = 0b0000_0010;
        const INTERRUPT_DISABLE_FLAG = 0b0000_0100;
        const DECIMAL_FLAG = 0b0000_1000;
        const B_FLAG = 0b0001_0000;
        const B_FLAG_2 = 0b0010_0000;
        const OVERFLOW_FLAG =  0b0100_0000;
        const NEGATIVE_FLAG = 0b1000_0000;
    }
}

impl ProcessorStatus {
    const INITIAL_STATE: u8 = 0b0010_0100;

    pub fn new() -> Self {
        ProcessorStatus::from_bits_retain(Self::INITIAL_STATE)
    }

    pub fn update(&mut self, value: u8) {
        *self = ProcessorStatus::from_bits_retain(value & 0b1110_1111 | Self::INITIAL_STATE);
    }

    pub fn reset(&mut self) {
        self.update(Self::INITIAL_STATE);
    }

    pub fn get(&self) -> u8 {
        self.bits()
    }

    pub fn get_carry_flag(&self) -> u8 {
        self.contains(ProcessorStatus::CARRY_FLAG) as u8
    }

    pub fn is_carry_flag_set(&self) -> bool {
        self.contains(ProcessorStatus::CARRY_FLAG)
    }

    pub fn is_zero_flag_set(&self) -> bool {
        self.contains(ProcessorStatus::ZERO_FLAG)
    }

    pub fn is_overflow_flag_set(&self) -> bool {
        self.contains(ProcessorStatus::OVERFLOW_FLAG)
    }

    pub fn is_negative_flag_set(&self) -> bool {
        self.contains(ProcessorStatus::NEGATIVE_FLAG)
    }

    pub fn set_carry_flag_to(&mut self, activate: bool) {
        self.set(ProcessorStatus::CARRY_FLAG, activate);
    }

    pub fn set_interrupt_disable_flag_to(&mut self, activate: bool) {
        self.set(ProcessorStatus::INTERRUPT_DISABLE_FLAG, activate);
    }

    pub fn set_decimal_mode_flag_to(&mut self, activate: bool) {
        self.set(ProcessorStatus::DECIMAL_FLAG, activate);
    }

    pub fn set_overflow_flag_to(&mut self, activate: bool) {
        self.set(ProcessorStatus::OVERFLOW_FLAG, activate);
    }

    pub fn set_negative_flag(&mut self, value: u8) {
        self.set(
            ProcessorStatus::NEGATIVE_FLAG,
            value & Self::NEGATIVE_FLAG.bits() != 0,
        );
    }

    pub fn set_zero_flag(&mut self, value: u8) {
        self.set(ProcessorStatus::ZERO_FLAG, value == 0);
    }
}
