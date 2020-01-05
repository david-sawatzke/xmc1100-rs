#[doc = "Reader of register PHCR1"]
pub type R = crate::R<u32, super::PHCR1>;
#[doc = "Writer for register PHCR1"]
pub type W = crate::W<u32, super::PHCR1>;
#[doc = "Register PHCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PHCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PH8`"]
pub type PH8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH8`"]
pub struct PH8_W<'a> {
    w: &'a mut W,
}
impl<'a> PH8_W<'a> {
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
#[doc = "Reader of field `PH9`"]
pub type PH9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH9`"]
pub struct PH9_W<'a> {
    w: &'a mut W,
}
impl<'a> PH9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PH10`"]
pub type PH10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH10`"]
pub struct PH10_W<'a> {
    w: &'a mut W,
}
impl<'a> PH10_W<'a> {
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
#[doc = "Reader of field `PH11`"]
pub type PH11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH11`"]
pub struct PH11_W<'a> {
    w: &'a mut W,
}
impl<'a> PH11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Pad Hysteresis for P2.8"]
    #[inline(always)]
    pub fn ph8(&self) -> PH8_R {
        PH8_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pad Hysteresis for P2.9"]
    #[inline(always)]
    pub fn ph9(&self) -> PH9_R {
        PH9_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pad Hysteresis for P2.10"]
    #[inline(always)]
    pub fn ph10(&self) -> PH10_R {
        PH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pad Hysteresis for P2.11"]
    #[inline(always)]
    pub fn ph11(&self) -> PH11_R {
        PH11_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pad Hysteresis for P2.8"]
    #[inline(always)]
    pub fn ph8(&mut self) -> PH8_W {
        PH8_W { w: self }
    }
    #[doc = "Bit 6 - Pad Hysteresis for P2.9"]
    #[inline(always)]
    pub fn ph9(&mut self) -> PH9_W {
        PH9_W { w: self }
    }
    #[doc = "Bit 10 - Pad Hysteresis for P2.10"]
    #[inline(always)]
    pub fn ph10(&mut self) -> PH10_W {
        PH10_W { w: self }
    }
    #[doc = "Bit 14 - Pad Hysteresis for P2.11"]
    #[inline(always)]
    pub fn ph11(&mut self) -> PH11_W {
        PH11_W { w: self }
    }
}
