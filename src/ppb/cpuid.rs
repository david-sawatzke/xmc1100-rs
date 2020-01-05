#[doc = "Reader of register CPUID"]
pub type R = crate::R<u32, super::CPUID>;
#[doc = "Revision Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVISION_A {
    #[doc = "0: Patch 0"]
    VALUE1,
}
impl From<REVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: REVISION_A) -> Self {
        match variant {
            REVISION_A::VALUE1 => 0,
        }
    }
}
#[doc = "Reader of field `Revision`"]
pub type REVISION_R = crate::R<u8, REVISION_A>;
impl REVISION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVISION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVISION_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REVISION_A::VALUE1
    }
}
#[doc = "Part Number of the Processor\n\nValue on reset: 3104"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARTNO_A {
    #[doc = "3104: Cortex-M0"]
    VALUE1,
}
impl From<PARTNO_A> for u16 {
    #[inline(always)]
    fn from(variant: PARTNO_A) -> Self {
        match variant {
            PARTNO_A::VALUE1 => 3104,
        }
    }
}
#[doc = "Reader of field `PartNo`"]
pub type PARTNO_R = crate::R<u16, PARTNO_A>;
impl PARTNO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PARTNO_A> {
        use crate::Variant::*;
        match self.bits {
            3104 => Val(PARTNO_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PARTNO_A::VALUE1
    }
}
#[doc = "Architecture\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARCHITECTURE_A {
    #[doc = "12: ARMv6-M"]
    VALUE1,
}
impl From<ARCHITECTURE_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCHITECTURE_A) -> Self {
        match variant {
            ARCHITECTURE_A::VALUE1 => 12,
        }
    }
}
#[doc = "Reader of field `Architecture`"]
pub type ARCHITECTURE_R = crate::R<u8, ARCHITECTURE_A>;
impl ARCHITECTURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ARCHITECTURE_A> {
        use crate::Variant::*;
        match self.bits {
            12 => Val(ARCHITECTURE_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARCHITECTURE_A::VALUE1
    }
}
#[doc = "Variant Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VARIANT_A {
    #[doc = "0: Revision 0"]
    VALUE1,
}
impl From<VARIANT_A> for u8 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        match variant {
            VARIANT_A::VALUE1 => 0,
        }
    }
}
#[doc = "Reader of field `Variant`"]
pub type VARIANT_R = crate::R<u8, VARIANT_A>;
impl VARIANT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VARIANT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VARIANT_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VARIANT_A::VALUE1
    }
}
#[doc = "Implementer Code\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPLEMENTER_A {
    #[doc = "65: ARM"]
    VALUE1,
}
impl From<IMPLEMENTER_A> for u8 {
    #[inline(always)]
    fn from(variant: IMPLEMENTER_A) -> Self {
        match variant {
            IMPLEMENTER_A::VALUE1 => 65,
        }
    }
}
#[doc = "Reader of field `Implementer`"]
pub type IMPLEMENTER_R = crate::R<u8, IMPLEMENTER_A>;
impl IMPLEMENTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IMPLEMENTER_A> {
        use crate::Variant::*;
        match self.bits {
            65 => Val(IMPLEMENTER_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IMPLEMENTER_A::VALUE1
    }
}
impl R {
    #[doc = "Bits 0:3 - Revision Number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Part Number of the Processor"]
    #[inline(always)]
    pub fn part_no(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Architecture"]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Variant Number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer Code"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
