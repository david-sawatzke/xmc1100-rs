#[doc = "Reader of register TIMCFG0"]
pub type R = crate::R<u32, super::TIMCFG0>;
#[doc = "Writer for register TIMCFG0"]
pub type W = crate::W<u32, super::TIMCFG0>;
#[doc = "Register TIMCFG0 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TIMCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Accelerated Timing\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AT_A {
    #[doc = "0: Compatible timing: Result available after standard conversion time"]
    VALUE1,
    #[doc = "1: Accelerated timing: Result available as soon as converted"]
    VALUE2,
}
impl From<AT_A> for bool {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        match variant {
            AT_A::VALUE1 => false,
            AT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AT`"]
pub type AT_R = crate::R<bool, AT_A>;
impl AT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AT_A {
        match self.bits {
            false => AT_A::VALUE1,
            true => AT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AT_A::VALUE2
    }
}
#[doc = "Write proxy for field `AT`"]
pub struct AT_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compatible timing: Result available after standard conversion time"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AT_A::VALUE1)
    }
    #[doc = "Accelerated timing: Result available as soon as converted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AT_A::VALUE2)
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
#[doc = "Fast Compare Mode Response Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCRT_A {
    #[doc = "0: Result after tADCI o 2"]
    VALUE1,
    #[doc = "15: Result after tADCI o 32"]
    VALUE2,
}
impl From<FCRT_A> for u8 {
    #[inline(always)]
    fn from(variant: FCRT_A) -> Self {
        match variant {
            FCRT_A::VALUE1 => 0,
            FCRT_A::VALUE2 => 15,
        }
    }
}
#[doc = "Reader of field `FCRT`"]
pub type FCRT_R = crate::R<u8, FCRT_A>;
impl FCRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FCRT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FCRT_A::VALUE1),
            15 => Val(FCRT_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCRT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCRT_A::VALUE2
    }
}
#[doc = "Write proxy for field `FCRT`"]
pub struct FCRT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCRT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Result after tADCI o 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCRT_A::VALUE1)
    }
    #[doc = "Result after tADCI o 32"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCRT_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Short Sample Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SST_A {
    #[doc = "0: Compatible timing: Sample time is defined by DIVA and STC."]
    VALUE1,
    #[doc = "1: Sample time is tADC o 1"]
    VALUE2,
    #[doc = "63: Sample time is tADC o 63"]
    VALUE3,
}
impl From<SST_A> for u8 {
    #[inline(always)]
    fn from(variant: SST_A) -> Self {
        match variant {
            SST_A::VALUE1 => 0,
            SST_A::VALUE2 => 1,
            SST_A::VALUE3 => 63,
        }
    }
}
#[doc = "Reader of field `SST`"]
pub type SST_R = crate::R<u8, SST_A>;
impl SST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SST_A::VALUE1),
            1 => Val(SST_A::VALUE2),
            63 => Val(SST_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SST_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SST_A::VALUE3
    }
}
#[doc = "Write proxy for field `SST`"]
pub struct SST_W<'a> {
    w: &'a mut W,
}
impl<'a> SST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Compatible timing: Sample time is defined by DIVA and STC."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SST_A::VALUE1)
    }
    #[doc = "Sample time is tADC o 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SST_A::VALUE2)
    }
    #[doc = "Sample time is tADC o 63"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SST_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TGEN`"]
pub type TGEN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Accelerated Timing"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Fast Compare Mode Response Time"]
    #[inline(always)]
    pub fn fcrt(&self) -> FCRT_R {
        FCRT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Short Sample Time"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Timing Generator"]
    #[inline(always)]
    pub fn tgen(&self) -> TGEN_R {
        TGEN_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Accelerated Timing"]
    #[inline(always)]
    pub fn at(&mut self) -> AT_W {
        AT_W { w: self }
    }
    #[doc = "Bits 4:7 - Fast Compare Mode Response Time"]
    #[inline(always)]
    pub fn fcrt(&mut self) -> FCRT_W {
        FCRT_W { w: self }
    }
    #[doc = "Bits 8:13 - Short Sample Time"]
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W {
        SST_W { w: self }
    }
}
