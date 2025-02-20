#[doc = "Register `SAR_TOUCH_THRES9` reader"]
pub struct R(crate::R<SAR_TOUCH_THRES9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_THRES9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_THRES9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_THRES9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_THRES9` writer"]
pub struct W(crate::W<SAR_TOUCH_THRES9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_THRES9_SPEC>;
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
impl From<crate::W<SAR_TOUCH_THRES9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_THRES9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_TOUCH_OUT_TH9` reader - Finger threshold for touch pad 9"]
pub type SAR_TOUCH_OUT_TH9_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH9` writer - Finger threshold for touch pad 9"]
pub type SAR_TOUCH_OUT_TH9_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_THRES9_SPEC, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 9"]
    #[inline(always)]
    pub fn sar_touch_out_th9(&self) -> SAR_TOUCH_OUT_TH9_R {
        SAR_TOUCH_OUT_TH9_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES9")
            .field(
                "sar_touch_out_th9",
                &format_args!("{}", self.sar_touch_out_th9().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 9"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_out_th9(&mut self) -> SAR_TOUCH_OUT_TH9_W<0> {
        SAR_TOUCH_OUT_TH9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch thres of touch pad\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_thres9](index.html) module"]
pub struct SAR_TOUCH_THRES9_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_thres9::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_thres9::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES9 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
