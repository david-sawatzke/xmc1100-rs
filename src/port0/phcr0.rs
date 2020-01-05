#[doc = "Reader of register PHCR0"]
pub type R = crate::R<u32, super::PHCR0>;
#[doc = "Writer for register PHCR0"]
pub type W = crate::W<u32, super::PHCR0>;
#[doc = "Register PHCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PHCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PH0`"]
pub type PH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH0`"]
pub struct PH0_W<'a> {
    w: &'a mut W,
}
impl<'a> PH0_W<'a> {
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
#[doc = "Reader of field `PH1`"]
pub type PH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH1`"]
pub struct PH1_W<'a> {
    w: &'a mut W,
}
impl<'a> PH1_W<'a> {
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
#[doc = "Reader of field `PH2`"]
pub type PH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH2`"]
pub struct PH2_W<'a> {
    w: &'a mut W,
}
impl<'a> PH2_W<'a> {
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
#[doc = "Reader of field `PH3`"]
pub type PH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH3`"]
pub struct PH3_W<'a> {
    w: &'a mut W,
}
impl<'a> PH3_W<'a> {
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
#[doc = "Reader of field `PH4`"]
pub type PH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH4`"]
pub struct PH4_W<'a> {
    w: &'a mut W,
}
impl<'a> PH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PH5`"]
pub type PH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH5`"]
pub struct PH5_W<'a> {
    w: &'a mut W,
}
impl<'a> PH5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PH6`"]
pub type PH6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH6`"]
pub struct PH6_W<'a> {
    w: &'a mut W,
}
impl<'a> PH6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PH7`"]
pub type PH7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PH7`"]
pub struct PH7_W<'a> {
    w: &'a mut W,
}
impl<'a> PH7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Pad Hysteresis for Pn.0"]
    #[inline(always)]
    pub fn ph0(&self) -> PH0_R {
        PH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pad Hysteresis for Pn.1"]
    #[inline(always)]
    pub fn ph1(&self) -> PH1_R {
        PH1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pad Hysteresis for Pn.2"]
    #[inline(always)]
    pub fn ph2(&self) -> PH2_R {
        PH2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pad Hysteresis for Pn.3"]
    #[inline(always)]
    pub fn ph3(&self) -> PH3_R {
        PH3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pad Hysteresis for Pn.4"]
    #[inline(always)]
    pub fn ph4(&self) -> PH4_R {
        PH4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pad Hysteresis for Pn.5"]
    #[inline(always)]
    pub fn ph5(&self) -> PH5_R {
        PH5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pad Hysteresis for Pn.6"]
    #[inline(always)]
    pub fn ph6(&self) -> PH6_R {
        PH6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Pad Hysteresis for Pn.7"]
    #[inline(always)]
    pub fn ph7(&self) -> PH7_R {
        PH7_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pad Hysteresis for Pn.0"]
    #[inline(always)]
    pub fn ph0(&mut self) -> PH0_W {
        PH0_W { w: self }
    }
    #[doc = "Bit 6 - Pad Hysteresis for Pn.1"]
    #[inline(always)]
    pub fn ph1(&mut self) -> PH1_W {
        PH1_W { w: self }
    }
    #[doc = "Bit 10 - Pad Hysteresis for Pn.2"]
    #[inline(always)]
    pub fn ph2(&mut self) -> PH2_W {
        PH2_W { w: self }
    }
    #[doc = "Bit 14 - Pad Hysteresis for Pn.3"]
    #[inline(always)]
    pub fn ph3(&mut self) -> PH3_W {
        PH3_W { w: self }
    }
    #[doc = "Bit 18 - Pad Hysteresis for Pn.4"]
    #[inline(always)]
    pub fn ph4(&mut self) -> PH4_W {
        PH4_W { w: self }
    }
    #[doc = "Bit 22 - Pad Hysteresis for Pn.5"]
    #[inline(always)]
    pub fn ph5(&mut self) -> PH5_W {
        PH5_W { w: self }
    }
    #[doc = "Bit 26 - Pad Hysteresis for Pn.6"]
    #[inline(always)]
    pub fn ph6(&mut self) -> PH6_W {
        PH6_W { w: self }
    }
    #[doc = "Bit 30 - Pad Hysteresis for Pn.7"]
    #[inline(always)]
    pub fn ph7(&mut self) -> PH7_W {
        PH7_W { w: self }
    }
}
