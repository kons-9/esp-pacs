#[doc = "Register `MULT_MODE` reader"]
pub struct R(crate::R<MULT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MULT_MODE` writer"]
pub struct W(crate::W<MULT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULT_MODE_SPEC>;
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
impl From<crate::W<MULT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MULT_MODE` reader - This register contains the mode of modular multiplication and multiplication."]
pub type MULT_MODE_R = crate::BitReader;
#[doc = "Field `MULT_MODE` writer - This register contains the mode of modular multiplication and multiplication."]
pub type MULT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MULT_MODE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This register contains the mode of modular multiplication and multiplication."]
    #[inline(always)]
    pub fn mult_mode(&self) -> MULT_MODE_R {
        MULT_MODE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_MODE")
            .field("mult_mode", &format_args!("{}", self.mult_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This register contains the mode of modular multiplication and multiplication."]
    #[inline(always)]
    #[must_use]
    pub fn mult_mode(&mut self) -> MULT_MODE_W<0> {
        MULT_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_mode](index.html) module"]
pub struct MULT_MODE_SPEC;
impl crate::RegisterSpec for MULT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mult_mode::R](R) reader structure"]
impl crate::Readable for MULT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mult_mode::W](W) writer structure"]
impl crate::Writable for MULT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULT_MODE to value 0"]
impl crate::Resettable for MULT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
