#![doc = "Peripheral access API for STM32F334X microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::ops::Deref;
use core::marker::PhantomData;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2_TSC();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn I2C1_EV_EXTI23();
    fn I2C1_ER();
    fn SPI1();
    fn USART1_EXTI25();
    fn USART2_EXTI26();
    fn USART3_EXTI28();
    fn EXTI15_10();
    fn RTCALARM();
    fn UART4_EXTI34();
    fn UART5_EXTI35();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn COMP123();
    fn COMP456();
    fn COMP7();
    fn USB_WKUP_EXTI();
    fn FPU();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 82] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector {
        _handler: EXTI2_TSC,
    },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1_2 },
    Vector {
        _handler: USB_HP_CAN_TX,
    },
    Vector {
        _handler: USB_LP_CAN_RX0,
    },
    Vector { _handler: CAN_RX1 },
    Vector { _handler: CAN_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM15,
    },
    Vector {
        _handler: TIM1_UP_TIM16,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _reserved: 0 },
    Vector {
        _handler: I2C1_EV_EXTI23,
    },
    Vector { _handler: I2C1_ER },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USART1_EXTI25,
    },
    Vector {
        _handler: USART2_EXTI26,
    },
    Vector {
        _handler: USART3_EXTI28,
    },
    Vector {
        _handler: EXTI15_10,
    },
    Vector { _handler: RTCALARM },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: UART4_EXTI34,
    },
    Vector {
        _handler: UART5_EXTI35,
    },
    Vector {
        _handler: TIM6_DACUNDER,
    },
    Vector { _handler: TIM7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: COMP123 },
    Vector { _handler: COMP456 },
    Vector { _handler: COMP7 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB_WKUP_EXTI,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name: ident, $handler: path,state: $State: ty = $initial_state: expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name: ident, $handler: path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper and TimeStamp interrupts"]
    TAMP_STAMP,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line3 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 and Touch sensing interrupts"]
    EXTI2_TSC,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 channel 1 interrupt"]
    DMA1_CH1,
    #[doc = "12 - DMA1 channel 2 interrupt"]
    DMA1_CH2,
    #[doc = "13 - DMA1 channel 3 interrupt"]
    DMA1_CH3,
    #[doc = "14 - DMA1 channel 4 interrupt"]
    DMA1_CH4,
    #[doc = "15 - DMA1 channel 5 interrupt"]
    DMA1_CH5,
    #[doc = "16 - DMA1 channel 6 interrupt"]
    DMA1_CH6,
    #[doc = "17 - DMA1 channel 7interrupt"]
    DMA1_CH7,
    #[doc = "18 - ADC1 and ADC2 global interrupt"]
    ADC1_2,
    #[doc = "19 - USB High Priority/CAN_TX interrupts"]
    USB_HP_CAN_TX,
    #[doc = "20 - USB Low Priority/CAN_RX0 interrupts"]
    USB_LP_CAN_RX0,
    #[doc = "21 - CAN_RX1 interrupt"]
    CAN_RX1,
    #[doc = "22 - CAN_SCE interrupt"]
    CAN_SCE,
    #[doc = "23 - EXTI Line5 to Line9 interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break/TIM15 global interruts"]
    TIM1_BRK_TIM15,
    #[doc = "25 - TIM1 Update/TIM16 global interrupts"]
    TIM1_UP_TIM16,
    #[doc = "26 - TIM1 trigger and commutation/TIM17 interrupts"]
    TIM1_TRG_COM_TIM17,
    #[doc = "27 - TIM1 capture compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "31 - I2C1 event interrupt and EXTI Line23 interrupt"]
    I2C1_EV_EXTI23,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "37 - USART1 global interrupt and EXTI Line 25 interrupt"]
    USART1_EXTI25,
    #[doc = "38 - USART2 global interrupt and EXTI Line 26 interrupt"]
    USART2_EXTI26,
    #[doc = "39 - USART3 global interrupt and EXTI Line 28 interrupt"]
    USART3_EXTI28,
    #[doc = "40 - EXTI Line15 to Line10 interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC alarm interrupt"]
    RTCALARM,
    #[doc = "52 - UART4 global and EXTI Line 34 interrupts"]
    UART4_EXTI34,
    #[doc = "53 - UART5 global and EXTI Line 35 interrupts"]
    UART5_EXTI35,
    #[doc = "54 - TIM6 global and DAC12 underrun interrupts"]
    TIM6_DACUNDER,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "64 - COMP1 & COMP2 & COMP3 interrupts combined with EXTI Lines 21, 22 and 29 interrupts"]
    COMP123,
    #[doc = "65 - COMP4 & COMP5 & COMP6 interrupts combined with EXTI Lines 30, 31 and 32 interrupts"]
    COMP456,
    #[doc = "66 - COMP7 interrupt combined with EXTI Line 33 interrupt"]
    COMP7,
    #[doc = "76 - USB wakeup from Suspend and EXTI Line 18"]
    USB_WKUP_EXTI,
    #[doc = "81 - Floating point interrupt"]
    FPU,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2_TSC => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CH1 => 11,
            Interrupt::DMA1_CH2 => 12,
            Interrupt::DMA1_CH3 => 13,
            Interrupt::DMA1_CH4 => 14,
            Interrupt::DMA1_CH5 => 15,
            Interrupt::DMA1_CH6 => 16,
            Interrupt::DMA1_CH7 => 17,
            Interrupt::ADC1_2 => 18,
            Interrupt::USB_HP_CAN_TX => 19,
            Interrupt::USB_LP_CAN_RX0 => 20,
            Interrupt::CAN_RX1 => 21,
            Interrupt::CAN_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM15 => 24,
            Interrupt::TIM1_UP_TIM16 => 25,
            Interrupt::TIM1_TRG_COM_TIM17 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::I2C1_EV_EXTI23 => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::SPI1 => 35,
            Interrupt::USART1_EXTI25 => 37,
            Interrupt::USART2_EXTI26 => 38,
            Interrupt::USART3_EXTI28 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTCALARM => 41,
            Interrupt::UART4_EXTI34 => 52,
            Interrupt::UART5_EXTI35 => 53,
            Interrupt::TIM6_DACUNDER => 54,
            Interrupt::TIM7 => 55,
            Interrupt::COMP123 => 64,
            Interrupt::COMP456 => 65,
            Interrupt::COMP7 => 66,
            Interrupt::USB_WKUP_EXTI => 76,
            Interrupt::FPU => 81,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207960576 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207961600 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207962624 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOF"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207964672 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &tsc::RegisterBlock {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "DMA controller 1"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "DMA controller 1"]
pub mod dma1;
#[doc = "General purpose timer"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General purpose timer"]
pub mod tim2;
#[doc = "TIM3"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim15::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    fn deref(&self) -> &tim15::RegisterBlock {
        unsafe { &*TIM15::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim15;
#[doc = "General-purpose-timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim16;
#[doc = "General purpose timer"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim17::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim17::RegisterBlock;
    fn deref(&self) -> &tim17::RegisterBlock {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "General purpose timer"]
pub mod tim17;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const comp::RegisterBlock {
        1073807388 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &comp::RegisterBlock {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Controller area network"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can::RegisterBlock {
        1073767424 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &can::RegisterBlock {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Basic timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Digital-to-analog converter 1"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac1::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    fn deref(&self) -> &dac1::RegisterBlock {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "Digital-to-analog converter 1"]
pub mod dac1;
#[doc = "DAC2"]
pub struct DAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC2 {}
impl DAC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac1::RegisterBlock {
        1073780736 as *const _
    }
}
impl Deref for DAC2 {
    type Target = dac1::RegisterBlock;
    fn deref(&self) -> &dac1::RegisterBlock {
        unsafe { &*DAC2::ptr() }
    }
}
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbgmcu::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "Advanced timer"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced timer"]
pub mod tim1;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1_2 {}
impl ADC1_2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1_2::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for ADC1_2 {
    type Target = adc1_2::RegisterBlock;
    fn deref(&self) -> &adc1_2::RegisterBlock {
        unsafe { &*ADC1_2::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc1_2;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Operational amplifier"]
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const opamp::RegisterBlock {
        1073807416 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    fn deref(&self) -> &opamp::RegisterBlock {
        unsafe { &*OPAMP::ptr() }
    }
}
#[doc = "Operational amplifier"]
pub mod opamp;
#[doc = "High Resolution Timer: Master Timers"]
pub struct HRTIM_MASTER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_MASTER {}
impl HRTIM_MASTER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_master::RegisterBlock {
        1073837056 as *const _
    }
}
impl Deref for HRTIM_MASTER {
    type Target = hrtim_master::RegisterBlock;
    fn deref(&self) -> &hrtim_master::RegisterBlock {
        unsafe { &*HRTIM_MASTER::ptr() }
    }
}
#[doc = "High Resolution Timer: Master Timers"]
pub mod hrtim_master;
#[doc = "High Resolution Timer: TIMA"]
pub struct HRTIM_TIMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMA {}
impl HRTIM_TIMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_tima::RegisterBlock {
        1073837184 as *const _
    }
}
impl Deref for HRTIM_TIMA {
    type Target = hrtim_tima::RegisterBlock;
    fn deref(&self) -> &hrtim_tima::RegisterBlock {
        unsafe { &*HRTIM_TIMA::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMA"]
pub mod hrtim_tima;
#[doc = "High Resolution Timer: TIMB"]
pub struct HRTIM_TIMB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMB {}
impl HRTIM_TIMB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_timb::RegisterBlock {
        1073837312 as *const _
    }
}
impl Deref for HRTIM_TIMB {
    type Target = hrtim_timb::RegisterBlock;
    fn deref(&self) -> &hrtim_timb::RegisterBlock {
        unsafe { &*HRTIM_TIMB::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMB"]
pub mod hrtim_timb;
#[doc = "High Resolution Timer: TIMC"]
pub struct HRTIM_TIMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMC {}
impl HRTIM_TIMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_timc::RegisterBlock {
        1073837440 as *const _
    }
}
impl Deref for HRTIM_TIMC {
    type Target = hrtim_timc::RegisterBlock;
    fn deref(&self) -> &hrtim_timc::RegisterBlock {
        unsafe { &*HRTIM_TIMC::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMC"]
pub mod hrtim_timc;
#[doc = "High Resolution Timer: TIMD"]
pub struct HRTIM_TIMD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMD {}
impl HRTIM_TIMD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_timd::RegisterBlock {
        1073837568 as *const _
    }
}
impl Deref for HRTIM_TIMD {
    type Target = hrtim_timd::RegisterBlock;
    fn deref(&self) -> &hrtim_timd::RegisterBlock {
        unsafe { &*HRTIM_TIMD::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMD"]
pub mod hrtim_timd;
#[doc = "High Resolution Timer: TIME"]
pub struct HRTIM_TIME {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIME {}
impl HRTIM_TIME {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_time::RegisterBlock {
        1073837696 as *const _
    }
}
impl Deref for HRTIM_TIME {
    type Target = hrtim_time::RegisterBlock;
    fn deref(&self) -> &hrtim_time::RegisterBlock {
        unsafe { &*HRTIM_TIME::ptr() }
    }
}
#[doc = "High Resolution Timer: TIME"]
pub mod hrtim_time;
#[doc = "High Resolution Timer: Common functions"]
pub struct HRTIM_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_COMMON {}
impl HRTIM_COMMON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hrtim_common::RegisterBlock {
        1073837952 as *const _
    }
}
impl Deref for HRTIM_COMMON {
    type Target = hrtim_common::RegisterBlock;
    fn deref(&self) -> &hrtim_common::RegisterBlock {
        unsafe { &*HRTIM_COMMON::ptr() }
    }
}
#[doc = "High Resolution Timer: Common functions"]
pub mod hrtim_common;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "DAC2"]
    pub DAC2: DAC2,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "ADC1_2"]
    pub ADC1_2: ADC1_2,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "HRTIM_MASTER"]
    pub HRTIM_MASTER: HRTIM_MASTER,
    #[doc = "HRTIM_TIMA"]
    pub HRTIM_TIMA: HRTIM_TIMA,
    #[doc = "HRTIM_TIMB"]
    pub HRTIM_TIMB: HRTIM_TIMB,
    #[doc = "HRTIM_TIMC"]
    pub HRTIM_TIMC: HRTIM_TIMC,
    #[doc = "HRTIM_TIMD"]
    pub HRTIM_TIMD: HRTIM_TIMD,
    #[doc = "HRTIM_TIME"]
    pub HRTIM_TIME: HRTIM_TIME,
    #[doc = "HRTIM_COMMON"]
    pub HRTIM_COMMON: HRTIM_COMMON,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            CAN: CAN {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            DAC2: DAC2 {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            ADC1_2: ADC1_2 {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            HRTIM_MASTER: HRTIM_MASTER {
                _marker: PhantomData,
            },
            HRTIM_TIMA: HRTIM_TIMA {
                _marker: PhantomData,
            },
            HRTIM_TIMB: HRTIM_TIMB {
                _marker: PhantomData,
            },
            HRTIM_TIMC: HRTIM_TIMC {
                _marker: PhantomData,
            },
            HRTIM_TIMD: HRTIM_TIMD {
                _marker: PhantomData,
            },
            HRTIM_TIME: HRTIM_TIME {
                _marker: PhantomData,
            },
            HRTIM_COMMON: HRTIM_COMMON {
                _marker: PhantomData,
            },
        }
    }
}
