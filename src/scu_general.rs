#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug System ROM ID Register"]
    pub dbgromid: DBGROMID,
    #[doc = "0x04 - Chip ID Register"]
    pub idchip: IDCHIP,
    #[doc = "0x08 - SCU Module ID Register"]
    pub id: ID,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - SSW Register 0"]
    pub ssw0: SSW0,
    _reserved4: [u8; 12usize],
    #[doc = "0x24 - Password Register"]
    pub passwd: PASSWD,
    _reserved5: [u8; 8usize],
    #[doc = "0x30 - CCU Control Register"]
    pub ccucon: CCUCON,
    _reserved6: [u8; 20usize],
    #[doc = "0x48 - Mirror Update Status Register"]
    pub mirrsts: MIRRSTS,
    _reserved7: [u8; 8usize],
    #[doc = "0x54 - Parity Memory Test Select Register"]
    pub pmtsr: PMTSR,
}
#[doc = "Debug System ROM ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dbgromid](dbgromid) module"]
pub type DBGROMID = crate::Reg<u32, _DBGROMID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGROMID;
#[doc = "`read()` method returns [dbgromid::R](dbgromid::R) reader structure"]
impl crate::Readable for DBGROMID {}
#[doc = "Debug System ROM ID Register"]
pub mod dbgromid;
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idchip](idchip) module"]
pub type IDCHIP = crate::Reg<u32, _IDCHIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCHIP;
#[doc = "`read()` method returns [idchip::R](idchip::R) reader structure"]
impl crate::Readable for IDCHIP {}
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "SCU Module ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "SSW Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssw0](ssw0) module"]
pub type SSW0 = crate::Reg<u32, _SSW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSW0;
#[doc = "`read()` method returns [ssw0::R](ssw0::R) reader structure"]
impl crate::Readable for SSW0 {}
#[doc = "`write(|w| ..)` method takes [ssw0::W](ssw0::W) writer structure"]
impl crate::Writable for SSW0 {}
#[doc = "SSW Register 0"]
pub mod ssw0;
#[doc = "Password Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [passwd](passwd) module"]
pub type PASSWD = crate::Reg<u32, _PASSWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PASSWD;
#[doc = "`read()` method returns [passwd::R](passwd::R) reader structure"]
impl crate::Readable for PASSWD {}
#[doc = "`write(|w| ..)` method takes [passwd::W](passwd::W) writer structure"]
impl crate::Writable for PASSWD {}
#[doc = "Password Register"]
pub mod passwd;
#[doc = "CCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccucon](ccucon) module"]
pub type CCUCON = crate::Reg<u32, _CCUCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCUCON;
#[doc = "`read()` method returns [ccucon::R](ccucon::R) reader structure"]
impl crate::Readable for CCUCON {}
#[doc = "`write(|w| ..)` method takes [ccucon::W](ccucon::W) writer structure"]
impl crate::Writable for CCUCON {}
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "Mirror Update Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mirrsts](mirrsts) module"]
pub type MIRRSTS = crate::Reg<u32, _MIRRSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIRRSTS;
#[doc = "`read()` method returns [mirrsts::R](mirrsts::R) reader structure"]
impl crate::Readable for MIRRSTS {}
#[doc = "Mirror Update Status Register"]
pub mod mirrsts;
#[doc = "Parity Memory Test Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmtsr](pmtsr) module"]
pub type PMTSR = crate::Reg<u32, _PMTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMTSR;
#[doc = "`read()` method returns [pmtsr::R](pmtsr::R) reader structure"]
impl crate::Readable for PMTSR {}
#[doc = "`write(|w| ..)` method takes [pmtsr::W](pmtsr::W) writer structure"]
impl crate::Writable for PMTSR {}
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
