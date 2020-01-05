#[doc = "Reader of register NVMPROG"]
pub type R = crate::R<u16, super::NVMPROG>;
#[doc = "Writer for register NVMPROG"]
pub type W = crate::W<u16, super::NVMPROG>;
#[doc = "Register NVMPROG `reset()`'s with value 0"]
impl crate::ResetValue for super::NVMPROG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset ECC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTECC_A {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: Reset of .ECCxREAD and NVMSTATUS.WRPERR."]
    VALUE2,
}
impl From<RSTECC_A> for bool {
    #[inline(always)]
    fn from(variant: RSTECC_A) -> Self {
        match variant {
            RSTECC_A::VALUE1 => false,
            RSTECC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RSTECC`"]
pub type RSTECC_R = crate::R<bool, RSTECC_A>;
impl RSTECC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTECC_A {
        match self.bits {
            false => RSTECC_A::VALUE1,
            true => RSTECC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSTECC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSTECC_A::VALUE2
    }
}
#[doc = "Write proxy for field `RSTECC`"]
pub struct RSTECC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTECC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTECC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSTECC_A::VALUE1)
    }
    #[doc = "Reset of .ECCxREAD and NVMSTATUS.WRPERR."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSTECC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reset Verify Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTVERR_A {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: Reset of .VERR."]
    VALUE2,
}
impl From<RSTVERR_A> for bool {
    #[inline(always)]
    fn from(variant: RSTVERR_A) -> Self {
        match variant {
            RSTVERR_A::VALUE1 => false,
            RSTVERR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RSTVERR`"]
pub type RSTVERR_R = crate::R<bool, RSTVERR_A>;
impl RSTVERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTVERR_A {
        match self.bits {
            false => RSTVERR_A::VALUE1,
            true => RSTVERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RSTVERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RSTVERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `RSTVERR`"]
pub struct RSTVERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTVERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTVERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RSTVERR_A::VALUE1)
    }
    #[doc = "Reset of .VERR."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RSTVERR_A::VALUE2)
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
#[doc = "ACTION: \\[VERIFY, ONE_SHOT, OPTYPE\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTION_A {
    #[doc = "0: Idle state, no action triggered. Writing 0x00 exits current mode."]
    VALUE1,
    #[doc = "81: Start one-shot write operation with automatic verify."]
    VALUE2,
    #[doc = "145: Start one-shot write operation without verify."]
    VALUE3,
    #[doc = "97: Start continuous write operation with automatic verify of every write."]
    VALUE4,
    #[doc = "161: Start continuous write operation without verify."]
    VALUE5,
    #[doc = "146: Start one-shot page erase operation."]
    VALUE6,
    #[doc = "162: Start continuous page erase operation."]
    VALUE7,
    #[doc = "208: Start one-shot verify-only: Written data is compared to array content."]
    VALUE8,
    #[doc = "224: Start continuous verify-only: Written data is compared to array content."]
    VALUE9,
}
impl From<ACTION_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        match variant {
            ACTION_A::VALUE1 => 0,
            ACTION_A::VALUE2 => 81,
            ACTION_A::VALUE3 => 145,
            ACTION_A::VALUE4 => 97,
            ACTION_A::VALUE5 => 161,
            ACTION_A::VALUE6 => 146,
            ACTION_A::VALUE7 => 162,
            ACTION_A::VALUE8 => 208,
            ACTION_A::VALUE9 => 224,
        }
    }
}
#[doc = "Reader of field `ACTION`"]
pub type ACTION_R = crate::R<u8, ACTION_A>;
impl ACTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACTION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACTION_A::VALUE1),
            81 => Val(ACTION_A::VALUE2),
            145 => Val(ACTION_A::VALUE3),
            97 => Val(ACTION_A::VALUE4),
            161 => Val(ACTION_A::VALUE5),
            146 => Val(ACTION_A::VALUE6),
            162 => Val(ACTION_A::VALUE7),
            208 => Val(ACTION_A::VALUE8),
            224 => Val(ACTION_A::VALUE9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACTION_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACTION_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ACTION_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ACTION_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ACTION_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ACTION_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == ACTION_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == ACTION_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == ACTION_A::VALUE9
    }
}
#[doc = "Write proxy for field `ACTION`"]
pub struct ACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Idle state, no action triggered. Writing 0x00 exits current mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE1)
    }
    #[doc = "Start one-shot write operation with automatic verify."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE2)
    }
    #[doc = "Start one-shot write operation without verify."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE3)
    }
    #[doc = "Start continuous write operation with automatic verify of every write."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE4)
    }
    #[doc = "Start continuous write operation without verify."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE5)
    }
    #[doc = "Start one-shot page erase operation."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE6)
    }
    #[doc = "Start continuous page erase operation."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE7)
    }
    #[doc = "Start one-shot verify-only: Written data is compared to array content."]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE8)
    }
    #[doc = "Start continuous verify-only: Written data is compared to array content."]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(ACTION_A::VALUE9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Reset ECC"]
    #[inline(always)]
    pub fn rstecc(&self) -> RSTECC_R {
        RSTECC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reset Verify Error"]
    #[inline(always)]
    pub fn rstverr(&self) -> RSTVERR_R {
        RSTVERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - ACTION: \\[VERIFY, ONE_SHOT, OPTYPE\\]"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - Reset ECC"]
    #[inline(always)]
    pub fn rstecc(&mut self) -> RSTECC_W {
        RSTECC_W { w: self }
    }
    #[doc = "Bit 12 - Reset Verify Error"]
    #[inline(always)]
    pub fn rstverr(&mut self) -> RSTVERR_W {
        RSTVERR_W { w: self }
    }
    #[doc = "Bits 0:7 - ACTION: \\[VERIFY, ONE_SHOT, OPTYPE\\]"]
    #[inline(always)]
    pub fn action(&mut self) -> ACTION_W {
        ACTION_W { w: self }
    }
}
