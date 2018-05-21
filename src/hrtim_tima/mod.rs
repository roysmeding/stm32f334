#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timacr: TIMACR,
    #[doc = "0x04 - TimerA Interrupt Status Register"]
    pub timaisr: TIMAISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timaicr: TIMAICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timadier5: TIMADIER5,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntar: CNTAR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perar: PERAR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repar: REPAR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1ar: CMP1AR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1car: CMP1CAR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2ar: CMP2AR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3ar: CMP3AR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4ar: CMP4AR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1ar: CPT1AR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2ar: CPT2AR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtar: DTAR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub seta1r: SETA1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rsta1r: RSTA1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub seta2r: SETA2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rsta2r: RSTA2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefar1: EEFAR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefar2: EEFAR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstar: RSTAR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpar: CHPAR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1acr: CPT1ACR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2acr: CPT2ACR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outar: OUTAR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltar: FLTAR,
}
#[doc = "Timerx Control Register"]
pub struct TIMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Control Register"]
pub mod timacr;
#[doc = "TimerA Interrupt Status Register"]
pub struct TIMAISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TimerA Interrupt Status Register"]
pub mod timaisr;
#[doc = "Timerx Interrupt Clear Register"]
pub struct TIMAICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timaicr;
#[doc = "TIMxDIER5"]
pub struct TIMADIER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMxDIER5"]
pub mod timadier5;
#[doc = "Timerx Counter Register"]
pub struct CNTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Counter Register"]
pub mod cntar;
#[doc = "Timerx Period Register"]
pub struct PERAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Period Register"]
pub mod perar;
#[doc = "Timerx Repetition Register"]
pub struct REPAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Repetition Register"]
pub mod repar;
#[doc = "Timerx Compare 1 Register"]
pub struct CMP1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1ar;
#[doc = "Timerx Compare 1 Compound Register"]
pub struct CMP1CAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1car;
#[doc = "Timerx Compare 2 Register"]
pub struct CMP2AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2ar;
#[doc = "Timerx Compare 3 Register"]
pub struct CMP3AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3ar;
#[doc = "Timerx Compare 4 Register"]
pub struct CMP4AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4ar;
#[doc = "Timerx Capture 1 Register"]
pub struct CPT1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1ar;
#[doc = "Timerx Capture 2 Register"]
pub struct CPT2AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2ar;
#[doc = "Timerx Deadtime Register"]
pub struct DTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Deadtime Register"]
pub mod dtar;
#[doc = "Timerx Output1 Set Register"]
pub struct SETA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Set Register"]
pub mod seta1r;
#[doc = "Timerx Output1 Reset Register"]
pub struct RSTA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output1 Reset Register"]
pub mod rsta1r;
#[doc = "Timerx Output2 Set Register"]
pub struct SETA2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Set Register"]
pub mod seta2r;
#[doc = "Timerx Output2 Reset Register"]
pub struct RSTA2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output2 Reset Register"]
pub mod rsta2r;
#[doc = "Timerx External Event Filtering Register 1"]
pub struct EEFAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefar1;
#[doc = "Timerx External Event Filtering Register 2"]
pub struct EEFAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefar2;
#[doc = "TimerA Reset Register"]
pub struct RSTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TimerA Reset Register"]
pub mod rstar;
#[doc = "Timerx Chopper Register"]
pub struct CHPAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Chopper Register"]
pub mod chpar;
#[doc = "Timerx Capture 2 Control Register"]
pub struct CPT1ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1acr;
#[doc = "CPT2xCR"]
pub struct CPT2ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPT2xCR"]
pub mod cpt2acr;
#[doc = "Timerx Output Register"]
pub struct OUTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Output Register"]
pub mod outar;
#[doc = "Timerx Fault Register"]
pub struct FLTAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timerx Fault Register"]
pub mod fltar;
