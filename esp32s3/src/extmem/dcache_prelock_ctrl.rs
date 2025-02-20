#[doc = "Register `DCACHE_PRELOCK_CTRL` reader"]
pub struct R(crate::R<DCACHE_PRELOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_PRELOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_PRELOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_PRELOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_PRELOCK_CTRL` writer"]
pub struct W(crate::W<DCACHE_PRELOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_PRELOCK_CTRL_SPEC>;
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
impl From<crate::W<DCACHE_PRELOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_PRELOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function."]
pub type DCACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function."]
pub type DCACHE_PRELOCK_SCT0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, DCACHE_PRELOCK_CTRL_SPEC, O>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function."]
pub type DCACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function."]
pub type DCACHE_PRELOCK_SCT1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, DCACHE_PRELOCK_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct0_en(&self) -> DCACHE_PRELOCK_SCT0_EN_R {
        DCACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct1_en(&self) -> DCACHE_PRELOCK_SCT1_EN_R {
        DCACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOCK_CTRL")
            .field(
                "dcache_prelock_sct0_en",
                &format_args!("{}", self.dcache_prelock_sct0_en().bit()),
            )
            .field(
                "dcache_prelock_sct1_en",
                &format_args!("{}", self.dcache_prelock_sct1_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_PRELOCK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct0_en(&mut self) -> DCACHE_PRELOCK_SCT0_EN_W<0> {
        DCACHE_PRELOCK_SCT0_EN_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct1_en(&mut self) -> DCACHE_PRELOCK_SCT1_EN_W<1> {
        DCACHE_PRELOCK_SCT1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_prelock_ctrl](index.html) module"]
pub struct DCACHE_PRELOCK_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_prelock_ctrl::R](R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_prelock_ctrl::W](W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_CTRL to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
