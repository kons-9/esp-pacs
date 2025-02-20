#[doc = "Register `TX_STATUS` reader"]
pub struct R(crate::R<TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_TX_FULL` reader - "]
pub type SLC0_TX_FULL_R = crate::BitReader;
#[doc = "Field `SLC0_TX_EMPTY` reader - "]
pub type SLC0_TX_EMPTY_R = crate::BitReader;
#[doc = "Field `SLC1_TX_FULL` reader - "]
pub type SLC1_TX_FULL_R = crate::BitReader;
#[doc = "Field `SLC1_TX_EMPTY` reader - "]
pub type SLC1_TX_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_full(&self) -> SLC0_TX_FULL_R {
        SLC0_TX_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_empty(&self) -> SLC0_TX_EMPTY_R {
        SLC0_TX_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_full(&self) -> SLC1_TX_FULL_R {
        SLC1_TX_FULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_empty(&self) -> SLC1_TX_EMPTY_R {
        SLC1_TX_EMPTY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_STATUS")
            .field(
                "slc0_tx_full",
                &format_args!("{}", self.slc0_tx_full().bit()),
            )
            .field(
                "slc0_tx_empty",
                &format_args!("{}", self.slc0_tx_empty().bit()),
            )
            .field(
                "slc1_tx_full",
                &format_args!("{}", self.slc1_tx_full().bit()),
            )
            .field(
                "slc1_tx_empty",
                &format_args!("{}", self.slc1_tx_empty().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_status](index.html) module"]
pub struct TX_STATUS_SPEC;
impl crate::RegisterSpec for TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_status::R](R) reader structure"]
impl crate::Readable for TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_STATUS to value 0x0002_0002"]
impl crate::Resettable for TX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0002;
}
