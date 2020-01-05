#[doc = "Reader of register RAM0SIZE"]
pub type R = crate::R<u32, super::RAM0SIZE>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:12 - RAM0 Size"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
