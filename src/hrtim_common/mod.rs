#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control Register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x0c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Output Enable Register"]
    pub oenr: OENR,
    #[doc = "0x18 - DISR"]
    pub disr: DISR,
    #[doc = "0x1c - Output Disable Status Register"]
    pub odsr: ODSR,
    #[doc = "0x20 - Burst Mode Control Register"]
    pub bmcr: BMCR,
    #[doc = "0x24 - BMTRG"]
    pub bmtrg: BMTRG,
    #[doc = "0x28 - BMCMPR6"]
    pub bmcmpr6: BMCMPR6,
    #[doc = "0x2c - Burst Mode Period Register"]
    pub bmper: BMPER,
    #[doc = "0x30 - Timer External Event Control Register 1"]
    pub eecr1: EECR1,
    #[doc = "0x34 - Timer External Event Control Register 2"]
    pub eecr2: EECR2,
    #[doc = "0x38 - Timer External Event Control Register 3"]
    pub eecr3: EECR3,
    #[doc = "0x3c - ADC Trigger 1 Register"]
    pub adc1r: ADC1R,
    #[doc = "0x40 - ADC Trigger 2 Register"]
    pub adc2r: ADC2R,
    #[doc = "0x44 - ADC Trigger 3 Register"]
    pub adc3r: ADC3R,
    #[doc = "0x48 - ADC Trigger 4 Register"]
    pub adc4r: ADC4R,
    #[doc = "0x4c - DLL Control Register"]
    pub dllcr: DLLCR,
    #[doc = "0x50 - HRTIM Fault Input Register 1"]
    pub fltinr1: FLTINR1,
    #[doc = "0x54 - HRTIM Fault Input Register 2"]
    pub fltinr2: FLTINR2,
    #[doc = "0x58 - BDMUPDR"]
    pub bdmupdr: BDMUPDR,
    #[doc = "0x5c - Burst DMA Timerx update Register"]
    pub bdtx_upr: BDTXUPR,
    #[doc = "0x60 - Burst DMA Data Register"]
    pub bdmadr: BDMADR,
}
#[doc = "Control Register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "Control Register 2"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Output Enable Register"]
pub struct OENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "DISR"]
pub struct DISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DISR"]
pub mod disr;
#[doc = "Output Disable Status Register"]
pub struct ODSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Disable Status Register"]
pub mod odsr;
#[doc = "Burst Mode Control Register"]
pub struct BMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Burst Mode Control Register"]
pub mod bmcr;
#[doc = "BMTRG"]
pub struct BMTRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMTRG"]
pub mod bmtrg;
#[doc = "BMCMPR6"]
pub struct BMCMPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMCMPR6"]
pub mod bmcmpr6;
#[doc = "Burst Mode Period Register"]
pub struct BMPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Burst Mode Period Register"]
pub mod bmper;
#[doc = "Timer External Event Control Register 1"]
pub struct EECR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer External Event Control Register 1"]
pub mod eecr1;
#[doc = "Timer External Event Control Register 2"]
pub struct EECR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer External Event Control Register 2"]
pub mod eecr2;
#[doc = "Timer External Event Control Register 3"]
pub struct EECR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer External Event Control Register 3"]
pub mod eecr3;
#[doc = "ADC Trigger 1 Register"]
pub struct ADC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger 1 Register"]
pub mod adc1r;
#[doc = "ADC Trigger 2 Register"]
pub struct ADC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger 2 Register"]
pub mod adc2r;
#[doc = "ADC Trigger 3 Register"]
pub struct ADC3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger 3 Register"]
pub mod adc3r;
#[doc = "ADC Trigger 4 Register"]
pub struct ADC4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger 4 Register"]
pub mod adc4r;
#[doc = "DLL Control Register"]
pub struct DLLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL Control Register"]
pub mod dllcr;
#[doc = "HRTIM Fault Input Register 1"]
pub struct FLTINR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HRTIM Fault Input Register 1"]
pub mod fltinr1;
#[doc = "HRTIM Fault Input Register 2"]
pub struct FLTINR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HRTIM Fault Input Register 2"]
pub mod fltinr2;
#[doc = "BDMUPDR"]
pub struct BDMUPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BDMUPDR"]
pub mod bdmupdr;
#[doc = "Burst DMA Timerx update Register"]
pub struct BDTXUPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtx_upr;
#[doc = "Burst DMA Data Register"]
pub struct BDMADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Burst DMA Data Register"]
pub mod bdmadr;
