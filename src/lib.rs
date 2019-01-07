#![doc = "Peripheral access API for XMC1100 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn SCU_0();
    fn SCU_1();
    fn SCU_2();
    fn ERU0_0();
    fn ERU0_1();
    fn ERU0_2();
    fn ERU0_3();
    fn USIC0_0();
    fn USIC0_1();
    fn USIC0_2();
    fn USIC0_3();
    fn USIC0_4();
    fn USIC0_5();
    fn VADC0_C0_0();
    fn VADC0_C0_1();
    fn CCU40_0();
    fn CCU40_1();
    fn CCU40_2();
    fn CCU40_3();
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
pub static __INTERRUPTS: [Vector; 25] = [
    Vector { _handler: SCU_0 },
    Vector { _handler: SCU_1 },
    Vector { _handler: SCU_2 },
    Vector { _handler: ERU0_0 },
    Vector { _handler: ERU0_1 },
    Vector { _handler: ERU0_2 },
    Vector { _handler: ERU0_3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USIC0_0 },
    Vector { _handler: USIC0_1 },
    Vector { _handler: USIC0_2 },
    Vector { _handler: USIC0_3 },
    Vector { _handler: USIC0_4 },
    Vector { _handler: USIC0_5 },
    Vector {
        _handler: VADC0_C0_0,
    },
    Vector {
        _handler: VADC0_C0_1,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CCU40_0 },
    Vector { _handler: CCU40_1 },
    Vector { _handler: CCU40_2 },
    Vector { _handler: CCU40_3 },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - System Control"]
    SCU_0,
    #[doc = "1 - System Control"]
    SCU_1,
    #[doc = "2 - System Control"]
    SCU_2,
    #[doc = "3 - External Request Unit 0"]
    ERU0_0,
    #[doc = "4 - External Request Unit 0"]
    ERU0_1,
    #[doc = "5 - External Request Unit 0"]
    ERU0_2,
    #[doc = "6 - External Request Unit 0"]
    ERU0_3,
    #[doc = "9 - Universal Serial Interface Channel (Module 0)"]
    USIC0_0,
    #[doc = "10 - Universal Serial Interface Channel (Module 0)"]
    USIC0_1,
    #[doc = "11 - Universal Serial Interface Channel (Module 0)"]
    USIC0_2,
    #[doc = "12 - Universal Serial Interface Channel (Module 0)"]
    USIC0_3,
    #[doc = "13 - Universal Serial Interface Channel (Module 0)"]
    USIC0_4,
    #[doc = "14 - Universal Serial Interface Channel (Module 0)"]
    USIC0_5,
    #[doc = "15 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_0,
    #[doc = "16 - Analog to Digital Converter Common Block 0"]
    VADC0_C0_1,
    #[doc = "21 - Capture Compare Unit 4 (Module 0)"]
    CCU40_0,
    #[doc = "22 - Capture Compare Unit 4 (Module 0)"]
    CCU40_1,
    #[doc = "23 - Capture Compare Unit 4 (Module 0)"]
    CCU40_2,
    #[doc = "24 - Capture Compare Unit 4 (Module 0)"]
    CCU40_3,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SCU_0 => 0,
            Interrupt::SCU_1 => 1,
            Interrupt::SCU_2 => 2,
            Interrupt::ERU0_0 => 3,
            Interrupt::ERU0_1 => 4,
            Interrupt::ERU0_2 => 5,
            Interrupt::ERU0_3 => 6,
            Interrupt::USIC0_0 => 9,
            Interrupt::USIC0_1 => 10,
            Interrupt::USIC0_2 => 11,
            Interrupt::USIC0_3 => 12,
            Interrupt::USIC0_4 => 13,
            Interrupt::USIC0_5 => 14,
            Interrupt::VADC0_C0_0 => 15,
            Interrupt::VADC0_C0_1 => 16,
            Interrupt::CCU40_0 => 21,
            Interrupt::CCU40_1 => 22,
            Interrupt::CCU40_2 => 23,
            Interrupt::CCU40_3 => 24,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Cortex-M0 Private Peripheral Block"]
pub struct PPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPB {}
impl PPB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ppb::RegisterBlock {
        3758153728 as *const _
    }
}
impl Deref for PPB {
    type Target = ppb::RegisterBlock;
    fn deref(&self) -> &ppb::RegisterBlock {
        unsafe { &*PPB::ptr() }
    }
}
#[doc = "Cortex-M0 Private Peripheral Block"]
pub mod ppb;
#[doc = "Event Request Unit 0"]
pub struct ERU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERU0 {}
impl ERU0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eru0::RegisterBlock {
        1073808896 as *const _
    }
}
impl Deref for ERU0 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &eru0::RegisterBlock {
        unsafe { &*ERU0::ptr() }
    }
}
#[doc = "Event Request Unit 0"]
pub mod eru0;
#[doc = "PAU Unit"]
pub struct PAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAU {}
impl PAU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pau::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for PAU {
    type Target = pau::RegisterBlock;
    fn deref(&self) -> &pau::RegisterBlock {
        unsafe { &*PAU::ptr() }
    }
}
#[doc = "PAU Unit"]
pub mod pau;
#[doc = "NVM Unit"]
pub struct NVM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVM {}
impl NVM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvm::RegisterBlock {
        1074069504 as *const _
    }
}
impl Deref for NVM {
    type Target = nvm::RegisterBlock;
    fn deref(&self) -> &nvm::RegisterBlock {
        unsafe { &*NVM::ptr() }
    }
}
#[doc = "NVM Unit"]
pub mod nvm;
#[doc = "Watch Dog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watch Dog Timer"]
pub mod wdt;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073809920 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "PRNG Unit"]
pub struct PRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRNG {}
impl PRNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prng::RegisterBlock {
        1208090624 as *const _
    }
}
impl Deref for PRNG {
    type Target = prng::RegisterBlock;
    fn deref(&self) -> &prng::RegisterBlock {
        unsafe { &*PRNG::ptr() }
    }
}
#[doc = "PRNG Unit"]
pub mod prng;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0 {}
impl USIC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0::RegisterBlock {
        1207959560 as *const _
    }
}
impl Deref for USIC0 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &usic0::RegisterBlock {
        unsafe { &*USIC0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH0 {}
impl USIC0_CH0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0_ch0::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for USIC0_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        unsafe { &*USIC0_CH0::ptr() }
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0_ch0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USIC0_CH1 {}
impl USIC0_CH1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usic0_ch0::RegisterBlock {
        1207960064 as *const _
    }
}
impl Deref for USIC0_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        unsafe { &*USIC0_CH1::ptr() }
    }
}
#[doc = "System Control Unit"]
pub struct SCU_GENERAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_GENERAL {}
impl SCU_GENERAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_general::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for SCU_GENERAL {
    type Target = scu_general::RegisterBlock;
    fn deref(&self) -> &scu_general::RegisterBlock {
        unsafe { &*SCU_GENERAL::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_general;
#[doc = "System Control Unit"]
pub struct SCU_INTERRUPT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_INTERRUPT {}
impl SCU_INTERRUPT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_interrupt::RegisterBlock {
        1073807416 as *const _
    }
}
impl Deref for SCU_INTERRUPT {
    type Target = scu_interrupt::RegisterBlock;
    fn deref(&self) -> &scu_interrupt::RegisterBlock {
        unsafe { &*SCU_INTERRUPT::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_interrupt;
#[doc = "System Control Unit"]
pub struct SCU_POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_POWER {}
impl SCU_POWER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_power::RegisterBlock {
        1073807872 as *const _
    }
}
impl Deref for SCU_POWER {
    type Target = scu_power::RegisterBlock;
    fn deref(&self) -> &scu_power::RegisterBlock {
        unsafe { &*SCU_POWER::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_power;
#[doc = "System Control Unit"]
pub struct SCU_CLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_CLK {}
impl SCU_CLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_clk::RegisterBlock {
        1073808128 as *const _
    }
}
impl Deref for SCU_CLK {
    type Target = scu_clk::RegisterBlock;
    fn deref(&self) -> &scu_clk::RegisterBlock {
        unsafe { &*SCU_CLK::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_clk;
#[doc = "System Control Unit"]
pub struct SCU_RESET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_RESET {}
impl SCU_RESET {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_reset::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for SCU_RESET {
    type Target = scu_reset::RegisterBlock;
    fn deref(&self) -> &scu_reset::RegisterBlock {
        unsafe { &*SCU_RESET::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_reset;
#[doc = "System Control Unit"]
pub struct SCU_ANALOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCU_ANALOG {}
impl SCU_ANALOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scu_analog::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for SCU_ANALOG {
    type Target = scu_analog::RegisterBlock;
    fn deref(&self) -> &scu_analog::RegisterBlock {
        unsafe { &*SCU_ANALOG::ptr() }
    }
}
#[doc = "System Control Unit"]
pub mod scu_analog;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40 {}
impl CCU40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40::RegisterBlock {
        1208221696 as *const _
    }
}
impl Deref for CCU40 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        unsafe { &*CCU40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC40 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC40 {}
impl CCU40_CC40 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1208221952 as *const _
    }
}
impl Deref for CCU40_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC40::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40_cc40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC41 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC41 {}
impl CCU40_CC41 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1208222208 as *const _
    }
}
impl Deref for CCU40_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC41::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC42 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC42 {}
impl CCU40_CC42 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1208222464 as *const _
    }
}
impl Deref for CCU40_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC42::ptr() }
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC43 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU40_CC43 {}
impl CCU40_CC43 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccu40_cc40::RegisterBlock {
        1208222720 as *const _
    }
}
impl Deref for CCU40_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        unsafe { &*CCU40_CC43::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub struct VADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VADC {}
impl VADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vadc::RegisterBlock {
        1208156160 as *const _
    }
}
impl Deref for VADC {
    type Target = vadc::RegisterBlock;
    fn deref(&self) -> &vadc::RegisterBlock {
        unsafe { &*VADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod vadc;
#[doc = "Sample and Hold ADC Sequencer"]
pub struct SHS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHS0 {}
impl SHS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const shs0::RegisterBlock {
        1208172544 as *const _
    }
}
impl Deref for SHS0 {
    type Target = shs0::RegisterBlock;
    fn deref(&self) -> &shs0::RegisterBlock {
        unsafe { &*SHS0::ptr() }
    }
}
#[doc = "Sample and Hold ADC Sequencer"]
pub mod shs0;
#[doc = "Port 0"]
pub struct PORT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT0 {}
impl PORT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for PORT0 {
    type Target = port0::RegisterBlock;
    fn deref(&self) -> &port0::RegisterBlock {
        unsafe { &*PORT0::ptr() }
    }
}
#[doc = "Port 0"]
pub mod port0;
#[doc = "Port 1"]
pub struct PORT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT1 {}
impl PORT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port1::RegisterBlock {
        1074004224 as *const _
    }
}
impl Deref for PORT1 {
    type Target = port1::RegisterBlock;
    fn deref(&self) -> &port1::RegisterBlock {
        unsafe { &*PORT1::ptr() }
    }
}
#[doc = "Port 1"]
pub mod port1;
#[doc = "Port 2"]
pub struct PORT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT2 {}
impl PORT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port2::RegisterBlock {
        1074004480 as *const _
    }
}
impl Deref for PORT2 {
    type Target = port2::RegisterBlock;
    fn deref(&self) -> &port2::RegisterBlock {
        unsafe { &*PORT2::ptr() }
    }
}
#[doc = "Port 2"]
pub mod port2;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PPB"]
    pub PPB: PPB,
    #[doc = "ERU0"]
    pub ERU0: ERU0,
    #[doc = "PAU"]
    pub PAU: PAU,
    #[doc = "NVM"]
    pub NVM: NVM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "PRNG"]
    pub PRNG: PRNG,
    #[doc = "USIC0"]
    pub USIC0: USIC0,
    #[doc = "USIC0_CH0"]
    pub USIC0_CH0: USIC0_CH0,
    #[doc = "USIC0_CH1"]
    pub USIC0_CH1: USIC0_CH1,
    #[doc = "SCU_GENERAL"]
    pub SCU_GENERAL: SCU_GENERAL,
    #[doc = "SCU_INTERRUPT"]
    pub SCU_INTERRUPT: SCU_INTERRUPT,
    #[doc = "SCU_POWER"]
    pub SCU_POWER: SCU_POWER,
    #[doc = "SCU_CLK"]
    pub SCU_CLK: SCU_CLK,
    #[doc = "SCU_RESET"]
    pub SCU_RESET: SCU_RESET,
    #[doc = "SCU_ANALOG"]
    pub SCU_ANALOG: SCU_ANALOG,
    #[doc = "CCU40"]
    pub CCU40: CCU40,
    #[doc = "CCU40_CC40"]
    pub CCU40_CC40: CCU40_CC40,
    #[doc = "CCU40_CC41"]
    pub CCU40_CC41: CCU40_CC41,
    #[doc = "CCU40_CC42"]
    pub CCU40_CC42: CCU40_CC42,
    #[doc = "CCU40_CC43"]
    pub CCU40_CC43: CCU40_CC43,
    #[doc = "VADC"]
    pub VADC: VADC,
    #[doc = "SHS0"]
    pub SHS0: SHS0,
    #[doc = "PORT0"]
    pub PORT0: PORT0,
    #[doc = "PORT1"]
    pub PORT1: PORT1,
    #[doc = "PORT2"]
    pub PORT2: PORT2,
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
            PPB: PPB {
                _marker: PhantomData,
            },
            ERU0: ERU0 {
                _marker: PhantomData,
            },
            PAU: PAU {
                _marker: PhantomData,
            },
            NVM: NVM {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            PRNG: PRNG {
                _marker: PhantomData,
            },
            USIC0: USIC0 {
                _marker: PhantomData,
            },
            USIC0_CH0: USIC0_CH0 {
                _marker: PhantomData,
            },
            USIC0_CH1: USIC0_CH1 {
                _marker: PhantomData,
            },
            SCU_GENERAL: SCU_GENERAL {
                _marker: PhantomData,
            },
            SCU_INTERRUPT: SCU_INTERRUPT {
                _marker: PhantomData,
            },
            SCU_POWER: SCU_POWER {
                _marker: PhantomData,
            },
            SCU_CLK: SCU_CLK {
                _marker: PhantomData,
            },
            SCU_RESET: SCU_RESET {
                _marker: PhantomData,
            },
            SCU_ANALOG: SCU_ANALOG {
                _marker: PhantomData,
            },
            CCU40: CCU40 {
                _marker: PhantomData,
            },
            CCU40_CC40: CCU40_CC40 {
                _marker: PhantomData,
            },
            CCU40_CC41: CCU40_CC41 {
                _marker: PhantomData,
            },
            CCU40_CC42: CCU40_CC42 {
                _marker: PhantomData,
            },
            CCU40_CC43: CCU40_CC43 {
                _marker: PhantomData,
            },
            VADC: VADC {
                _marker: PhantomData,
            },
            SHS0: SHS0 {
                _marker: PhantomData,
            },
            PORT0: PORT0 {
                _marker: PhantomData,
            },
            PORT1: PORT1 {
                _marker: PhantomData,
            },
            PORT2: PORT2 {
                _marker: PhantomData,
            },
        }
    }
}
