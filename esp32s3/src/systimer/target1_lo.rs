#[doc = "Register `TARGET1_LO` reader"]
pub struct R(crate::R<TARGET1_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET1_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET1_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET1_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET1_LO` writer"]
pub struct W(crate::W<TARGET1_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET1_LO_SPEC>;
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
impl From<crate::W<TARGET1_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET1_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TARGET1_LO` reader - timer taget1 low 32 bits"]
pub type TIMER_TARGET1_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET1_LO` writer - timer taget1 low 32 bits"]
pub type TIMER_TARGET1_LO_W<'a, const O: u8> = crate::FieldWriter<'a, TARGET1_LO_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - timer taget1 low 32 bits"]
    #[inline(always)]
    pub fn timer_target1_lo(&self) -> TIMER_TARGET1_LO_R {
        TIMER_TARGET1_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET1_LO")
            .field(
                "timer_target1_lo",
                &format_args!("{}", self.timer_target1_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET1_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer taget1 low 32 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_target1_lo(&mut self) -> TIMER_TARGET1_LO_W<0> {
        TIMER_TARGET1_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer comp1 value low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target1_lo](index.html) module"]
pub struct TARGET1_LO_SPEC;
impl crate::RegisterSpec for TARGET1_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target1_lo::R](R) reader structure"]
impl crate::Readable for TARGET1_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target1_lo::W](W) writer structure"]
impl crate::Writable for TARGET1_LO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET1_LO to value 0"]
impl crate::Resettable for TARGET1_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
