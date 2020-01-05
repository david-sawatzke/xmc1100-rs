#[doc = "Reader of register SRMSK"]
pub type R = crate::R<u32, super::SRMSK>;
#[doc = "Writer for register SRMSK"]
pub type W = crate::W<u32, super::SRMSK>;
#[doc = "Register SRMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::SRMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRWARN`"]
pub type PRWARN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRWARN`"]
pub struct PRWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRWARN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `VDDPI`"]
pub type VDDPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDPI`"]
pub struct VDDPI_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDPI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `VDROPI`"]
pub type VDROPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDROPI`"]
pub struct VDROPI_W<'a> {
    w: &'a mut W,
}
impl<'a> VDROPI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `LOCI`"]
pub type LOCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCI`"]
pub struct LOCI_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCI_W<'a> {
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
#[doc = "Reader of field `PESRAMI`"]
pub type PESRAMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PESRAMI`"]
pub struct PESRAMI_W<'a> {
    w: &'a mut W,
}
impl<'a> PESRAMI_W<'a> {
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
#[doc = "Reader of field `PEU0I`"]
pub type PEU0I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEU0I`"]
pub struct PEU0I_W<'a> {
    w: &'a mut W,
}
impl<'a> PEU0I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `FLECC2I`"]
pub type FLECC2I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLECC2I`"]
pub struct FLECC2I_W<'a> {
    w: &'a mut W,
}
impl<'a> FLECC2I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `VCLIPI`"]
pub type VCLIPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCLIPI`"]
pub struct VCLIPI_W<'a> {
    w: &'a mut W,
}
impl<'a> VCLIPI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SBYCLKFI`"]
pub type SBYCLKFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBYCLKFI`"]
pub struct SBYCLKFI_W<'a> {
    w: &'a mut W,
}
impl<'a> SBYCLKFI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_CTR`"]
pub type RTC_CTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CTR`"]
pub struct RTC_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTC_ATIM0`"]
pub type RTC_ATIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_ATIM0`"]
pub struct RTC_ATIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ATIM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_ATIM1`"]
pub type RTC_ATIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_ATIM1`"]
pub struct RTC_ATIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ATIM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RTC_TIM0`"]
pub type RTC_TIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_TIM0`"]
pub struct RTC_TIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_TIM1`"]
pub type RTC_TIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_TIM1`"]
pub struct RTC_TIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TSE_DONE`"]
pub type TSE_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE_DONE`"]
pub struct TSE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TSE_HIGH`"]
pub type TSE_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE_HIGH`"]
pub struct TSE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TSE_LOW`"]
pub type TSE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE_LOW`"]
pub struct TSE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - VDDP pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn vddpi(&self) -> VDDPI_R {
        VDDPI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VDROP Interrupt Mask"]
    #[inline(always)]
    pub fn vdropi(&self) -> VDROPI_R {
        VDROPI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Loss of Clock Interrupt Mask"]
    #[inline(always)]
    pub fn loci(&self) -> LOCI_R {
        LOCI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 16kbytes SRAM Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pesrami(&self) -> PESRAMI_R {
        PESRAMI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USIC0 SRAM Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn peu0i(&self) -> PEU0I_R {
        PEU0I_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flash Double Bit ECC Interrupt Mask"]
    #[inline(always)]
    pub fn flecc2i(&self) -> FLECC2I_R {
        FLECC2I_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VCLIP Interrupt Mask"]
    #[inline(always)]
    pub fn vclipi(&self) -> VCLIPI_R {
        VCLIPI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Standby Clock Failure Interrupt Mask"]
    #[inline(always)]
    pub fn sbyclkfi(&self) -> SBYCLKFI_R {
        SBYCLKFI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TSE Measurement Done Interrupt Mask"]
    #[inline(always)]
    pub fn tse_done(&self) -> TSE_DONE_R {
        TSE_DONE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TSE Compare High Temperature Interrupt Mask"]
    #[inline(always)]
    pub fn tse_high(&self) -> TSE_HIGH_R {
        TSE_HIGH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TSE Compare Low Temperature Interrupt Mask"]
    #[inline(always)]
    pub fn tse_low(&self) -> TSE_LOW_R {
        TSE_LOW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&mut self) -> PRWARN_W {
        PRWARN_W { w: self }
    }
    #[doc = "Bit 3 - VDDP pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn vddpi(&mut self) -> VDDPI_W {
        VDDPI_W { w: self }
    }
    #[doc = "Bit 7 - VDROP Interrupt Mask"]
    #[inline(always)]
    pub fn vdropi(&mut self) -> VDROPI_W {
        VDROPI_W { w: self }
    }
    #[doc = "Bit 16 - Loss of Clock Interrupt Mask"]
    #[inline(always)]
    pub fn loci(&mut self) -> LOCI_W {
        LOCI_W { w: self }
    }
    #[doc = "Bit 17 - 16kbytes SRAM Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pesrami(&mut self) -> PESRAMI_W {
        PESRAMI_W { w: self }
    }
    #[doc = "Bit 18 - USIC0 SRAM Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn peu0i(&mut self) -> PEU0I_W {
        PEU0I_W { w: self }
    }
    #[doc = "Bit 19 - Flash Double Bit ECC Interrupt Mask"]
    #[inline(always)]
    pub fn flecc2i(&mut self) -> FLECC2I_W {
        FLECC2I_W { w: self }
    }
    #[doc = "Bit 21 - VCLIP Interrupt Mask"]
    #[inline(always)]
    pub fn vclipi(&mut self) -> VCLIPI_W {
        VCLIPI_W { w: self }
    }
    #[doc = "Bit 22 - Standby Clock Failure Interrupt Mask"]
    #[inline(always)]
    pub fn sbyclkfi(&mut self) -> SBYCLKFI_W {
        SBYCLKFI_W { w: self }
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W {
        RTC_CTR_W { w: self }
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W {
        RTC_ATIM0_W { w: self }
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W {
        RTC_ATIM1_W { w: self }
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W {
        RTC_TIM0_W { w: self }
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W {
        RTC_TIM1_W { w: self }
    }
    #[doc = "Bit 29 - TSE Measurement Done Interrupt Mask"]
    #[inline(always)]
    pub fn tse_done(&mut self) -> TSE_DONE_W {
        TSE_DONE_W { w: self }
    }
    #[doc = "Bit 30 - TSE Compare High Temperature Interrupt Mask"]
    #[inline(always)]
    pub fn tse_high(&mut self) -> TSE_HIGH_W {
        TSE_HIGH_W { w: self }
    }
    #[doc = "Bit 31 - TSE Compare Low Temperature Interrupt Mask"]
    #[inline(always)]
    pub fn tse_low(&mut self) -> TSE_LOW_W {
        TSE_LOW_W { w: self }
    }
}
