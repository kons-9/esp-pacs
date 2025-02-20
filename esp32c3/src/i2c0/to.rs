#[doc = "Register `TO` reader"]
pub struct R(crate::R<TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TO` writer"]
pub struct W(crate::W<TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TO_SPEC>;
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
impl From<crate::W<TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_OUT_VALUE` reader - reg_time_out_value"]
pub type TIME_OUT_VALUE_R = crate::FieldReader;
#[doc = "Field `TIME_OUT_VALUE` writer - reg_time_out_value"]
pub type TIME_OUT_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, TO_SPEC, 5, O>;
#[doc = "Field `TIME_OUT_EN` reader - reg_time_out_en"]
pub type TIME_OUT_EN_R = crate::BitReader;
#[doc = "Field `TIME_OUT_EN` writer - reg_time_out_en"]
pub type TIME_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, TO_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - reg_time_out_value"]
    #[inline(always)]
    pub fn time_out_value(&self) -> TIME_OUT_VALUE_R {
        TIME_OUT_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - reg_time_out_en"]
    #[inline(always)]
    pub fn time_out_en(&self) -> TIME_OUT_EN_R {
        TIME_OUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO")
            .field(
                "time_out_value",
                &format_args!("{}", self.time_out_value().bits()),
            )
            .field("time_out_en", &format_args!("{}", self.time_out_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_time_out_value"]
    #[inline(always)]
    #[must_use]
    pub fn time_out_value(&mut self) -> TIME_OUT_VALUE_W<0> {
        TIME_OUT_VALUE_W::new(self)
    }
    #[doc = "Bit 5 - reg_time_out_en"]
    #[inline(always)]
    #[must_use]
    pub fn time_out_en(&mut self) -> TIME_OUT_EN_W<5> {
        TIME_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_TO_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [to](index.html) module"]
pub struct TO_SPEC;
impl crate::RegisterSpec for TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [to::R](R) reader structure"]
impl crate::Readable for TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [to::W](W) writer structure"]
impl crate::Writable for TO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TO to value 0x10"]
impl crate::Resettable for TO_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
