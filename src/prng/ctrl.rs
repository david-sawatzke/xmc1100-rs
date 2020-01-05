#[doc = "Reader of register CTRL"]
pub type R = crate::R<u16, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u16, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key Load Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KLD_A {
    #[doc = "0: Streaming mode (default)"]
    VALUE1,
    #[doc = "1: Key loading mode"]
    VALUE2,
}
impl From<KLD_A> for bool {
    #[inline(always)]
    fn from(variant: KLD_A) -> Self {
        match variant {
            KLD_A::VALUE1 => false,
            KLD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `KLD`"]
pub type KLD_R = crate::R<bool, KLD_A>;
impl KLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KLD_A {
        match self.bits {
            false => KLD_A::VALUE1,
            true => KLD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == KLD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == KLD_A::VALUE2
    }
}
#[doc = "Write proxy for field `KLD`"]
pub struct KLD_W<'a> {
    w: &'a mut W,
}
impl<'a> KLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Streaming mode (default)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(KLD_A::VALUE1)
    }
    #[doc = "Key loading mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(KLD_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Random Data Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDBS_A {
    #[doc = "0: Reset state (no random data block size defined), value of PRNG_WORD is undefined."]
    VALUE1,
    #[doc = "1: 8 bits in PRNG_WORD.RDATA\\[7:0\\]"]
    VALUE2,
    #[doc = "2: 16 bits in PRNG_WORD.RDATA\\[15:0\\]"]
    VALUE3,
}
impl From<RDBS_A> for u8 {
    #[inline(always)]
    fn from(variant: RDBS_A) -> Self {
        match variant {
            RDBS_A::VALUE1 => 0,
            RDBS_A::VALUE2 => 1,
            RDBS_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `RDBS`"]
pub type RDBS_R = crate::R<u8, RDBS_A>;
impl RDBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDBS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDBS_A::VALUE1),
            1 => Val(RDBS_A::VALUE2),
            2 => Val(RDBS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDBS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDBS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RDBS_A::VALUE3
    }
}
#[doc = "Write proxy for field `RDBS`"]
pub struct RDBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDBS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reset state (no random data block size defined), value of PRNG_WORD is undefined."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDBS_A::VALUE1)
    }
    #[doc = "8 bits in PRNG_WORD.RDATA\\[7:0\\]"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDBS_A::VALUE2)
    }
    #[doc = "16 bits in PRNG_WORD.RDATA\\[15:0\\]"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RDBS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u16) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Key Load Operation Mode"]
    #[inline(always)]
    pub fn kld(&self) -> KLD_R {
        KLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Random Data Block Size"]
    #[inline(always)]
    pub fn rdbs(&self) -> RDBS_R {
        RDBS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Key Load Operation Mode"]
    #[inline(always)]
    pub fn kld(&mut self) -> KLD_W {
        KLD_W { w: self }
    }
    #[doc = "Bits 1:2 - Random Data Block Size"]
    #[inline(always)]
    pub fn rdbs(&mut self) -> RDBS_W {
        RDBS_W { w: self }
    }
}
