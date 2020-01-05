#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved1: [u8; 52usize],
    #[doc = "0x40 - SHS Configuration Register"]
    pub shscfg: SHSCFG,
    #[doc = "0x44 - Stepper Configuration Register"]
    pub stepcfg: STEPCFG,
    _reserved3: [u8; 8usize],
    #[doc = "0x50 - Loop Control Register"]
    pub loop_: LOOP,
    _reserved4: [u8; 44usize],
    #[doc = "0x80 - Timing Configuration Register 0"]
    pub timcfg0: TIMCFG0,
    #[doc = "0x84 - Timing Configuration Register 1"]
    pub timcfg1: TIMCFG1,
    _reserved6: [u8; 52usize],
    #[doc = "0xbc - Calibration Control Register"]
    pub calctr: CALCTR,
    #[doc = "0xc0 - Gain Calibration Control Register 0"]
    pub calgc0: CALGC0,
    #[doc = "0xc4 - Gain Calibration Control Register 1"]
    pub calgc1: CALGC1,
    _reserved9: [u8; 184usize],
    #[doc = "0x180 - Gain Control Register 00"]
    pub gnctr00: GNCTR00,
    _reserved10: [u8; 12usize],
    #[doc = "0x190 - Gain Control Register 10"]
    pub gnctr10: GNCTR10,
}
#[doc = "Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "SHS Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shscfg](shscfg) module"]
pub type SHSCFG = crate::Reg<u32, _SHSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHSCFG;
#[doc = "`read()` method returns [shscfg::R](shscfg::R) reader structure"]
impl crate::Readable for SHSCFG {}
#[doc = "`write(|w| ..)` method takes [shscfg::W](shscfg::W) writer structure"]
impl crate::Writable for SHSCFG {}
#[doc = "SHS Configuration Register"]
pub mod shscfg;
#[doc = "Stepper Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stepcfg](stepcfg) module"]
pub type STEPCFG = crate::Reg<u32, _STEPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STEPCFG;
#[doc = "`read()` method returns [stepcfg::R](stepcfg::R) reader structure"]
impl crate::Readable for STEPCFG {}
#[doc = "`write(|w| ..)` method takes [stepcfg::W](stepcfg::W) writer structure"]
impl crate::Writable for STEPCFG {}
#[doc = "Stepper Configuration Register"]
pub mod stepcfg;
#[doc = "Timing Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timcfg0](timcfg0) module"]
pub type TIMCFG0 = crate::Reg<u32, _TIMCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG0;
#[doc = "`read()` method returns [timcfg0::R](timcfg0::R) reader structure"]
impl crate::Readable for TIMCFG0 {}
#[doc = "`write(|w| ..)` method takes [timcfg0::W](timcfg0::W) writer structure"]
impl crate::Writable for TIMCFG0 {}
#[doc = "Timing Configuration Register 0"]
pub mod timcfg0;
#[doc = "Timing Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timcfg1](timcfg1) module"]
pub type TIMCFG1 = crate::Reg<u32, _TIMCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG1;
#[doc = "`read()` method returns [timcfg1::R](timcfg1::R) reader structure"]
impl crate::Readable for TIMCFG1 {}
#[doc = "`write(|w| ..)` method takes [timcfg1::W](timcfg1::W) writer structure"]
impl crate::Writable for TIMCFG1 {}
#[doc = "Timing Configuration Register 1"]
pub mod timcfg1;
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calctr](calctr) module"]
pub type CALCTR = crate::Reg<u32, _CALCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALCTR;
#[doc = "`read()` method returns [calctr::R](calctr::R) reader structure"]
impl crate::Readable for CALCTR {}
#[doc = "`write(|w| ..)` method takes [calctr::W](calctr::W) writer structure"]
impl crate::Writable for CALCTR {}
#[doc = "Calibration Control Register"]
pub mod calctr;
#[doc = "Gain Calibration Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calgc0](calgc0) module"]
pub type CALGC0 = crate::Reg<u32, _CALGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALGC0;
#[doc = "`read()` method returns [calgc0::R](calgc0::R) reader structure"]
impl crate::Readable for CALGC0 {}
#[doc = "`write(|w| ..)` method takes [calgc0::W](calgc0::W) writer structure"]
impl crate::Writable for CALGC0 {}
#[doc = "Gain Calibration Control Register 0"]
pub mod calgc0;
#[doc = "Gain Calibration Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calgc1](calgc1) module"]
pub type CALGC1 = crate::Reg<u32, _CALGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALGC1;
#[doc = "`read()` method returns [calgc1::R](calgc1::R) reader structure"]
impl crate::Readable for CALGC1 {}
#[doc = "`write(|w| ..)` method takes [calgc1::W](calgc1::W) writer structure"]
impl crate::Writable for CALGC1 {}
#[doc = "Gain Calibration Control Register 1"]
pub mod calgc1;
#[doc = "Gain Control Register 00\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnctr00](gnctr00) module"]
pub type GNCTR00 = crate::Reg<u32, _GNCTR00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNCTR00;
#[doc = "`read()` method returns [gnctr00::R](gnctr00::R) reader structure"]
impl crate::Readable for GNCTR00 {}
#[doc = "`write(|w| ..)` method takes [gnctr00::W](gnctr00::W) writer structure"]
impl crate::Writable for GNCTR00 {}
#[doc = "Gain Control Register 00"]
pub mod gnctr00;
#[doc = "Gain Control Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnctr10](gnctr10) module"]
pub type GNCTR10 = crate::Reg<u32, _GNCTR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNCTR10;
#[doc = "`read()` method returns [gnctr10::R](gnctr10::R) reader structure"]
impl crate::Readable for GNCTR10 {}
#[doc = "`write(|w| ..)` method takes [gnctr10::W](gnctr10::W) writer structure"]
impl crate::Writable for GNCTR10 {}
#[doc = "Gain Control Register 10"]
pub mod gnctr10;
#[doc = "Loop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [loop_](loop_) module"]
pub type LOOP = crate::Reg<u32, _LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOOP;
#[doc = "`read()` method returns [loop_::R](loop_::R) reader structure"]
impl crate::Readable for LOOP {}
#[doc = "`write(|w| ..)` method takes [loop_::W](loop_::W) writer structure"]
impl crate::Writable for LOOP {}
#[doc = "Loop Control Register"]
pub mod loop_;
