#[doc = "Reader of register ROMSIZE"]
pub type R = crate::R<u32, super::ROMSIZE>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:13 - ROM Size"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
