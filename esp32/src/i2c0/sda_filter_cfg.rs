#[doc = "Register `SDA_FILTER_CFG` reader"]
pub struct R(crate::R<SDA_FILTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDA_FILTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDA_FILTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDA_FILTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDA_FILTER_CFG` writer"]
pub struct W(crate::W<SDA_FILTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDA_FILTER_CFG_SPEC>;
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
impl From<crate::W<SDA_FILTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDA_FILTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FILTER_THRES` reader - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub type SDA_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `SDA_FILTER_THRES` writer - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub type SDA_FILTER_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, SDA_FILTER_CFG_SPEC, 3, O>;
#[doc = "Field `SDA_FILTER_EN` reader - This is the filter enable bit for SDA."]
pub type SDA_FILTER_EN_R = crate::BitReader;
#[doc = "Field `SDA_FILTER_EN` writer - This is the filter enable bit for SDA."]
pub type SDA_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, SDA_FILTER_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_FILTER_CFG")
            .field(
                "sda_filter_thres",
                &format_args!("{}", self.sda_filter_thres().bits()),
            )
            .field(
                "sda_filter_en",
                &format_args!("{}", self.sda_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_FILTER_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    #[must_use]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W<0> {
        SDA_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SDA."]
    #[inline(always)]
    #[must_use]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W<3> {
        SDA_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sda_filter_cfg](index.html) module"]
pub struct SDA_FILTER_CFG_SPEC;
impl crate::RegisterSpec for SDA_FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sda_filter_cfg::R](R) reader structure"]
impl crate::Readable for SDA_FILTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sda_filter_cfg::W](W) writer structure"]
impl crate::Writable for SDA_FILTER_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDA_FILTER_CFG to value 0x08"]
impl crate::Resettable for SDA_FILTER_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
