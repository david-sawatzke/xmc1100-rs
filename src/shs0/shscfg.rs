#[doc = "Reader of register SHSCFG"]
pub type R = crate::R<u32, super::SHSCFG>;
#[doc = "Writer for register SHSCFG"]
pub type W = crate::W<u32, super::SHSCFG>;
#[doc = "Register SHSCFG `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::SHSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Divider Factor for the SHS Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVS_A {
    #[doc = "0: fSH = fCONV / 1"]
    VALUE1,
    #[doc = "1: fSH = fCONV / 2"]
    VALUE2,
    #[doc = "15: fSH = fCONV / 16"]
    VALUE3,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        match variant {
            DIVS_A::VALUE1 => 0,
            DIVS_A::VALUE2 => 1,
            DIVS_A::VALUE3 => 15,
        }
    }
}
#[doc = "Reader of field `DIVS`"]
pub type DIVS_R = crate::R<u8, DIVS_A>;
impl DIVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVS_A::VALUE1),
            1 => Val(DIVS_A::VALUE2),
            15 => Val(DIVS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVS_A::VALUE3
    }
}
#[doc = "Write proxy for field `DIVS`"]
pub struct DIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fSH = fCONV / 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVS_A::VALUE1)
    }
    #[doc = "fSH = fCONV / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVS_A::VALUE2)
    }
    #[doc = "fSH = fCONV / 16"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Analog Reference Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREF_A {
    #[doc = "0: External reference, upper supply range"]
    VALUE1,
    #[doc = "2: Internal reference, upper supply range"]
    VALUE3,
    #[doc = "3: Internal reference, lower supply range"]
    VALUE4,
}
impl From<AREF_A> for u8 {
    #[inline(always)]
    fn from(variant: AREF_A) -> Self {
        match variant {
            AREF_A::VALUE1 => 0,
            AREF_A::VALUE3 => 2,
            AREF_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `AREF`"]
pub type AREF_R = crate::R<u8, AREF_A>;
impl AREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AREF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AREF_A::VALUE1),
            2 => Val(AREF_A::VALUE3),
            3 => Val(AREF_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AREF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == AREF_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == AREF_A::VALUE4
    }
}
#[doc = "Write proxy for field `AREF`"]
pub struct AREF_W<'a> {
    w: &'a mut W,
}
impl<'a> AREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AREF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External reference, upper supply range"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AREF_A::VALUE1)
    }
    #[doc = "Internal reference, upper supply range"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(AREF_A::VALUE3)
    }
    #[doc = "Internal reference, lower supply range"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(AREF_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Analog Converter Power Down Force\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANOFF_A {
    #[doc = "0: Converter is on"]
    VALUE1,
    #[doc = "1: Converter is permanently off"]
    VALUE2,
}
impl From<ANOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANOFF_A) -> Self {
        match variant {
            ANOFF_A::VALUE1 => false,
            ANOFF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ANOFF`"]
pub type ANOFF_R = crate::R<bool, ANOFF_A>;
impl ANOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANOFF_A {
        match self.bits {
            false => ANOFF_A::VALUE1,
            true => ANOFF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ANOFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ANOFF_A::VALUE2
    }
}
#[doc = "Write proxy for field `ANOFF`"]
pub struct ANOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Converter is on"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ANOFF_A::VALUE1)
    }
    #[doc = "Converter is permanently off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ANOFF_A::VALUE2)
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
#[doc = "Analog Converter Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANRDY_A {
    #[doc = "0: Converter is in power-down mode"]
    VALUE1,
    #[doc = "1: Converter is operable"]
    VALUE2,
}
impl From<ANRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ANRDY_A) -> Self {
        match variant {
            ANRDY_A::VALUE1 => false,
            ANRDY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ANRDY`"]
pub type ANRDY_R = crate::R<bool, ANRDY_A>;
impl ANRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANRDY_A {
        match self.bits {
            false => ANRDY_A::VALUE1,
            true => ANRDY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ANRDY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ANRDY_A::VALUE2
    }
}
#[doc = "Write Control for SHS Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCWC_AW {
    #[doc = "0: No write access to SHS configuration"]
    VALUE1,
    #[doc = "1: Bitfields ANOFF, AREF, DIVS can be written"]
    VALUE2,
}
impl From<SCWC_AW> for bool {
    #[inline(always)]
    fn from(variant: SCWC_AW) -> Self {
        match variant {
            SCWC_AW::VALUE1 => false,
            SCWC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SCWC`"]
pub struct SCWC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCWC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No write access to SHS configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCWC_AW::VALUE1)
    }
    #[doc = "Bitfields ANOFF, AREF, DIVS can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCWC_AW::VALUE2)
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
#[doc = "Sample Pending on S&H Unit x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP0_A {
    #[doc = "0: No sample pending"]
    VALUE1,
    #[doc = "1: S&H unit x has finished the sample phase"]
    VALUE2,
}
impl From<SP0_A> for bool {
    #[inline(always)]
    fn from(variant: SP0_A) -> Self {
        match variant {
            SP0_A::VALUE1 => false,
            SP0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SP0`"]
pub type SP0_R = crate::R<bool, SP0_A>;
impl SP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP0_A {
        match self.bits {
            false => SP0_A::VALUE1,
            true => SP0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SP0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SP0_A::VALUE2
    }
}
#[doc = "Sample Pending on S&H Unit x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP1_A {
    #[doc = "0: No sample pending"]
    VALUE1,
    #[doc = "1: S&H unit x has finished the sample phase"]
    VALUE2,
}
impl From<SP1_A> for bool {
    #[inline(always)]
    fn from(variant: SP1_A) -> Self {
        match variant {
            SP1_A::VALUE1 => false,
            SP1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SP1`"]
pub type SP1_R = crate::R<bool, SP1_A>;
impl SP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP1_A {
        match self.bits {
            false => SP1_A::VALUE1,
            true => SP1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SP1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SP1_A::VALUE2
    }
}
#[doc = "Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "11: Internal test functions enabled"]
    VALUE1,
}
impl From<TC_A> for u8 {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        match variant {
            TC_A::VALUE1 => 11,
        }
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<u8, TC_A>;
impl TC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TC_A> {
        use crate::Variant::*;
        match self.bits {
            11 => Val(TC_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TC_A::VALUE1
    }
}
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal test functions enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TC_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Current State of Sequencer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Idle"]
    VALUE1,
    #[doc = "1: Offset calibration active"]
    VALUE2,
    #[doc = "2: Gain calibration active"]
    VALUE3,
    #[doc = "3: Startup calibration active"]
    VALUE4,
    #[doc = "8: Stepper process active for S&H unit 0"]
    VALUE5,
    #[doc = "9: Stepper process active for S&H unit 1"]
    VALUE6,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::VALUE1 => 0,
            STATE_A::VALUE2 => 1,
            STATE_A::VALUE3 => 2,
            STATE_A::VALUE4 => 3,
            STATE_A::VALUE5 => 8,
            STATE_A::VALUE6 => 9,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::VALUE1),
            1 => Val(STATE_A::VALUE2),
            2 => Val(STATE_A::VALUE3),
            3 => Val(STATE_A::VALUE4),
            8 => Val(STATE_A::VALUE5),
            9 => Val(STATE_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STATE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STATE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STATE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STATE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == STATE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == STATE_A::VALUE6
    }
}
impl R {
    #[doc = "Bits 0:3 - Divider Factor for the SHS Clock"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Analog Reference Voltage Selection"]
    #[inline(always)]
    pub fn aref(&self) -> AREF_R {
        AREF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Analog Converter Power Down Force"]
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Analog Converter Ready"]
    #[inline(always)]
    pub fn anrdy(&self) -> ANRDY_R {
        ANRDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Sample Pending on S&H Unit x"]
    #[inline(always)]
    pub fn sp0(&self) -> SP0_R {
        SP0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Sample Pending on S&H Unit x"]
    #[inline(always)]
    pub fn sp1(&self) -> SP1_R {
        SP1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Test Control"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Current State of Sequencer"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Divider Factor for the SHS Clock"]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W {
        DIVS_W { w: self }
    }
    #[doc = "Bits 10:11 - Analog Reference Voltage Selection"]
    #[inline(always)]
    pub fn aref(&mut self) -> AREF_W {
        AREF_W { w: self }
    }
    #[doc = "Bit 12 - Analog Converter Power Down Force"]
    #[inline(always)]
    pub fn anoff(&mut self) -> ANOFF_W {
        ANOFF_W { w: self }
    }
    #[doc = "Bit 15 - Write Control for SHS Configuration"]
    #[inline(always)]
    pub fn scwc(&mut self) -> SCWC_W {
        SCWC_W { w: self }
    }
    #[doc = "Bits 24:27 - Test Control"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
}
