#![doc = "Peripheral access API for GD32F30X_XD microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDGT();
    fn TAMPER();
    fn RTC();
    fn FMC();
    fn RCU_CTC();
    fn EXTI_LINE0();
    fn EXTI_LINE1();
    fn EXTI_LINE2();
    fn EXTI_LINE3();
    fn EXTI_LINE4();
    fn DMA0_CHANNEL0();
    fn DMA0_CHANNEL1();
    fn DMA0_CHANNEL2();
    fn DMA0_CHANNEL3();
    fn DMA0_CHANNEL4();
    fn DMA0_CHANNEL5();
    fn DMA0_CHANNEL6();
    fn ADC0_1();
    fn USBD_HP_CAN0_TX();
    fn USBD_LP_CAN0_RX0();
    fn CAN0_RX1();
    fn CAN0_EWMC();
    fn EXTI_LINE9_5();
    fn TIMER0_BRK();
    fn TIMER0_UP();
    fn TIMER0_TRG_CMT();
    fn TIMER0_CC();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn I2C0_EV();
    fn I2C0_ER();
    fn I2C1_EV();
    fn I2C1_ER();
    fn SPI0();
    fn SPI1();
    fn USART0();
    fn USART1();
    fn USART2();
    fn EXTI_LINE15_10();
    fn RTC_ALARM();
    fn USBD_WKUP();
    fn TIMER7_BRK();
    fn TIMER7_UP();
    fn TIMER7_TRG_CMT();
    fn TIMER7_CC();
    fn ADC2();
    fn EXMC();
    fn SDIO();
    fn TIMER4();
    fn SPI2();
    fn UART3();
    fn UART4();
    fn TIMER5();
    fn TIMER6();
    fn DMA1_CHANNEL0();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3_4();
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
pub static __INTERRUPTS: [Vector; 60] = [
    Vector { _handler: WWDGT },
    Vector { _reserved: 0 },
    Vector { _handler: TAMPER },
    Vector { _handler: RTC },
    Vector { _handler: FMC },
    Vector { _handler: RCU_CTC },
    Vector {
        _handler: EXTI_LINE0,
    },
    Vector {
        _handler: EXTI_LINE1,
    },
    Vector {
        _handler: EXTI_LINE2,
    },
    Vector {
        _handler: EXTI_LINE3,
    },
    Vector {
        _handler: EXTI_LINE4,
    },
    Vector {
        _handler: DMA0_CHANNEL0,
    },
    Vector {
        _handler: DMA0_CHANNEL1,
    },
    Vector {
        _handler: DMA0_CHANNEL2,
    },
    Vector {
        _handler: DMA0_CHANNEL3,
    },
    Vector {
        _handler: DMA0_CHANNEL4,
    },
    Vector {
        _handler: DMA0_CHANNEL5,
    },
    Vector {
        _handler: DMA0_CHANNEL6,
    },
    Vector { _handler: ADC0_1 },
    Vector {
        _handler: USBD_HP_CAN0_TX,
    },
    Vector {
        _handler: USBD_LP_CAN0_RX0,
    },
    Vector { _handler: CAN0_RX1 },
    Vector {
        _handler: CAN0_EWMC,
    },
    Vector {
        _handler: EXTI_LINE9_5,
    },
    Vector {
        _handler: TIMER0_BRK,
    },
    Vector {
        _handler: TIMER0_UP,
    },
    Vector {
        _handler: TIMER0_TRG_CMT,
    },
    Vector {
        _handler: TIMER0_CC,
    },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector { _handler: I2C0_EV },
    Vector { _handler: I2C0_ER },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: EXTI_LINE15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: USBD_WKUP,
    },
    Vector {
        _handler: TIMER7_BRK,
    },
    Vector {
        _handler: TIMER7_UP,
    },
    Vector {
        _handler: TIMER7_TRG_CMT,
    },
    Vector {
        _handler: TIMER7_CC,
    },
    Vector { _handler: ADC2 },
    Vector { _handler: EXMC },
    Vector { _handler: SDIO },
    Vector { _handler: TIMER4 },
    Vector { _handler: SPI2 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _handler: TIMER5 },
    Vector { _handler: TIMER6 },
    Vector {
        _handler: DMA1_CHANNEL0,
    },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3_4,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - WWDGT"]
    WWDGT = 0,
    #[doc = "2 - Tamper"]
    TAMPER = 2,
    #[doc = "3 - RTC"]
    RTC = 3,
    #[doc = "4 - FMC"]
    FMC = 4,
    #[doc = "5 - RCU_CTC"]
    RCU_CTC = 5,
    #[doc = "6 - EXTI_Line0"]
    EXTI_LINE0 = 6,
    #[doc = "7 - EXTI_Line1"]
    EXTI_LINE1 = 7,
    #[doc = "8 - EXTI_Line2"]
    EXTI_LINE2 = 8,
    #[doc = "9 - EXTI_Line3"]
    EXTI_LINE3 = 9,
    #[doc = "10 - EXTI_Line4"]
    EXTI_LINE4 = 10,
    #[doc = "11 - DMA0_Channel0"]
    DMA0_CHANNEL0 = 11,
    #[doc = "12 - DMA0_Channel1"]
    DMA0_CHANNEL1 = 12,
    #[doc = "13 - DMA0_Channel2"]
    DMA0_CHANNEL2 = 13,
    #[doc = "14 - DMA0_Channel3"]
    DMA0_CHANNEL3 = 14,
    #[doc = "15 - DMA0_Channel4"]
    DMA0_CHANNEL4 = 15,
    #[doc = "16 - DMA0_Channel5"]
    DMA0_CHANNEL5 = 16,
    #[doc = "17 - DMA0_Channel6"]
    DMA0_CHANNEL6 = 17,
    #[doc = "18 - ADC0_1"]
    ADC0_1 = 18,
    #[doc = "19 - USBD_HP_CAN0_TX"]
    USBD_HP_CAN0_TX = 19,
    #[doc = "20 - USBD_LP_CAN0_RX0"]
    USBD_LP_CAN0_RX0 = 20,
    #[doc = "21 - CAN0_RX1"]
    CAN0_RX1 = 21,
    #[doc = "22 - CAN0_EWMC"]
    CAN0_EWMC = 22,
    #[doc = "23 - EXTI_line9_5"]
    EXTI_LINE9_5 = 23,
    #[doc = "24 - TIMER0_BRK"]
    TIMER0_BRK = 24,
    #[doc = "25 - TIMER0_UP"]
    TIMER0_UP = 25,
    #[doc = "26 - TIMER0_TRG_CMT"]
    TIMER0_TRG_CMT = 26,
    #[doc = "27 - TIMER0_CC"]
    TIMER0_CC = 27,
    #[doc = "28 - TIMER1"]
    TIMER1 = 28,
    #[doc = "29 - TIMER2"]
    TIMER2 = 29,
    #[doc = "30 - TIMER3"]
    TIMER3 = 30,
    #[doc = "31 - I2C0_EV"]
    I2C0_EV = 31,
    #[doc = "32 - I2C0_ER"]
    I2C0_ER = 32,
    #[doc = "33 - I2C1_EV"]
    I2C1_EV = 33,
    #[doc = "34 - I2C1_ER"]
    I2C1_ER = 34,
    #[doc = "35 - SPI0"]
    SPI0 = 35,
    #[doc = "36 - SPI1"]
    SPI1 = 36,
    #[doc = "37 - USART0"]
    USART0 = 37,
    #[doc = "38 - USART1"]
    USART1 = 38,
    #[doc = "39 - USART2"]
    USART2 = 39,
    #[doc = "40 - EXTI_line15_10"]
    EXTI_LINE15_10 = 40,
    #[doc = "41 - RTC_Alarm"]
    RTC_ALARM = 41,
    #[doc = "42 - USBD_WKUP"]
    USBD_WKUP = 42,
    #[doc = "43 - TIMER7_BRK"]
    TIMER7_BRK = 43,
    #[doc = "44 - TIMER7_UP"]
    TIMER7_UP = 44,
    #[doc = "45 - TIMER7_TRG_CMT"]
    TIMER7_TRG_CMT = 45,
    #[doc = "46 - TIMER7_CC"]
    TIMER7_CC = 46,
    #[doc = "47 - ADC2"]
    ADC2 = 47,
    #[doc = "48 - EXMC"]
    EXMC = 48,
    #[doc = "49 - SDIO"]
    SDIO = 49,
    #[doc = "50 - TIMER4"]
    TIMER4 = 50,
    #[doc = "51 - SPI2"]
    SPI2 = 51,
    #[doc = "52 - UART3"]
    UART3 = 52,
    #[doc = "53 - UART4"]
    UART4 = 53,
    #[doc = "54 - TIMER5"]
    TIMER5 = 54,
    #[doc = "55 - TIMER6"]
    TIMER6 = 55,
    #[doc = "56 - DMA1_Channel0"]
    DMA1_CHANNEL0 = 56,
    #[doc = "57 - DMA1_Channel1"]
    DMA1_CHANNEL1 = 57,
    #[doc = "58 - DMA1_Channel2"]
    DMA1_CHANNEL2 = 58,
    #[doc = "59 - DMA1_Channel3_4"]
    DMA1_CHANNEL3_4 = 59,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog to digital converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4001_2400 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog to digital converter"]
pub mod adc0;
#[doc = "Analog to digital converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4001_2800 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog to digital converter"]
pub mod adc1;
#[doc = "Analog to digital converter"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4001_3c00 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "Alternate-function I/Os"]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFIO {}
impl AFIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afio::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFIO::ptr() }
    }
}
#[doc = "Alternate-function I/Os"]
pub mod afio;
#[doc = "Backup registers"]
pub struct BKP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BKP {}
impl BKP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bkp::RegisterBlock {
        0x4000_6c00 as *const _
    }
}
impl Deref for BKP {
    type Target = bkp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BKP::ptr() }
    }
}
#[doc = "Backup registers"]
pub mod bkp;
#[doc = "Controller area network"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4000_6400 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can0;
#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Clock trim controller"]
pub struct CTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTC {}
impl CTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctc::RegisterBlock {
        0x4000_c800 as *const _
    }
}
impl Deref for CTC {
    type Target = ctc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTC::ptr() }
    }
}
#[doc = "Clock trim controller"]
pub mod ctc;
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Debug support"]
pub struct DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBG {}
impl DBG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbg::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for DBG {
    type Target = dbg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBG::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbg;
#[doc = "DMA controller"]
pub struct DMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for DMA0 {
    type Target = dma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA0::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma0;
#[doc = "DMA controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "External memory controller"]
pub struct EXMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXMC {}
impl EXMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exmc::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for EXMC {
    type Target = exmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXMC::ptr() }
    }
}
#[doc = "External memory controller"]
pub mod exmc;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "free watchdog timer"]
pub struct FWDGT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FWDGT {}
impl FWDGT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fwdgt::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for FWDGT {
    type Target = fwdgt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FWDGT::ptr() }
    }
}
#[doc = "free watchdog timer"]
pub mod fwdgt;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_0800 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
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
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_0c00 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_1400 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_1800 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_1c00 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
pub mod i2c0;
#[doc = "Inter integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Power management unit"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "Power management unit"]
pub mod pmu;
#[doc = "Reset and clock unit"]
pub struct RCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCU {}
impl RCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcu::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for RCU {
    type Target = rcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCU::ptr() }
    }
}
#[doc = "Reset and clock unit"]
pub mod rcu;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Secure digital input/output interface"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdio::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdio;
#[doc = "Serial peripheral interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi0;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod timer0;
#[doc = "General-purpose-timers"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod timer1;
#[doc = "General-purpose-timers"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER4 {}
impl TIMER4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIMER4 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER4::ptr() }
    }
}
#[doc = "Basic-timers"]
pub struct TIMER5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER5 {}
impl TIMER5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer5::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIMER5 {
    type Target = timer5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER5::ptr() }
    }
}
#[doc = "Basic-timers"]
pub mod timer5;
#[doc = "Basic-timers"]
pub struct TIMER6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER6 {}
impl TIMER6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer5::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for TIMER6 {
    type Target = timer5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER6::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub struct TIMER7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER7 {}
impl TIMER7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4001_3400 as *const _
    }
}
impl Deref for TIMER7 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER7::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart0;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal asynchronous receiver transmitter"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        0x4000_4c00 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Universal asynchronous receiver transmitter"]
pub mod uart3;
#[doc = "Universal asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub struct USBD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBD {}
impl USBD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbd::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for USBD {
    type Target = usbd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBD::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub mod usbd;
#[doc = "Window watchdog timer"]
pub struct WWDGT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDGT {}
impl WWDGT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdgt::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDGT {
    type Target = wwdgt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDGT::ptr() }
    }
}
#[doc = "Window watchdog timer"]
pub mod wwdgt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "AFIO"]
    pub AFIO: AFIO,
    #[doc = "BKP"]
    pub BKP: BKP,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "CTC"]
    pub CTC: CTC,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DBG"]
    pub DBG: DBG,
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "EXMC"]
    pub EXMC: EXMC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "FWDGT"]
    pub FWDGT: FWDGT,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "RCU"]
    pub RCU: RCU,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: TIMER4,
    #[doc = "TIMER5"]
    pub TIMER5: TIMER5,
    #[doc = "TIMER6"]
    pub TIMER6: TIMER6,
    #[doc = "TIMER7"]
    pub TIMER7: TIMER7,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "USBD"]
    pub USBD: USBD,
    #[doc = "WWDGT"]
    pub WWDGT: WWDGT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            AFIO: AFIO {
                _marker: PhantomData,
            },
            BKP: BKP {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            CTC: CTC {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DBG: DBG {
                _marker: PhantomData,
            },
            DMA0: DMA0 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            EXMC: EXMC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            FWDGT: FWDGT {
                _marker: PhantomData,
            },
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
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            RCU: RCU {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            TIMER4: TIMER4 {
                _marker: PhantomData,
            },
            TIMER5: TIMER5 {
                _marker: PhantomData,
            },
            TIMER6: TIMER6 {
                _marker: PhantomData,
            },
            TIMER7: TIMER7 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            USBD: USBD {
                _marker: PhantomData,
            },
            WWDGT: WWDGT {
                _marker: PhantomData,
            },
        }
    }
}
