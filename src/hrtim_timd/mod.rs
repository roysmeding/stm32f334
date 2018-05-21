#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timdcr: TIMDCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timdisr: TIMDISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timdicr: TIMDICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timddier5: TIMDDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntdr: CNTDR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perdr: PERDR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repdr: REPDR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1dr: CMP1DR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cdr: CMP1CDR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2dr: CMP2DR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3dr: CMP3DR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4dr: CMP4DR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1dr: CPT1DR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2dr: CPT2DR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtdr: DTDR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setd1r: SETD1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstd1r: RSTD1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setd2r: SETD2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstd2r: RSTD2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefdr1: EEFDR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefdr2: EEFDR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstdr: RSTDR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpdr: CHPDR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1dcr: CPT1DCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2dcr: CPT2DCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outdr: OUTDR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltdr: FLTDR,
}
#[doc = "Timerx Control Register"]
pub struct TIMDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Control Register"]
pub mod timdcr;
#[doc = "Timerx Interrupt Status Register"]
pub struct TIMDISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Status Register"]
pub mod timdisr;
#[doc = "Timerx Interrupt Clear Register"]
pub struct TIMDICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timdicr;
#[doc = "TIMxDIER5"]
pub struct TIMDDIER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMxDIER5"]
pub mod timddier5;
#[doc = "Timerx Counter Register"]
pub struct CNTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Counter Register"]
pub mod cntdr;
#[doc = "Timerx Period Register"]
pub struct PERDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Period Register"]
pub mod perdr;
#[doc = "Timerx Repetition Register"]
pub struct REPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Repetition Register"]
pub mod repdr;
#[doc = "Timerx Compare 1 Register"]
pub struct CMP1DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1dr;
#[doc = "Timerx Compare 1 Compound Register"]
pub struct CMP1CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cdr;
#[doc = "Timerx Compare 2 Register"]
pub struct CMP2DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2dr;
#[doc = "Timerx Compare 3 Register"]
pub struct CMP3DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3dr;
#[doc = "Timerx Compare 4 Register"]
pub struct CMP4DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4dr;
#[doc = "Timerx Capture 1 Register"]
pub struct CPT1DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1dr;
#[doc = "Timerx Capture 2 Register"]
pub struct CPT2DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2dr;
#[doc = "Timerx Deadtime Register"]
pub struct DTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Deadtime Register"]
pub mod dtdr;
#[doc = "Timerx Output1 Set Register"]
pub struct SETD1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Set Register"]
pub mod setd1r;
#[doc = "Timerx Output1 Reset Register"]
pub struct RSTD1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Reset Register"]
pub mod rstd1r;
#[doc = "Timerx Output2 Set Register"]
pub struct SETD2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Set Register"]
pub mod setd2r;
#[doc = "Timerx Output2 Reset Register"]
pub struct RSTD2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Reset Register"]
pub mod rstd2r;
#[doc = "Timerx External Event Filtering Register 1"]
pub struct EEFDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefdr1;
#[doc = "Timerx External Event Filtering Register 2"]
pub struct EEFDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefdr2;
#[doc = "TimerA Reset Register"]
pub struct RSTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TimerA Reset Register"]
pub mod rstdr;
#[doc = "Timerx Chopper Register"]
pub struct CHPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Chopper Register"]
pub mod chpdr;
#[doc = "Timerx Capture 2 Control Register"]
pub struct CPT1DCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1dcr;
#[doc = "CPT2xCR"]
pub struct CPT2DCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPT2xCR"]
pub mod cpt2dcr;
#[doc = "Timerx Output Register"]
pub struct OUTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output Register"]
pub mod outdr;
#[doc = "Timerx Fault Register"]
pub struct FLTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Fault Register"]
pub mod fltdr;
