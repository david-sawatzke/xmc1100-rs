#[doc = "Reader of register OUT"]
pub type R = crate::R<u32, super::OUT>;
#[doc = "Writer for register OUT"]
pub type W = crate::W<u32, super::OUT>;
#[doc = "Register OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 2 Output Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P0_A> for bool {
    #[inline(always)]
    fn from(variant: P0_A) -> Self {
        match variant {
            P0_A::VALUE1 => false,
            P0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P0`"]
pub type P0_R = crate::R<bool, P0_A>;
impl P0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_A {
        match self.bits {
            false => P0_A::VALUE1,
            true => P0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P0_A::VALUE2
    }
}
#[doc = "Write proxy for field `P0`"]
pub struct P0_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P0_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P0_A::VALUE2)
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
#[doc = "Port 2 Output Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P1_A> for bool {
    #[inline(always)]
    fn from(variant: P1_A) -> Self {
        match variant {
            P1_A::VALUE1 => false,
            P1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P1`"]
pub type P1_R = crate::R<bool, P1_A>;
impl P1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_A {
        match self.bits {
            false => P1_A::VALUE1,
            true => P1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P1_A::VALUE2
    }
}
#[doc = "Write proxy for field `P1`"]
pub struct P1_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P1_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P1_A::VALUE2)
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
#[doc = "Port 2 Output Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P2_A> for bool {
    #[inline(always)]
    fn from(variant: P2_A) -> Self {
        match variant {
            P2_A::VALUE1 => false,
            P2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P2`"]
pub type P2_R = crate::R<bool, P2_A>;
impl P2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_A {
        match self.bits {
            false => P2_A::VALUE1,
            true => P2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P2_A::VALUE2
    }
}
#[doc = "Write proxy for field `P2`"]
pub struct P2_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P2_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P2_A::VALUE2)
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
#[doc = "Port 2 Output Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P3_A> for bool {
    #[inline(always)]
    fn from(variant: P3_A) -> Self {
        match variant {
            P3_A::VALUE1 => false,
            P3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P3`"]
pub type P3_R = crate::R<bool, P3_A>;
impl P3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_A {
        match self.bits {
            false => P3_A::VALUE1,
            true => P3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P3_A::VALUE2
    }
}
#[doc = "Write proxy for field `P3`"]
pub struct P3_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P3_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P3_A::VALUE2)
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
#[doc = "Port 2 Output Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P4_A> for bool {
    #[inline(always)]
    fn from(variant: P4_A) -> Self {
        match variant {
            P4_A::VALUE1 => false,
            P4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P4`"]
pub type P4_R = crate::R<bool, P4_A>;
impl P4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_A {
        match self.bits {
            false => P4_A::VALUE1,
            true => P4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P4_A::VALUE2
    }
}
#[doc = "Write proxy for field `P4`"]
pub struct P4_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P4_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P4_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Port 2 Output Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P5_A> for bool {
    #[inline(always)]
    fn from(variant: P5_A) -> Self {
        match variant {
            P5_A::VALUE1 => false,
            P5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P5`"]
pub type P5_R = crate::R<bool, P5_A>;
impl P5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P5_A {
        match self.bits {
            false => P5_A::VALUE1,
            true => P5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P5_A::VALUE2
    }
}
#[doc = "Write proxy for field `P5`"]
pub struct P5_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P5_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P5_A::VALUE2)
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
#[doc = "Port 2 Output Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P6_A> for bool {
    #[inline(always)]
    fn from(variant: P6_A) -> Self {
        match variant {
            P6_A::VALUE1 => false,
            P6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P6`"]
pub type P6_R = crate::R<bool, P6_A>;
impl P6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P6_A {
        match self.bits {
            false => P6_A::VALUE1,
            true => P6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P6_A::VALUE2
    }
}
#[doc = "Write proxy for field `P6`"]
pub struct P6_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P6_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P6_A::VALUE2)
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
#[doc = "Port 2 Output Bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P7_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P7_A> for bool {
    #[inline(always)]
    fn from(variant: P7_A) -> Self {
        match variant {
            P7_A::VALUE1 => false,
            P7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P7`"]
pub type P7_R = crate::R<bool, P7_A>;
impl P7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P7_A {
        match self.bits {
            false => P7_A::VALUE1,
            true => P7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P7_A::VALUE2
    }
}
#[doc = "Write proxy for field `P7`"]
pub struct P7_W<'a> {
    w: &'a mut W,
}
impl<'a> P7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P7_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P7_A::VALUE2)
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
#[doc = "Port 2 Output Bit 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P8_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P8_A> for bool {
    #[inline(always)]
    fn from(variant: P8_A) -> Self {
        match variant {
            P8_A::VALUE1 => false,
            P8_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P8`"]
pub type P8_R = crate::R<bool, P8_A>;
impl P8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P8_A {
        match self.bits {
            false => P8_A::VALUE1,
            true => P8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P8_A::VALUE2
    }
}
#[doc = "Write proxy for field `P8`"]
pub struct P8_W<'a> {
    w: &'a mut W,
}
impl<'a> P8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P8_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P8_A::VALUE2)
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
#[doc = "Port 2 Output Bit 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P9_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P9_A> for bool {
    #[inline(always)]
    fn from(variant: P9_A) -> Self {
        match variant {
            P9_A::VALUE1 => false,
            P9_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P9`"]
pub type P9_R = crate::R<bool, P9_A>;
impl P9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P9_A {
        match self.bits {
            false => P9_A::VALUE1,
            true => P9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P9_A::VALUE2
    }
}
#[doc = "Write proxy for field `P9`"]
pub struct P9_W<'a> {
    w: &'a mut W,
}
impl<'a> P9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P9_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P9_A::VALUE2)
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
#[doc = "Port 2 Output Bit 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P10_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P10_A> for bool {
    #[inline(always)]
    fn from(variant: P10_A) -> Self {
        match variant {
            P10_A::VALUE1 => false,
            P10_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P10`"]
pub type P10_R = crate::R<bool, P10_A>;
impl P10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P10_A {
        match self.bits {
            false => P10_A::VALUE1,
            true => P10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P10_A::VALUE2
    }
}
#[doc = "Write proxy for field `P10`"]
pub struct P10_W<'a> {
    w: &'a mut W,
}
impl<'a> P10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P10_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P10_A::VALUE2)
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
#[doc = "Port 2 Output Bit 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P11_A {
    #[doc = "0: The output level of P2.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P2.x is 1."]
    VALUE2,
}
impl From<P11_A> for bool {
    #[inline(always)]
    fn from(variant: P11_A) -> Self {
        match variant {
            P11_A::VALUE1 => false,
            P11_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `P11`"]
pub type P11_R = crate::R<bool, P11_A>;
impl P11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P11_A {
        match self.bits {
            false => P11_A::VALUE1,
            true => P11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P11_A::VALUE2
    }
}
#[doc = "Write proxy for field `P11`"]
pub struct P11_W<'a> {
    w: &'a mut W,
}
impl<'a> P11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The output level of P2.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P11_A::VALUE1)
    }
    #[doc = "The output level of P2.x is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(P11_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Port 2 Output Bit 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 2 Output Bit 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 2 Output Bit 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 2 Output Bit 3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 2 Output Bit 4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 2 Output Bit 5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 2 Output Bit 6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port 2 Output Bit 7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 2 Output Bit 8"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 2 Output Bit 9"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 2 Output Bit 10"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port 2 Output Bit 11"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 2 Output Bit 0"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W {
        P0_W { w: self }
    }
    #[doc = "Bit 1 - Port 2 Output Bit 1"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W {
        P1_W { w: self }
    }
    #[doc = "Bit 2 - Port 2 Output Bit 2"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W {
        P2_W { w: self }
    }
    #[doc = "Bit 3 - Port 2 Output Bit 3"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W {
        P3_W { w: self }
    }
    #[doc = "Bit 4 - Port 2 Output Bit 4"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W {
        P4_W { w: self }
    }
    #[doc = "Bit 5 - Port 2 Output Bit 5"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W {
        P5_W { w: self }
    }
    #[doc = "Bit 6 - Port 2 Output Bit 6"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W {
        P6_W { w: self }
    }
    #[doc = "Bit 7 - Port 2 Output Bit 7"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7_W {
        P7_W { w: self }
    }
    #[doc = "Bit 8 - Port 2 Output Bit 8"]
    #[inline(always)]
    pub fn p8(&mut self) -> P8_W {
        P8_W { w: self }
    }
    #[doc = "Bit 9 - Port 2 Output Bit 9"]
    #[inline(always)]
    pub fn p9(&mut self) -> P9_W {
        P9_W { w: self }
    }
    #[doc = "Bit 10 - Port 2 Output Bit 10"]
    #[inline(always)]
    pub fn p10(&mut self) -> P10_W {
        P10_W { w: self }
    }
    #[doc = "Bit 11 - Port 2 Output Bit 11"]
    #[inline(always)]
    pub fn p11(&mut self) -> P11_W {
        P11_W { w: self }
    }
}
