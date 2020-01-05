#[doc = "Reader of register ANATSECTRL"]
pub type R = crate::R<u16, super::ANATSECTRL>;
#[doc = "Writer for register ANATSECTRL"]
pub type W = crate::W<u16, super::ANATSECTRL>;
#[doc = "Register ANATSECTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ANATSECTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSE_EN_A {
    #[doc = "0: Temperature sensor is disabled"]
    VALUE1,
    #[doc = "1: Temperature sensor is switched on"]
    VALUE2,
}
impl From<TSE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_EN_A) -> Self {
        match variant {
            TSE_EN_A::VALUE1 => false,
            TSE_EN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TSE_EN`"]
pub type TSE_EN_R = crate::R<bool, TSE_EN_A>;
impl TSE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSE_EN_A {
        match self.bits {
            false => TSE_EN_A::VALUE1,
            true => TSE_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSE_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSE_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `TSE_EN`"]
pub struct TSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Temperature sensor is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSE_EN_A::VALUE1)
    }
    #[doc = "Temperature sensor is switched on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSE_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tse_en(&self) -> TSE_EN_R {
        TSE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tse_en(&mut self) -> TSE_EN_W {
        TSE_EN_W { w: self }
    }
}
