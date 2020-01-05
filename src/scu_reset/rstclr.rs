#[doc = "Writer for register RSTCLR"]
pub type W = crate::W<u32, super::RSTCLR>;
#[doc = "Register RSTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSCLR_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: Clears field RSTSTAT.RSTSTAT"]
    VALUE2,
}
impl From<RSCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: RSCLR_AW) -> Self {
        match variant {
            RSCLR_AW::VALUE1 => false,
            RSCLR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `RSCLR`"]
pub struct RSCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSCLR_AW::VALUE1)
    }
    #[doc = "Clears field RSTSTAT.RSTSTAT"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSCLR_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKEN_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: Disable reset when Lockup gets asserted"]
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
    #[doc = "Disable reset when Lockup gets asserted"]
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
    #[doc = "Bit 0 - Clear Reset Status"]
    #[inline(always)]
    pub fn rsclr(&mut self) -> RSCLR_W {
        RSCLR_W { w: self }
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    pub fn lcken(&mut self) -> LCKEN_W {
        LCKEN_W { w: self }
    }
}
