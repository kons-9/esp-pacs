#[doc = "Register `OUT1_W1TC` writer"]
pub struct W(crate::W<OUT1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT1_W1TC_SPEC>;
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
impl From<crate::W<OUT1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT1_W1TC` writer - GPIO output clear register for GPIO32-34"]
pub type OUT1_W1TC_W<'a, const O: u8> = crate::FieldWriter<'a, OUT1_W1TC_SPEC, 3, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT1_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO output clear register for GPIO32-34"]
    #[inline(always)]
    #[must_use]
    pub fn out1_w1tc(&mut self) -> OUT1_W1TC_W<0> {
        OUT1_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output clear register for GPIO32-34\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1_w1tc](index.html) module"]
pub struct OUT1_W1TC_SPEC;
impl crate::RegisterSpec for OUT1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out1_w1tc::W](W) writer structure"]
impl crate::Writable for OUT1_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT1_W1TC to value 0"]
impl crate::Resettable for OUT1_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
