#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_8` reader"]
pub struct R(crate::R<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_8` writer"]
pub struct W(crate::W<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>;
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
impl From<crate::W<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_5` reader - Region 4 end address and Region 5 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_5_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_ADDR_5` writer - Region 4 end address and Region 5 start address for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_ADDR_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_8_SPEC, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 4 end address and Region 5 start address for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_addr_5(&self) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_5_R {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_5_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_REGION_PMS_CONSTRAIN_8")
            .field(
                "core_1_region_pms_constrain_addr_5",
                &format_args!("{}", self.core_1_region_pms_constrain_addr_5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_REGION_PMS_CONSTRAIN_8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 4 end address and Region 5 start address for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_addr_5(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_ADDR_5_W<0> {
        CORE_1_REGION_PMS_CONSTRAIN_ADDR_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 region permission register 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_region_pms_constrain_8](index.html) module"]
pub struct CORE_1_REGION_PMS_CONSTRAIN_8_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_region_pms_constrain_8::R](R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_region_pms_constrain_8::W](W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_8 to value 0"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
