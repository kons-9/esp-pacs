#[doc = "Register `PWM0_INTR_MAP` reader"]
pub struct R(crate::R<PWM0_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM0_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM0_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM0_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM0_INTR_MAP` writer"]
pub struct W(crate::W<PWM0_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM0_INTR_MAP_SPEC>;
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
impl From<crate::W<PWM0_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM0_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM0_INTR_MAP` reader - this register used to map pwm0 interrupt to one of core1's external interrupt"]
pub type PWM0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PWM0_INTR_MAP` writer - this register used to map pwm0 interrupt to one of core1's external interrupt"]
pub type PWM0_INTR_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, PWM0_INTR_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map pwm0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn pwm0_intr_map(&self) -> PWM0_INTR_MAP_R {
        PWM0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0_INTR_MAP")
            .field(
                "pwm0_intr_map",
                &format_args!("{}", self.pwm0_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWM0_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map pwm0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0_intr_map(&mut self) -> PWM0_INTR_MAP_W<0> {
        PWM0_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm0 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_intr_map](index.html) module"]
pub struct PWM0_INTR_MAP_SPEC;
impl crate::RegisterSpec for PWM0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm0_intr_map::R](R) reader structure"]
impl crate::Readable for PWM0_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm0_intr_map::W](W) writer structure"]
impl crate::Writable for PWM0_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM0_INTR_MAP to value 0x10"]
impl crate::Resettable for PWM0_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
