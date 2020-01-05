#[doc = "Reader of register CHK"]
pub type R = crate::R<u16, super::CHK>;
#[doc = "Random Data / Key Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDV_A {
    #[doc = "0: New random data block is not yet ready to be read. In ) this flag is set to #0 while loading is in progress."]
    VALUE1,
    #[doc = "1: Random data block is valid. In key loading mode this value indicates that the next partial key word can be written to PRNG_WORD."]
    VALUE2,
}
impl From<RDV_A> for bool {
    #[inline(always)]
    fn from(variant: RDV_A) -> Self {
        match variant {
            RDV_A::VALUE1 => false,
            RDV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RDV`"]
pub type RDV_R = crate::R<bool, RDV_A>;
impl RDV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDV_A {
        match self.bits {
            false => RDV_A::VALUE1,
            true => RDV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDV_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Random Data / Key Valid Flag"]
    #[inline(always)]
    pub fn rdv(&self) -> RDV_R {
        RDV_R::new((self.bits & 0x01) != 0)
    }
}
