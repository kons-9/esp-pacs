#[doc = "Register `PRO_DCACHE_DBUG1` reader"]
pub struct R(crate::R<PRO_DCACHE_DBUG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_DBUG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_DBUG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_DBUG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_CTAG_RAM_RDATA` reader - "]
pub type PRO_CTAG_RAM_RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pro_ctag_ram_rdata(&self) -> PRO_CTAG_RAM_RDATA_R {
        PRO_CTAG_RAM_RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG1")
            .field(
                "pro_ctag_ram_rdata",
                &format_args!("{}", self.pro_ctag_ram_rdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_DBUG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_dbug1](index.html) module"]
pub struct PRO_DCACHE_DBUG1_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_dbug1::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG1 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
