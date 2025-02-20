#[doc = "Register `EXT_WAKEUP1` reader"]
pub struct R(crate::R<EXT_WAKEUP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP1` writer"]
pub struct W(crate::W<EXT_WAKEUP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP1_SPEC>;
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
impl From<crate::W<EXT_WAKEUP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_WAKEUP1_SEL` reader - Bitmap to select RTC pads for ext wakeup1"]
pub type EXT_WAKEUP1_SEL_R = crate::FieldReader<u32>;
#[doc = "Field `EXT_WAKEUP1_SEL` writer - Bitmap to select RTC pads for ext wakeup1"]
pub type EXT_WAKEUP1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, EXT_WAKEUP1_SPEC, 22, O, u32>;
#[doc = "Field `EXT_WAKEUP1_STATUS_CLR` writer - clear ext wakeup1 status"]
pub type EXT_WAKEUP1_STATUS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, EXT_WAKEUP1_SPEC, O>;
impl R {
    #[doc = "Bits 0:21 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    pub fn ext_wakeup1_sel(&self) -> EXT_WAKEUP1_SEL_R {
        EXT_WAKEUP1_SEL_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP1")
            .field(
                "ext_wakeup1_sel",
                &format_args!("{}", self.ext_wakeup1_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Bitmap to select RTC pads for ext wakeup1"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_sel(&mut self) -> EXT_WAKEUP1_SEL_W<0> {
        EXT_WAKEUP1_SEL_W::new(self)
    }
    #[doc = "Bit 22 - clear ext wakeup1 status"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_status_clr(&mut self) -> EXT_WAKEUP1_STATUS_CLR_W<22> {
        EXT_WAKEUP1_STATUS_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ext1 wakeup\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup1](index.html) module"]
pub struct EXT_WAKEUP1_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup1::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup1::W](W) writer structure"]
impl crate::Writable for EXT_WAKEUP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP1 to value 0"]
impl crate::Resettable for EXT_WAKEUP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
