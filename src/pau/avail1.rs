#[doc = "Reader of register AVAIL1"]
pub type R = crate::R<u32, super::AVAIL1>;
#[doc = "USIC0 Channel 0 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL0_A {
    #[doc = "0: USIC0 Channel 0 is not available."]
    VALUE1,
    #[doc = "1: USIC0 Channel 0 is available."]
    VALUE2,
}
impl From<AVAIL0_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL0_A) -> Self {
        match variant {
            AVAIL0_A::VALUE1 => false,
            AVAIL0_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL0`"]
pub type AVAIL0_R = crate::R<bool, AVAIL0_A>;
impl AVAIL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL0_A {
        match self.bits {
            false => AVAIL0_A::VALUE1,
            true => AVAIL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL0_A::VALUE2
    }
}
#[doc = "USIC0 Channel 1 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL1_A {
    #[doc = "0: USIC0 Channel 1 is not available."]
    VALUE1,
    #[doc = "1: USIC0 Channel 1 is available."]
    VALUE2,
}
impl From<AVAIL1_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL1_A) -> Self {
        match variant {
            AVAIL1_A::VALUE1 => false,
            AVAIL1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL1`"]
pub type AVAIL1_R = crate::R<bool, AVAIL1_A>;
impl AVAIL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL1_A {
        match self.bits {
            false => AVAIL1_A::VALUE1,
            true => AVAIL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL1_A::VALUE2
    }
}
#[doc = "PRNG Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL4_A {
    #[doc = "0: PRNG is not available."]
    VALUE1,
    #[doc = "1: PRNG is available."]
    VALUE2,
}
impl From<AVAIL4_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL4_A) -> Self {
        match variant {
            AVAIL4_A::VALUE1 => false,
            AVAIL4_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL4`"]
pub type AVAIL4_R = crate::R<bool, AVAIL4_A>;
impl AVAIL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL4_A {
        match self.bits {
            false => AVAIL4_A::VALUE1,
            true => AVAIL4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL4_A::VALUE2
    }
}
#[doc = "VADC0 Basic SFRs Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL5_A {
    #[doc = "0: VADC0 Basic SFRs are not available."]
    VALUE1,
    #[doc = "1: VADC0 Basic SFRs are available."]
    VALUE2,
}
impl From<AVAIL5_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL5_A) -> Self {
        match variant {
            AVAIL5_A::VALUE1 => false,
            AVAIL5_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL5`"]
pub type AVAIL5_R = crate::R<bool, AVAIL5_A>;
impl AVAIL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL5_A {
        match self.bits {
            false => AVAIL5_A::VALUE1,
            true => AVAIL5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL5_A::VALUE2
    }
}
#[doc = "SHS0 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL8_A {
    #[doc = "0: SHS0 is not available."]
    VALUE1,
    #[doc = "1: SHS0 is available."]
    VALUE2,
}
impl From<AVAIL8_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL8_A) -> Self {
        match variant {
            AVAIL8_A::VALUE1 => false,
            AVAIL8_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL8`"]
pub type AVAIL8_R = crate::R<bool, AVAIL8_A>;
impl AVAIL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL8_A {
        match self.bits {
            false => AVAIL8_A::VALUE1,
            true => AVAIL8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL8_A::VALUE2
    }
}
#[doc = "CC40 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL9_A {
    #[doc = "0: CC40 is not available."]
    VALUE1,
    #[doc = "1: CC40 is available."]
    VALUE2,
}
impl From<AVAIL9_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL9_A) -> Self {
        match variant {
            AVAIL9_A::VALUE1 => false,
            AVAIL9_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL9`"]
pub type AVAIL9_R = crate::R<bool, AVAIL9_A>;
impl AVAIL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL9_A {
        match self.bits {
            false => AVAIL9_A::VALUE1,
            true => AVAIL9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL9_A::VALUE2
    }
}
#[doc = "CC41 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL10_A {
    #[doc = "0: CC41 is not available."]
    VALUE1,
    #[doc = "1: CC41 is available."]
    VALUE2,
}
impl From<AVAIL10_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL10_A) -> Self {
        match variant {
            AVAIL10_A::VALUE1 => false,
            AVAIL10_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL10`"]
pub type AVAIL10_R = crate::R<bool, AVAIL10_A>;
impl AVAIL10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL10_A {
        match self.bits {
            false => AVAIL10_A::VALUE1,
            true => AVAIL10_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL10_A::VALUE2
    }
}
#[doc = "CC42 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL11_A {
    #[doc = "0: CC42 is not available."]
    VALUE1,
    #[doc = "1: CC42 is available."]
    VALUE2,
}
impl From<AVAIL11_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL11_A) -> Self {
        match variant {
            AVAIL11_A::VALUE1 => false,
            AVAIL11_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL11`"]
pub type AVAIL11_R = crate::R<bool, AVAIL11_A>;
impl AVAIL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL11_A {
        match self.bits {
            false => AVAIL11_A::VALUE1,
            true => AVAIL11_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL11_A::VALUE2
    }
}
#[doc = "CC43 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL12_A {
    #[doc = "0: CC43 is not available."]
    VALUE1,
    #[doc = "1: CC43 is available."]
    VALUE2,
}
impl From<AVAIL12_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL12_A) -> Self {
        match variant {
            AVAIL12_A::VALUE1 => false,
            AVAIL12_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL12`"]
pub type AVAIL12_R = crate::R<bool, AVAIL12_A>;
impl AVAIL12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL12_A {
        match self.bits {
            false => AVAIL12_A::VALUE1,
            true => AVAIL12_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL12_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL12_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - USIC0 Channel 0 Availability Flag"]
    #[inline(always)]
    pub fn avail0(&self) -> AVAIL0_R {
        AVAIL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USIC0 Channel 1 Availability Flag"]
    #[inline(always)]
    pub fn avail1(&self) -> AVAIL1_R {
        AVAIL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRNG Availability Flag"]
    #[inline(always)]
    pub fn avail4(&self) -> AVAIL4_R {
        AVAIL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VADC0 Basic SFRs Availability Flag"]
    #[inline(always)]
    pub fn avail5(&self) -> AVAIL5_R {
        AVAIL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SHS0 Availability Flag"]
    #[inline(always)]
    pub fn avail8(&self) -> AVAIL8_R {
        AVAIL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CC40 Availability Flag"]
    #[inline(always)]
    pub fn avail9(&self) -> AVAIL9_R {
        AVAIL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CC41 Availability Flag"]
    #[inline(always)]
    pub fn avail10(&self) -> AVAIL10_R {
        AVAIL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CC42 Availability Flag"]
    #[inline(always)]
    pub fn avail11(&self) -> AVAIL11_R {
        AVAIL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CC43 Availability Flag"]
    #[inline(always)]
    pub fn avail12(&self) -> AVAIL12_R {
        AVAIL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
