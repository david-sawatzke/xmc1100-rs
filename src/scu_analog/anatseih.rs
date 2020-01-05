#[doc = "Reader of register ANATSEIH"]
pub type R = crate::R<u16, super::ANATSEIH>;
#[doc = "Writer for register ANATSEIH"]
pub type W = crate::W<u16, super::ANATSEIH>;
#[doc = "Register ANATSEIH `reset()`'s with value 0"]
impl crate::ResetValue for super::ANATSEIH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSE_IH`"]
pub type TSE_IH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSE_IH`"]
pub struct TSE_IH_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_IH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter value for high temperature interrupt"]
    #[inline(always)]
    pub fn tse_ih(&self) -> TSE_IH_R {
        TSE_IH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value for high temperature interrupt"]
    #[inline(always)]
    pub fn tse_ih(&mut self) -> TSE_IH_W {
        TSE_IH_W { w: self }
    }
}
