#[doc = "Register `SET_MESSAGE_PAD` writer"]
pub struct W(crate::W<SET_MESSAGE_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_MESSAGE_PAD_SPEC>;
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
impl From<crate::W<SET_MESSAGE_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_MESSAGE_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_TEXT_PAD` writer - Start software padding."]
pub type SET_TEXT_PAD_W<'a, const O: u8> = crate::BitWriter<'a, SET_MESSAGE_PAD_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_MESSAGE_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start software padding."]
    #[inline(always)]
    #[must_use]
    pub fn set_text_pad(&mut self) -> SET_TEXT_PAD_W<0> {
        SET_TEXT_PAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process control register 5.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_message_pad](index.html) module"]
pub struct SET_MESSAGE_PAD_SPEC;
impl crate::RegisterSpec for SET_MESSAGE_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_message_pad::W](W) writer structure"]
impl crate::Writable for SET_MESSAGE_PAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_MESSAGE_PAD to value 0"]
impl crate::Resettable for SET_MESSAGE_PAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
