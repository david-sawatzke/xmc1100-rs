#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage Detector Status Register"]
    pub vdesr: VDESR,
}
#[doc = "Voltage Detector Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vdesr](vdesr) module"]
pub type VDESR = crate::Reg<u32, _VDESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDESR;
#[doc = "`read()` method returns [vdesr::R](vdesr::R) reader structure"]
impl crate::Readable for VDESR {}
#[doc = "Voltage Detector Status Register"]
pub mod vdesr;
