#[doc = "Reader of register ANAVDEL"]
pub type R = crate::R<u16, super::ANAVDEL>;
#[doc = "Writer for register ANAVDEL"]
pub type W = crate::W<u16, super::ANAVDEL>;
#[doc = "Register ANAVDEL `reset()`'s with value 0x1c"]
impl crate::ResetValue for super::ANAVDEL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1c
    }
}
#[doc = "VDEL Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDEL_SELECT_A {
    #[doc = "0: 2.25V"]
    VALUE1,
    #[doc = "1: 3.0V"]
    VALUE2,
    #[doc = "2: 4.4V"]
    VALUE3,
}
impl From<VDEL_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: VDEL_SELECT_A) -> Self {
        match variant {
            VDEL_SELECT_A::VALUE1 => 0,
            VDEL_SELECT_A::VALUE2 => 1,
            VDEL_SELECT_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `VDEL_SELECT`"]
pub type VDEL_SELECT_R = crate::R<u8, VDEL_SELECT_A>;
impl VDEL_SELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VDEL_SELECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VDEL_SELECT_A::VALUE1),
            1 => Val(VDEL_SELECT_A::VALUE2),
            2 => Val(VDEL_SELECT_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VDEL_SELECT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VDEL_SELECT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VDEL_SELECT_A::VALUE3
    }
}
#[doc = "Write proxy for field `VDEL_SELECT`"]
pub struct VDEL_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VDEL_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDEL_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2.25V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VDEL_SELECT_A::VALUE1)
    }
    #[doc = "3.0V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VDEL_SELECT_A::VALUE2)
    }
    #[doc = "4.4V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(VDEL_SELECT_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "VDEL Timing Setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDEL_TIM_ADJ_A {
    #[doc = "0: typ 1us - slowest response time"]
    VALUE1,
    #[doc = "1: typ 500n"]
    VALUE2,
    #[doc = "2: typ 250n"]
    VALUE3,
    #[doc = "3: no delay - fastest response time."]
    VALUE4,
}
impl From<VDEL_TIM_ADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: VDEL_TIM_ADJ_A) -> Self {
        match variant {
            VDEL_TIM_ADJ_A::VALUE1 => 0,
            VDEL_TIM_ADJ_A::VALUE2 => 1,
            VDEL_TIM_ADJ_A::VALUE3 => 2,
            VDEL_TIM_ADJ_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `VDEL_TIM_ADJ`"]
pub type VDEL_TIM_ADJ_R = crate::R<u8, VDEL_TIM_ADJ_A>;
impl VDEL_TIM_ADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDEL_TIM_ADJ_A {
        match self.bits {
            0 => VDEL_TIM_ADJ_A::VALUE1,
            1 => VDEL_TIM_ADJ_A::VALUE2,
            2 => VDEL_TIM_ADJ_A::VALUE3,
            3 => VDEL_TIM_ADJ_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VDEL_TIM_ADJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VDEL_TIM_ADJ_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VDEL_TIM_ADJ_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VDEL_TIM_ADJ_A::VALUE4
    }
}
#[doc = "Write proxy for field `VDEL_TIM_ADJ`"]
pub struct VDEL_TIM_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> VDEL_TIM_ADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDEL_TIM_ADJ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "typ 1us - slowest response time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJ_A::VALUE1)
    }
    #[doc = "typ 500n"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJ_A::VALUE2)
    }
    #[doc = "typ 250n"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJ_A::VALUE3)
    }
    #[doc = "no delay - fastest response time."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(VDEL_TIM_ADJ_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "VDEL unit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDEL_EN_A {
    #[doc = "0: VDEL is disabled"]
    VALUE1,
    #[doc = "1: VDEL is active"]
    VALUE2,
}
impl From<VDEL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VDEL_EN_A) -> Self {
        match variant {
            VDEL_EN_A::VALUE1 => false,
            VDEL_EN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VDEL_EN`"]
pub type VDEL_EN_R = crate::R<bool, VDEL_EN_A>;
impl VDEL_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDEL_EN_A {
        match self.bits {
            false => VDEL_EN_A::VALUE1,
            true => VDEL_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VDEL_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VDEL_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `VDEL_EN`"]
pub struct VDEL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDEL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDEL_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VDEL is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VDEL_EN_A::VALUE1)
    }
    #[doc = "VDEL is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VDEL_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - VDEL Range Select"]
    #[inline(always)]
    pub fn vdel_select(&self) -> VDEL_SELECT_R {
        VDEL_SELECT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - VDEL Timing Setting"]
    #[inline(always)]
    pub fn vdel_tim_adj(&self) -> VDEL_TIM_ADJ_R {
        VDEL_TIM_ADJ_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - VDEL unit Enable"]
    #[inline(always)]
    pub fn vdel_en(&self) -> VDEL_EN_R {
        VDEL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VDEL Range Select"]
    #[inline(always)]
    pub fn vdel_select(&mut self) -> VDEL_SELECT_W {
        VDEL_SELECT_W { w: self }
    }
    #[doc = "Bits 2:3 - VDEL Timing Setting"]
    #[inline(always)]
    pub fn vdel_tim_adj(&mut self) -> VDEL_TIM_ADJ_W {
        VDEL_TIM_ADJ_W { w: self }
    }
    #[doc = "Bit 4 - VDEL unit Enable"]
    #[inline(always)]
    pub fn vdel_en(&mut self) -> VDEL_EN_W {
        VDEL_EN_W { w: self }
    }
}
