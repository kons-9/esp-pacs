#[doc = "Register `LOG_MEM_CURRENT_ADDR` reader"]
pub struct R(crate::R<LOG_MEM_CURRENT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MEM_CURRENT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MEM_CURRENT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MEM_CURRENT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOG_MEM_CURRENT_ADDR` reader - means next writing address"]
pub type LOG_MEM_CURRENT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - means next writing address"]
    #[inline(always)]
    pub fn log_mem_current_addr(&self) -> LOG_MEM_CURRENT_ADDR_R {
        LOG_MEM_CURRENT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MEM_CURRENT_ADDR")
            .field(
                "log_mem_current_addr",
                &format_args!("{}", self.log_mem_current_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MEM_CURRENT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "current writing address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_current_addr](index.html) module"]
pub struct LOG_MEM_CURRENT_ADDR_SPEC;
impl crate::RegisterSpec for LOG_MEM_CURRENT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_mem_current_addr::R](R) reader structure"]
impl crate::Readable for LOG_MEM_CURRENT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOG_MEM_CURRENT_ADDR to value 0"]
impl crate::Resettable for LOG_MEM_CURRENT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
