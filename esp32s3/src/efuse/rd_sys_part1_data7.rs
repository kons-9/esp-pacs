#[doc = "Register `RD_SYS_PART1_DATA7` reader"]
pub struct R(crate::R<RD_SYS_PART1_DATA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_SYS_PART1_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_SYS_PART1_DATA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_SYS_PART1_DATA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DATA_PART1_7` reader - Stores the seventh 32 bits of the first part of system data."]
pub type SYS_DATA_PART1_7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the seventh 32 bits of the first part of system data."]
    #[inline(always)]
    pub fn sys_data_part1_7(&self) -> SYS_DATA_PART1_7_R {
        SYS_DATA_PART1_7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA7")
            .field(
                "sys_data_part1_7",
                &format_args!("{}", self.sys_data_part1_7().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_SYS_PART1_DATA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 7 of BLOCK2 (system).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data7](index.html) module"]
pub struct RD_SYS_PART1_DATA7_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_sys_part1_data7::R](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA7 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
