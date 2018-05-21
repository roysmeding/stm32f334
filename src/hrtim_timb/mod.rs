#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timbcr: TIMBCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timbisr: TIMBISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timbicr: TIMBICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timbdier5: TIMBDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntr: CNTR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perbr: PERBR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repbr: REPBR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1br: CMP1BR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cbr: CMP1CBR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2br: CMP2BR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3br: CMP3BR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4br: CMP4BR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1br: CPT1BR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2br: CPT2BR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtbr: DTBR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setb1r: SETB1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstb1r: RSTB1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setb2r: SETB2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstb2r: RSTB2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefbr1: EEFBR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefbr2: EEFBR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstbr: RSTBR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpbr: CHPBR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1bcr: CPT1BCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2bcr: CPT2BCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outbr: OUTBR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltbr: FLTBR,
}
#[doc = "Timerx Control Register"]
pub struct TIMBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Control Register"]
pub mod timbcr;
#[doc = "Timerx Interrupt Status Register"]
pub struct TIMBISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Status Register"]
pub mod timbisr;
#[doc = "Timerx Interrupt Clear Register"]
pub struct TIMBICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timbicr;
#[doc = "TIMxDIER5"]
pub struct TIMBDIER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMxDIER5"]
pub mod timbdier5;
#[doc = "Timerx Counter Register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Counter Register"]
pub mod cntr;
#[doc = "Timerx Period Register"]
pub struct PERBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Period Register"]
pub mod perbr;
#[doc = "Timerx Repetition Register"]
pub struct REPBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Repetition Register"]
pub mod repbr;
#[doc = "Timerx Compare 1 Register"]
pub struct CMP1BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1br;
#[doc = "Timerx Compare 1 Compound Register"]
pub struct CMP1CBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cbr;
#[doc = "Timerx Compare 2 Register"]
pub struct CMP2BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2br;
#[doc = "Timerx Compare 3 Register"]
pub struct CMP3BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3br;
#[doc = "Timerx Compare 4 Register"]
pub struct CMP4BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4br;
#[doc = "Timerx Capture 1 Register"]
pub struct CPT1BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1br;
#[doc = "Timerx Capture 2 Register"]
pub struct CPT2BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2br;
#[doc = "Timerx Deadtime Register"]
pub struct DTBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Deadtime Register"]
pub mod dtbr;
#[doc = "Timerx Output1 Set Register"]
pub struct SETB1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Set Register"]
pub mod setb1r;
#[doc = "Timerx Output1 Reset Register"]
pub struct RSTB1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Reset Register"]
pub mod rstb1r;
#[doc = "Timerx Output2 Set Register"]
pub struct SETB2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Set Register"]
pub mod setb2r;
#[doc = "Timerx Output2 Reset Register"]
pub struct RSTB2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Reset Register"]
pub mod rstb2r;
#[doc = "Timerx External Event Filtering Register 1"]
pub struct EEFBR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefbr1;
#[doc = "Timerx External Event Filtering Register 2"]
pub struct EEFBR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefbr2;
#[doc = "TimerA Reset Register"]
pub struct RSTBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TimerA Reset Register"]
pub mod rstbr;
#[doc = "Timerx Chopper Register"]
pub struct CHPBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Chopper Register"]
pub mod chpbr;
#[doc = "Timerx Capture 2 Control Register"]
pub struct CPT1BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1bcr;
#[doc = "CPT2xCR"]
pub struct CPT2BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPT2xCR"]
pub mod cpt2bcr;
#[doc = "Timerx Output Register"]
pub struct OUTBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output Register"]
pub mod outbr;
#[doc = "Timerx Fault Register"]
pub struct FLTBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Fault Register"]
pub mod fltbr;
