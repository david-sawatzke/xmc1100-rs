#[doc = "Reader of register PPS"]
pub type R = crate::R<u32, super::PPS>;
#[doc = "Writer for register PPS"]
pub type W = crate::W<u32, super::PPS>;
#[doc = "Register PPS `reset()`'s with value 0"]
impl crate::ResetValue for super::PPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 1 Pin Power Save Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS0_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS0_A> for bool {
    #[inline(always)]
    fn from(variant: PPS0_A) -> Self {
        match variant {
            PPS0_A::VALUE1 => false,
            PPS0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS0`"]
pub type PPS0_R = crate::R<bool, PPS0_A>;
impl PPS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS0_A {
        match self.bits {
            false => PPS0_A::VALUE1,
            true => PPS0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS0`"]
pub struct PPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS0_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS0_A::VALUE2)
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
#[doc = "Port 1 Pin Power Save Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS1_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS1_A> for bool {
    #[inline(always)]
    fn from(variant: PPS1_A) -> Self {
        match variant {
            PPS1_A::VALUE1 => false,
            PPS1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS1`"]
pub type PPS1_R = crate::R<bool, PPS1_A>;
impl PPS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS1_A {
        match self.bits {
            false => PPS1_A::VALUE1,
            true => PPS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS1`"]
pub struct PPS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS1_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS1_A::VALUE2)
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
#[doc = "Port 1 Pin Power Save Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS2_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS2_A> for bool {
    #[inline(always)]
    fn from(variant: PPS2_A) -> Self {
        match variant {
            PPS2_A::VALUE1 => false,
            PPS2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS2`"]
pub type PPS2_R = crate::R<bool, PPS2_A>;
impl PPS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS2_A {
        match self.bits {
            false => PPS2_A::VALUE1,
            true => PPS2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS2_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS2`"]
pub struct PPS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS2_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS2_A::VALUE2)
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
#[doc = "Port 1 Pin Power Save Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS3_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS3_A> for bool {
    #[inline(always)]
    fn from(variant: PPS3_A) -> Self {
        match variant {
            PPS3_A::VALUE1 => false,
            PPS3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS3`"]
pub type PPS3_R = crate::R<bool, PPS3_A>;
impl PPS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS3_A {
        match self.bits {
            false => PPS3_A::VALUE1,
            true => PPS3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS3_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS3`"]
pub struct PPS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS3_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS3_A::VALUE2)
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
#[doc = "Port 1 Pin Power Save Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS4_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS4_A> for bool {
    #[inline(always)]
    fn from(variant: PPS4_A) -> Self {
        match variant {
            PPS4_A::VALUE1 => false,
            PPS4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS4`"]
pub type PPS4_R = crate::R<bool, PPS4_A>;
impl PPS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS4_A {
        match self.bits {
            false => PPS4_A::VALUE1,
            true => PPS4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS4_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS4`"]
pub struct PPS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS4_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS4_A::VALUE2)
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
#[doc = "Port 1 Pin Power Save Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS5_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS5_A> for bool {
    #[inline(always)]
    fn from(variant: PPS5_A) -> Self {
        match variant {
            PPS5_A::VALUE1 => false,
            PPS5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS5`"]
pub type PPS5_R = crate::R<bool, PPS5_A>;
impl PPS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS5_A {
        match self.bits {
            false => PPS5_A::VALUE1,
            true => PPS5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS5_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS5`"]
pub struct PPS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS5_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS5_A::VALUE2)
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
#[doc = "Port 1 Pin Power Save Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPS6_A {
    #[doc = "0: Pin Power Save of P1.x is disabled."]
    VALUE1,
    #[doc = "1: Pin Power Save of P1.x is enabled."]
    VALUE2,
}
impl From<PPS6_A> for bool {
    #[inline(always)]
    fn from(variant: PPS6_A) -> Self {
        match variant {
            PPS6_A::VALUE1 => false,
            PPS6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PPS6`"]
pub type PPS6_R = crate::R<bool, PPS6_A>;
impl PPS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPS6_A {
        match self.bits {
            false => PPS6_A::VALUE1,
            true => PPS6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPS6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPS6_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPS6`"]
pub struct PPS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Power Save of P1.x is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPS6_A::VALUE1)
    }
    #[doc = "Pin Power Save of P1.x is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPS6_A::VALUE2)
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
    #[doc = "Bit 0 - Port 1 Pin Power Save Bit 0"]
    #[inline(always)]
    pub fn pps0(&self) -> PPS0_R {
        PPS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 1 Pin Power Save Bit 1"]
    #[inline(always)]
    pub fn pps1(&self) -> PPS1_R {
        PPS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 1 Pin Power Save Bit 2"]
    #[inline(always)]
    pub fn pps2(&self) -> PPS2_R {
        PPS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 1 Pin Power Save Bit 3"]
    #[inline(always)]
    pub fn pps3(&self) -> PPS3_R {
        PPS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 1 Pin Power Save Bit 4"]
    #[inline(always)]
    pub fn pps4(&self) -> PPS4_R {
        PPS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 1 Pin Power Save Bit 5"]
    #[inline(always)]
    pub fn pps5(&self) -> PPS5_R {
        PPS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 1 Pin Power Save Bit 6"]
    #[inline(always)]
    pub fn pps6(&self) -> PPS6_R {
        PPS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 1 Pin Power Save Bit 0"]
    #[inline(always)]
    pub fn pps0(&mut self) -> PPS0_W {
        PPS0_W { w: self }
    }
    #[doc = "Bit 1 - Port 1 Pin Power Save Bit 1"]
    #[inline(always)]
    pub fn pps1(&mut self) -> PPS1_W {
        PPS1_W { w: self }
    }
    #[doc = "Bit 2 - Port 1 Pin Power Save Bit 2"]
    #[inline(always)]
    pub fn pps2(&mut self) -> PPS2_W {
        PPS2_W { w: self }
    }
    #[doc = "Bit 3 - Port 1 Pin Power Save Bit 3"]
    #[inline(always)]
    pub fn pps3(&mut self) -> PPS3_W {
        PPS3_W { w: self }
    }
    #[doc = "Bit 4 - Port 1 Pin Power Save Bit 4"]
    #[inline(always)]
    pub fn pps4(&mut self) -> PPS4_W {
        PPS4_W { w: self }
    }
    #[doc = "Bit 5 - Port 1 Pin Power Save Bit 5"]
    #[inline(always)]
    pub fn pps5(&mut self) -> PPS5_W {
        PPS5_W { w: self }
    }
    #[doc = "Bit 6 - Port 1 Pin Power Save Bit 6"]
    #[inline(always)]
    pub fn pps6(&mut self) -> PPS6_W {
        PPS6_W { w: self }
    }
}
