#[doc = "Reader of register CALGC1"]
pub type R = crate::R<u32, super::CALGC1>;
#[doc = "Writer for register CALGC1"]
pub type W = crate::W<u32, super::CALGC1>;
#[doc = "Register CALGC1 `reset()`'s with value 0x2000_2000"]
impl crate::ResetValue for super::CALGC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_2000
    }
}
#[doc = "Reader of field `CALGNVALS`"]
pub type CALGNVALS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CALGNVALS`"]
pub struct CALGNVALS_W<'a> {
    w: &'a mut W,
}
impl<'a> CALGNVALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Gain Calibration Write Control, Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNSWC_AW {
    #[doc = "0: No write access to gain calibration parameter"]
    VALUE1,
    #[doc = "1: CALGNVALS can be written"]
    VALUE2,
}
impl From<GNSWC_AW> for bool {
    #[inline(always)]
    fn from(variant: GNSWC_AW) -> Self {
        match variant {
            GNSWC_AW::VALUE1 => false,
            GNSWC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `GNSWC`"]
pub struct GNSWC_W<'a> {
    w: &'a mut W,
}
impl<'a> GNSWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GNSWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to gain calibration parameter"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GNSWC_AW::VALUE1)
    }
    #[doc = "CALGNVALS can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GNSWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CALGNVALA`"]
pub type CALGNVALA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CALGNVALA`"]
pub struct CALGNVALA_W<'a> {
    w: &'a mut W,
}
impl<'a> CALGNVALA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Gain Calibration Write Control, Alternate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GNAWC_AW {
    #[doc = "0: No write access to gain calibration parameter"]
    VALUE1,
    #[doc = "1: CALGNVALA can be written"]
    VALUE2,
}
impl From<GNAWC_AW> for bool {
    #[inline(always)]
    fn from(variant: GNAWC_AW) -> Self {
        match variant {
            GNAWC_AW::VALUE1 => false,
            GNAWC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `GNAWC`"]
pub struct GNAWC_W<'a> {
    w: &'a mut W,
}
impl<'a> GNAWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GNAWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to gain calibration parameter"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GNAWC_AW::VALUE1)
    }
    #[doc = "CALGNVALA can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GNAWC_AW::VALUE2)
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
    #[doc = "Bits 0:13 - Gain Calibration Value, Standard Reference"]
    #[inline(always)]
    pub fn calgnvals(&self) -> CALGNVALS_R {
        CALGNVALS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Gain Calibration Value, Alternate Reference"]
    #[inline(always)]
    pub fn calgnvala(&self) -> CALGNVALA_R {
        CALGNVALA_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Gain Calibration Value, Standard Reference"]
    #[inline(always)]
    pub fn calgnvals(&mut self) -> CALGNVALS_W {
        CALGNVALS_W { w: self }
    }
    #[doc = "Bit 15 - Gain Calibration Write Control, Standard"]
    #[inline(always)]
    pub fn gnswc(&mut self) -> GNSWC_W {
        GNSWC_W { w: self }
    }
    #[doc = "Bits 16:29 - Gain Calibration Value, Alternate Reference"]
    #[inline(always)]
    pub fn calgnvala(&mut self) -> CALGNVALA_W {
        CALGNVALA_W { w: self }
    }
    #[doc = "Bit 31 - Gain Calibration Write Control, Alternate"]
    #[inline(always)]
    pub fn gnawc(&mut self) -> GNAWC_W {
        GNAWC_W { w: self }
    }
}
