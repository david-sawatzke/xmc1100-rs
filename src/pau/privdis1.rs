#[doc = "Reader of register PRIVDIS1"]
pub type R = crate::R<u32, super::PRIVDIS1>;
#[doc = "Writer for register PRIVDIS1"]
pub type W = crate::W<u32, super::PRIVDIS1>;
#[doc = "Register PRIVDIS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVDIS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USIC0 Channel 0 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS0_A {
    #[doc = "0: USIC0 Channel 0 is accessible."]
    VALUE1,
    #[doc = "1: USIC0 Channel 0 is not accessible."]
    VALUE2,
}
impl From<PDIS0_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS0_A) -> Self {
        match variant {
            PDIS0_A::VALUE1 => false,
            PDIS0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS0`"]
pub type PDIS0_R = crate::R<bool, PDIS0_A>;
impl PDIS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS0_A {
        match self.bits {
            false => PDIS0_A::VALUE1,
            true => PDIS0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS0`"]
pub struct PDIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USIC0 Channel 0 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS0_A::VALUE1)
    }
    #[doc = "USIC0 Channel 0 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS0_A::VALUE2)
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
#[doc = "USIC0 Channel 1 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS1_A {
    #[doc = "0: USIC0 Channel 1 is accessible."]
    VALUE1,
    #[doc = "1: USIC0 Channel 1 is not accessible."]
    VALUE2,
}
impl From<PDIS1_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS1_A) -> Self {
        match variant {
            PDIS1_A::VALUE1 => false,
            PDIS1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS1`"]
pub type PDIS1_R = crate::R<bool, PDIS1_A>;
impl PDIS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS1_A {
        match self.bits {
            false => PDIS1_A::VALUE1,
            true => PDIS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS1`"]
pub struct PDIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USIC0 Channel 1 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS1_A::VALUE1)
    }
    #[doc = "USIC0 Channel 1 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS1_A::VALUE2)
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
#[doc = "VADC0 Basic SFRs Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5_A {
    #[doc = "0: VADC0 Basic SFRs are accessible."]
    VALUE1,
    #[doc = "1: VADC0 Basic SFRs are not accessible."]
    VALUE2,
}
impl From<PDIS5_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS5_A) -> Self {
        match variant {
            PDIS5_A::VALUE1 => false,
            PDIS5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS5`"]
pub type PDIS5_R = crate::R<bool, PDIS5_A>;
impl PDIS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS5_A {
        match self.bits {
            false => PDIS5_A::VALUE1,
            true => PDIS5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS5_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS5`"]
pub struct PDIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VADC0 Basic SFRs are accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS5_A::VALUE1)
    }
    #[doc = "VADC0 Basic SFRs are not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS5_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "SHS0 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS8_A {
    #[doc = "0: SHS is accessible."]
    VALUE1,
    #[doc = "1: SHS is not accessible."]
    VALUE2,
}
impl From<PDIS8_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS8_A) -> Self {
        match variant {
            PDIS8_A::VALUE1 => false,
            PDIS8_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS8`"]
pub type PDIS8_R = crate::R<bool, PDIS8_A>;
impl PDIS8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS8_A {
        match self.bits {
            false => PDIS8_A::VALUE1,
            true => PDIS8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS8_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS8`"]
pub struct PDIS8_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SHS is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS8_A::VALUE1)
    }
    #[doc = "SHS is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS8_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "CC40 and CCU40 Kernel SFRs Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS9_A {
    #[doc = "0: CC40 and CCU40 Kernel SFRs are accessible."]
    VALUE1,
    #[doc = "1: CC40 and CCU40 Kernel SFRs are not accessible."]
    VALUE2,
}
impl From<PDIS9_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS9_A) -> Self {
        match variant {
            PDIS9_A::VALUE1 => false,
            PDIS9_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS9`"]
pub type PDIS9_R = crate::R<bool, PDIS9_A>;
impl PDIS9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS9_A {
        match self.bits {
            false => PDIS9_A::VALUE1,
            true => PDIS9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS9_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS9`"]
pub struct PDIS9_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC40 and CCU40 Kernel SFRs are accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS9_A::VALUE1)
    }
    #[doc = "CC40 and CCU40 Kernel SFRs are not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS9_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "CC41 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS10_A {
    #[doc = "0: CC41 is accessible."]
    VALUE1,
    #[doc = "1: CC41 is not accessible."]
    VALUE2,
}
impl From<PDIS10_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS10_A) -> Self {
        match variant {
            PDIS10_A::VALUE1 => false,
            PDIS10_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS10`"]
pub type PDIS10_R = crate::R<bool, PDIS10_A>;
impl PDIS10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS10_A {
        match self.bits {
            false => PDIS10_A::VALUE1,
            true => PDIS10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS10_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS10`"]
pub struct PDIS10_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC41 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS10_A::VALUE1)
    }
    #[doc = "CC41 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS10_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "CC42 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS11_A {
    #[doc = "0: CC42 is accessible."]
    VALUE1,
    #[doc = "1: CC42 is not accessible."]
    VALUE2,
}
impl From<PDIS11_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS11_A) -> Self {
        match variant {
            PDIS11_A::VALUE1 => false,
            PDIS11_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS11`"]
pub type PDIS11_R = crate::R<bool, PDIS11_A>;
impl PDIS11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS11_A {
        match self.bits {
            false => PDIS11_A::VALUE1,
            true => PDIS11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS11_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS11`"]
pub struct PDIS11_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC42 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS11_A::VALUE1)
    }
    #[doc = "CC42 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS11_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "CC43 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS12_A {
    #[doc = "0: CC43 is accessible."]
    VALUE1,
    #[doc = "1: CC43 is not accessible."]
    VALUE2,
}
impl From<PDIS12_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS12_A) -> Self {
        match variant {
            PDIS12_A::VALUE1 => false,
            PDIS12_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS12`"]
pub type PDIS12_R = crate::R<bool, PDIS12_A>;
impl PDIS12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS12_A {
        match self.bits {
            false => PDIS12_A::VALUE1,
            true => PDIS12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS12_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS12`"]
pub struct PDIS12_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CC43 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS12_A::VALUE1)
    }
    #[doc = "CC43 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS12_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USIC0 Channel 0 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis0(&self) -> PDIS0_R {
        PDIS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USIC0 Channel 1 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis1(&self) -> PDIS1_R {
        PDIS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VADC0 Basic SFRs Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis5(&self) -> PDIS5_R {
        PDIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SHS0 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis8(&self) -> PDIS8_R {
        PDIS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CC40 and CCU40 Kernel SFRs Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis9(&self) -> PDIS9_R {
        PDIS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CC41 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis10(&self) -> PDIS10_R {
        PDIS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CC42 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis11(&self) -> PDIS11_R {
        PDIS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CC43 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis12(&self) -> PDIS12_R {
        PDIS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USIC0 Channel 0 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis0(&mut self) -> PDIS0_W {
        PDIS0_W { w: self }
    }
    #[doc = "Bit 1 - USIC0 Channel 1 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis1(&mut self) -> PDIS1_W {
        PDIS1_W { w: self }
    }
    #[doc = "Bit 5 - VADC0 Basic SFRs Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis5(&mut self) -> PDIS5_W {
        PDIS5_W { w: self }
    }
    #[doc = "Bit 8 - SHS0 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis8(&mut self) -> PDIS8_W {
        PDIS8_W { w: self }
    }
    #[doc = "Bit 9 - CC40 and CCU40 Kernel SFRs Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis9(&mut self) -> PDIS9_W {
        PDIS9_W { w: self }
    }
    #[doc = "Bit 10 - CC41 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis10(&mut self) -> PDIS10_W {
        PDIS10_W { w: self }
    }
    #[doc = "Bit 11 - CC42 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis11(&mut self) -> PDIS11_W {
        PDIS11_W { w: self }
    }
    #[doc = "Bit 12 - CC43 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis12(&mut self) -> PDIS12_W {
        PDIS12_W { w: self }
    }
}
