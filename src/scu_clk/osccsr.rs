#[doc = "Reader of register OSCCSR"]
pub type R = crate::R<u32, super::OSCCSR>;
#[doc = "Writer for register OSCCSR"]
pub type W = crate::W<u32, super::OSCCSR>;
#[doc = "Register OSCCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Oscillator Valid Low Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC2L_A {
    #[doc = "0: The OSC frequency is usable"]
    VALUE1,
    #[doc = "1: The OSC frequency is not usable. Frequency is too low."]
    VALUE2,
}
impl From<OSC2L_A> for bool {
    #[inline(always)]
    fn from(variant: OSC2L_A) -> Self {
        match variant {
            OSC2L_A::VALUE1 => false,
            OSC2L_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OSC2L`"]
pub type OSC2L_R = crate::R<bool, OSC2L_A>;
impl OSC2L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC2L_A {
        match self.bits {
            false => OSC2L_A::VALUE1,
            true => OSC2L_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSC2L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSC2L_A::VALUE2
    }
}
#[doc = "Oscillator Valid High Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC2H_A {
    #[doc = "0: The OSC frequency is usable"]
    VALUE1,
    #[doc = "1: The OSC frequency is not usable. Frequency is too high."]
    VALUE2,
}
impl From<OSC2H_A> for bool {
    #[inline(always)]
    fn from(variant: OSC2H_A) -> Self {
        match variant {
            OSC2H_A::VALUE1 => false,
            OSC2H_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OSC2H`"]
pub type OSC2H_R = crate::R<bool, OSC2H_A>;
impl OSC2H_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC2H_A {
        match self.bits {
            false => OSC2H_A::VALUE1,
            true => OSC2H_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSC2H_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSC2H_A::VALUE2
    }
}
#[doc = "Oscillator Watchdog Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWDRES_A {
    #[doc = "0: The Oscillator Watchdog is not cleared and remains active"]
    VALUE1,
    #[doc = "1: The Oscillator Watchdog is cleared and restarted. The OSC2L and OSC2H flag will be held in the last value until it is updated after 3 standby clock cycles."]
    VALUE2,
}
impl From<OWDRES_A> for bool {
    #[inline(always)]
    fn from(variant: OWDRES_A) -> Self {
        match variant {
            OWDRES_A::VALUE1 => false,
            OWDRES_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OWDRES`"]
pub type OWDRES_R = crate::R<bool, OWDRES_A>;
impl OWDRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWDRES_A {
        match self.bits {
            false => OWDRES_A::VALUE1,
            true => OWDRES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OWDRES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OWDRES_A::VALUE2
    }
}
#[doc = "Write proxy for field `OWDRES`"]
pub struct OWDRES_W<'a> {
    w: &'a mut W,
}
impl<'a> OWDRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWDRES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Oscillator Watchdog is not cleared and remains active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OWDRES_A::VALUE1)
    }
    #[doc = "The Oscillator Watchdog is cleared and restarted. The OSC2L and OSC2H flag will be held in the last value until it is updated after 3 standby clock cycles."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OWDRES_A::VALUE2)
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
#[doc = "Oscillator Watchdog Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWDEN_A {
    #[doc = "0: The Oscillator Watchdog is disabled"]
    VALUE1,
    #[doc = "1: The Oscillator Watchdog is enabled"]
    VALUE2,
}
impl From<OWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: OWDEN_A) -> Self {
        match variant {
            OWDEN_A::VALUE1 => false,
            OWDEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OWDEN`"]
pub type OWDEN_R = crate::R<bool, OWDEN_A>;
impl OWDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OWDEN_A {
        match self.bits {
            false => OWDEN_A::VALUE1,
            true => OWDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OWDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OWDEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `OWDEN`"]
pub struct OWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OWDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OWDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Oscillator Watchdog is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OWDEN_A::VALUE1)
    }
    #[doc = "The Oscillator Watchdog is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OWDEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator Valid Low Status Bit"]
    #[inline(always)]
    pub fn osc2l(&self) -> OSC2L_R {
        OSC2L_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Oscillator Valid High Status Bit"]
    #[inline(always)]
    pub fn osc2h(&self) -> OSC2H_R {
        OSC2H_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn owdres(&self) -> OWDRES_R {
        OWDRES_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Enable"]
    #[inline(always)]
    pub fn owden(&self) -> OWDEN_R {
        OWDEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn owdres(&mut self) -> OWDRES_W {
        OWDRES_W { w: self }
    }
    #[doc = "Bit 17 - Oscillator Watchdog Enable"]
    #[inline(always)]
    pub fn owden(&mut self) -> OWDEN_W {
        OWDEN_W { w: self }
    }
}
