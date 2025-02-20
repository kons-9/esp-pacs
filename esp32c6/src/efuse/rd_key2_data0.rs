#[doc = "Register `RD_KEY2_DATA0` reader"]
pub struct R(crate::R<RD_KEY2_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY2_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY2_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY2_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY2_DATA0` reader - Stores the zeroth 32 bits of KEY2."]
pub type KEY2_DATA0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the zeroth 32 bits of KEY2."]
    #[inline(always)]
    pub fn key2_data0(&self) -> KEY2_DATA0_R {
        KEY2_DATA0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY2_DATA0")
            .field("key2_data0", &format_args!("{}", self.key2_data0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY2_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register $n of BLOCK6 (KEY2).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key2_data0](index.html) module"]
pub struct RD_KEY2_DATA0_SPEC;
impl crate::RegisterSpec for RD_KEY2_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key2_data0::R](R) reader structure"]
impl crate::Readable for RD_KEY2_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY2_DATA0 to value 0"]
impl crate::Resettable for RD_KEY2_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
