#[doc = "Register `APP_DCACHE_DBUG2` reader"]
pub struct R(crate::R<APP_DCACHE_DBUG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_DCACHE_DBUG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_DCACHE_DBUG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_DCACHE_DBUG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APP_CACHE_VADDR` reader - "]
pub type APP_CACHE_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn app_cache_vaddr(&self) -> APP_CACHE_VADDR_R {
        APP_CACHE_VADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG2")
            .field(
                "app_cache_vaddr",
                &format_args!("{}", self.app_cache_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DCACHE_DBUG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_dcache_dbug2](index.html) module"]
pub struct APP_DCACHE_DBUG2_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_dcache_dbug2::R](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_DCACHE_DBUG2 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
