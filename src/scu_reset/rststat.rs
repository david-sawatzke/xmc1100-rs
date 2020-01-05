#[doc = "Reader of register RSTSTAT"]
pub type R = crate::R<u32, super::RSTSTAT>;
#[doc = "Reader of field `RSTSTAT`"]
pub type RSTSTAT_R = crate::R<u16, u16>;
#[doc = "Enable Lockup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKEN_A {
    #[doc = "0: Reset by Lockup disabled"]
    VALUE1,
    #[doc = "1: Reset by Lockup enabled"]
    VALUE2,
}
impl From<LCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCKEN_A) -> Self {
        match variant {
            LCKEN_A::VALUE1 => false,
            LCKEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LCKEN`"]
pub type LCKEN_R = crate::R<bool, LCKEN_A>;
impl LCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCKEN_A {
        match self.bits {
            false => LCKEN_A::VALUE1,
            true => LCKEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LCKEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LCKEN_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:9 - Reset Status Information"]
    #[inline(always)]
    pub fn rststat(&self) -> RSTSTAT_R {
        RSTSTAT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Enable Lockup Status"]
    #[inline(always)]
    pub fn lcken(&self) -> LCKEN_R {
        LCKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
