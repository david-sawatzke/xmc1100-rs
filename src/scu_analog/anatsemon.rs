#[doc = "Reader of register ANATSEMON"]
pub type R = crate::R<u16, super::ANATSEMON>;
#[doc = "Reader of field `TSE_MON`"]
pub type TSE_MON_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Monitor Counter2 value; loaded by TSE_DONE"]
    #[inline(always)]
    pub fn tse_mon(&self) -> TSE_MON_R {
        TSE_MON_R::new((self.bits & 0xffff) as u16)
    }
}
