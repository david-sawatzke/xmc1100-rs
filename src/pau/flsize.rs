#[doc = "Reader of register FLSIZE"]
pub type R = crate::R<u32, super::FLSIZE>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 12:17 - Flash Size"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
