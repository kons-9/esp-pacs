#[doc = "Register `TIMER0_CFG1` reader"]
pub struct R(crate::R<TIMER0_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_CFG1` writer"]
pub struct W(crate::W<TIMER0_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CFG1_SPEC>;
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
impl From<crate::W<TIMER0_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_START` reader - "]
pub type TIMER0_START_R = crate::FieldReader;
#[doc = "Field `TIMER0_START` writer - "]
pub type TIMER0_START_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER0_CFG1_SPEC, 3, O>;
#[doc = "Field `TIMER0_MOD` reader - "]
pub type TIMER0_MOD_R = crate::FieldReader;
#[doc = "Field `TIMER0_MOD` writer - "]
pub type TIMER0_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER0_CFG1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_start(&self) -> TIMER0_START_R {
        TIMER0_START_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer0_mod(&self) -> TIMER0_MOD_R {
        TIMER0_MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0_CFG1")
            .field(
                "timer0_start",
                &format_args!("{}", self.timer0_start().bits()),
            )
            .field("timer0_mod", &format_args!("{}", self.timer0_mod().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER0_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_start(&mut self) -> TIMER0_START_W<0> {
        TIMER0_START_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_mod(&mut self) -> TIMER0_MOD_W<3> {
        TIMER0_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_cfg1](index.html) module"]
pub struct TIMER0_CFG1_SPEC;
impl crate::RegisterSpec for TIMER0_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_cfg1::R](R) reader structure"]
impl crate::Readable for TIMER0_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_cfg1::W](W) writer structure"]
impl crate::Writable for TIMER0_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER0_CFG1 to value 0"]
impl crate::Resettable for TIMER0_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
