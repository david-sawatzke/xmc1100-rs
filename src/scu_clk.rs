#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clkcr: CLKCR,
    #[doc = "0x04 - Power Save Control Register"]
    pub pwrsvcr: PWRSVCR,
    #[doc = "0x08 - Peripheral 0 Clock Gating Status"]
    pub cgatstat0: CGATSTAT0,
    #[doc = "0x0c - Peripheral 0 Clock Gating Set"]
    pub cgatset0: CGATSET0,
    #[doc = "0x10 - Peripheral 0 Clock Gating Clear"]
    pub cgatclr0: CGATCLR0,
    #[doc = "0x14 - Oscillator Control and Status Register"]
    pub osccsr: OSCCSR,
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkcr](clkcr) module"]
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
#[doc = "`read()` method returns [clkcr::R](clkcr::R) reader structure"]
impl crate::Readable for CLKCR {}
#[doc = "`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure"]
impl crate::Writable for CLKCR {}
#[doc = "Clock Control Register"]
pub mod clkcr;
#[doc = "Power Save Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwrsvcr](pwrsvcr) module"]
pub type PWRSVCR = crate::Reg<u32, _PWRSVCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRSVCR;
#[doc = "`read()` method returns [pwrsvcr::R](pwrsvcr::R) reader structure"]
impl crate::Readable for PWRSVCR {}
#[doc = "`write(|w| ..)` method takes [pwrsvcr::W](pwrsvcr::W) writer structure"]
impl crate::Writable for PWRSVCR {}
#[doc = "Power Save Control Register"]
pub mod pwrsvcr;
#[doc = "Peripheral 0 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgatstat0](cgatstat0) module"]
pub type CGATSTAT0 = crate::Reg<u32, _CGATSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSTAT0;
#[doc = "`read()` method returns [cgatstat0::R](cgatstat0::R) reader structure"]
impl crate::Readable for CGATSTAT0 {}
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "Peripheral 0 Clock Gating Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgatset0](cgatset0) module"]
pub type CGATSET0 = crate::Reg<u32, _CGATSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATSET0;
#[doc = "`write(|w| ..)` method takes [cgatset0::W](cgatset0::W) writer structure"]
impl crate::Writable for CGATSET0 {}
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "Peripheral 0 Clock Gating Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgatclr0](cgatclr0) module"]
pub type CGATCLR0 = crate::Reg<u32, _CGATCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGATCLR0;
#[doc = "`write(|w| ..)` method takes [cgatclr0::W](cgatclr0::W) writer structure"]
impl crate::Writable for CGATCLR0 {}
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "Oscillator Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [osccsr](osccsr) module"]
pub type OSCCSR = crate::Reg<u32, _OSCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCSR;
#[doc = "`read()` method returns [osccsr::R](osccsr::R) reader structure"]
impl crate::Readable for OSCCSR {}
#[doc = "`write(|w| ..)` method takes [osccsr::W](osccsr::W) writer structure"]
impl crate::Writable for OSCCSR {}
#[doc = "Oscillator Control and Status Register"]
pub mod osccsr;
