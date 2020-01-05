#[doc = "Reader of register AVAIL0"]
pub type R = crate::R<u32, super::AVAIL0>;
#[doc = "RAM Block 1 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL5_A {
    #[doc = "0: RAM block 1 is not available."]
    VALUE1,
    #[doc = "1: RAM block 1 is available."]
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
#[doc = "RAM Block 2 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL6_A {
    #[doc = "0: RAM block 2 is not available."]
    VALUE1,
    #[doc = "1: RAM block 2 is available."]
    VALUE2,
}
impl From<AVAIL6_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL6_A) -> Self {
        match variant {
            AVAIL6_A::VALUE1 => false,
            AVAIL6_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL6`"]
pub type AVAIL6_R = crate::R<bool, AVAIL6_A>;
impl AVAIL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL6_A {
        match self.bits {
            false => AVAIL6_A::VALUE1,
            true => AVAIL6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL6_A::VALUE2
    }
}
#[doc = "RAM Block 3 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL7_A {
    #[doc = "0: RAM block 3 is not available."]
    VALUE1,
    #[doc = "1: RAM block 3 is available."]
    VALUE2,
}
impl From<AVAIL7_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL7_A) -> Self {
        match variant {
            AVAIL7_A::VALUE1 => false,
            AVAIL7_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL7`"]
pub type AVAIL7_R = crate::R<bool, AVAIL7_A>;
impl AVAIL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL7_A {
        match self.bits {
            false => AVAIL7_A::VALUE1,
            true => AVAIL7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL7_A::VALUE2
    }
}
#[doc = "Port 0 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL22_A {
    #[doc = "0: Port 0 is not available."]
    VALUE1,
    #[doc = "1: Port 0 is available."]
    VALUE2,
}
impl From<AVAIL22_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL22_A) -> Self {
        match variant {
            AVAIL22_A::VALUE1 => false,
            AVAIL22_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL22`"]
pub type AVAIL22_R = crate::R<bool, AVAIL22_A>;
impl AVAIL22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL22_A {
        match self.bits {
            false => AVAIL22_A::VALUE1,
            true => AVAIL22_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL22_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL22_A::VALUE2
    }
}
#[doc = "Port 1 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL23_A {
    #[doc = "0: Port 1 is not available."]
    VALUE1,
    #[doc = "1: Port 1 is available."]
    VALUE2,
}
impl From<AVAIL23_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL23_A) -> Self {
        match variant {
            AVAIL23_A::VALUE1 => false,
            AVAIL23_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL23`"]
pub type AVAIL23_R = crate::R<bool, AVAIL23_A>;
impl AVAIL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL23_A {
        match self.bits {
            false => AVAIL23_A::VALUE1,
            true => AVAIL23_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL23_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL23_A::VALUE2
    }
}
#[doc = "Port 0 Availability Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVAIL24_A {
    #[doc = "0: Port 2 is not available."]
    VALUE1,
    #[doc = "1: Port 2 is available."]
    VALUE2,
}
impl From<AVAIL24_A> for bool {
    #[inline(always)]
    fn from(variant: AVAIL24_A) -> Self {
        match variant {
            AVAIL24_A::VALUE1 => false,
            AVAIL24_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AVAIL24`"]
pub type AVAIL24_R = crate::R<bool, AVAIL24_A>;
impl AVAIL24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVAIL24_A {
        match self.bits {
            false => AVAIL24_A::VALUE1,
            true => AVAIL24_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AVAIL24_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AVAIL24_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 5 - RAM Block 1 Availability Flag"]
    #[inline(always)]
    pub fn avail5(&self) -> AVAIL5_R {
        AVAIL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RAM Block 2 Availability Flag"]
    #[inline(always)]
    pub fn avail6(&self) -> AVAIL6_R {
        AVAIL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RAM Block 3 Availability Flag"]
    #[inline(always)]
    pub fn avail7(&self) -> AVAIL7_R {
        AVAIL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port 0 Availability Flag"]
    #[inline(always)]
    pub fn avail22(&self) -> AVAIL22_R {
        AVAIL22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port 1 Availability Flag"]
    #[inline(always)]
    pub fn avail23(&self) -> AVAIL23_R {
        AVAIL23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port 0 Availability Flag"]
    #[inline(always)]
    pub fn avail24(&self) -> AVAIL24_R {
        AVAIL24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
