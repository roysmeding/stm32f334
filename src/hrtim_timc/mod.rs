#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timccr: TIMCCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timcisr: TIMCISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timcicr: TIMCICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timcdier5: TIMCDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntcr: CNTCR,
    #[doc = "0x14 - Timerx Period Register"]
    pub percr: PERCR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repcr: REPCR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1cr: CMP1CR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1ccr: CMP1CCR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2cr: CMP2CR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3cr: CMP3CR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4cr: CMP4CR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1cr: CPT1CR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2cr: CPT2CR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtcr: DTCR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setc1r: SETC1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstc1r: RSTC1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setc2r: SETC2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstc2r: RSTC2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefcr1: EEFCR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefcr2: EEFCR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstcr: RSTCR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpcr: CHPCR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ccr: CPT1CCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ccr: CPT2CCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outcr: OUTCR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltcr: FLTCR,
}
#[doc = "Timerx Control Register"]
pub struct TIMCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Control Register"]
pub mod timccr;
#[doc = "Timerx Interrupt Status Register"]
pub struct TIMCISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Status Register"]
pub mod timcisr;
#[doc = "Timerx Interrupt Clear Register"]
pub struct TIMCICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timcicr;
#[doc = "TIMxDIER5"]
pub struct TIMCDIER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMxDIER5"]
pub mod timcdier5;
#[doc = "Timerx Counter Register"]
pub struct CNTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Counter Register"]
pub mod cntcr;
#[doc = "Timerx Period Register"]
pub struct PERCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Period Register"]
pub mod percr;
#[doc = "Timerx Repetition Register"]
pub struct REPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Repetition Register"]
pub mod repcr;
#[doc = "Timerx Compare 1 Register"]
pub struct CMP1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1cr;
#[doc = "Timerx Compare 1 Compound Register"]
pub struct CMP1CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1ccr;
#[doc = "Timerx Compare 2 Register"]
pub struct CMP2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2cr;
#[doc = "Timerx Compare 3 Register"]
pub struct CMP3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3cr;
#[doc = "Timerx Compare 4 Register"]
pub struct CMP4CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4cr;
#[doc = "Timerx Capture 1 Register"]
pub struct CPT1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1cr;
#[doc = "Timerx Capture 2 Register"]
pub struct CPT2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2cr;
#[doc = "Timerx Deadtime Register"]
pub struct DTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Deadtime Register"]
pub mod dtcr;
#[doc = "Timerx Output1 Set Register"]
pub struct SETC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Set Register"]
pub mod setc1r;
#[doc = "Timerx Output1 Reset Register"]
pub struct RSTC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Reset Register"]
pub mod rstc1r;
#[doc = "Timerx Output2 Set Register"]
pub struct SETC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Set Register"]
pub mod setc2r;
#[doc = "Timerx Output2 Reset Register"]
pub struct RSTC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Reset Register"]
pub mod rstc2r;
#[doc = "Timerx External Event Filtering Register 1"]
pub struct EEFCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefcr1;
#[doc = "Timerx External Event Filtering Register 2"]
pub struct EEFCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefcr2;
#[doc = "TimerA Reset Register"]
pub struct RSTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TimerA Reset Register"]
pub mod rstcr;
#[doc = "Timerx Chopper Register"]
pub struct CHPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Chopper Register"]
pub mod chpcr;
#[doc = "Timerx Capture 2 Control Register"]
pub struct CPT1CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ccr;
#[doc = "CPT2xCR"]
pub struct CPT2CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPT2xCR"]
pub mod cpt2ccr;
#[doc = "Timerx Output Register"]
pub struct OUTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output Register"]
pub mod outcr;
#[doc = "Timerx Fault Register"]
pub struct FLTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Fault Register"]
pub mod fltcr;
