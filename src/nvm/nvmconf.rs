#[doc = "Reader of register NVMCONF"]
pub type R = crate::R<u16, super::NVMCONF>;
#[doc = "Writer for register NVMCONF"]
pub type W = crate::W<u16, super::NVMCONF>;
#[doc = "Register NVMCONF `reset()`'s with value 0x9000"]
impl crate::ResetValue for super::NVMCONF {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9000
    }
}
#[doc = "NVM On\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVM_ON_A {
    #[doc = "0: NVM is switched to or stays in sleep mode."]
    VALUE1,
    #[doc = "1: NVM is switched to or stays in normal mode."]
    VALUE2,
}
impl From<NVM_ON_A> for bool {
    #[inline(always)]
    fn from(variant: NVM_ON_A) -> Self {
        match variant {
            NVM_ON_A::VALUE1 => false,
            NVM_ON_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `NVM_ON`"]
pub type NVM_ON_R = crate::R<bool, NVM_ON_A>;
impl NVM_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NVM_ON_A {
        match self.bits {
            false => NVM_ON_A::VALUE1,
            true => NVM_ON_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NVM_ON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NVM_ON_A::VALUE2
    }
}
#[doc = "Write proxy for field `NVM_ON`"]
pub struct NVM_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> NVM_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NVM_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NVM is switched to or stays in sleep mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NVM_ON_A::VALUE1)
    }
    #[doc = "NVM is switched to or stays in normal mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NVM_ON_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
#[doc = "Interrupt On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ON_A {
    #[doc = "0: No NVM ready interrupts are generated."]
    VALUE1,
    #[doc = "1: NVM ready interrupts are generated."]
    VALUE2,
}
impl From<INT_ON_A> for bool {
    #[inline(always)]
    fn from(variant: INT_ON_A) -> Self {
        match variant {
            INT_ON_A::VALUE1 => false,
            INT_ON_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `INT_ON`"]
pub type INT_ON_R = crate::R<bool, INT_ON_A>;
impl INT_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_ON_A {
        match self.bits {
            false => INT_ON_A::VALUE1,
            true => INT_ON_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INT_ON_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INT_ON_A::VALUE2
    }
}
#[doc = "Write proxy for field `INT_ON`"]
pub struct INT_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No NVM ready interrupts are generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INT_ON_A::VALUE1)
    }
    #[doc = "NVM ready interrupts are generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INT_ON_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Number of fixed Wait States\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WS_A {
    #[doc = "0: 0 fixed wait states."]
    VALUE1,
    #[doc = "1: 1 fixed wait state."]
    VALUE2,
}
impl From<WS_A> for bool {
    #[inline(always)]
    fn from(variant: WS_A) -> Self {
        match variant {
            WS_A::VALUE1 => false,
            WS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WS`"]
pub type WS_R = crate::R<bool, WS_A>;
impl WS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WS_A {
        match self.bits {
            false => WS_A::VALUE1,
            true => WS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WS_A::VALUE2
    }
}
#[doc = "Write proxy for field `WS`"]
pub struct WS_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0 fixed wait states."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WS_A::VALUE1)
    }
    #[doc = "1 fixed wait state."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SECPROT`"]
pub type SECPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECPROT`"]
pub struct SECPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u16) & 0xff) << 4);
        self.w
    }
}
#[doc = "Hardread Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRLEV_A {
    #[doc = "0: Normal read"]
    VALUE1,
    #[doc = "1: Hardread written"]
    VALUE2,
    #[doc = "2: Hardread erased"]
    VALUE3,
}
impl From<HRLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: HRLEV_A) -> Self {
        match variant {
            HRLEV_A::VALUE1 => 0,
            HRLEV_A::VALUE2 => 1,
            HRLEV_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HRLEV`"]
pub type HRLEV_R = crate::R<u8, HRLEV_A>;
impl HRLEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HRLEV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HRLEV_A::VALUE1),
            1 => Val(HRLEV_A::VALUE2),
            2 => Val(HRLEV_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRLEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRLEV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HRLEV_A::VALUE3
    }
}
#[doc = "Write proxy for field `HRLEV`"]
pub struct HRLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> HRLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRLEV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal read"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRLEV_A::VALUE1)
    }
    #[doc = "Hardread written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRLEV_A::VALUE2)
    }
    #[doc = "Hardread erased"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HRLEV_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u16) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - NVM On"]
    #[inline(always)]
    pub fn nvm_on(&self) -> NVM_ON_R {
        NVM_ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt On"]
    #[inline(always)]
    pub fn int_on(&self) -> INT_ON_R {
        INT_ON_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Number of fixed Wait States"]
    #[inline(always)]
    pub fn ws(&self) -> WS_R {
        WS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 4:11 - Sector Protection"]
    #[inline(always)]
    pub fn secprot(&self) -> SECPROT_R {
        SECPROT_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 1:2 - Hardread Level"]
    #[inline(always)]
    pub fn hrlev(&self) -> HRLEV_R {
        HRLEV_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - NVM On"]
    #[inline(always)]
    pub fn nvm_on(&mut self) -> NVM_ON_W {
        NVM_ON_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt On"]
    #[inline(always)]
    pub fn int_on(&mut self) -> INT_ON_W {
        INT_ON_W { w: self }
    }
    #[doc = "Bit 12 - Number of fixed Wait States"]
    #[inline(always)]
    pub fn ws(&mut self) -> WS_W {
        WS_W { w: self }
    }
    #[doc = "Bits 4:11 - Sector Protection"]
    #[inline(always)]
    pub fn secprot(&mut self) -> SECPROT_W {
        SECPROT_W { w: self }
    }
    #[doc = "Bits 1:2 - Hardread Level"]
    #[inline(always)]
    pub fn hrlev(&mut self) -> HRLEV_W {
        HRLEV_W { w: self }
    }
}
