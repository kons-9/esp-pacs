#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET0_INT_CLR` writer - Interrupt clear bit of system timer target 0."]
pub type TARGET0_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TARGET1_INT_CLR` writer - Interrupt clear bit of system timer target 1."]
pub type TARGET1_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TARGET2_INT_CLR` writer - Interrupt clear bit of system timer target 2."]
pub type TARGET2_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt clear bit of system timer target 0."]
    #[inline(always)]
    #[must_use]
    pub fn target0_int_clr(&mut self) -> TARGET0_INT_CLR_W<0> {
        TARGET0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt clear bit of system timer target 1."]
    #[inline(always)]
    #[must_use]
    pub fn target1_int_clr(&mut self) -> TARGET1_INT_CLR_W<1> {
        TARGET1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt clear bit of system timer target 2."]
    #[inline(always)]
    #[must_use]
    pub fn target2_int_clr(&mut self) -> TARGET2_INT_CLR_W<2> {
        TARGET2_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System timer interrupt clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
