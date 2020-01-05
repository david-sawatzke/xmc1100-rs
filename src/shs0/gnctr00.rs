#[doc = "Reader of register GNCTR00"]
pub type R = crate::R<u32, super::GNCTR00>;
#[doc = "Writer for register GNCTR00"]
pub type W = crate::W<u32, super::GNCTR00>;
#[doc = "Register GNCTR00 `reset()`'s with value 0"]
impl crate::ResetValue for super::GNCTR00 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Gain Control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN0_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN0_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN0_A) -> Self {
        match variant {
            GAIN0_A::VALUE1 => 0,
            GAIN0_A::VALUE2 => 1,
            GAIN0_A::VALUE3 => 2,
            GAIN0_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN0`"]
pub type GAIN0_R = crate::R<u8, GAIN0_A>;
impl GAIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN0_A::VALUE1),
            1 => Val(GAIN0_A::VALUE2),
            2 => Val(GAIN0_A::VALUE3),
            3 => Val(GAIN0_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN0_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN0`"]
pub struct GAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN0_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN0_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN0_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN0_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Gain Control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN1_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN1_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN1_A) -> Self {
        match variant {
            GAIN1_A::VALUE1 => 0,
            GAIN1_A::VALUE2 => 1,
            GAIN1_A::VALUE3 => 2,
            GAIN1_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN1`"]
pub type GAIN1_R = crate::R<u8, GAIN1_A>;
impl GAIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN1_A::VALUE1),
            1 => Val(GAIN1_A::VALUE2),
            2 => Val(GAIN1_A::VALUE3),
            3 => Val(GAIN1_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN1_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN1`"]
pub struct GAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN1_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN1_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN1_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Gain Control 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN2_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN2_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN2_A) -> Self {
        match variant {
            GAIN2_A::VALUE1 => 0,
            GAIN2_A::VALUE2 => 1,
            GAIN2_A::VALUE3 => 2,
            GAIN2_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN2`"]
pub type GAIN2_R = crate::R<u8, GAIN2_A>;
impl GAIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN2_A::VALUE1),
            1 => Val(GAIN2_A::VALUE2),
            2 => Val(GAIN2_A::VALUE3),
            3 => Val(GAIN2_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN2_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN2`"]
pub struct GAIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN2_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN2_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN2_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN2_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Gain Control 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN3_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN3_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN3_A) -> Self {
        match variant {
            GAIN3_A::VALUE1 => 0,
            GAIN3_A::VALUE2 => 1,
            GAIN3_A::VALUE3 => 2,
            GAIN3_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN3`"]
pub type GAIN3_R = crate::R<u8, GAIN3_A>;
impl GAIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN3_A::VALUE1),
            1 => Val(GAIN3_A::VALUE2),
            2 => Val(GAIN3_A::VALUE3),
            3 => Val(GAIN3_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN3_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN3`"]
pub struct GAIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN3_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN3_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN3_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN3_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Gain Control 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN4_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN4_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN4_A) -> Self {
        match variant {
            GAIN4_A::VALUE1 => 0,
            GAIN4_A::VALUE2 => 1,
            GAIN4_A::VALUE3 => 2,
            GAIN4_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN4`"]
pub type GAIN4_R = crate::R<u8, GAIN4_A>;
impl GAIN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN4_A::VALUE1),
            1 => Val(GAIN4_A::VALUE2),
            2 => Val(GAIN4_A::VALUE3),
            3 => Val(GAIN4_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN4_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN4_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN4_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN4`"]
pub struct GAIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN4_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN4_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN4_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN4_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Gain Control 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN5_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN5_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN5_A) -> Self {
        match variant {
            GAIN5_A::VALUE1 => 0,
            GAIN5_A::VALUE2 => 1,
            GAIN5_A::VALUE3 => 2,
            GAIN5_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN5`"]
pub type GAIN5_R = crate::R<u8, GAIN5_A>;
impl GAIN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN5_A::VALUE1),
            1 => Val(GAIN5_A::VALUE2),
            2 => Val(GAIN5_A::VALUE3),
            3 => Val(GAIN5_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN5_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN5_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN5_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN5`"]
pub struct GAIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN5_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN5_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN5_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN5_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Gain Control 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN6_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN6_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN6_A) -> Self {
        match variant {
            GAIN6_A::VALUE1 => 0,
            GAIN6_A::VALUE2 => 1,
            GAIN6_A::VALUE3 => 2,
            GAIN6_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN6`"]
pub type GAIN6_R = crate::R<u8, GAIN6_A>;
impl GAIN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN6_A::VALUE1),
            1 => Val(GAIN6_A::VALUE2),
            2 => Val(GAIN6_A::VALUE3),
            3 => Val(GAIN6_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN6_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN6_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN6_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN6`"]
pub struct GAIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN6_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN6_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN6_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN6_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Gain Control 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GAIN7_A {
    #[doc = "0: Gain factor = 1"]
    VALUE1,
    #[doc = "1: Gain factor = 3"]
    VALUE2,
    #[doc = "2: Gain factor = 6"]
    VALUE3,
    #[doc = "3: Gain factor = 12"]
    VALUE4,
}
impl From<GAIN7_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN7_A) -> Self {
        match variant {
            GAIN7_A::VALUE1 => 0,
            GAIN7_A::VALUE2 => 1,
            GAIN7_A::VALUE3 => 2,
            GAIN7_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `GAIN7`"]
pub type GAIN7_R = crate::R<u8, GAIN7_A>;
impl GAIN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAIN7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAIN7_A::VALUE1),
            1 => Val(GAIN7_A::VALUE2),
            2 => Val(GAIN7_A::VALUE3),
            3 => Val(GAIN7_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GAIN7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GAIN7_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GAIN7_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GAIN7_A::VALUE4
    }
}
#[doc = "Write proxy for field `GAIN7`"]
pub struct GAIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Gain factor = 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GAIN7_A::VALUE1)
    }
    #[doc = "Gain factor = 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GAIN7_A::VALUE2)
    }
    #[doc = "Gain factor = 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GAIN7_A::VALUE3)
    }
    #[doc = "Gain factor = 12"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GAIN7_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Gain Control 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Gain Control 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Gain Control 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Gain Control 3"]
    #[inline(always)]
    pub fn gain3(&self) -> GAIN3_R {
        GAIN3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Gain Control 4"]
    #[inline(always)]
    pub fn gain4(&self) -> GAIN4_R {
        GAIN4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Gain Control 5"]
    #[inline(always)]
    pub fn gain5(&self) -> GAIN5_R {
        GAIN5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Gain Control 6"]
    #[inline(always)]
    pub fn gain6(&self) -> GAIN6_R {
        GAIN6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Gain Control 7"]
    #[inline(always)]
    pub fn gain7(&self) -> GAIN7_R {
        GAIN7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Gain Control 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> GAIN0_W {
        GAIN0_W { w: self }
    }
    #[doc = "Bits 4:7 - Gain Control 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W {
        GAIN1_W { w: self }
    }
    #[doc = "Bits 8:11 - Gain Control 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W {
        GAIN2_W { w: self }
    }
    #[doc = "Bits 12:15 - Gain Control 3"]
    #[inline(always)]
    pub fn gain3(&mut self) -> GAIN3_W {
        GAIN3_W { w: self }
    }
    #[doc = "Bits 16:19 - Gain Control 4"]
    #[inline(always)]
    pub fn gain4(&mut self) -> GAIN4_W {
        GAIN4_W { w: self }
    }
    #[doc = "Bits 20:23 - Gain Control 5"]
    #[inline(always)]
    pub fn gain5(&mut self) -> GAIN5_W {
        GAIN5_W { w: self }
    }
    #[doc = "Bits 24:27 - Gain Control 6"]
    #[inline(always)]
    pub fn gain6(&mut self) -> GAIN6_W {
        GAIN6_W { w: self }
    }
    #[doc = "Bits 28:31 - Gain Control 7"]
    #[inline(always)]
    pub fn gain7(&mut self) -> GAIN7_W {
        GAIN7_W { w: self }
    }
}
