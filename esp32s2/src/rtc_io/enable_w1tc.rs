#[doc = "Register `ENABLE_W1TC` writer"]
pub struct W(crate::W<ENABLE_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_W1TC_SPEC>;
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
impl From<crate::W<ENABLE_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_W1TC` writer - GPIO0 ~ 21 output enable clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_ENABLE_REG will be cleared. Recommended operation: use this register to clear RTCIO_RTC_GPIO_ENABLE_REG."]
pub type ENABLE_W1TC_W<'a, const O: u8> = crate::FieldWriter<'a, ENABLE_W1TC_SPEC, 22, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_ENABLE_REG will be cleared. Recommended operation: use this register to clear RTCIO_RTC_GPIO_ENABLE_REG."]
    #[inline(always)]
    #[must_use]
    pub fn enable_w1tc(&mut self) -> ENABLE_W1TC_W<10> {
        ENABLE_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC GPIO output enable bit clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_w1tc](index.html) module"]
pub struct ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enable_w1tc::W](W) writer structure"]
impl crate::Writable for ENABLE_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE_W1TC to value 0"]
impl crate::Resettable for ENABLE_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
