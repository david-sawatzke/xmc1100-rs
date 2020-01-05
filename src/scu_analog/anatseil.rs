#[doc = "Reader of register ANATSEIL"]
pub type R = crate::R<u16, super::ANATSEIL>;
#[doc = "Writer for register ANATSEIL"]
pub type W = crate::W<u16, super::ANATSEIL>;
#[doc = "Register ANATSEIL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::ANATSEIL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TSE_IL`"]
pub type TSE_IL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSE_IL`"]
pub struct TSE_IL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_IL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter value for low temperature interrupt"]
    #[inline(always)]
    pub fn tse_il(&self) -> TSE_IL_R {
        TSE_IL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value for low temperature interrupt"]
    #[inline(always)]
    pub fn tse_il(&mut self) -> TSE_IL_W {
        TSE_IL_W { w: self }
    }
}
