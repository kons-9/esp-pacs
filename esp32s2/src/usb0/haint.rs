#[doc = "Register `HAINT` reader"]
pub struct R(crate::R<HAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HAINT` reader - "]
pub type HAINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINT")
            .field("haint", &format_args!("{}", self.haint().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HAINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haint](index.html) module"]
pub struct HAINT_SPEC;
impl crate::RegisterSpec for HAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haint::R](R) reader structure"]
impl crate::Readable for HAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
