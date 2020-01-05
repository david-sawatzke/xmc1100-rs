#[doc = "Reader of register PMTSR"]
pub type R = crate::R<u32, super::PMTSR>;
#[doc = "Writer for register PMTSR"]
pub type W = crate::W<u32, super::PMTSR>;
#[doc = "Register PMTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Test Enable Control for 16kbytes SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENS_A {
    #[doc = "0: standard operation"]
    VALUE1,
    #[doc = "1: generate an inverted parity bit during a write operation"]
    VALUE2,
}
impl From<MTENS_A> for bool {
    #[inline(always)]
    fn from(variant: MTENS_A) -> Self {
        match variant {
            MTENS_A::VALUE1 => false,
            MTENS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MTENS`"]
pub type MTENS_R = crate::R<bool, MTENS_A>;
impl MTENS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENS_A {
        match self.bits {
            false => MTENS_A::VALUE1,
            true => MTENS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MTENS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MTENS_A::VALUE2
    }
}
#[doc = "Write proxy for field `MTENS`"]
pub struct MTENS_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENS_A::VALUE1)
    }
    #[doc = "generate an inverted parity bit during a write operation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENS_A::VALUE2)
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
    #[doc = "Bit 0 - Parity Test Enable Control for 16kbytes SRAM"]
    #[inline(always)]
    pub fn mtens(&self) -> MTENS_R {
        MTENS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Test Enable Control for 16kbytes SRAM"]
    #[inline(always)]
    pub fn mtens(&mut self) -> MTENS_W {
        MTENS_W { w: self }
    }
}
