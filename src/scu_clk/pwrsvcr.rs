#[doc = "Reader of register PWRSVCR"]
pub type R = crate::R<u32, super::PWRSVCR>;
#[doc = "Writer for register PWRSVCR"]
pub type W = crate::W<u32, super::PWRSVCR>;
#[doc = "Register PWRSVCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRSVCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPD_A {
    #[doc = "0: no effect"]
    VALUE1,
    #[doc = "1: Flash power down when entering power save mode. Upon wake-up, CPU is able to fetch code from flash."]
    VALUE2,
}
impl From<FPD_A> for bool {
    #[inline(always)]
    fn from(variant: FPD_A) -> Self {
        match variant {
            FPD_A::VALUE1 => false,
            FPD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FPD`"]
pub type FPD_R = crate::R<bool, FPD_A>;
impl FPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPD_A {
        match self.bits {
            false => FPD_A::VALUE1,
            true => FPD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FPD_A::VALUE2
    }
}
#[doc = "Write proxy for field `FPD`"]
pub struct FPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPD_A::VALUE1)
    }
    #[doc = "Flash power down when entering power save mode. Upon wake-up, CPU is able to fetch code from flash."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPD_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Power Down"]
    #[inline(always)]
    pub fn fpd(&self) -> FPD_R {
        FPD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Power Down"]
    #[inline(always)]
    pub fn fpd(&mut self) -> FPD_W {
        FPD_W { w: self }
    }
}
