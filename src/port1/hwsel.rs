#[doc = "Reader of register HWSEL"]
pub type R = crate::R<u32, super::HWSEL>;
#[doc = "Writer for register HWSEL"]
pub type W = crate::W<u32, super::HWSEL>;
#[doc = "Register HWSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HWSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW0_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW0_A> for u8 {
    #[inline(always)]
    fn from(variant: HW0_A) -> Self {
        match variant {
            HW0_A::VALUE1 => 0,
            HW0_A::VALUE2 => 1,
            HW0_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW0`"]
pub type HW0_R = crate::R<u8, HW0_A>;
impl HW0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW0_A::VALUE1),
            1 => Val(HW0_A::VALUE2),
            2 => Val(HW0_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW0_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW0`"]
pub struct HW0_W<'a> {
    w: &'a mut W,
}
impl<'a> HW0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW0_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW0_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW0_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW1_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW1_A> for u8 {
    #[inline(always)]
    fn from(variant: HW1_A) -> Self {
        match variant {
            HW1_A::VALUE1 => 0,
            HW1_A::VALUE2 => 1,
            HW1_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW1`"]
pub type HW1_R = crate::R<u8, HW1_A>;
impl HW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW1_A::VALUE1),
            1 => Val(HW1_A::VALUE2),
            2 => Val(HW1_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW1_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW1`"]
pub struct HW1_W<'a> {
    w: &'a mut W,
}
impl<'a> HW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW1_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW1_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW1_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW2_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW2_A> for u8 {
    #[inline(always)]
    fn from(variant: HW2_A) -> Self {
        match variant {
            HW2_A::VALUE1 => 0,
            HW2_A::VALUE2 => 1,
            HW2_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW2`"]
pub type HW2_R = crate::R<u8, HW2_A>;
impl HW2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW2_A::VALUE1),
            1 => Val(HW2_A::VALUE2),
            2 => Val(HW2_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW2_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW2`"]
pub struct HW2_W<'a> {
    w: &'a mut W,
}
impl<'a> HW2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW2_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW2_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW2_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW3_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW3_A> for u8 {
    #[inline(always)]
    fn from(variant: HW3_A) -> Self {
        match variant {
            HW3_A::VALUE1 => 0,
            HW3_A::VALUE2 => 1,
            HW3_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW3`"]
pub type HW3_R = crate::R<u8, HW3_A>;
impl HW3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW3_A::VALUE1),
            1 => Val(HW3_A::VALUE2),
            2 => Val(HW3_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW3_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW3`"]
pub struct HW3_W<'a> {
    w: &'a mut W,
}
impl<'a> HW3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW3_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW3_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW3_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW4_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW4_A> for u8 {
    #[inline(always)]
    fn from(variant: HW4_A) -> Self {
        match variant {
            HW4_A::VALUE1 => 0,
            HW4_A::VALUE2 => 1,
            HW4_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW4`"]
pub type HW4_R = crate::R<u8, HW4_A>;
impl HW4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW4_A::VALUE1),
            1 => Val(HW4_A::VALUE2),
            2 => Val(HW4_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW4_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW4_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW4`"]
pub struct HW4_W<'a> {
    w: &'a mut W,
}
impl<'a> HW4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW4_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW4_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW4_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW5_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW5_A> for u8 {
    #[inline(always)]
    fn from(variant: HW5_A) -> Self {
        match variant {
            HW5_A::VALUE1 => 0,
            HW5_A::VALUE2 => 1,
            HW5_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW5`"]
pub type HW5_R = crate::R<u8, HW5_A>;
impl HW5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW5_A::VALUE1),
            1 => Val(HW5_A::VALUE2),
            2 => Val(HW5_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW5_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW5_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW5`"]
pub struct HW5_W<'a> {
    w: &'a mut W,
}
impl<'a> HW5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW5_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW5_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW5_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port 1 Pin Hardware Select Bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW6_A {
    #[doc = "0: Software control only."]
    VALUE1,
    #[doc = "1: HW0 control path can override the software configuration."]
    VALUE2,
    #[doc = "2: HW1 control path can override the software configuration."]
    VALUE3,
}
impl From<HW6_A> for u8 {
    #[inline(always)]
    fn from(variant: HW6_A) -> Self {
        match variant {
            HW6_A::VALUE1 => 0,
            HW6_A::VALUE2 => 1,
            HW6_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `HW6`"]
pub type HW6_R = crate::R<u8, HW6_A>;
impl HW6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HW6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HW6_A::VALUE1),
            1 => Val(HW6_A::VALUE2),
            2 => Val(HW6_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HW6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HW6_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HW6_A::VALUE3
    }
}
#[doc = "Write proxy for field `HW6`"]
pub struct HW6_W<'a> {
    w: &'a mut W,
}
impl<'a> HW6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software control only."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HW6_A::VALUE1)
    }
    #[doc = "HW0 control path can override the software configuration."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HW6_A::VALUE2)
    }
    #[doc = "HW1 control path can override the software configuration."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HW6_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 Pin Hardware Select Bit 0"]
    #[inline(always)]
    pub fn hw0(&self) -> HW0_R {
        HW0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 Pin Hardware Select Bit 1"]
    #[inline(always)]
    pub fn hw1(&self) -> HW1_R {
        HW1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 Pin Hardware Select Bit 2"]
    #[inline(always)]
    pub fn hw2(&self) -> HW2_R {
        HW2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 Pin Hardware Select Bit 3"]
    #[inline(always)]
    pub fn hw3(&self) -> HW3_R {
        HW3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 Pin Hardware Select Bit 4"]
    #[inline(always)]
    pub fn hw4(&self) -> HW4_R {
        HW4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 Pin Hardware Select Bit 5"]
    #[inline(always)]
    pub fn hw5(&self) -> HW5_R {
        HW5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 Pin Hardware Select Bit 6"]
    #[inline(always)]
    pub fn hw6(&self) -> HW6_R {
        HW6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 Pin Hardware Select Bit 0"]
    #[inline(always)]
    pub fn hw0(&mut self) -> HW0_W {
        HW0_W { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 Pin Hardware Select Bit 1"]
    #[inline(always)]
    pub fn hw1(&mut self) -> HW1_W {
        HW1_W { w: self }
    }
    #[doc = "Bits 4:5 - Port 1 Pin Hardware Select Bit 2"]
    #[inline(always)]
    pub fn hw2(&mut self) -> HW2_W {
        HW2_W { w: self }
    }
    #[doc = "Bits 6:7 - Port 1 Pin Hardware Select Bit 3"]
    #[inline(always)]
    pub fn hw3(&mut self) -> HW3_W {
        HW3_W { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 Pin Hardware Select Bit 4"]
    #[inline(always)]
    pub fn hw4(&mut self) -> HW4_W {
        HW4_W { w: self }
    }
    #[doc = "Bits 10:11 - Port 1 Pin Hardware Select Bit 5"]
    #[inline(always)]
    pub fn hw5(&mut self) -> HW5_W {
        HW5_W { w: self }
    }
    #[doc = "Bits 12:13 - Port 1 Pin Hardware Select Bit 6"]
    #[inline(always)]
    pub fn hw6(&mut self) -> HW6_W {
        HW6_W { w: self }
    }
}
