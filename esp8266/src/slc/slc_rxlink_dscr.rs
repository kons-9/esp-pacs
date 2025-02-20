#[doc = "Register `SLC_RXLINK_DSCR` reader"]
pub struct R(crate::R<SLC_RXLINK_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_RXLINK_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_RXLINK_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_RXLINK_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_RXLINK_DSCR` writer"]
pub struct W(crate::W<SLC_RXLINK_DSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_RXLINK_DSCR_SPEC>;
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
impl From<crate::W<SLC_RXLINK_DSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_RXLINK_DSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_RXLINK_DSCR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_RXLINK_DSCR")
            .field("register", &format_args!("{}", self.register().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_RXLINK_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<0> {
        REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_RXLINK_DSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_rxlink_dscr](index.html) module"]
pub struct SLC_RXLINK_DSCR_SPEC;
impl crate::RegisterSpec for SLC_RXLINK_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_rxlink_dscr::R](R) reader structure"]
impl crate::Readable for SLC_RXLINK_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_rxlink_dscr::W](W) writer structure"]
impl crate::Writable for SLC_RXLINK_DSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_RXLINK_DSCR to value 0"]
impl crate::Resettable for SLC_RXLINK_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
