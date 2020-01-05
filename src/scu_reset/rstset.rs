#[doc = "Writer for register RSTSET"]
pub type W = crate::W<u32, super::RSTSET>;
#[doc = "Register RSTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKEN_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: Enable reset when Lockup gets asserted"]
    VALUE2,
}
impl From<LCKEN_AW> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_AW) -> Self {
        match variant {
            LCKEN_AW::VALUE1 => false,
            LCKEN_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `LCKEN`"]
pub struct LCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCKEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LCKEN_AW::VALUE1)
    }
    #[doc = "Enable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LCKEN_AW::VALUE2)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    pub fn lcken(&mut self) -> LCKEN_W {
        LCKEN_W { w: self }
    }
}
