#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 64usize],
    #[doc = "0x40 - Peripheral Availability Register 0"]
    pub avail0: AVAIL0,
    #[doc = "0x44 - Peripheral Availability Register 1"]
    pub avail1: AVAIL1,
    #[doc = "0x48 - Peripheral Availability Register 2"]
    pub avail2: AVAIL2,
    _reserved3: [u8; 52usize],
    #[doc = "0x80 - Peripheral Privilege Access Register 0"]
    pub privdis0: PRIVDIS0,
    #[doc = "0x84 - Peripheral Privilege Access Register 1"]
    pub privdis1: PRIVDIS1,
    _reserved5: [u8; 888usize],
    #[doc = "0x400 - ROM Size Register"]
    pub romsize: ROMSIZE,
    #[doc = "0x404 - Flash Size Register"]
    pub flsize: FLSIZE,
    _reserved7: [u8; 8usize],
    #[doc = "0x410 - RAM0 Size Register"]
    pub ram0size: RAM0SIZE,
}
#[doc = "Peripheral Availability Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [avail0](avail0) module"]
pub type AVAIL0 = crate::Reg<u32, _AVAIL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AVAIL0;
#[doc = "`read()` method returns [avail0::R](avail0::R) reader structure"]
impl crate::Readable for AVAIL0 {}
#[doc = "Peripheral Availability Register 0"]
pub mod avail0;
#[doc = "Peripheral Availability Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [avail1](avail1) module"]
pub type AVAIL1 = crate::Reg<u32, _AVAIL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AVAIL1;
#[doc = "`read()` method returns [avail1::R](avail1::R) reader structure"]
impl crate::Readable for AVAIL1 {}
#[doc = "Peripheral Availability Register 1"]
pub mod avail1;
#[doc = "Peripheral Availability Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [avail2](avail2) module"]
pub type AVAIL2 = crate::Reg<u32, _AVAIL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AVAIL2;
#[doc = "`read()` method returns [avail2::R](avail2::R) reader structure"]
impl crate::Readable for AVAIL2 {}
#[doc = "`write(|w| ..)` method takes [avail2::W](avail2::W) writer structure"]
impl crate::Writable for AVAIL2 {}
#[doc = "Peripheral Availability Register 2"]
pub mod avail2;
#[doc = "Peripheral Privilege Access Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [privdis0](privdis0) module"]
pub type PRIVDIS0 = crate::Reg<u32, _PRIVDIS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIVDIS0;
#[doc = "`read()` method returns [privdis0::R](privdis0::R) reader structure"]
impl crate::Readable for PRIVDIS0 {}
#[doc = "`write(|w| ..)` method takes [privdis0::W](privdis0::W) writer structure"]
impl crate::Writable for PRIVDIS0 {}
#[doc = "Peripheral Privilege Access Register 0"]
pub mod privdis0;
#[doc = "Peripheral Privilege Access Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [privdis1](privdis1) module"]
pub type PRIVDIS1 = crate::Reg<u32, _PRIVDIS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIVDIS1;
#[doc = "`read()` method returns [privdis1::R](privdis1::R) reader structure"]
impl crate::Readable for PRIVDIS1 {}
#[doc = "`write(|w| ..)` method takes [privdis1::W](privdis1::W) writer structure"]
impl crate::Writable for PRIVDIS1 {}
#[doc = "Peripheral Privilege Access Register 1"]
pub mod privdis1;
#[doc = "ROM Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [romsize](romsize) module"]
pub type ROMSIZE = crate::Reg<u32, _ROMSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROMSIZE;
#[doc = "`read()` method returns [romsize::R](romsize::R) reader structure"]
impl crate::Readable for ROMSIZE {}
#[doc = "ROM Size Register"]
pub mod romsize;
#[doc = "Flash Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flsize](flsize) module"]
pub type FLSIZE = crate::Reg<u32, _FLSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLSIZE;
#[doc = "`read()` method returns [flsize::R](flsize::R) reader structure"]
impl crate::Readable for FLSIZE {}
#[doc = "Flash Size Register"]
pub mod flsize;
#[doc = "RAM0 Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ram0size](ram0size) module"]
pub type RAM0SIZE = crate::Reg<u32, _RAM0SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0SIZE;
#[doc = "`read()` method returns [ram0size::R](ram0size::R) reader structure"]
impl crate::Readable for RAM0SIZE {}
#[doc = "RAM0 Size Register"]
pub mod ram0size;
