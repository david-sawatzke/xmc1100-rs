#[doc = "Reader of register CLKCR"]
pub type R = crate::R<u32, super::CLKCR>;
#[doc = "Writer for register CLKCR"]
pub type W = crate::W<u32, super::CLKCR>;
#[doc = "Register CLKCR `reset()`'s with value 0x3ff0_0400"]
impl crate::ResetValue for super::CLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3ff0_0400
    }
}
#[doc = "Reader of field `FDIV`"]
pub type FDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDIV`"]
pub struct FDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Divider Selection\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDIV_A {
    #[doc = "0: Divider is bypassed."]
    VALUE1,
    #[doc = "1: 1; MCLK = 32 MHz"]
    VALUE2,
    #[doc = "2: 2; MCLK = 16 MHz"]
    VALUE3,
    #[doc = "3: 3; MCLK = 10.67 MHz"]
    VALUE4,
    #[doc = "4: 4; MCLK = 8 MHz"]
    VALUE5,
    #[doc = "254: 254; MCLK = 126 kHz"]
    VALUE6,
    #[doc = "255: 255; MCLK = 125.5 kHz"]
    VALUE7,
}
impl From<IDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: IDIV_A) -> Self {
        match variant {
            IDIV_A::VALUE1 => 0,
            IDIV_A::VALUE2 => 1,
            IDIV_A::VALUE3 => 2,
            IDIV_A::VALUE4 => 3,
            IDIV_A::VALUE5 => 4,
            IDIV_A::VALUE6 => 254,
            IDIV_A::VALUE7 => 255,
        }
    }
}
#[doc = "Reader of field `IDIV`"]
pub type IDIV_R = crate::R<u8, IDIV_A>;
impl IDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDIV_A::VALUE1),
            1 => Val(IDIV_A::VALUE2),
            2 => Val(IDIV_A::VALUE3),
            3 => Val(IDIV_A::VALUE4),
            4 => Val(IDIV_A::VALUE5),
            254 => Val(IDIV_A::VALUE6),
            255 => Val(IDIV_A::VALUE7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDIV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IDIV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == IDIV_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == IDIV_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == IDIV_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == IDIV_A::VALUE7
    }
}
#[doc = "Write proxy for field `IDIV`"]
pub struct IDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divider is bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE1)
    }
    #[doc = "1; MCLK = 32 MHz"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE2)
    }
    #[doc = "2; MCLK = 16 MHz"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE3)
    }
    #[doc = "3; MCLK = 10.67 MHz"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE4)
    }
    #[doc = "4; MCLK = 8 MHz"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE5)
    }
    #[doc = "254; MCLK = 126 kHz"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE6)
    }
    #[doc = "255; MCLK = 125.5 kHz"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(IDIV_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "PCLK Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLKSEL_A {
    #[doc = "0: PCLK = MCLK"]
    VALUE1,
    #[doc = "1: PCLK = 2 x MCLK"]
    VALUE2,
}
impl From<PCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PCLKSEL_A) -> Self {
        match variant {
            PCLKSEL_A::VALUE1 => false,
            PCLKSEL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PCLKSEL`"]
pub type PCLKSEL_R = crate::R<bool, PCLKSEL_A>;
impl PCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLKSEL_A {
        match self.bits {
            false => PCLKSEL_A::VALUE1,
            true => PCLKSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCLKSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCLKSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `PCLKSEL`"]
pub struct PCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCLK = MCLK"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCLKSEL_A::VALUE1)
    }
    #[doc = "PCLK = 2 x MCLK"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCLKSEL_A::VALUE2)
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
#[doc = "Reader of field `RTCCLKSEL`"]
pub type RTCCLKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCCLKSEL`"]
pub struct RTCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Counter Adjustment\n\nValue on reset: 1023"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTADJ_A {
    #[doc = "0: 1 clock cycles of the DCO1, 64MHz clock"]
    VALUE1,
    #[doc = "1: 2 clock cycles of the DCO1, 64MHz clock"]
    VALUE2,
    #[doc = "2: 3 clock cycles of the DCO1, 64MHz clock"]
    VALUE3,
    #[doc = "3: 4 clock cycles of the DCO1, 64MHz clock"]
    VALUE4,
    #[doc = "4: 5 clock cycles of the DCO1, 64MHz clock"]
    VALUE5,
    #[doc = "1022: 1023 clock cycles of the DCO1, 64MHz clock"]
    VALUE6,
    #[doc = "1023: 1024 clock cycles of the DCO1, 64MHz clock"]
    VALUE7,
}
impl From<CNTADJ_A> for u16 {
    #[inline(always)]
    fn from(variant: CNTADJ_A) -> Self {
        match variant {
            CNTADJ_A::VALUE1 => 0,
            CNTADJ_A::VALUE2 => 1,
            CNTADJ_A::VALUE3 => 2,
            CNTADJ_A::VALUE4 => 3,
            CNTADJ_A::VALUE5 => 4,
            CNTADJ_A::VALUE6 => 1022,
            CNTADJ_A::VALUE7 => 1023,
        }
    }
}
#[doc = "Reader of field `CNTADJ`"]
pub type CNTADJ_R = crate::R<u16, CNTADJ_A>;
impl CNTADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CNTADJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CNTADJ_A::VALUE1),
            1 => Val(CNTADJ_A::VALUE2),
            2 => Val(CNTADJ_A::VALUE3),
            3 => Val(CNTADJ_A::VALUE4),
            4 => Val(CNTADJ_A::VALUE5),
            1022 => Val(CNTADJ_A::VALUE6),
            1023 => Val(CNTADJ_A::VALUE7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CNTADJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CNTADJ_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CNTADJ_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CNTADJ_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == CNTADJ_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CNTADJ_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == CNTADJ_A::VALUE7
    }
}
#[doc = "Write proxy for field `CNTADJ`"]
pub struct CNTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE1)
    }
    #[doc = "2 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE2)
    }
    #[doc = "3 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE3)
    }
    #[doc = "4 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE4)
    }
    #[doc = "5 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE5)
    }
    #[doc = "1023 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE6)
    }
    #[doc = "1024 clock cycles of the DCO1, 64MHz clock"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(CNTADJ_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "VDDC too low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDC2LOW_A {
    #[doc = "0: VDDC is not too low and the fractional divider input clock is running at the targeted frequency"]
    VALUE1,
    #[doc = "1: VDDC is too low and the fractional divider input clock is not running at the targeted frequency"]
    VALUE2,
}
impl From<VDDC2LOW_A> for bool {
    #[inline(always)]
    fn from(variant: VDDC2LOW_A) -> Self {
        match variant {
            VDDC2LOW_A::VALUE1 => false,
            VDDC2LOW_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VDDC2LOW`"]
pub type VDDC2LOW_R = crate::R<bool, VDDC2LOW_A>;
impl VDDC2LOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDC2LOW_A {
        match self.bits {
            false => VDDC2LOW_A::VALUE1,
            true => VDDC2LOW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VDDC2LOW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VDDC2LOW_A::VALUE2
    }
}
#[doc = "VDDC too high\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDC2HIGH_A {
    #[doc = "0: VDDC is not too high"]
    VALUE1,
    #[doc = "1: VDDC is too high"]
    VALUE2,
}
impl From<VDDC2HIGH_A> for bool {
    #[inline(always)]
    fn from(variant: VDDC2HIGH_A) -> Self {
        match variant {
            VDDC2HIGH_A::VALUE1 => false,
            VDDC2HIGH_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VDDC2HIGH`"]
pub type VDDC2HIGH_R = crate::R<bool, VDDC2HIGH_A>;
impl VDDC2HIGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDC2HIGH_A {
        match self.bits {
            false => VDDC2HIGH_A::VALUE1,
            true => VDDC2HIGH_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VDDC2HIGH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VDDC2HIGH_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:7 - Fractional Divider Selection"]
    #[inline(always)]
    pub fn fdiv(&self) -> FDIV_R {
        FDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Divider Selection"]
    #[inline(always)]
    pub fn idiv(&self) -> IDIV_R {
        IDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - PCLK Clock Select"]
    #[inline(always)]
    pub fn pclksel(&self) -> PCLKSEL_R {
        PCLKSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - RTC Clock Select"]
    #[inline(always)]
    pub fn rtcclksel(&self) -> RTCCLKSEL_R {
        RTCCLKSEL_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:29 - Counter Adjustment"]
    #[inline(always)]
    pub fn cntadj(&self) -> CNTADJ_R {
        CNTADJ_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - VDDC too low"]
    #[inline(always)]
    pub fn vddc2low(&self) -> VDDC2LOW_R {
        VDDC2LOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - VDDC too high"]
    #[inline(always)]
    pub fn vddc2high(&self) -> VDDC2HIGH_R {
        VDDC2HIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fractional Divider Selection"]
    #[inline(always)]
    pub fn fdiv(&mut self) -> FDIV_W {
        FDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Divider Selection"]
    #[inline(always)]
    pub fn idiv(&mut self) -> IDIV_W {
        IDIV_W { w: self }
    }
    #[doc = "Bit 16 - PCLK Clock Select"]
    #[inline(always)]
    pub fn pclksel(&mut self) -> PCLKSEL_W {
        PCLKSEL_W { w: self }
    }
    #[doc = "Bits 17:19 - RTC Clock Select"]
    #[inline(always)]
    pub fn rtcclksel(&mut self) -> RTCCLKSEL_W {
        RTCCLKSEL_W { w: self }
    }
    #[doc = "Bits 20:29 - Counter Adjustment"]
    #[inline(always)]
    pub fn cntadj(&mut self) -> CNTADJ_W {
        CNTADJ_W { w: self }
    }
}
