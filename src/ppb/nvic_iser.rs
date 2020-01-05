#[doc = "Reader of register NVIC_ISER"]
pub type R = crate::R<u32, super::NVIC_ISER>;
#[doc = "Writer for register NVIC_ISER"]
pub type W = crate::W<u32, super::NVIC_ISER>;
#[doc = "Register NVIC_ISER `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ISER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Node Set-enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETENA_A {
    #[doc = "0: Read: Interrupt node disabled. Write: No effect."]
    VALUE1,
    #[doc = "1: Read: Interrupt node enabled. Write: Enable interrupt node"]
    VALUE2,
}
impl From<SETENA_A> for u32 {
    #[inline(always)]
    fn from(variant: SETENA_A) -> Self {
        match variant {
            SETENA_A::VALUE1 => 0,
            SETENA_A::VALUE2 => 1,
        }
    }
}
#[doc = "Reader of field `SETENA`"]
pub type SETENA_R = crate::R<u32, SETENA_A>;
impl SETENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, SETENA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETENA_A::VALUE1),
            1 => Val(SETENA_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SETENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SETENA_A::VALUE2
    }
}
#[doc = "Write proxy for field `SETENA`"]
pub struct SETENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETENA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read: Interrupt node disabled. Write: No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SETENA_A::VALUE1)
    }
    #[doc = "Read: Interrupt node enabled. Write: Enable interrupt node"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SETENA_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Node Set-enable"]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Node Set-enable"]
    #[inline(always)]
    pub fn setena(&mut self) -> SETENA_W {
        SETENA_W { w: self }
    }
}
