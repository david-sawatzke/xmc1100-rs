#[doc = "Reader of register SRRAW"]
pub type R = crate::R<u32, super::SRRAW>;
#[doc = "Reader of field `PRWARN`"]
pub type PRWARN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PI`"]
pub type PI_R = crate::R<bool, bool>;
#[doc = "Reader of field `AI`"]
pub type AI_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDPI`"]
pub type VDDPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDROPI`"]
pub type VDROPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCI`"]
pub type LOCI_R = crate::R<bool, bool>;
#[doc = "Reader of field `PESRAMI`"]
pub type PESRAMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEU0I`"]
pub type PEU0I_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLECC2I`"]
pub type FLECC2I_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLCMPLTI`"]
pub type FLCMPLTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCLIPI`"]
pub type VCLIPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBYCLKFI`"]
pub type SBYCLKFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_CTR`"]
pub type RTC_CTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_ATIM0`"]
pub type RTC_ATIM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_ATIM1`"]
pub type RTC_ATIM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_TIM0`"]
pub type RTC_TIM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_TIM1`"]
pub type RTC_TIM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSE_DONE`"]
pub type TSE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSE_HIGH`"]
pub type TSE_HIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSE_LOW`"]
pub type TSE_LOW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - WDT pre-warning Event Status Before Masking"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Raw Periodic Event Status Before Masking"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Raw Alarm Event Status Before Masking"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VDDP pre-warning Event Status Before Masking"]
    #[inline(always)]
    pub fn vddpi(&self) -> VDDPI_R {
        VDDPI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VDROP Event Status Before Masking"]
    #[inline(always)]
    pub fn vdropi(&self) -> VDROPI_R {
        VDROPI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Loss of Clock Event Status Before Masking"]
    #[inline(always)]
    pub fn loci(&self) -> LOCI_R {
        LOCI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 16kbytes SRAM Parity Error Event Status Before Masking"]
    #[inline(always)]
    pub fn pesrami(&self) -> PESRAMI_R {
        PESRAMI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USIC0 SRAM Parity Error Event Status Before Masking"]
    #[inline(always)]
    pub fn peu0i(&self) -> PEU0I_R {
        PEU0I_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flash Double Bit ECC Event Status Before Masking"]
    #[inline(always)]
    pub fn flecc2i(&self) -> FLECC2I_R {
        FLECC2I_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Flash Operation Complete Event Status Before Masking"]
    #[inline(always)]
    pub fn flcmplti(&self) -> FLCMPLTI_R {
        FLCMPLTI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VCLIP Event Status Before Masking"]
    #[inline(always)]
    pub fn vclipi(&self) -> VCLIPI_R {
        VCLIPI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Standby Clock Failure Event Status Before Masking"]
    #[inline(always)]
    pub fn sbyclkfi(&self) -> SBYCLKFI_R {
        SBYCLKFI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Before Masking"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TSE Measurement Done Event Status Before Masking"]
    #[inline(always)]
    pub fn tse_done(&self) -> TSE_DONE_R {
        TSE_DONE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TSE Compare High Temperature Event Status Before Masking"]
    #[inline(always)]
    pub fn tse_high(&self) -> TSE_HIGH_R {
        TSE_HIGH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TSE Compare Low Temperature Event Status Before Masking"]
    #[inline(always)]
    pub fn tse_low(&self) -> TSE_LOW_R {
        TSE_LOW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
