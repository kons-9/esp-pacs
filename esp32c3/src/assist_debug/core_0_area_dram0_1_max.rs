#[doc = "Register `CORE_0_AREA_DRAM0_1_MAX` reader"]
pub struct R(crate::R<CORE_0_AREA_DRAM0_1_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_AREA_DRAM0_1_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_AREA_DRAM0_1_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_AREA_DRAM0_1_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_AREA_DRAM0_1_MAX` writer"]
pub struct W(crate::W<CORE_0_AREA_DRAM0_1_MAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_AREA_DRAM0_1_MAX_SPEC>;
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
impl From<crate::W<CORE_0_AREA_DRAM0_1_MAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_AREA_DRAM0_1_MAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_MAX` reader - reg_core_0_area_dram0_1_max"]
pub type CORE_0_AREA_DRAM0_1_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_MAX` writer - reg_core_0_area_dram0_1_max"]
pub type CORE_0_AREA_DRAM0_1_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_AREA_DRAM0_1_MAX_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_area_dram0_1_max"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_max(&self) -> CORE_0_AREA_DRAM0_1_MAX_R {
        CORE_0_AREA_DRAM0_1_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_AREA_DRAM0_1_MAX")
            .field(
                "core_0_area_dram0_1_max",
                &format_args!("{}", self.core_0_area_dram0_1_max().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_AREA_DRAM0_1_MAX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_core_0_area_dram0_1_max"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_max(&mut self) -> CORE_0_AREA_DRAM0_1_MAX_W<0> {
        CORE_0_AREA_DRAM0_1_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_area_dram0_1_max](index.html) module"]
pub struct CORE_0_AREA_DRAM0_1_MAX_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_DRAM0_1_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_area_dram0_1_max::R](R) reader structure"]
impl crate::Readable for CORE_0_AREA_DRAM0_1_MAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_area_dram0_1_max::W](W) writer structure"]
impl crate::Writable for CORE_0_AREA_DRAM0_1_MAX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_AREA_DRAM0_1_MAX to value 0"]
impl crate::Resettable for CORE_0_AREA_DRAM0_1_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
