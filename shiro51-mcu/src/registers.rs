use num_enum::{FromPrimitive, IntoPrimitive};

use crate::addr::Addr8;

#[derive(Debug, FromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SFR {
    #[default]
    NONE = 0x00, // Default
    ACC = 0xE0,      // Accumulator
    ADC0CF = 0xBC,   // ADC0 Configuration
    ADC0CN = 0xE8,   // ADC0 Control
    ADC0GTH = 0xC4,  // ADC0 Greater-Than Compare High
    ADC0GTL = 0xC3,  // ADC0 Greater-Than Compare Low
    ADC0H = 0xBE,    // ADC0 High
    ADC0L = 0xBD,    // ADC0 Low
    ADC0LTH = 0xC6,  // ADC0 Less-Than Compare Word High
    ADC0LTL = 0xC5,  // ADC0 Less-Than Compare Word Low
    AMX0N = 0xBA,    // AMUX0 Negative Channel Select,
    AMX0P = 0xBB,    // AMUX0 Positive Channel Select,
    B = 0xF0,        // B Register
    CKCON = 0x8E,    // Clock Control
    CLKMUL = 0xB9,   // Clock Multiplier
    CLKSEL = 0xA9,   // Clock Select
    CPT0CN = 0x9B,   // Comparator0 Control
    CPT0MD = 0x9D,   // Comparator0 Mode Selection
    CPT0MX = 0x9F,   // Comparator0 MUX Selection
    CPT1CN = 0x9A,   // Comparator1 Control
    CPT1MD = 0x9C,   // Comparator1 Mode Selection
    CPT1MX = 0x9E,   // Comparator1 MUX Selection
    DPH = 0x83,      // Data Pointer High
    DPL = 0x82,      // Data Pointer Low
    EIE1 = 0xE6,     // Extended Interrupt Enable 1
    EIE2 = 0xE7,     // Extended Interrupt Enable 2
    EIP1 = 0xF6,     // Extended Interrupt Priority 1
    EIP2 = 0xF7,     // Extended Interrupt Priority 2
    EMI0CN = 0xAA,   // External Memory Interface Control
    EMI0CF = 0x85,   // External Memory Interface Configuration
    EMI0TC = 0x84,   // External Memory Interface Timing
    FLKEY = 0xB7,    // Flash Lock and Key
    FLSCL = 0xB6,    // Flash Scale
    IE = 0xA8,       // Interrupt Enable
    IP = 0xB8,       // Interrupt Priority
    IT01CF = 0xE4,   // INT0/INT1 Configuration
    OSCICL = 0xB3,   // Internal Oscillator Calibration
    OSCICN = 0xB2,   // Internal Oscillator Control
    OSCLCN = 0x86,   // Internal Low-Frequency Oscillator Control
    OSCXCN = 0xB1,   // External Oscillator Control
    P0 = 0x80,       // Port 0 Latch
    P0MDIN = 0xF1,   // Port 0 Input Mode Configuration
    P0MDOUT = 0xA4,  // Port 0 Output Mode Configuration
    P0SKIP = 0xD4,   // Port 0 Skip
    P1 = 0x90,       // Port 1 Latch
    P1MDIN = 0xF2,   // Port 1 Input Mode Configuration
    P1MDOUT = 0xA5,  // Port 1 Output Mode Configuration
    P1SKIP = 0xD5,   // Port 1 Skip
    P2 = 0xA0,       // Port 2 Latch
    P2MDIN = 0xF3,   // Port 2 Input Mode Configuration
    P2MDOUT = 0xA6,  // Port 2 Output Mode Configuration
    P2SKIP = 0xD6,   // Port 2 Skip
    P3 = 0xB0,       // Port 3 Latch
    P3MDIN = 0xF4,   // Port 3 Input Mode Configuration
    P3MDOUT = 0xA7,  // Port 3 Output Mode Configuration
    P3SKIP = 0xDF,   // Port 3 Skip
    P4 = 0xC7,       // Port 4 Latch
    P4MDIN = 0xF5,   // Port 4 Input Mode Configuration
    P4MDOUT = 0xAE,  // Port 4 Output Mode Configuration
    PCA0CN = 0xD8,   // PCA Control
    PCA0CPH0 = 0xFC, // PCA Capture 0 High
    PCA0CPH1 = 0xEA, // PCA Capture 1 High
    PCA0CPH2 = 0xEC, // PCA Capture 2 High
    PCA0CPH3 = 0xEE, // PCA Capture 3 High
    PCA0CPH4 = 0xFE, // PCA Capture 4 High
    PCA0CPL0 = 0xFB, // PCA Capture 0 Low
    PCA0CPL1 = 0xE9, // PCA Capture 1 Low
    PCA0CPL2 = 0xEB, // PCA Capture 2 Low
    PCA0CPL3 = 0xED, // PCA Capture 3 Low
    PCA0CPL4 = 0xFD, // PCA Capture 4 Low
    PCA0CPM0 = 0xDA, // PCA Module 0 Mode Register
    PCA0CPM1 = 0xDB, // PCA Module 1 Mode Register
    PCA0CPM2 = 0xDC, // PCA Module 2 Mode Register
    PCA0CPM3 = 0xDD, // PCA Module 3 Mode Register
    PCA0CPM4 = 0xDE, // PCA Module 4 Mode Register
    PCA0H = 0xFA,    // PCA Counter High
    PCA0L = 0xF9,    // PCA Counter Low
    PCA0MD = 0xD9,   // PCA Mode
    PCON = 0x87,     // Power Control
    PFE0CN = 0xAF,   // Prefetch Engine Control
    PSCTL = 0x8F,    // Program Store R/W Control
    PSW = 0xD0,      // Program Status Word
    REF0CN = 0xD1,   // Voltage Reference Control
    REG0CN = 0xC9,   // Voltage Regulator Control
    RSTSRC = 0xEF,   // Reset Source Configuration/Status
    SBCON1 = 0xAC,   // UART1 Baud Rate Generator Control
    SBRLH1 = 0xB5,   // UART1 Baud Rate Generator High
    SBRLL1 = 0xB4,   // UART1 Baud Rate Generator Low
    SBUF1 = 0xD3,    // UART1 Data Buffer
    SCON1 = 0xD2,    // UART1 Control
    SBUF0 = 0x99,    // UART0 Data Buffer
    SCON0 = 0x98,    // UART0 Control
    SMB0CF = 0xC1,   // SMBus Configuration
    SMB0CN = 0xC0,   // SMBus Control
    SMB0DAT = 0xC2,  // SMBus Data
    SMOD1 = 0xE5,    // UART1 Mode
    SP = 0x81,       // Stack Pointer
    SPI0CFG = 0xA1,  // SPI Configuration
    SPI0CKR = 0xA2,  // SPI Clock Rate Control
    SPI0CN = 0xF8,   // SPI Control
    SPI0DAT = 0xA3,  // SPI Data
    TCON = 0x88,     // Timer/Counter Control
    TH0 = 0x8C,      // Timer/Counter 0 High
    TH1 = 0x8D,      // Timer/Counter 1 High
    TL0 = 0x8A,      // Timer/Counter 0 Low
    TL1 = 0x8B,      // Timer/Counter 1 Low
    TMOD = 0x89,     // Timer/Counter Mode
    TMR2CN = 0xC8,   // Timer/Counter 2 Control
    TMR2H = 0xCD,    // Timer/Counter 2 High
    TMR2L = 0xCC,    // Timer/Counter 2 Low
    TMR2RLH = 0xCB,  // Timer/Counter 2 Reload High
    TMR2RLL = 0xCA,  // Timer/Counter 2 Reload Low
    TMR3CN = 0x91,   // Timer/Counter 3 Control
    TMR3H = 0x95,    // Timer/Counter 3 High
    TMR3L = 0x94,    // Timer/Counter 3 Low
    TMR3RLH = 0x93,  // Timer/Counter 3 Reload High
    TMR3RLL = 0x92,  // Timer/Counter 3 Reload Low
    VDM0CN = 0xFF,   // VDD Monitor Control
    USB0ADR = 0x96,  // USB0 Indirect Address Register
    USB0DAT = 0x97,  // USB0 Data Register
    USB0XCN = 0xD7,  // USB0 Transceiver Control
    XBR0 = 0xE1,     // Port I/O Crossbar Control 0
    XBR1 = 0xE2,     // Port I/O Crossbar Control 1
    XBR2 = 0xE3,     // Port I/O Crossbar Control 2
}

impl SFR {
    #[inline]
    pub fn addr(self) -> Addr8 {
        Addr8::new(self as u8)
    }
}

impl From<Addr8> for SFR {
    #[inline]
    fn from(addr: Addr8) -> Self {
        SFR::from(addr.as_u8())
    }
}

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum GPR {
    R0 = 0x00,
    R1 = 0x01,
    R2 = 0x02,
    R3 = 0x03,
    R4 = 0x04,
    R5 = 0x05,
    R6 = 0x06,
    R7 = 0x07,
    #[default]
    Unknown = 0xff,
}

impl GPR {
    #[inline]
    pub fn addr(self) -> Addr8 {
        Addr8::new(self as u8)
    }
}