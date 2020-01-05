#[doc = "Reader of register NVIC_ICER"]
pub type R = crate::R<u32, super::NVIC_ICER>;
#[doc = "Writer for register NVIC_ICER"]
pub type W = crate::W<u32, super::NVIC_ICER>;
#[doc = "Register NVIC_ICER `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_ICER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Node Clear-enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRENA_A {
    #[doc = "0: Read: Interrupt node disabled. Write: No effect"]
    VALUE1,
    #[doc = "1: Read: Interrupt node enabled. Write: Disable interrupt node."]
    VALUE2,
}
impl From<CLRENA_A> for u32 {
    #[inline(always)]
    fn from(variant: CLRENA_A) -> Self {
        match variant {
            CLRENA_A::VALUE1 => 0,
            CLRENA_A::VALUE2 => 1,
        }
    }
}
#[doc = "Reader of field `CLRENA`"]
pub type CLRENA_R = crate::R<u32, CLRENA_A>;
impl CLRENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLRENA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLRENA_A::VALUE1),
            1 => Val(CLRENA_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLRENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLRENA_A::VALUE2
    }
}
#[doc = "Write proxy for field `CLRENA`"]
pub struct CLRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRENA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read: Interrupt node disabled. Write: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLRENA_A::VALUE1)
    }
    #[doc = "Read: Interrupt node enabled. Write: Disable interrupt node."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLRENA_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Node Clear-enable"]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Node Clear-enable"]
    #[inline(always)]
    pub fn clrena(&mut self) -> CLRENA_W {
        CLRENA_W { w: self }
    }
}
