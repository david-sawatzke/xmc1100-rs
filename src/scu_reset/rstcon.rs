#[doc = "Reader of register RSTCON"]
pub type R = crate::R<u32, super::RSTCON>;
#[doc = "Writer for register RSTCON"]
pub type W = crate::W<u32, super::RSTCON>;
#[doc = "Register RSTCON `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable ECC Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCRSTEN_A {
    #[doc = "0: No reset when ECC double bit error occur"]
    VALUE1,
    #[doc = "1: Reset when ECC double bit error occur"]
    VALUE2,
}
impl From<ECCRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECCRSTEN_A) -> Self {
        match variant {
            ECCRSTEN_A::VALUE1 => false,
            ECCRSTEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ECCRSTEN`"]
pub type ECCRSTEN_R = crate::R<bool, ECCRSTEN_A>;
impl ECCRSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCRSTEN_A {
        match self.bits {
            false => ECCRSTEN_A::VALUE1,
            true => ECCRSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECCRSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECCRSTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ECCRSTEN`"]
pub struct ECCRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCRSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset when ECC double bit error occur"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECCRSTEN_A::VALUE1)
    }
    #[doc = "Reset when ECC double bit error occur"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECCRSTEN_A::VALUE2)
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
#[doc = "Enable Loss of Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRSTEN_A {
    #[doc = "0: No reset when loss of clock occur"]
    VALUE1,
    #[doc = "1: Reset when loss of clock occur"]
    VALUE2,
}
impl From<LOCRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCRSTEN_A) -> Self {
        match variant {
            LOCRSTEN_A::VALUE1 => false,
            LOCRSTEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LOCRSTEN`"]
pub type LOCRSTEN_R = crate::R<bool, LOCRSTEN_A>;
impl LOCRSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCRSTEN_A {
        match self.bits {
            false => LOCRSTEN_A::VALUE1,
            true => LOCRSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCRSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCRSTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `LOCRSTEN`"]
pub struct LOCRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCRSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset when loss of clock occur"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCRSTEN_A::VALUE1)
    }
    #[doc = "Reset when loss of clock occur"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCRSTEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable 16kbytes SRAM Parity Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPERSTEN_A {
    #[doc = "0: No reset when SRAM parity error occur"]
    VALUE1,
    #[doc = "1: Reset when SRAM parity error occur"]
    VALUE2,
}
impl From<SPERSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPERSTEN_A) -> Self {
        match variant {
            SPERSTEN_A::VALUE1 => false,
            SPERSTEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SPERSTEN`"]
pub type SPERSTEN_R = crate::R<bool, SPERSTEN_A>;
impl SPERSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPERSTEN_A {
        match self.bits {
            false => SPERSTEN_A::VALUE1,
            true => SPERSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SPERSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SPERSTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SPERSTEN`"]
pub struct SPERSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPERSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPERSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset when SRAM parity error occur"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SPERSTEN_A::VALUE1)
    }
    #[doc = "Reset when SRAM parity error occur"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SPERSTEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable USIC0 SRAM Parity Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum U0PERSTEN_A {
    #[doc = "0: No reset when USIC0 memory parity error occur"]
    VALUE1,
    #[doc = "1: Reset when USIC0 memory parity error occur"]
    VALUE2,
}
impl From<U0PERSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: U0PERSTEN_A) -> Self {
        match variant {
            U0PERSTEN_A::VALUE1 => false,
            U0PERSTEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `U0PERSTEN`"]
pub type U0PERSTEN_R = crate::R<bool, U0PERSTEN_A>;
impl U0PERSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> U0PERSTEN_A {
        match self.bits {
            false => U0PERSTEN_A::VALUE1,
            true => U0PERSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == U0PERSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == U0PERSTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `U0PERSTEN`"]
pub struct U0PERSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> U0PERSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: U0PERSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset when USIC0 memory parity error occur"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(U0PERSTEN_A::VALUE1)
    }
    #[doc = "Reset when USIC0 memory parity error occur"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(U0PERSTEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable Master Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRSTEN_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Triggered Master reset"]
    VALUE2,
}
impl From<MRSTEN_AW> for bool {
    #[inline(always)]
    fn from(variant: MRSTEN_AW) -> Self {
        match variant {
            MRSTEN_AW::VALUE1 => false,
            MRSTEN_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `MRSTEN`"]
pub struct MRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRSTEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MRSTEN_AW::VALUE1)
    }
    #[doc = "Triggered Master reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MRSTEN_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable ECC Error Reset"]
    #[inline(always)]
    pub fn eccrsten(&self) -> ECCRSTEN_R {
        ECCRSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Loss of Clock Reset"]
    #[inline(always)]
    pub fn locrsten(&self) -> LOCRSTEN_R {
        LOCRSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable 16kbytes SRAM Parity Error Reset"]
    #[inline(always)]
    pub fn spersten(&self) -> SPERSTEN_R {
        SPERSTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable USIC0 SRAM Parity Error Reset"]
    #[inline(always)]
    pub fn u0persten(&self) -> U0PERSTEN_R {
        U0PERSTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC Error Reset"]
    #[inline(always)]
    pub fn eccrsten(&mut self) -> ECCRSTEN_W {
        ECCRSTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Loss of Clock Reset"]
    #[inline(always)]
    pub fn locrsten(&mut self) -> LOCRSTEN_W {
        LOCRSTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable 16kbytes SRAM Parity Error Reset"]
    #[inline(always)]
    pub fn spersten(&mut self) -> SPERSTEN_W {
        SPERSTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable USIC0 SRAM Parity Error Reset"]
    #[inline(always)]
    pub fn u0persten(&mut self) -> U0PERSTEN_W {
        U0PERSTEN_W { w: self }
    }
    #[doc = "Bit 16 - Enable Master Reset"]
    #[inline(always)]
    pub fn mrsten(&mut self) -> MRSTEN_W {
        MRSTEN_W { w: self }
    }
}
