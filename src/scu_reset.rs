#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    pub rststat: RSTSTAT,
    #[doc = "0x04 - RCU Reset Set Register"]
    pub rstset: RSTSET,
    #[doc = "0x08 - RCU Reset Clear Register"]
    pub rstclr: RSTCLR,
    #[doc = "0x0c - RCU Reset Control Register"]
    pub rstcon: RSTCON,
}
#[doc = "RCU Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rststat](rststat) module"]
pub type RSTSTAT = crate::Reg<u32, _RSTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTSTAT;
#[doc = "`read()` method returns [rststat::R](rststat::R) reader structure"]
impl crate::Readable for RSTSTAT {}
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RCU Reset Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstset](rstset) module"]
pub type RSTSET = crate::Reg<u32, _RSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTSET;
#[doc = "`write(|w| ..)` method takes [rstset::W](rstset::W) writer structure"]
impl crate::Writable for RSTSET {}
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RCU Reset Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstclr](rstclr) module"]
pub type RSTCLR = crate::Reg<u32, _RSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCLR;
#[doc = "`write(|w| ..)` method takes [rstclr::W](rstclr::W) writer structure"]
impl crate::Writable for RSTCLR {}
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "RCU Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstcon](rstcon) module"]
pub type RSTCON = crate::Reg<u32, _RSTCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCON;
#[doc = "`read()` method returns [rstcon::R](rstcon::R) reader structure"]
impl crate::Readable for RSTCON {}
#[doc = "`write(|w| ..)` method takes [rstcon::W](rstcon::W) writer structure"]
impl crate::Writable for RSTCON {}
#[doc = "RCU Reset Control Register"]
pub mod rstcon;
