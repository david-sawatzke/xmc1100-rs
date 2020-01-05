#[doc = "Writer for register CGATSET0"]
pub type W = crate::W<u32, super::CGATSET0>;
#[doc = "Register CGATSET0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CGATSET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "VADC and SHS Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: enable gating"]
    VALUE2,
}
impl From<VADC_AW> for bool {
    #[inline(always)]
    fn from(variant: VADC_AW) -> Self {
        match variant {
            VADC_AW::VALUE1 => false,
            VADC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `VADC`"]
pub struct VADC_W<'a> {
    w: &'a mut W,
}
impl<'a> VADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VADC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADC_AW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADC_AW::VALUE2)
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
#[doc = "CCU40 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: enable gating"]
    VALUE2,
}
impl From<CCU40_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40_AW) -> Self {
        match variant {
            CCU40_AW::VALUE1 => false,
            CCU40_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCU40`"]
pub struct CCU40_W<'a> {
    w: &'a mut W,
}
impl<'a> CCU40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCU40_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40_AW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "USIC0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: enable gating"]
    VALUE2,
}
impl From<USIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0_AW) -> Self {
        match variant {
            USIC0_AW::VALUE1 => false,
            USIC0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `USIC0`"]
pub struct USIC0_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0_AW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "WDT Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: enable gating"]
    VALUE2,
}
impl From<WDT_AW> for bool {
    #[inline(always)]
    fn from(variant: WDT_AW) -> Self {
        match variant {
            WDT_AW::VALUE1 => false,
            WDT_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDT_AW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDT_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "RTC Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_AW {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: enable gating"]
    VALUE2,
}
impl From<RTC_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_AW) -> Self {
        match variant {
            RTC_AW::VALUE1 => false,
            RTC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_AW::VALUE1)
    }
    #[doc = "enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_AW::VALUE2)
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
    #[doc = "Bit 0 - VADC and SHS Gating Set"]
    #[inline(always)]
    pub fn vadc(&mut self) -> VADC_W {
        VADC_W { w: self }
    }
    #[doc = "Bit 2 - CCU40 Gating Set"]
    #[inline(always)]
    pub fn ccu40(&mut self) -> CCU40_W {
        CCU40_W { w: self }
    }
    #[doc = "Bit 3 - USIC0 Gating Set"]
    #[inline(always)]
    pub fn usic0(&mut self) -> USIC0_W {
        USIC0_W { w: self }
    }
    #[doc = "Bit 9 - WDT Gating Set"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 10 - RTC Gating Set"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
