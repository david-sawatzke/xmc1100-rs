#[doc = "Reader of register ICSR"]
pub type R = crate::R<u32, super::ICSR>;
#[doc = "Writer for register ICSR"]
pub type W = crate::W<u32, super::ICSR>;
#[doc = "Register ICSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Active Exception Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTACTIVE_A {
    #[doc = "0: Thread mode"]
    VALUE1,
}
impl From<VECTACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: VECTACTIVE_A) -> Self {
        match variant {
            VECTACTIVE_A::VALUE1 => 0,
        }
    }
}
#[doc = "Reader of field `VECTACTIVE`"]
pub type VECTACTIVE_R = crate::R<u8, VECTACTIVE_A>;
impl VECTACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VECTACTIVE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VECTACTIVE_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VECTACTIVE_A::VALUE1
    }
}
#[doc = "Pending Exception Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTPENDING_A {
    #[doc = "0: No pending exceptions"]
    VALUE1,
}
impl From<VECTPENDING_A> for u8 {
    #[inline(always)]
    fn from(variant: VECTPENDING_A) -> Self {
        match variant {
            VECTPENDING_A::VALUE1 => 0,
        }
    }
}
#[doc = "Reader of field `VECTPENDING`"]
pub type VECTPENDING_R = crate::R<u8, VECTPENDING_A>;
impl VECTPENDING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VECTPENDING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VECTPENDING_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VECTPENDING_A::VALUE1
    }
}
#[doc = "Interrupt Pending Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPENDING_A {
    #[doc = "0: Interrupt not pending"]
    VALUE1,
    #[doc = "1: Interrupt pending."]
    VALUE2,
}
impl From<ISRPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ISRPENDING_A) -> Self {
        match variant {
            ISRPENDING_A::VALUE1 => false,
            ISRPENDING_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ISRPENDING`"]
pub type ISRPENDING_R = crate::R<bool, ISRPENDING_A>;
impl ISRPENDING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISRPENDING_A {
        match self.bits {
            false => ISRPENDING_A::VALUE1,
            true => ISRPENDING_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ISRPENDING_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ISRPENDING_A::VALUE2
    }
}
#[doc = "SysTick Exception Clear-pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLR_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: removes the pending state from the SysTick exception."]
    VALUE2,
}
impl From<PENDSTCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSTCLR_AW) -> Self {
        match variant {
            PENDSTCLR_AW::VALUE1 => false,
            PENDSTCLR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `PENDSTCLR`"]
pub struct PENDSTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::VALUE1)
    }
    #[doc = "removes the pending state from the SysTick exception."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTCLR_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "SysTick Exception Set-pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSET_A {
    #[doc = "0: SysTick exception is not pending"]
    VALUE1,
    #[doc = "1: SysTick exception is pending."]
    VALUE2,
}
impl From<PENDSTSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSTSET_A) -> Self {
        match variant {
            PENDSTSET_A::VALUE1 => false,
            PENDSTSET_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PENDSTSET`"]
pub type PENDSTSET_R = crate::R<bool, PENDSTSET_A>;
impl PENDSTSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSTSET_A {
        match self.bits {
            false => PENDSTSET_A::VALUE1,
            true => PENDSTSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PENDSTSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PENDSTSET_A::VALUE2
    }
}
#[doc = "Write proxy for field `PENDSTSET`"]
pub struct PENDSTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SysTick exception is not pending"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE1)
    }
    #[doc = "SysTick exception is pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSTSET_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "PendSV Clear Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLR_AW {
    #[doc = "0: Do not clear."]
    VALUE1,
    #[doc = "1: Removes pending state from PendSV exception."]
    VALUE2,
}
impl From<PENDSVCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: PENDSVCLR_AW) -> Self {
        match variant {
            PENDSVCLR_AW::VALUE1 => false,
            PENDSVCLR_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `PENDSVCLR`"]
pub struct PENDSVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not clear."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::VALUE1)
    }
    #[doc = "Removes pending state from PendSV exception."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSVCLR_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "PendSV Set Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSET_A {
    #[doc = "0: PendSV exception is not pending."]
    VALUE1,
    #[doc = "1: PendSV excepton is pending."]
    VALUE2,
}
impl From<PENDSVSET_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVSET_A) -> Self {
        match variant {
            PENDSVSET_A::VALUE1 => false,
            PENDSVSET_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PENDSVSET`"]
pub type PENDSVSET_R = crate::R<bool, PENDSVSET_A>;
impl PENDSVSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVSET_A {
        match self.bits {
            false => PENDSVSET_A::VALUE1,
            true => PENDSVSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PENDSVSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PENDSVSET_A::VALUE2
    }
}
#[doc = "Write proxy for field `PENDSVSET`"]
pub struct PENDSVSET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PendSV exception is not pending."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PENDSVSET_A::VALUE1)
    }
    #[doc = "PendSV excepton is pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PENDSVSET_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Active Exception Number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Pending Exception Number"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Interrupt Pending Flag"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SysTick Exception Set-pending"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PendSV Set Pending"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick Exception Clear-pending"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> PENDSTCLR_W {
        PENDSTCLR_W { w: self }
    }
    #[doc = "Bit 26 - SysTick Exception Set-pending"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> PENDSTSET_W {
        PENDSTSET_W { w: self }
    }
    #[doc = "Bit 27 - PendSV Clear Pending"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> PENDSVCLR_W {
        PENDSVCLR_W { w: self }
    }
    #[doc = "Bit 28 - PendSV Set Pending"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> PENDSVSET_W {
        PENDSVSET_W { w: self }
    }
}
