#[doc = "Register `MULT_INT_CLR` writer"]
pub struct W(crate::W<MULT_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULT_INT_CLR_SPEC>;
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
impl From<crate::W<MULT_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULT_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALC_DONE_INT_CLR` writer - Set this bit to clear the i2s_rx_done_int interrupt"]
pub type CALC_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, MULT_INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn calc_done_int_clr(&mut self) -> CALC_DONE_INT_CLR_W<0> {
        CALC_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S interrupt clear register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_int_clr](index.html) module"]
pub struct MULT_INT_CLR_SPEC;
impl crate::RegisterSpec for MULT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mult_int_clr::W](W) writer structure"]
impl crate::Writable for MULT_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULT_INT_CLR to value 0"]
impl crate::Resettable for MULT_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
