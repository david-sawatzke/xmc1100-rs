#[doc = "Reader of register PASSWD"]
pub type R = crate::R<u32, super::PASSWD>;
#[doc = "Writer for register PASSWD"]
pub type W = crate::W<u32, super::PASSWD>;
#[doc = "Register PASSWD `reset()`'s with value 0x07"]
impl crate::ResetValue for super::PASSWD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Bit Protection Scheme Control Bits\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Scheme disabled - direct access to the protected bits is allowed."]
    VALUE1,
    #[doc = "3: Scheme enabled - the bit field PASS has to be written with the passwords to open and close the access to the protected bits. (Default)"]
    VALUE2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::VALUE1 => 0,
            MODE_A::VALUE2 => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::VALUE1),
            3 => Val(MODE_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODE_A::VALUE2
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Scheme disabled - direct access to the protected bits is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODE_A::VALUE1)
    }
    #[doc = "Scheme enabled - the bit field PASS has to be written with the passwords to open and close the access to the protected bits. (Default)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODE_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Bit Protection Signal Status Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTS_A {
    #[doc = "0: Software is able to write to all protected bits."]
    VALUE1,
    #[doc = "1: Software is unable to write to any of the protected bits."]
    VALUE2,
}
impl From<PROTS_A> for bool {
    #[inline(always)]
    fn from(variant: PROTS_A) -> Self {
        match variant {
            PROTS_A::VALUE1 => false,
            PROTS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PROTS`"]
pub type PROTS_R = crate::R<bool, PROTS_A>;
impl PROTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTS_A {
        match self.bits {
            false => PROTS_A::VALUE1,
            true => PROTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PROTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PROTS_A::VALUE2
    }
}
#[doc = "Password Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PASS_AW {
    #[doc = "24: Enables writing of the bit field MODE."]
    VALUE1,
    #[doc = "19: Opens access to writing of all protected bits."]
    VALUE2,
    #[doc = "21: Closes access to writing of all protected bits."]
    VALUE3,
}
impl From<PASS_AW> for u8 {
    #[inline(always)]
    fn from(variant: PASS_AW) -> Self {
        match variant {
            PASS_AW::VALUE1 => 24,
            PASS_AW::VALUE2 => 19,
            PASS_AW::VALUE3 => 21,
        }
    }
}
#[doc = "Write proxy for field `PASS`"]
pub struct PASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PASS_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enables writing of the bit field MODE."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PASS_AW::VALUE1)
    }
    #[doc = "Opens access to writing of all protected bits."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PASS_AW::VALUE2)
    }
    #[doc = "Closes access to writing of all protected bits."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PASS_AW::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Bit Protection Scheme Control Bits"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Bit Protection Signal Status Bit"]
    #[inline(always)]
    pub fn prots(&self) -> PROTS_R {
        PROTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Bit Protection Scheme Control Bits"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 3:7 - Password Bits"]
    #[inline(always)]
    pub fn pass(&mut self) -> PASS_W {
        PASS_W { w: self }
    }
}
