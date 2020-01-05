#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - NVM Status Register"]
    pub nvmstatus: NVMSTATUS,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - NVM Programming Control Register"]
    pub nvmprog: NVMPROG,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - NVM Configuration Register"]
    pub nvmconf: NVMCONF,
    _reserved3: [u8; 62usize],
    #[doc = "0x48 - Configuration 1 Register"]
    pub config1: CONFIG1,
}
#[doc = "NVM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvmstatus](nvmstatus) module"]
pub type NVMSTATUS = crate::Reg<u16, _NVMSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMSTATUS;
#[doc = "`read()` method returns [nvmstatus::R](nvmstatus::R) reader structure"]
impl crate::Readable for NVMSTATUS {}
#[doc = "NVM Status Register"]
pub mod nvmstatus;
#[doc = "NVM Programming Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvmprog](nvmprog) module"]
pub type NVMPROG = crate::Reg<u16, _NVMPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMPROG;
#[doc = "`read()` method returns [nvmprog::R](nvmprog::R) reader structure"]
impl crate::Readable for NVMPROG {}
#[doc = "`write(|w| ..)` method takes [nvmprog::W](nvmprog::W) writer structure"]
impl crate::Writable for NVMPROG {}
#[doc = "NVM Programming Control Register"]
pub mod nvmprog;
#[doc = "NVM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nvmconf](nvmconf) module"]
pub type NVMCONF = crate::Reg<u16, _NVMCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMCONF;
#[doc = "`read()` method returns [nvmconf::R](nvmconf::R) reader structure"]
impl crate::Readable for NVMCONF {}
#[doc = "`write(|w| ..)` method takes [nvmconf::W](nvmconf::W) writer structure"]
impl crate::Writable for NVMCONF {}
#[doc = "NVM Configuration Register"]
pub mod nvmconf;
#[doc = "Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config1](config1) module"]
pub type CONFIG1 = crate::Reg<u16, _CONFIG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG1;
#[doc = "`read()` method returns [config1::R](config1::R) reader structure"]
impl crate::Readable for CONFIG1 {}
#[doc = "`write(|w| ..)` method takes [config1::W](config1::W) writer structure"]
impl crate::Writable for CONFIG1 {}
#[doc = "Configuration 1 Register"]
pub mod config1;
