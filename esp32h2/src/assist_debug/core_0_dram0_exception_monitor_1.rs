#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub struct R(crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DRAM0_RECORDING_PC_0` reader - reg_core_0_dram0_recording_pc_0"]
pub type CORE_0_DRAM0_RECORDING_PC_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_dram0_recording_pc_0"]
    #[inline(always)]
    pub fn core_0_dram0_recording_pc_0(&self) -> CORE_0_DRAM0_RECORDING_PC_0_R {
        CORE_0_DRAM0_RECORDING_PC_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_0_dram0_recording_pc_0",
                &format_args!("{}", self.core_0_dram0_recording_pc_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "exception monitor status register3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_dram0_exception_monitor_1](index.html) module"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_dram0_exception_monitor_1::R](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
