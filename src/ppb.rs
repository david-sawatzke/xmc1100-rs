#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - SysTick Control and Status Register"]
    pub syst_csr: SYST_CSR,
    #[doc = "0x14 - SysTick Reload Value Register"]
    pub syst_rvr: SYST_RVR,
    #[doc = "0x18 - SysTick Current Value Register"]
    pub syst_cvr: SYST_CVR,
    #[doc = "0x1c - SysTick Calibration Value Register"]
    pub syst_calib: SYST_CALIB,
    _reserved4: [u8; 224usize],
    #[doc = "0x100 - Interrupt Set-enable Register"]
    pub nvic_iser: NVIC_ISER,
    _reserved5: [u8; 124usize],
    #[doc = "0x180 - IInterrupt Clear-enable Register"]
    pub nvic_icer: NVIC_ICER,
    _reserved6: [u8; 124usize],
    #[doc = "0x200 - Interrupt Set-pending Register"]
    pub nvic_ispr: NVIC_ISPR,
    _reserved7: [u8; 124usize],
    #[doc = "0x280 - Interrupt Clear-pending Register"]
    pub nvic_icpr: NVIC_ICPR,
    _reserved8: [u8; 380usize],
    #[doc = "0x400 - Interrupt Priority Register 0"]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0x404 - Interrupt Priority Register 1"]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0x408 - Interrupt Priority Register 2"]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0x40c - Interrupt Priority Register 3"]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0x410 - Interrupt Priority Register 4"]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0x414 - Interrupt Priority Register 5"]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0x418 - Interrupt Priority Register 6"]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0x41c - Interrupt Priority Register 7"]
    pub nvic_ipr7: NVIC_IPR7,
    _reserved16: [u8; 2272usize],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    _reserved18: [u8; 4usize],
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    _reserved21: [u8; 4usize],
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_csr](syst_csr) module"]
pub type SYST_CSR = crate::Reg<u32, _SYST_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CSR;
#[doc = "`read()` method returns [syst_csr::R](syst_csr::R) reader structure"]
impl crate::Readable for SYST_CSR {}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](syst_csr::W) writer structure"]
impl crate::Writable for SYST_CSR {}
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SysTick Reload Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_rvr](syst_rvr) module"]
pub type SYST_RVR = crate::Reg<u32, _SYST_RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_RVR;
#[doc = "`read()` method returns [syst_rvr::R](syst_rvr::R) reader structure"]
impl crate::Readable for SYST_RVR {}
#[doc = "`write(|w| ..)` method takes [syst_rvr::W](syst_rvr::W) writer structure"]
impl crate::Writable for SYST_RVR {}
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SysTick Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_cvr](syst_cvr) module"]
pub type SYST_CVR = crate::Reg<u32, _SYST_CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CVR;
#[doc = "`read()` method returns [syst_cvr::R](syst_cvr::R) reader structure"]
impl crate::Readable for SYST_CVR {}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](syst_cvr::W) writer structure"]
impl crate::Writable for SYST_CVR {}
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SysTick Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syst_calib](syst_calib) module"]
pub type SYST_CALIB = crate::Reg<u32, _SYST_CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYST_CALIB;
#[doc = "`read()` method returns [syst_calib::R](syst_calib::R) reader structure"]
impl crate::Readable for SYST_CALIB {}
#[doc = "SysTick Calibration Value Register"]
pub mod syst_calib;
#[doc = "Interrupt Set-enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_iser](nvic_iser) module"]
pub type NVIC_ISER = crate::Reg<u32, _NVIC_ISER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISER;
#[doc = "`read()` method returns [nvic_iser::R](nvic_iser::R) reader structure"]
impl crate::Readable for NVIC_ISER {}
#[doc = "`write(|w| ..)` method takes [nvic_iser::W](nvic_iser::W) writer structure"]
impl crate::Writable for NVIC_ISER {}
#[doc = "Interrupt Set-enable Register"]
pub mod nvic_iser;
#[doc = "IInterrupt Clear-enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icer](nvic_icer) module"]
pub type NVIC_ICER = crate::Reg<u32, _NVIC_ICER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICER;
#[doc = "`read()` method returns [nvic_icer::R](nvic_icer::R) reader structure"]
impl crate::Readable for NVIC_ICER {}
#[doc = "`write(|w| ..)` method takes [nvic_icer::W](nvic_icer::W) writer structure"]
impl crate::Writable for NVIC_ICER {}
#[doc = "IInterrupt Clear-enable Register"]
pub mod nvic_icer;
#[doc = "Interrupt Set-pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ispr](nvic_ispr) module"]
pub type NVIC_ISPR = crate::Reg<u32, _NVIC_ISPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ISPR;
#[doc = "`read()` method returns [nvic_ispr::R](nvic_ispr::R) reader structure"]
impl crate::Readable for NVIC_ISPR {}
#[doc = "`write(|w| ..)` method takes [nvic_ispr::W](nvic_ispr::W) writer structure"]
impl crate::Writable for NVIC_ISPR {}
#[doc = "Interrupt Set-pending Register"]
pub mod nvic_ispr;
#[doc = "Interrupt Clear-pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_icpr](nvic_icpr) module"]
pub type NVIC_ICPR = crate::Reg<u32, _NVIC_ICPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_ICPR;
#[doc = "`read()` method returns [nvic_icpr::R](nvic_icpr::R) reader structure"]
impl crate::Readable for NVIC_ICPR {}
#[doc = "`write(|w| ..)` method takes [nvic_icpr::W](nvic_icpr::W) writer structure"]
impl crate::Writable for NVIC_ICPR {}
#[doc = "Interrupt Clear-pending Register"]
pub mod nvic_icpr;
#[doc = "Interrupt Priority Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr0](nvic_ipr0) module"]
pub type NVIC_IPR0 = crate::Reg<u32, _NVIC_IPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR0;
#[doc = "`read()` method returns [nvic_ipr0::R](nvic_ipr0::R) reader structure"]
impl crate::Readable for NVIC_IPR0 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](nvic_ipr0::W) writer structure"]
impl crate::Writable for NVIC_IPR0 {}
#[doc = "Interrupt Priority Register 0"]
pub mod nvic_ipr0;
#[doc = "Interrupt Priority Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr1](nvic_ipr1) module"]
pub type NVIC_IPR1 = crate::Reg<u32, _NVIC_IPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR1;
#[doc = "`read()` method returns [nvic_ipr1::R](nvic_ipr1::R) reader structure"]
impl crate::Readable for NVIC_IPR1 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](nvic_ipr1::W) writer structure"]
impl crate::Writable for NVIC_IPR1 {}
#[doc = "Interrupt Priority Register 1"]
pub mod nvic_ipr1;
#[doc = "Interrupt Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr2](nvic_ipr2) module"]
pub type NVIC_IPR2 = crate::Reg<u32, _NVIC_IPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR2;
#[doc = "`read()` method returns [nvic_ipr2::R](nvic_ipr2::R) reader structure"]
impl crate::Readable for NVIC_IPR2 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](nvic_ipr2::W) writer structure"]
impl crate::Writable for NVIC_IPR2 {}
#[doc = "Interrupt Priority Register 2"]
pub mod nvic_ipr2;
#[doc = "Interrupt Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr3](nvic_ipr3) module"]
pub type NVIC_IPR3 = crate::Reg<u32, _NVIC_IPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR3;
#[doc = "`read()` method returns [nvic_ipr3::R](nvic_ipr3::R) reader structure"]
impl crate::Readable for NVIC_IPR3 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](nvic_ipr3::W) writer structure"]
impl crate::Writable for NVIC_IPR3 {}
#[doc = "Interrupt Priority Register 3"]
pub mod nvic_ipr3;
#[doc = "Interrupt Priority Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr4](nvic_ipr4) module"]
pub type NVIC_IPR4 = crate::Reg<u32, _NVIC_IPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR4;
#[doc = "`read()` method returns [nvic_ipr4::R](nvic_ipr4::R) reader structure"]
impl crate::Readable for NVIC_IPR4 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](nvic_ipr4::W) writer structure"]
impl crate::Writable for NVIC_IPR4 {}
#[doc = "Interrupt Priority Register 4"]
pub mod nvic_ipr4;
#[doc = "Interrupt Priority Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr5](nvic_ipr5) module"]
pub type NVIC_IPR5 = crate::Reg<u32, _NVIC_IPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR5;
#[doc = "`read()` method returns [nvic_ipr5::R](nvic_ipr5::R) reader structure"]
impl crate::Readable for NVIC_IPR5 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](nvic_ipr5::W) writer structure"]
impl crate::Writable for NVIC_IPR5 {}
#[doc = "Interrupt Priority Register 5"]
pub mod nvic_ipr5;
#[doc = "Interrupt Priority Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr6](nvic_ipr6) module"]
pub type NVIC_IPR6 = crate::Reg<u32, _NVIC_IPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR6;
#[doc = "`read()` method returns [nvic_ipr6::R](nvic_ipr6::R) reader structure"]
impl crate::Readable for NVIC_IPR6 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](nvic_ipr6::W) writer structure"]
impl crate::Writable for NVIC_IPR6 {}
#[doc = "Interrupt Priority Register 6"]
pub mod nvic_ipr6;
#[doc = "Interrupt Priority Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvic_ipr7](nvic_ipr7) module"]
pub type NVIC_IPR7 = crate::Reg<u32, _NVIC_IPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVIC_IPR7;
#[doc = "`read()` method returns [nvic_ipr7::R](nvic_ipr7::R) reader structure"]
impl crate::Readable for NVIC_IPR7 {}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](nvic_ipr7::W) writer structure"]
impl crate::Writable for NVIC_IPR7 {}
#[doc = "Interrupt Priority Register 7"]
pub mod nvic_ipr7;
#[doc = "CPUID Base Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "Interrupt Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icsr](icsr) module"]
pub type ICSR = crate::Reg<u32, _ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSR;
#[doc = "`read()` method returns [icsr::R](icsr::R) reader structure"]
impl crate::Readable for ICSR {}
#[doc = "`write(|w| ..)` method takes [icsr::W](icsr::W) writer structure"]
impl crate::Writable for ICSR {}
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aircr](aircr) module"]
pub type AIRCR = crate::Reg<u32, _AIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIRCR;
#[doc = "`read()` method returns [aircr::R](aircr::R) reader structure"]
impl crate::Readable for AIRCR {}
#[doc = "`write(|w| ..)` method takes [aircr::W](aircr::W) writer structure"]
impl crate::Writable for AIRCR {}
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "System Control Register"]
pub mod scr;
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "System Handler Priority Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shpr2](shpr2) module"]
pub type SHPR2 = crate::Reg<u32, _SHPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR2;
#[doc = "`read()` method returns [shpr2::R](shpr2::R) reader structure"]
impl crate::Readable for SHPR2 {}
#[doc = "`write(|w| ..)` method takes [shpr2::W](shpr2::W) writer structure"]
impl crate::Writable for SHPR2 {}
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "System Handler Priority Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shpr3](shpr3) module"]
pub type SHPR3 = crate::Reg<u32, _SHPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHPR3;
#[doc = "`read()` method returns [shpr3::R](shpr3::R) reader structure"]
impl crate::Readable for SHPR3 {}
#[doc = "`write(|w| ..)` method takes [shpr3::W](shpr3::W) writer structure"]
impl crate::Writable for SHPR3 {}
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shcsr](shcsr) module"]
pub type SHCSR = crate::Reg<u32, _SHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHCSR;
#[doc = "`read()` method returns [shcsr::R](shcsr::R) reader structure"]
impl crate::Readable for SHCSR {}
#[doc = "`write(|w| ..)` method takes [shcsr::W](shcsr::W) writer structure"]
impl crate::Writable for SHCSR {}
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
