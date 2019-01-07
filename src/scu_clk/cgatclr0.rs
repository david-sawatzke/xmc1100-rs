#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CGATCLR0 {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `VADC`"]
pub enum VADCW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "disable gating"]
    VALUE2,
}
impl VADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VADCW::VALUE1 => false,
            VADCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VADCW<'a> {
    w: &'a mut W,
}
impl<'a> _VADCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VADCW::VALUE1)
    }
    #[doc = "disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VADCW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCU40`"]
pub enum CCU40W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "disable gating"]
    VALUE2,
}
impl CCU40W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCU40W::VALUE1 => false,
            CCU40W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCU40W<'a> {
    w: &'a mut W,
}
impl<'a> _CCU40W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCU40W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCU40W::VALUE1)
    }
    #[doc = "disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCU40W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USIC0`"]
pub enum USIC0W {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "disable gating"]
    VALUE2,
}
impl USIC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USIC0W::VALUE1 => false,
            USIC0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USIC0W<'a> {
    w: &'a mut W,
}
impl<'a> _USIC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USIC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0W::VALUE1)
    }
    #[doc = "disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDT`"]
pub enum WDTW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "disable gating"]
    VALUE2,
}
impl WDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTW::VALUE1 => false,
            WDTW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTW::VALUE1)
    }
    #[doc = "disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC`"]
pub enum RTCW {
    #[doc = "no effect"]
    VALUE1,
    #[doc = "disable gating"]
    VALUE2,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCW::VALUE1 => false,
            RTCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTCW::VALUE1)
    }
    #[doc = "disable gating"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTCW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - VADC and SHS Gating Clear"]
    #[inline]
    pub fn vadc(&mut self) -> _VADCW {
        _VADCW { w: self }
    }
    #[doc = "Bit 2 - CCU40 Gating Clear"]
    #[inline]
    pub fn ccu40(&mut self) -> _CCU40W {
        _CCU40W { w: self }
    }
    #[doc = "Bit 3 - USIC0 Gating Clear"]
    #[inline]
    pub fn usic0(&mut self) -> _USIC0W {
        _USIC0W { w: self }
    }
    #[doc = "Bit 9 - WDT Gating Clear"]
    #[inline]
    pub fn wdt(&mut self) -> _WDTW {
        _WDTW { w: self }
    }
    #[doc = "Bit 10 - RTC Gating Clear"]
    #[inline]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
}
