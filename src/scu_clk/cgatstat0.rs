#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CGATSTAT0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VADCR {
    #[doc = "gating de-asserted"]
    VALUE1,
    #[doc = "gating asserted"]
    VALUE2,
}
impl VADCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VADCR::VALUE1 => false,
            VADCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VADCR {
        match value {
            false => VADCR::VALUE1,
            true => VADCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VADCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VADCR::VALUE2
    }
}
#[doc = "Possible values of the field `CCU40`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCU40R {
    #[doc = "gating de-asserted"]
    VALUE1,
    #[doc = "gating asserted"]
    VALUE2,
}
impl CCU40R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CCU40R::VALUE1 => false,
            CCU40R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCU40R {
        match value {
            false => CCU40R::VALUE1,
            true => CCU40R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CCU40R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CCU40R::VALUE2
    }
}
#[doc = "Possible values of the field `USIC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0R {
    #[doc = "gating de-asserted"]
    VALUE1,
    #[doc = "gating asserted"]
    VALUE2,
}
impl USIC0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USIC0R::VALUE1 => false,
            USIC0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USIC0R {
        match value {
            false => USIC0R::VALUE1,
            true => USIC0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USIC0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USIC0R::VALUE2
    }
}
#[doc = "Possible values of the field `WDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTR {
    #[doc = "gating de-asserted"]
    VALUE1,
    #[doc = "gating asserted"]
    VALUE2,
}
impl WDTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WDTR::VALUE1 => false,
            WDTR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTR {
        match value {
            false => WDTR::VALUE1,
            true => WDTR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WDTR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WDTR::VALUE2
    }
}
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "gating de-asserted"]
    VALUE1,
    #[doc = "gating asserted"]
    VALUE2,
}
impl RTCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RTCR::VALUE1 => false,
            RTCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCR {
        match value {
            false => RTCR::VALUE1,
            true => RTCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RTCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RTCR::VALUE2
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VADC and SHS Gating Status"]
    #[inline]
    pub fn vadc(&self) -> VADCR {
        VADCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline]
    pub fn ccu40(&self) -> CCU40R {
        CCU40R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - USIC0 Gating Status"]
    #[inline]
    pub fn usic0(&self) -> USIC0R {
        USIC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - WDT Gating Status"]
    #[inline]
    pub fn wdt(&self) -> WDTR {
        WDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - RTC Gating Status"]
    #[inline]
    pub fn rtc(&self) -> RTCR {
        RTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
