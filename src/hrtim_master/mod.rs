#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Timer Control Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Master Timer Interrupt Status Register"]
    pub misr: MISR,
    #[doc = "0x08 - Master Timer Interrupt Clear Register"]
    pub micr: MICR,
    #[doc = "0x0c - MDIER4"]
    pub mdier4: MDIER4,
    #[doc = "0x10 - Master Timer Counter Register"]
    pub mcntr: MCNTR,
    #[doc = "0x14 - Master Timer Period Register"]
    pub mper: MPER,
    #[doc = "0x18 - Master Timer Repetition Register"]
    pub mrep: MREP,
    #[doc = "0x1c - Master Timer Compare 1 Register"]
    pub mcmp1r: MCMP1R,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - Master Timer Compare 2 Register"]
    pub mcmp2r: MCMP2R,
    #[doc = "0x28 - Master Timer Compare 3 Register"]
    pub mcmp3r: MCMP3R,
    #[doc = "0x2c - Master Timer Compare 4 Register"]
    pub mcmp4r: MCMP4R,
}
#[doc = "Master Timer Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Control Register"]
pub mod mcr;
#[doc = "Master Timer Interrupt Status Register"]
pub struct MISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Interrupt Status Register"]
pub mod misr;
#[doc = "Master Timer Interrupt Clear Register"]
pub struct MICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Interrupt Clear Register"]
pub mod micr;
#[doc = "MDIER4"]
pub struct MDIER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIER4"]
pub mod mdier4;
#[doc = "Master Timer Counter Register"]
pub struct MCNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Counter Register"]
pub mod mcntr;
#[doc = "Master Timer Period Register"]
pub struct MPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Period Register"]
pub mod mper;
#[doc = "Master Timer Repetition Register"]
pub struct MREP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Repetition Register"]
pub mod mrep;
#[doc = "Master Timer Compare 1 Register"]
pub struct MCMP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Compare 1 Register"]
pub mod mcmp1r;
#[doc = "Master Timer Compare 2 Register"]
pub struct MCMP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Compare 2 Register"]
pub mod mcmp2r;
#[doc = "Master Timer Compare 3 Register"]
pub struct MCMP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Compare 3 Register"]
pub mod mcmp3r;
#[doc = "Master Timer Compare 4 Register"]
pub struct MCMP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Timer Compare 4 Register"]
pub mod mcmp4r;
