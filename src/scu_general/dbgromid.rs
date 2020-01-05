#[doc = "Reader of register DBGROMID"]
pub type R = crate::R<u32, super::DBGROMID>;
#[doc = "Reader of field `MANUFID`"]
pub type MANUFID_R = crate::R<u16, u16>;
#[doc = "Reader of field `PARTNO`"]
pub type PARTNO_R = crate::R<u16, u16>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 1:11 - Manufactory Identity"]
    #[inline(always)]
    pub fn manufid(&self) -> MANUFID_R {
        MANUFID_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:27 - Part Number"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - Product version"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
