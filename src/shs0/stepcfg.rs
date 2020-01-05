#[doc = "Reader of register STEPCFG"]
pub type R = crate::R<u32, super::STEPCFG>;
#[doc = "Writer for register STEPCFG"]
pub type W = crate::W<u32, super::STEPCFG>;
#[doc = "Register STEPCFG `reset()`'s with value 0x98"]
impl crate::ResetValue for super::STEPCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x98
    }
}
#[doc = "Reader of field `KSEL0`"]
pub type KSEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL0`"]
pub struct KSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `KSEL1`"]
pub type KSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL1`"]
pub struct KSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `KSEL2`"]
pub type KSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL2`"]
pub struct KSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `KSEL3`"]
pub type KSEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL3`"]
pub struct KSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `KSEL4`"]
pub type KSEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL4`"]
pub struct KSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `KSEL5`"]
pub type KSEL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL5`"]
pub struct KSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `KSEL6`"]
pub type KSEL6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL6`"]
pub struct KSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `KSEL7`"]
pub type KSEL7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KSEL7`"]
pub struct KSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> KSEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Step x Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN0_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SEN0_A) -> Self {
        match variant {
            SEN0_A::VALUE1 => false,
            SEN0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN0`"]
pub type SEN0_R = crate::R<bool, SEN0_A>;
impl SEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN0_A {
        match self.bits {
            false => SEN0_A::VALUE1,
            true => SEN0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN0_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN0`"]
pub struct SEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN0_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN0_A::VALUE2)
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
#[doc = "Step x Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN1_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SEN1_A) -> Self {
        match variant {
            SEN1_A::VALUE1 => false,
            SEN1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN1`"]
pub type SEN1_R = crate::R<bool, SEN1_A>;
impl SEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN1_A {
        match self.bits {
            false => SEN1_A::VALUE1,
            true => SEN1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN1_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN1`"]
pub struct SEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN1_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN1_A::VALUE2)
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
#[doc = "Step x Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN2_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SEN2_A) -> Self {
        match variant {
            SEN2_A::VALUE1 => false,
            SEN2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN2`"]
pub type SEN2_R = crate::R<bool, SEN2_A>;
impl SEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN2_A {
        match self.bits {
            false => SEN2_A::VALUE1,
            true => SEN2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN2_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN2`"]
pub struct SEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN2_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN2_A::VALUE2)
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
#[doc = "Step x Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN3_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SEN3_A) -> Self {
        match variant {
            SEN3_A::VALUE1 => false,
            SEN3_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN3`"]
pub type SEN3_R = crate::R<bool, SEN3_A>;
impl SEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN3_A {
        match self.bits {
            false => SEN3_A::VALUE1,
            true => SEN3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN3_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN3`"]
pub struct SEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN3_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Step x Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN4_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SEN4_A) -> Self {
        match variant {
            SEN4_A::VALUE1 => false,
            SEN4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN4`"]
pub type SEN4_R = crate::R<bool, SEN4_A>;
impl SEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN4_A {
        match self.bits {
            false => SEN4_A::VALUE1,
            true => SEN4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN4_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN4`"]
pub struct SEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN4_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN4_A::VALUE2)
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
#[doc = "Step x Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN5_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SEN5_A) -> Self {
        match variant {
            SEN5_A::VALUE1 => false,
            SEN5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN5`"]
pub type SEN5_R = crate::R<bool, SEN5_A>;
impl SEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN5_A {
        match self.bits {
            false => SEN5_A::VALUE1,
            true => SEN5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN5_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN5`"]
pub struct SEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN5_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN5_A::VALUE2)
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
#[doc = "Step x Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN6_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SEN6_A) -> Self {
        match variant {
            SEN6_A::VALUE1 => false,
            SEN6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN6`"]
pub type SEN6_R = crate::R<bool, SEN6_A>;
impl SEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN6_A {
        match self.bits {
            false => SEN6_A::VALUE1,
            true => SEN6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN6_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN6`"]
pub struct SEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN6_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN6_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Step x Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN7_A {
    #[doc = "0: Off: This step is not part of the stepper sequence"]
    VALUE1,
    #[doc = "1: Active: This step is executed during the sequence"]
    VALUE2,
}
impl From<SEN7_A> for bool {
    #[inline(always)]
    fn from(variant: SEN7_A) -> Self {
        match variant {
            SEN7_A::VALUE1 => false,
            SEN7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SEN7`"]
pub type SEN7_R = crate::R<bool, SEN7_A>;
impl SEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN7_A {
        match self.bits {
            false => SEN7_A::VALUE1,
            true => SEN7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEN7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEN7_A::VALUE2
    }
}
#[doc = "Write proxy for field `SEN7`"]
pub struct SEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off: This step is not part of the stepper sequence"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SEN7_A::VALUE1)
    }
    #[doc = "Active: This step is executed during the sequence"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SEN7_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Kernel Select"]
    #[inline(always)]
    pub fn ksel0(&self) -> KSEL0_R {
        KSEL0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Kernel Select"]
    #[inline(always)]
    pub fn ksel1(&self) -> KSEL1_R {
        KSEL1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Kernel Select"]
    #[inline(always)]
    pub fn ksel2(&self) -> KSEL2_R {
        KSEL2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Kernel Select"]
    #[inline(always)]
    pub fn ksel3(&self) -> KSEL3_R {
        KSEL3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Kernel Select"]
    #[inline(always)]
    pub fn ksel4(&self) -> KSEL4_R {
        KSEL4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Kernel Select"]
    #[inline(always)]
    pub fn ksel5(&self) -> KSEL5_R {
        KSEL5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Kernel Select"]
    #[inline(always)]
    pub fn ksel6(&self) -> KSEL6_R {
        KSEL6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Kernel Select"]
    #[inline(always)]
    pub fn ksel7(&self) -> KSEL7_R {
        KSEL7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Step x Enable"]
    #[inline(always)]
    pub fn sen0(&self) -> SEN0_R {
        SEN0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Step x Enable"]
    #[inline(always)]
    pub fn sen1(&self) -> SEN1_R {
        SEN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Step x Enable"]
    #[inline(always)]
    pub fn sen2(&self) -> SEN2_R {
        SEN2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Step x Enable"]
    #[inline(always)]
    pub fn sen3(&self) -> SEN3_R {
        SEN3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Step x Enable"]
    #[inline(always)]
    pub fn sen4(&self) -> SEN4_R {
        SEN4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Step x Enable"]
    #[inline(always)]
    pub fn sen5(&self) -> SEN5_R {
        SEN5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Step x Enable"]
    #[inline(always)]
    pub fn sen6(&self) -> SEN6_R {
        SEN6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Step x Enable"]
    #[inline(always)]
    pub fn sen7(&self) -> SEN7_R {
        SEN7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Kernel Select"]
    #[inline(always)]
    pub fn ksel0(&mut self) -> KSEL0_W {
        KSEL0_W { w: self }
    }
    #[doc = "Bits 4:6 - Kernel Select"]
    #[inline(always)]
    pub fn ksel1(&mut self) -> KSEL1_W {
        KSEL1_W { w: self }
    }
    #[doc = "Bits 8:10 - Kernel Select"]
    #[inline(always)]
    pub fn ksel2(&mut self) -> KSEL2_W {
        KSEL2_W { w: self }
    }
    #[doc = "Bits 12:14 - Kernel Select"]
    #[inline(always)]
    pub fn ksel3(&mut self) -> KSEL3_W {
        KSEL3_W { w: self }
    }
    #[doc = "Bits 16:18 - Kernel Select"]
    #[inline(always)]
    pub fn ksel4(&mut self) -> KSEL4_W {
        KSEL4_W { w: self }
    }
    #[doc = "Bits 20:22 - Kernel Select"]
    #[inline(always)]
    pub fn ksel5(&mut self) -> KSEL5_W {
        KSEL5_W { w: self }
    }
    #[doc = "Bits 24:26 - Kernel Select"]
    #[inline(always)]
    pub fn ksel6(&mut self) -> KSEL6_W {
        KSEL6_W { w: self }
    }
    #[doc = "Bits 28:30 - Kernel Select"]
    #[inline(always)]
    pub fn ksel7(&mut self) -> KSEL7_W {
        KSEL7_W { w: self }
    }
    #[doc = "Bit 3 - Step x Enable"]
    #[inline(always)]
    pub fn sen0(&mut self) -> SEN0_W {
        SEN0_W { w: self }
    }
    #[doc = "Bit 7 - Step x Enable"]
    #[inline(always)]
    pub fn sen1(&mut self) -> SEN1_W {
        SEN1_W { w: self }
    }
    #[doc = "Bit 11 - Step x Enable"]
    #[inline(always)]
    pub fn sen2(&mut self) -> SEN2_W {
        SEN2_W { w: self }
    }
    #[doc = "Bit 15 - Step x Enable"]
    #[inline(always)]
    pub fn sen3(&mut self) -> SEN3_W {
        SEN3_W { w: self }
    }
    #[doc = "Bit 19 - Step x Enable"]
    #[inline(always)]
    pub fn sen4(&mut self) -> SEN4_W {
        SEN4_W { w: self }
    }
    #[doc = "Bit 23 - Step x Enable"]
    #[inline(always)]
    pub fn sen5(&mut self) -> SEN5_W {
        SEN5_W { w: self }
    }
    #[doc = "Bit 27 - Step x Enable"]
    #[inline(always)]
    pub fn sen6(&mut self) -> SEN6_W {
        SEN6_W { w: self }
    }
    #[doc = "Bit 31 - Step x Enable"]
    #[inline(always)]
    pub fn sen7(&mut self) -> SEN7_W {
        SEN7_W { w: self }
    }
}
