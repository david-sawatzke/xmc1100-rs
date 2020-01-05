#[doc = "Reader of register VDESR"]
pub type R = crate::R<u32, super::VDESR>;
#[doc = "VCLIP Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCLIP_A {
    #[doc = "0: VCLIP is not active"]
    VALUE1,
    #[doc = "1: VCLIP is active"]
    VALUE2,
}
impl From<VCLIP_A> for bool {
    #[inline(always)]
    fn from(variant: VCLIP_A) -> Self {
        match variant {
            VCLIP_A::VALUE1 => false,
            VCLIP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VCLIP`"]
pub type VCLIP_R = crate::R<bool, VCLIP_A>;
impl VCLIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCLIP_A {
        match self.bits {
            false => VCLIP_A::VALUE1,
            true => VCLIP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VCLIP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VCLIP_A::VALUE2
    }
}
#[doc = "VDDPPW Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDPPW_A {
    #[doc = "0: VDDP is above pre-warning threshold"]
    VALUE1,
    #[doc = "1: VDDP is below pre-warningthreshold"]
    VALUE2,
}
impl From<VDDPPW_A> for bool {
    #[inline(always)]
    fn from(variant: VDDPPW_A) -> Self {
        match variant {
            VDDPPW_A::VALUE1 => false,
            VDDPPW_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VDDPPW`"]
pub type VDDPPW_R = crate::R<bool, VDDPPW_A>;
impl VDDPPW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDPPW_A {
        match self.bits {
            false => VDDPPW_A::VALUE1,
            true => VDDPPW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VDDPPW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VDDPPW_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VCLIP Indication"]
    #[inline(always)]
    pub fn vclip(&self) -> VCLIP_R {
        VCLIP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VDDPPW Indication"]
    #[inline(always)]
    pub fn vddppw(&self) -> VDDPPW_R {
        VDDPPW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
