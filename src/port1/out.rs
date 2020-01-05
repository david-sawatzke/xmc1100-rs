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
#[doc = "Port 1 Output Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P0_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
#[doc = "Port 1 Output Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P1_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
#[doc = "Port 1 Output Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P2_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
#[doc = "Port 1 Output Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P3_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
#[doc = "Port 1 Output Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P4_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
#[doc = "Port 1 Output Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P5_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P5_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
#[doc = "Port 1 Output Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P6_A {
    #[doc = "0: The output level of P1.x is 0."]
    VALUE1,
    #[doc = "1: The output level of P1.x is 1."]
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
    #[doc = "The output level of P1.x is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(P6_A::VALUE1)
    }
    #[doc = "The output level of P1.x is 1."]
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
impl R {
    #[doc = "Bit 0 - Port 1 Output Bit 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 1 Output Bit 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 1 Output Bit 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 1 Output Bit 3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 1 Output Bit 4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 1 Output Bit 5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 1 Output Bit 6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 1 Output Bit 0"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W {
        P0_W { w: self }
    }
    #[doc = "Bit 1 - Port 1 Output Bit 1"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W {
        P1_W { w: self }
    }
    #[doc = "Bit 2 - Port 1 Output Bit 2"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W {
        P2_W { w: self }
    }
    #[doc = "Bit 3 - Port 1 Output Bit 3"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W {
        P3_W { w: self }
    }
    #[doc = "Bit 4 - Port 1 Output Bit 4"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W {
        P4_W { w: self }
    }
    #[doc = "Bit 5 - Port 1 Output Bit 5"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W {
        P5_W { w: self }
    }
    #[doc = "Bit 6 - Port 1 Output Bit 6"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W {
        P6_W { w: self }
    }
}
