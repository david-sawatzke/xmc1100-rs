#[doc = "Reader of register CGATSTAT0"]
pub type R = crate::R<u32, super::CGATSTAT0>;
#[doc = "VADC and SHS Gating Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADC_A {
    #[doc = "0: gating de-asserted"]
    VALUE1,
    #[doc = "1: gating asserted"]
    VALUE2,
}
impl From<VADC_A> for bool {
    #[inline(always)]
    fn from(variant: VADC_A) -> Self {
        match variant {
            VADC_A::VALUE1 => false,
            VADC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VADC`"]
pub type VADC_R = crate::R<bool, VADC_A>;
impl VADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADC_A {
        match self.bits {
            false => VADC_A::VALUE1,
            true => VADC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VADC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VADC_A::VALUE2
    }
}
#[doc = "CCU40 Gating Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40_A {
    #[doc = "0: gating de-asserted"]
    VALUE1,
    #[doc = "1: gating asserted"]
    VALUE2,
}
impl From<CCU40_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40_A) -> Self {
        match variant {
            CCU40_A::VALUE1 => false,
            CCU40_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CCU40`"]
pub type CCU40_R = crate::R<bool, CCU40_A>;
impl CCU40_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40_A {
        match self.bits {
            false => CCU40_A::VALUE1,
            true => CCU40_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU40_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU40_A::VALUE2
    }
}
#[doc = "USIC0 Gating Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0_A {
    #[doc = "0: gating de-asserted"]
    VALUE1,
    #[doc = "1: gating asserted"]
    VALUE2,
}
impl From<USIC0_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0_A) -> Self {
        match variant {
            USIC0_A::VALUE1 => false,
            USIC0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `USIC0`"]
pub type USIC0_R = crate::R<bool, USIC0_A>;
impl USIC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0_A {
        match self.bits {
            false => USIC0_A::VALUE1,
            true => USIC0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0_A::VALUE2
    }
}
#[doc = "WDT Gating Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: gating de-asserted"]
    VALUE1,
    #[doc = "1: gating asserted"]
    VALUE2,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        match variant {
            WDT_A::VALUE1 => false,
            WDT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::VALUE1,
            true => WDT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDT_A::VALUE2
    }
}
#[doc = "RTC Gating Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: gating de-asserted"]
    VALUE1,
    #[doc = "1: gating asserted"]
    VALUE2,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        match variant {
            RTC_A::VALUE1 => false,
            RTC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::VALUE1,
            true => RTC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VADC and SHS Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VADC_R {
        VADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> CCU40_R {
        CCU40_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> USIC0_R {
        USIC0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WDT Gating Status"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC Gating Status"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
