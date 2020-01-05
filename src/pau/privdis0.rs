#[doc = "Reader of register PRIVDIS0"]
pub type R = crate::R<u32, super::PRIVDIS0>;
#[doc = "Writer for register PRIVDIS0"]
pub type W = crate::W<u32, super::PRIVDIS0>;
#[doc = "Register PRIVDIS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVDIS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flash SFRs Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS2_A {
    #[doc = "0: Flash SFRs are accessible."]
    VALUE1,
    #[doc = "1: Flash SFRs are not accessible."]
    VALUE2,
}
impl From<PDIS2_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS2_A) -> Self {
        match variant {
            PDIS2_A::VALUE1 => false,
            PDIS2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS2`"]
pub type PDIS2_R = crate::R<bool, PDIS2_A>;
impl PDIS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS2_A {
        match self.bits {
            false => PDIS2_A::VALUE1,
            true => PDIS2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS2_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS2`"]
pub struct PDIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash SFRs are accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS2_A::VALUE1)
    }
    #[doc = "Flash SFRs are not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS2_A::VALUE2)
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
#[doc = "RAM Block 1 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS5_A {
    #[doc = "0: RAM Block 1 is accessible."]
    VALUE1,
    #[doc = "1: RAM Block 1 is not accessible."]
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
    #[doc = "RAM Block 1 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS5_A::VALUE1)
    }
    #[doc = "RAM Block 1 is not accessible."]
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
#[doc = "RAM Block 2 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS6_A {
    #[doc = "0: RAM Block 2 is accessible."]
    VALUE1,
    #[doc = "1: RAM Block 2 is not accessible."]
    VALUE2,
}
impl From<PDIS6_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS6_A) -> Self {
        match variant {
            PDIS6_A::VALUE1 => false,
            PDIS6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS6`"]
pub type PDIS6_R = crate::R<bool, PDIS6_A>;
impl PDIS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS6_A {
        match self.bits {
            false => PDIS6_A::VALUE1,
            true => PDIS6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS6_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS6`"]
pub struct PDIS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM Block 2 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS6_A::VALUE1)
    }
    #[doc = "RAM Block 2 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS6_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "RAM Block 3 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS7_A {
    #[doc = "0: RAM Block 3 is accessible."]
    VALUE1,
    #[doc = "1: RAM Block 3 is not accessible."]
    VALUE2,
}
impl From<PDIS7_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS7_A) -> Self {
        match variant {
            PDIS7_A::VALUE1 => false,
            PDIS7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS7`"]
pub type PDIS7_R = crate::R<bool, PDIS7_A>;
impl PDIS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS7_A {
        match self.bits {
            false => PDIS7_A::VALUE1,
            true => PDIS7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS7_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS7`"]
pub struct PDIS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM Block 3 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS7_A::VALUE1)
    }
    #[doc = "RAM Block 3 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS7_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "WDT Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS19_A {
    #[doc = "0: WDT is accessible."]
    VALUE1,
    #[doc = "1: WDT is not accessible."]
    VALUE2,
}
impl From<PDIS19_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS19_A) -> Self {
        match variant {
            PDIS19_A::VALUE1 => false,
            PDIS19_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS19`"]
pub type PDIS19_R = crate::R<bool, PDIS19_A>;
impl PDIS19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS19_A {
        match self.bits {
            false => PDIS19_A::VALUE1,
            true => PDIS19_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS19_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS19_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS19`"]
pub struct PDIS19_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "WDT is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS19_A::VALUE1)
    }
    #[doc = "WDT is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS19_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Port 0 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS22_A {
    #[doc = "0: Port 0 is accessible."]
    VALUE1,
    #[doc = "1: Port 0 is not accessible."]
    VALUE2,
}
impl From<PDIS22_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS22_A) -> Self {
        match variant {
            PDIS22_A::VALUE1 => false,
            PDIS22_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS22`"]
pub type PDIS22_R = crate::R<bool, PDIS22_A>;
impl PDIS22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS22_A {
        match self.bits {
            false => PDIS22_A::VALUE1,
            true => PDIS22_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS22_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS22_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS22`"]
pub struct PDIS22_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port 0 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS22_A::VALUE1)
    }
    #[doc = "Port 0 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS22_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Port 1 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS23_A {
    #[doc = "0: Port 1 is accessible."]
    VALUE1,
    #[doc = "1: Port 1 is not accessible."]
    VALUE2,
}
impl From<PDIS23_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS23_A) -> Self {
        match variant {
            PDIS23_A::VALUE1 => false,
            PDIS23_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS23`"]
pub type PDIS23_R = crate::R<bool, PDIS23_A>;
impl PDIS23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS23_A {
        match self.bits {
            false => PDIS23_A::VALUE1,
            true => PDIS23_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS23_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS23_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS23`"]
pub struct PDIS23_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port 1 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS23_A::VALUE1)
    }
    #[doc = "Port 1 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS23_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port 2 Privilege Disable Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIS24_A {
    #[doc = "0: Port 2 is accessible."]
    VALUE1,
    #[doc = "1: Port 2 is not accessible."]
    VALUE2,
}
impl From<PDIS24_A> for bool {
    #[inline(always)]
    fn from(variant: PDIS24_A) -> Self {
        match variant {
            PDIS24_A::VALUE1 => false,
            PDIS24_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PDIS24`"]
pub type PDIS24_R = crate::R<bool, PDIS24_A>;
impl PDIS24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDIS24_A {
        match self.bits {
            false => PDIS24_A::VALUE1,
            true => PDIS24_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDIS24_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDIS24_A::VALUE2
    }
}
#[doc = "Write proxy for field `PDIS24`"]
pub struct PDIS24_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIS24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDIS24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port 2 is accessible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDIS24_A::VALUE1)
    }
    #[doc = "Port 2 is not accessible."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDIS24_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Flash SFRs Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis2(&self) -> PDIS2_R {
        PDIS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RAM Block 1 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis5(&self) -> PDIS5_R {
        PDIS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RAM Block 2 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis6(&self) -> PDIS6_R {
        PDIS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RAM Block 3 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis7(&self) -> PDIS7_R {
        PDIS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WDT Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis19(&self) -> PDIS19_R {
        PDIS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port 0 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis22(&self) -> PDIS22_R {
        PDIS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port 1 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis23(&self) -> PDIS23_R {
        PDIS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port 2 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis24(&self) -> PDIS24_R {
        PDIS24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Flash SFRs Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis2(&mut self) -> PDIS2_W {
        PDIS2_W { w: self }
    }
    #[doc = "Bit 5 - RAM Block 1 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis5(&mut self) -> PDIS5_W {
        PDIS5_W { w: self }
    }
    #[doc = "Bit 6 - RAM Block 2 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis6(&mut self) -> PDIS6_W {
        PDIS6_W { w: self }
    }
    #[doc = "Bit 7 - RAM Block 3 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis7(&mut self) -> PDIS7_W {
        PDIS7_W { w: self }
    }
    #[doc = "Bit 19 - WDT Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis19(&mut self) -> PDIS19_W {
        PDIS19_W { w: self }
    }
    #[doc = "Bit 22 - Port 0 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis22(&mut self) -> PDIS22_W {
        PDIS22_W { w: self }
    }
    #[doc = "Bit 23 - Port 1 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis23(&mut self) -> PDIS23_W {
        PDIS23_W { w: self }
    }
    #[doc = "Bit 24 - Port 2 Privilege Disable Flag"]
    #[inline(always)]
    pub fn pdis24(&mut self) -> PDIS24_W {
        PDIS24_W { w: self }
    }
}
