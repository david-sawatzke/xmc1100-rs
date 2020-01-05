#[doc = "Reader of register ANAOFFSET"]
pub type R = crate::R<u16, super::ANAOFFSET>;
#[doc = "Writer for register ANAOFFSET"]
pub type W = crate::W<u16, super::ANAOFFSET>;
#[doc = "Register ANAOFFSET `reset()`'s with value 0x04"]
impl crate::ResetValue for super::ANAOFFSET {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "ADJL Offset register\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADJL_OFFSET_A {
    #[doc = "0: - 3.75%, typ."]
    VALUE1,
    #[doc = "1: - 2.85%, typ."]
    VALUE2,
    #[doc = "4: 0, default"]
    VALUE3,
    #[doc = "5: + 0.95%, typ."]
    VALUE4,
    #[doc = "8: + 3.75%, typ."]
    VALUE5,
}
impl From<ADJL_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: ADJL_OFFSET_A) -> Self {
        match variant {
            ADJL_OFFSET_A::VALUE1 => 0,
            ADJL_OFFSET_A::VALUE2 => 1,
            ADJL_OFFSET_A::VALUE3 => 4,
            ADJL_OFFSET_A::VALUE4 => 5,
            ADJL_OFFSET_A::VALUE5 => 8,
        }
    }
}
#[doc = "Reader of field `ADJL_OFFSET`"]
pub type ADJL_OFFSET_R = crate::R<u8, ADJL_OFFSET_A>;
impl ADJL_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADJL_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADJL_OFFSET_A::VALUE1),
            1 => Val(ADJL_OFFSET_A::VALUE2),
            4 => Val(ADJL_OFFSET_A::VALUE3),
            5 => Val(ADJL_OFFSET_A::VALUE4),
            8 => Val(ADJL_OFFSET_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADJL_OFFSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADJL_OFFSET_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ADJL_OFFSET_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ADJL_OFFSET_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ADJL_OFFSET_A::VALUE5
    }
}
#[doc = "Write proxy for field `ADJL_OFFSET`"]
pub struct ADJL_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJL_OFFSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADJL_OFFSET_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "- 3.75%, typ."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADJL_OFFSET_A::VALUE1)
    }
    #[doc = "- 2.85%, typ."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADJL_OFFSET_A::VALUE2)
    }
    #[doc = "0, default"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ADJL_OFFSET_A::VALUE3)
    }
    #[doc = "+ 0.95%, typ."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ADJL_OFFSET_A::VALUE4)
    }
    #[doc = "+ 3.75%, typ."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ADJL_OFFSET_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADJL Offset register"]
    #[inline(always)]
    pub fn adjl_offset(&self) -> ADJL_OFFSET_R {
        ADJL_OFFSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADJL Offset register"]
    #[inline(always)]
    pub fn adjl_offset(&mut self) -> ADJL_OFFSET_W {
        ADJL_OFFSET_W { w: self }
    }
}
