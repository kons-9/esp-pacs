#[doc = "Register `RD_KEY1_DATA2` reader"]
pub struct R(crate::R<RD_KEY1_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY1_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY1_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY1_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY1_DATA2` reader - Stores the second 32 bits of KEY1."]
pub type KEY1_DATA2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the second 32 bits of KEY1."]
    #[inline(always)]
    pub fn key1_data2(&self) -> KEY1_DATA2_R {
        KEY1_DATA2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY1_DATA2")
            .field("key1_data2", &format_args!("{}", self.key1_data2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY1_DATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register $n of BLOCK5 (KEY1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key1_data2](index.html) module"]
pub struct RD_KEY1_DATA2_SPEC;
impl crate::RegisterSpec for RD_KEY1_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key1_data2::R](R) reader structure"]
impl crate::Readable for RD_KEY1_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY1_DATA2 to value 0"]
impl crate::Resettable for RD_KEY1_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
