#[doc = "Register `CORE_1_RCD_PDEBUGDATA` reader"]
pub struct R(crate::R<CORE_1_RCD_PDEBUGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_PDEBUGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_PDEBUGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_PDEBUGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_RCD_PDEBUGDATA` reader - Core1_pdebugdata"]
pub type CORE_1_RCD_PDEBUGDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Core1_pdebugdata"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugdata(&self) -> CORE_1_RCD_PDEBUGDATA_R {
        CORE_1_RCD_PDEBUGDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGDATA")
            .field(
                "core_1_rcd_pdebugdata",
                &format_args!("{}", self.core_1_rcd_pdebugdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_RCD_PDEBUGDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Core1 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_pdebugdata](index.html) module"]
pub struct CORE_1_RCD_PDEBUGDATA_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_pdebugdata::R](R) reader structure"]
impl crate::Readable for CORE_1_RCD_PDEBUGDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGDATA to value 0"]
impl crate::Resettable for CORE_1_RCD_PDEBUGDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
