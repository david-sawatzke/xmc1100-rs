#[doc = "Reader of register CONFIG1"]
pub type R = crate::R<u16, super::CONFIG1>;
#[doc = "Writer for register CONFIG1"]
pub type W = crate::W<u16, super::CONFIG1>;
#[doc = "Register CONFIG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wait States Scheme\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXWS_A {
    #[doc = "0: adaptive wait states."]
    CONST_0,
    #[doc = "1: fixed wait states."]
    CONST_1,
}
impl From<FIXWS_A> for bool {
    #[inline(always)]
    fn from(variant: FIXWS_A) -> Self {
        match variant {
            FIXWS_A::CONST_0 => false,
            FIXWS_A::CONST_1 => true,
        }
    }
}
#[doc = "Reader of field `FIXWS`"]
pub type FIXWS_R = crate::R<bool, FIXWS_A>;
impl FIXWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXWS_A {
        match self.bits {
            false => FIXWS_A::CONST_0,
            true => FIXWS_A::CONST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0`"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FIXWS_A::CONST_0
    }
    #[doc = "Checks if the value of the field is `CONST_1`"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FIXWS_A::CONST_1
    }
}
#[doc = "Write proxy for field `FIXWS`"]
pub struct FIXWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIXWS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "adaptive wait states."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut W {
        self.variant(FIXWS_A::CONST_0)
    }
    #[doc = "fixed wait states."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut W {
        self.variant(FIXWS_A::CONST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Wait States Scheme"]
    #[inline(always)]
    pub fn fixws(&self) -> FIXWS_R {
        FIXWS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Wait States Scheme"]
    #[inline(always)]
    pub fn fixws(&mut self) -> FIXWS_W {
        FIXWS_W { w: self }
    }
}
