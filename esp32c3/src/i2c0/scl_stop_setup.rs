#[doc = "Register `SCL_STOP_SETUP` reader"]
pub struct R(crate::R<SCL_STOP_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_STOP_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_STOP_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_STOP_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_STOP_SETUP` writer"]
pub struct W(crate::W<SCL_STOP_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_STOP_SETUP_SPEC>;
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
impl From<crate::W<SCL_STOP_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_STOP_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - reg_scl_stop_setup_time"]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - reg_scl_stop_setup_time"]
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, SCL_STOP_SETUP_SPEC, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - reg_scl_stop_setup_time"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STOP_SETUP")
            .field("time", &format_args!("{}", self.time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STOP_SETUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - reg_scl_stop_setup_time"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_SCL_STOP_SETUP_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stop_setup](index.html) module"]
pub struct SCL_STOP_SETUP_SPEC;
impl crate::RegisterSpec for SCL_STOP_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_stop_setup::R](R) reader structure"]
impl crate::Readable for SCL_STOP_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_stop_setup::W](W) writer structure"]
impl crate::Writable for SCL_STOP_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_STOP_SETUP to value 0x08"]
impl crate::Resettable for SCL_STOP_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
