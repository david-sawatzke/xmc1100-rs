#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Temperature Sensor Control Register"]
    pub anatsectrl: ANATSECTRL,
    _reserved1: [u8; 10usize],
    #[doc = "0x30 - Temperature Sensor High Temperature Interrupt Register"]
    pub anatseih: ANATSEIH,
    _reserved2: [u8; 2usize],
    #[doc = "0x34 - Temperature Sensor Low Temperature Interrupt Register"]
    pub anatseil: ANATSEIL,
    _reserved3: [u8; 10usize],
    #[doc = "0x40 - Temperature Sensor Counter2 Monitor Register"]
    pub anatsemon: ANATSEMON,
    _reserved4: [u8; 14usize],
    #[doc = "0x50 - Voltage Detector Control Register"]
    pub anavdel: ANAVDEL,
    _reserved5: [u8; 26usize],
    #[doc = "0x6c - DCO1 Offset Register"]
    pub anaoffset: ANAOFFSET,
}
#[doc = "Temperature Sensor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anatsectrl](anatsectrl) module"]
pub type ANATSECTRL = crate::Reg<u16, _ANATSECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANATSECTRL;
#[doc = "`read()` method returns [anatsectrl::R](anatsectrl::R) reader structure"]
impl crate::Readable for ANATSECTRL {}
#[doc = "`write(|w| ..)` method takes [anatsectrl::W](anatsectrl::W) writer structure"]
impl crate::Writable for ANATSECTRL {}
#[doc = "Temperature Sensor Control Register"]
pub mod anatsectrl;
#[doc = "Temperature Sensor High Temperature Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anatseih](anatseih) module"]
pub type ANATSEIH = crate::Reg<u16, _ANATSEIH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANATSEIH;
#[doc = "`read()` method returns [anatseih::R](anatseih::R) reader structure"]
impl crate::Readable for ANATSEIH {}
#[doc = "`write(|w| ..)` method takes [anatseih::W](anatseih::W) writer structure"]
impl crate::Writable for ANATSEIH {}
#[doc = "Temperature Sensor High Temperature Interrupt Register"]
pub mod anatseih;
#[doc = "Temperature Sensor Low Temperature Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anatseil](anatseil) module"]
pub type ANATSEIL = crate::Reg<u16, _ANATSEIL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANATSEIL;
#[doc = "`read()` method returns [anatseil::R](anatseil::R) reader structure"]
impl crate::Readable for ANATSEIL {}
#[doc = "`write(|w| ..)` method takes [anatseil::W](anatseil::W) writer structure"]
impl crate::Writable for ANATSEIL {}
#[doc = "Temperature Sensor Low Temperature Interrupt Register"]
pub mod anatseil;
#[doc = "Temperature Sensor Counter2 Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anatsemon](anatsemon) module"]
pub type ANATSEMON = crate::Reg<u16, _ANATSEMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANATSEMON;
#[doc = "`read()` method returns [anatsemon::R](anatsemon::R) reader structure"]
impl crate::Readable for ANATSEMON {}
#[doc = "Temperature Sensor Counter2 Monitor Register"]
pub mod anatsemon;
#[doc = "Voltage Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anavdel](anavdel) module"]
pub type ANAVDEL = crate::Reg<u16, _ANAVDEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANAVDEL;
#[doc = "`read()` method returns [anavdel::R](anavdel::R) reader structure"]
impl crate::Readable for ANAVDEL {}
#[doc = "`write(|w| ..)` method takes [anavdel::W](anavdel::W) writer structure"]
impl crate::Writable for ANAVDEL {}
#[doc = "Voltage Detector Control Register"]
pub mod anavdel;
#[doc = "DCO1 Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anaoffset](anaoffset) module"]
pub type ANAOFFSET = crate::Reg<u16, _ANAOFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANAOFFSET;
#[doc = "`read()` method returns [anaoffset::R](anaoffset::R) reader structure"]
impl crate::Readable for ANAOFFSET {}
#[doc = "`write(|w| ..)` method takes [anaoffset::W](anaoffset::W) writer structure"]
impl crate::Writable for ANAOFFSET {}
#[doc = "DCO1 Offset Register"]
pub mod anaoffset;
