#[doc = "Register `RELEASE` writer"]
pub struct W(crate::W<RELEASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RELEASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RELEASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RELEASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELEASE` writer - Set this bit to release the manual encrypted result, after that the result will be visible to spi"]
pub type RELEASE_W<'a, const O: u8> = crate::BitWriter<'a, RELEASE_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RELEASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to release the manual encrypted result, after that the result will be visible to spi"]
    #[inline(always)]
    #[must_use]
    pub fn release(&mut self) -> RELEASE_W<0> {
        RELEASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES release register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [release](index.html) module"]
pub struct RELEASE_SPEC;
impl crate::RegisterSpec for RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [release::W](W) writer structure"]
impl crate::Writable for RELEASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RELEASE to value 0"]
impl crate::Resettable for RELEASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
