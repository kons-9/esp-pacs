#[doc = "Register `SHA512_START` writer"]
pub struct W(crate::W<SHA512_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA512_START_SPEC>;
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
impl From<crate::W<SHA512_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA512_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA512_START` writer - Write 1 to start an SHA-512 operation on the first message block."]
pub type SHA512_START_W<'a, const O: u8> = crate::BitWriter<'a, SHA512_START_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA512_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start an SHA-512 operation on the first message block."]
    #[inline(always)]
    #[must_use]
    pub fn sha512_start(&mut self) -> SHA512_START_W<0> {
        SHA512_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha512_start](index.html) module"]
pub struct SHA512_START_SPEC;
impl crate::RegisterSpec for SHA512_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sha512_start::W](W) writer structure"]
impl crate::Writable for SHA512_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA512_START to value 0"]
impl crate::Resettable for SHA512_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
