#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GLOBICLASS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct STCSR {
    bits: u8,
}
impl STCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMSR {
    #[doc = "12-bit conversion"]
    VALUE1,
    #[doc = "10-bit conversion"]
    VALUE2,
    #[doc = "8-bit conversion"]
    VALUE3,
    #[doc = "10-bit fast compare mode"]
    VALUE6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMSR::VALUE1 => 0,
            CMSR::VALUE2 => 1,
            CMSR::VALUE3 => 2,
            CMSR::VALUE6 => 5,
            CMSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMSR {
        match value {
            0 => CMSR::VALUE1,
            1 => CMSR::VALUE2,
            2 => CMSR::VALUE3,
            5 => CMSR::VALUE6,
            i => CMSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == CMSR::VALUE6
    }
}
#[doc = r" Proxy"]
pub struct _STCSW<'a> {
    w: &'a mut W,
}
impl<'a> _STCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMS`"]
pub enum CMSW {
    #[doc = "12-bit conversion"]
    VALUE1,
    #[doc = "10-bit conversion"]
    VALUE2,
    #[doc = "8-bit conversion"]
    VALUE3,
    #[doc = "10-bit fast compare mode"]
    VALUE6,
}
impl CMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMSW::VALUE1 => 0,
            CMSW::VALUE2 => 1,
            CMSW::VALUE3 => 2,
            CMSW::VALUE6 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMSW<'a> {
    w: &'a mut W,
}
impl<'a> _CMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "12-bit conversion"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMSW::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMSW::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMSW::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(CMSW::VALUE6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline]
    pub fn stcs(&self) -> STCSR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STCSR { bits }
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline]
    pub fn cms(&self) -> CMSR {
        CMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline]
    pub fn stcs(&mut self) -> _STCSW {
        _STCSW { w: self }
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline]
    pub fn cms(&mut self) -> _CMSW {
        _CMSW { w: self }
    }
}
