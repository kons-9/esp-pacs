#[doc = "Register `CORE_0_SP_PC` reader"]
pub struct R(crate::R<CORE_0_SP_PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_SP_PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_SP_PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_SP_PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_SP_PC` reader - This regsiter stores the PC when trigger stack monitor."]
pub type CORE_0_SP_PC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This regsiter stores the PC when trigger stack monitor."]
    #[inline(always)]
    pub fn core_0_sp_pc(&self) -> CORE_0_SP_PC_R {
        CORE_0_SP_PC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_SP_PC")
            .field(
                "core_0_sp_pc",
                &format_args!("{}", self.core_0_sp_pc().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_SP_PC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "stack monitor pc status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_sp_pc](index.html) module"]
pub struct CORE_0_SP_PC_SPEC;
impl crate::RegisterSpec for CORE_0_SP_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_sp_pc::R](R) reader structure"]
impl crate::Readable for CORE_0_SP_PC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_SP_PC to value 0"]
impl crate::Resettable for CORE_0_SP_PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
