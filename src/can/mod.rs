#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: MCR,
    #[doc = "0x04 - master status register"]
    pub msr: MSR,
    #[doc = "0x08 - transmit status register"]
    pub tsr: TSR,
    #[doc = "0x0c - receive FIFO 0 register"]
    pub rf0r: RF0R,
    #[doc = "0x10 - receive FIFO 1 register"]
    pub rf1r: RF1R,
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x18 - error status register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - TX mailbox identifier register"]
    pub ti0r: TI0R,
    #[doc = "0x184 - mailbox data length control and time stamp register"]
    pub tdt0r: TDT0R,
    #[doc = "0x188 - mailbox data low register"]
    pub tdl0r: TDL0R,
    #[doc = "0x18c - mailbox data high register"]
    pub tdh0r: TDH0R,
    #[doc = "0x190 - TX mailbox identifier register"]
    pub ti1r: TI1R,
    #[doc = "0x194 - mailbox data length control and time stamp register"]
    pub tdt1r: TDT1R,
    #[doc = "0x198 - mailbox data low register"]
    pub tdl1r: TDL1R,
    #[doc = "0x19c - mailbox data high register"]
    pub tdh1r: TDH1R,
    #[doc = "0x1a0 - TX mailbox identifier register"]
    pub ti2r: TI2R,
    #[doc = "0x1a4 - mailbox data length control and time stamp register"]
    pub tdt2r: TDT2R,
    #[doc = "0x1a8 - mailbox data low register"]
    pub tdl2r: TDL2R,
    #[doc = "0x1ac - mailbox data high register"]
    pub tdh2r: TDH2R,
    #[doc = "0x1b0 - receive FIFO mailbox identifier register"]
    pub ri0r: RI0R,
    #[doc = "0x1b4 - receive FIFO mailbox data length control and time stamp register"]
    pub rdt0r: RDT0R,
    #[doc = "0x1b8 - receive FIFO mailbox data low register"]
    pub rdl0r: RDL0R,
    #[doc = "0x1bc - receive FIFO mailbox data high register"]
    pub rdh0r: RDH0R,
    #[doc = "0x1c0 - receive FIFO mailbox identifier register"]
    pub ri1r: RI1R,
    #[doc = "0x1c4 - receive FIFO mailbox data length control and time stamp register"]
    pub rdt1r: RDT1R,
    #[doc = "0x1c8 - receive FIFO mailbox data low register"]
    pub rdl1r: RDL1R,
    #[doc = "0x1cc - receive FIFO mailbox data high register"]
    pub rdh1r: RDH1R,
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - filter master register"]
    pub fmr: FMR,
    #[doc = "0x204 - filter mode register"]
    pub fm1r: FM1R,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: FS1R,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: FFA1R,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - CAN filter activation register"]
    pub fa1r: FA1R,
    _reserved5: [u8; 32usize],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: F0R1,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: F0R2,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: F1R1,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: F1R2,
    _reserved6: [u8; 200usize],
    #[doc = "0x318 - Filter bank 27 register 1"]
    pub f27r1: F27R1,
    #[doc = "0x31c - Filter bank 27 register 2"]
    pub f27r2: F27R2,
}
#[doc = "master control register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master control register"]
pub mod mcr;
#[doc = "master status register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master status register"]
pub mod msr;
#[doc = "transmit status register"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "receive FIFO 0 register"]
pub struct RF0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO 0 register"]
pub mod rf0r;
#[doc = "receive FIFO 1 register"]
pub struct RF1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO 1 register"]
pub mod rf1r;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "error status register"]
pub struct ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "error status register"]
pub mod esr;
#[doc = "bit timing register"]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "bit timing register"]
pub mod btr;
#[doc = "TX mailbox identifier register"]
pub struct TI0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX mailbox identifier register"]
pub mod ti0r;
#[doc = "mailbox data length control and time stamp register"]
pub struct TDT0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt0r;
#[doc = "mailbox data low register"]
pub struct TDL0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdl0r;
#[doc = "mailbox data high register"]
pub struct TDH0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdh0r;
#[doc = "TX mailbox identifier register"]
pub struct TI1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX mailbox identifier register"]
pub mod ti1r;
#[doc = "mailbox data length control and time stamp register"]
pub struct TDT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt1r;
#[doc = "mailbox data low register"]
pub struct TDL1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdl1r;
#[doc = "mailbox data high register"]
pub struct TDH1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdh1r;
#[doc = "TX mailbox identifier register"]
pub struct TI2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX mailbox identifier register"]
pub mod ti2r;
#[doc = "mailbox data length control and time stamp register"]
pub struct TDT2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt2r;
#[doc = "mailbox data low register"]
pub struct TDL2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data low register"]
pub mod tdl2r;
#[doc = "mailbox data high register"]
pub struct TDH2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "mailbox data high register"]
pub mod tdh2r;
#[doc = "receive FIFO mailbox identifier register"]
pub struct RI0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri0r;
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub struct RDT0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub mod rdt0r;
#[doc = "receive FIFO mailbox data low register"]
pub struct RDL0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data low register"]
pub mod rdl0r;
#[doc = "receive FIFO mailbox data high register"]
pub struct RDH0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh0r;
#[doc = "receive FIFO mailbox identifier register"]
pub struct RI1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri1r;
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub struct RDT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub mod rdt1r;
#[doc = "receive FIFO mailbox data low register"]
pub struct RDL1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data low register"]
pub mod rdl1r;
#[doc = "receive FIFO mailbox data high register"]
pub struct RDH1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh1r;
#[doc = "filter master register"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter master register"]
pub mod fmr;
#[doc = "filter mode register"]
pub struct FM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "filter scale register"]
pub struct FS1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "filter FIFO assignment register"]
pub struct FFA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "CAN filter activation register"]
pub struct FA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN filter activation register"]
pub mod fa1r;
#[doc = "Filter bank 0 register 1"]
pub struct F0R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "Filter bank 0 register 2"]
pub struct F0R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "Filter bank 1 register 1"]
pub struct F1R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "Filter bank 1 register 2"]
pub struct F1R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "Filter bank 27 register 1"]
pub struct F27R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "Filter bank 27 register 2"]
pub struct F27R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
