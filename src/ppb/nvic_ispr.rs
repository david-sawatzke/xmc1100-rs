#[doc = "Reader of register NVIC_ISPR"]
pub type R = crate::R<u32, super::NVIC_ISPR>;
#[doc = "Writer for register NVIC_ISPR"]
pub type W = crate::W<u32, super::NVIC_ISPR>;
#[doc = "Register NVIC_ISPR `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Node Set-pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETPEND_A {
    #[doc = "0: Read: Interrupt node is not pending. Write: No effect"]
    VALUE1,
    #[doc = "1: Read: Interrupt node is pending. Write: Change interrupt state to pending."]
    VALUE2,
}
impl From<SETPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: SETPEND_A) -> Self {
        match variant {
            SETPEND_A::VALUE1 => 0,
            SETPEND_A::VALUE2 => 1,
        }
    }
}
#[doc = "Reader of field `SETPEND`"]
pub type SETPEND_R = crate::R<u32, SETPEND_A>;
impl SETPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SETPEND_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETPEND_A::VALUE1),
            1 => Val(SETPEND_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SETPEND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SETPEND_A::VALUE2
    }
}
#[doc = "Write proxy for field `SETPEND`"]
pub struct SETPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETPEND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read: Interrupt node is not pending. Write: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SETPEND_A::VALUE1)
    }
    #[doc = "Read: Interrupt node is pending. Write: Change interrupt state to pending."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SETPEND_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Node Set-pending"]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Node Set-pending"]
    #[inline(always)]
    pub fn setpend(&mut self) -> SETPEND_W {
        SETPEND_W { w: self }
    }
}
