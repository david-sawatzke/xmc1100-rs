#[doc = "Reader of register CALCTR"]
pub type R = crate::R<u32, super::CALCTR>;
#[doc = "Writer for register CALCTR"]
pub type W = crate::W<u32, super::CALCTR>;
#[doc = "Register CALCTR `reset()`'s with value 0x0010_0400"]
impl crate::ResetValue for super::CALCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0400
    }
}
#[doc = "Calibration Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALORD_A {
    #[doc = "0: Do conversions then calibration"]
    VALUE1,
    #[doc = "1: Do calibration then conversions"]
    VALUE2,
}
impl From<CALORD_A> for bool {
    #[inline(always)]
    fn from(variant: CALORD_A) -> Self {
        match variant {
            CALORD_A::VALUE1 => false,
            CALORD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CALORD`"]
pub type CALORD_R = crate::R<bool, CALORD_A>;
impl CALORD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALORD_A {
        match self.bits {
            false => CALORD_A::VALUE1,
            true => CALORD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CALORD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CALORD_A::VALUE2
    }
}
#[doc = "Write proxy for field `CALORD`"]
pub struct CALORD_W<'a> {
    w: &'a mut W,
}
impl<'a> CALORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALORD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do conversions then calibration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CALORD_A::VALUE1)
    }
    #[doc = "Do calibration then conversions"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CALORD_A::VALUE2)
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
#[doc = "Reader of field `CALGNSTC`"]
pub type CALGNSTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALGNSTC`"]
pub struct CALGNSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CALGNSTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SUCALVAL`"]
pub type SUCALVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUCALVAL`"]
pub struct SUCALVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCALVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CALMAX`"]
pub type CALMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALMAX`"]
pub struct CALMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> CALMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Start-Up Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUCAL_AW {
    #[doc = "0: No action"]
    VALUE1,
    #[doc = "1: Initiate the start-up calibration phase (indication in bitfield SHSCFG.STATE)"]
    VALUE2,
}
impl From<SUCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: SUCAL_AW) -> Self {
        match variant {
            SUCAL_AW::VALUE1 => false,
            SUCAL_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SUCAL`"]
pub struct SUCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUCAL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUCAL_AW::VALUE1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bitfield SHSCFG.STATE)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUCAL_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Calibration Order"]
    #[inline(always)]
    pub fn calord(&self) -> CALORD_R {
        CALORD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Gain Calibration Sample Time Control"]
    #[inline(always)]
    pub fn calgnstc(&self) -> CALGNSTC_R {
        CALGNSTC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Startup Calibration Cycles"]
    #[inline(always)]
    pub fn sucalval(&self) -> SUCALVAL_R {
        SUCALVAL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - Calibration Maximum Timing"]
    #[inline(always)]
    pub fn calmax(&self) -> CALMAX_R {
        CALMAX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Order"]
    #[inline(always)]
    pub fn calord(&mut self) -> CALORD_W {
        CALORD_W { w: self }
    }
    #[doc = "Bits 8:13 - Gain Calibration Sample Time Control"]
    #[inline(always)]
    pub fn calgnstc(&mut self) -> CALGNSTC_W {
        CALGNSTC_W { w: self }
    }
    #[doc = "Bits 16:22 - Startup Calibration Cycles"]
    #[inline(always)]
    pub fn sucalval(&mut self) -> SUCALVAL_W {
        SUCALVAL_W { w: self }
    }
    #[doc = "Bits 24:29 - Calibration Maximum Timing"]
    #[inline(always)]
    pub fn calmax(&mut self) -> CALMAX_W {
        CALMAX_W { w: self }
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline(always)]
    pub fn sucal(&mut self) -> SUCAL_W {
        SUCAL_W { w: self }
    }
}
