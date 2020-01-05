#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PRNG Word Register"]
    pub word: WORD,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - PRNG Status Check Register"]
    pub chk: CHK,
    _reserved2: [u8; 6usize],
    #[doc = "0x0c - PRNG Control Register"]
    pub ctrl: CTRL,
}
#[doc = "PRNG Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [word](word) module"]
pub type WORD = crate::Reg<u16, _WORD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WORD;
#[doc = "`read()` method returns [word::R](word::R) reader structure"]
impl crate::Readable for WORD {}
#[doc = "`write(|w| ..)` method takes [word::W](word::W) writer structure"]
impl crate::Writable for WORD {}
#[doc = "PRNG Word Register"]
pub mod word;
#[doc = "PRNG Status Check Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chk](chk) module"]
pub type CHK = crate::Reg<u16, _CHK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHK;
#[doc = "`read()` method returns [chk::R](chk::R) reader structure"]
impl crate::Readable for CHK {}
#[doc = "PRNG Status Check Register"]
pub mod chk;
#[doc = "PRNG Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "PRNG Control Register"]
pub mod ctrl;
