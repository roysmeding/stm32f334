#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timecr: TIMECR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timeisr: TIMEISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timeicr: TIMEICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timedier5: TIMEDIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cnter: CNTER,
    #[doc = "0x14 - Timerx Period Register"]
    pub perer: PERER,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub reper: REPER,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1er: CMP1ER,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cer: CMP1CER,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2er: CMP2ER,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3er: CMP3ER,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4er: CMP4ER,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1er: CPT1ER,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2er: CPT2ER,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dter: DTER,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub sete1r: SETE1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rste1r: RSTE1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub sete2r: SETE2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rste2r: RSTE2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefer1: EEFER1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefer2: EEFER2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rster: RSTER,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chper: CHPER,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ecr: CPT1ECR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ecr: CPT2ECR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outer: OUTER,
    #[doc = "0x68 - Timerx Fault Register"]
    pub flter: FLTER,
}
#[doc = "Timerx Control Register"]
pub struct TIMECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Control Register"]
pub mod timecr;
#[doc = "Timerx Interrupt Status Register"]
pub struct TIMEISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Status Register"]
pub mod timeisr;
#[doc = "Timerx Interrupt Clear Register"]
pub struct TIMEICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timeicr;
#[doc = "TIMxDIER5"]
pub struct TIMEDIER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMxDIER5"]
pub mod timedier5;
#[doc = "Timerx Counter Register"]
pub struct CNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Counter Register"]
pub mod cnter;
#[doc = "Timerx Period Register"]
pub struct PERER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Period Register"]
pub mod perer;
#[doc = "Timerx Repetition Register"]
pub struct REPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Repetition Register"]
pub mod reper;
#[doc = "Timerx Compare 1 Register"]
pub struct CMP1ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1er;
#[doc = "Timerx Compare 1 Compound Register"]
pub struct CMP1CER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cer;
#[doc = "Timerx Compare 2 Register"]
pub struct CMP2ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2er;
#[doc = "Timerx Compare 3 Register"]
pub struct CMP3ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3er;
#[doc = "Timerx Compare 4 Register"]
pub struct CMP4ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4er;
#[doc = "Timerx Capture 1 Register"]
pub struct CPT1ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1er;
#[doc = "Timerx Capture 2 Register"]
pub struct CPT2ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2er;
#[doc = "Timerx Deadtime Register"]
pub struct DTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Deadtime Register"]
pub mod dter;
#[doc = "Timerx Output1 Set Register"]
pub struct SETE1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Set Register"]
pub mod sete1r;
#[doc = "Timerx Output1 Reset Register"]
pub struct RSTE1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "Timerx Output2 Set Register"]
pub struct SETE2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Set Register"]
pub mod sete2r;
#[doc = "Timerx Output2 Reset Register"]
pub struct RSTE2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Reset Register"]
pub mod rste2r;
#[doc = "Timerx External Event Filtering Register 1"]
pub struct EEFER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefer1;
#[doc = "Timerx External Event Filtering Register 2"]
pub struct EEFER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefer2;
#[doc = "TimerA Reset Register"]
pub struct RSTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TimerA Reset Register"]
pub mod rster;
#[doc = "Timerx Chopper Register"]
pub struct CHPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Chopper Register"]
pub mod chper;
#[doc = "Timerx Capture 2 Control Register"]
pub struct CPT1ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ecr;
#[doc = "CPT2xCR"]
pub struct CPT2ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPT2xCR"]
pub mod cpt2ecr;
#[doc = "Timerx Output Register"]
pub struct OUTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output Register"]
pub mod outer;
#[doc = "Timerx Fault Register"]
pub struct FLTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Fault Register"]
pub mod flter;
