#[doc = "Reader of register LOOP"]
pub type R = crate::R<u32, super::LOOP>;
#[doc = "Writer for register LOOP"]
pub type W = crate::W<u32, super::LOOP>;
#[doc = "Register LOOP `reset()`'s with value 0"]
impl crate::ResetValue for super::LOOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPCH0`"]
pub type LPCH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPCH0`"]
pub struct LPCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `LPCH1`"]
pub type LPCH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPCH1`"]
pub struct LPCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LPSH0`"]
pub type LPSH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPSH0`"]
pub struct LPSH0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LPSH1`"]
pub type LPSH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPSH1`"]
pub struct LPSH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Loop y Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN0_A {
    #[doc = "0: Off: standard operation"]
    VALUE1,
    #[doc = "1: ON: sigma-delta-loop is active"]
    VALUE2,
}
impl From<LPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: LPEN0_A) -> Self {
        match variant {
            LPEN0_A::VALUE1 => false,
            LPEN0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LPEN0`"]
pub type LPEN0_R = crate::R<bool, LPEN0_A>;
impl LPEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPEN0_A {
        match self.bits {
            false => LPEN0_A::VALUE1,
            true => LPEN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPEN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPEN0_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPEN0`"]
pub struct LPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPEN0_A::VALUE1)
    }
    #[doc = "ON: sigma-delta-loop is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPEN0_A::VALUE2)
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
#[doc = "Loop y Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN1_A {
    #[doc = "0: Off: standard operation"]
    VALUE1,
    #[doc = "1: ON: sigma-delta-loop is active"]
    VALUE2,
}
impl From<LPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: LPEN1_A) -> Self {
        match variant {
            LPEN1_A::VALUE1 => false,
            LPEN1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LPEN1`"]
pub type LPEN1_R = crate::R<bool, LPEN1_A>;
impl LPEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPEN1_A {
        match self.bits {
            false => LPEN1_A::VALUE1,
            true => LPEN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPEN1_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPEN1`"]
pub struct LPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPEN1_A::VALUE1)
    }
    #[doc = "ON: sigma-delta-loop is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPEN1_A::VALUE2)
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
    #[doc = "Bits 0:4 - Loop y Channel"]
    #[inline(always)]
    pub fn lpch0(&self) -> LPCH0_R {
        LPCH0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Loop y Channel"]
    #[inline(always)]
    pub fn lpch1(&self) -> LPCH1_R {
        LPCH1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Loop y Sample&Hold Unit"]
    #[inline(always)]
    pub fn lpsh0(&self) -> LPSH0_R {
        LPSH0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Loop y Sample&Hold Unit"]
    #[inline(always)]
    pub fn lpsh1(&self) -> LPSH1_R {
        LPSH1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Loop y Enable"]
    #[inline(always)]
    pub fn lpen0(&self) -> LPEN0_R {
        LPEN0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Loop y Enable"]
    #[inline(always)]
    pub fn lpen1(&self) -> LPEN1_R {
        LPEN1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Loop y Channel"]
    #[inline(always)]
    pub fn lpch0(&mut self) -> LPCH0_W {
        LPCH0_W { w: self }
    }
    #[doc = "Bits 16:20 - Loop y Channel"]
    #[inline(always)]
    pub fn lpch1(&mut self) -> LPCH1_W {
        LPCH1_W { w: self }
    }
    #[doc = "Bit 8 - Loop y Sample&Hold Unit"]
    #[inline(always)]
    pub fn lpsh0(&mut self) -> LPSH0_W {
        LPSH0_W { w: self }
    }
    #[doc = "Bit 24 - Loop y Sample&Hold Unit"]
    #[inline(always)]
    pub fn lpsh1(&mut self) -> LPSH1_W {
        LPSH1_W { w: self }
    }
    #[doc = "Bit 15 - Loop y Enable"]
    #[inline(always)]
    pub fn lpen0(&mut self) -> LPEN0_W {
        LPEN0_W { w: self }
    }
    #[doc = "Bit 31 - Loop y Enable"]
    #[inline(always)]
    pub fn lpen1(&mut self) -> LPEN1_W {
        LPEN1_W { w: self }
    }
}
