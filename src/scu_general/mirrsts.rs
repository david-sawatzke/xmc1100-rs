#[doc = "Reader of register MIRRSTS"]
pub type R = crate::R<u32, super::MIRRSTS>;
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
impl R {
    #[doc = "Bit 0 - RTC CTR Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC ATIM0 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC ATIM1 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC TIM0 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC TIM1 Mirror Register Update Status"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
