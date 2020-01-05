#[doc = "Reader of register WORD"]
pub type R = crate::R<u16, super::WORD>;
#[doc = "Writer for register WORD"]
pub type W = crate::W<u16, super::WORD>;
#[doc = "Register WORD `reset()`'s with value 0"]
impl crate::ResetValue for super::WORD {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RDATA`"]
pub struct RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Random Data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Random Data"]
    #[inline(always)]
    pub fn rdata(&mut self) -> RDATA_W {
        RDATA_W { w: self }
    }
}
