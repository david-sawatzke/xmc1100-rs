#[doc = "Reader of register CCUCON"]
pub type R = crate::R<u32, super::CCUCON>;
#[doc = "Writer for register CCUCON"]
pub type W = crate::W<u32, super::CCUCON>;
#[doc = "Register CCUCON `reset()`'s with value 0"]
impl crate::ResetValue for super::CCUCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GSC40`"]
pub type GSC40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GSC40`"]
pub struct GSC40_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&mut self) -> GSC40_W {
        GSC40_W { w: self }
    }
}
