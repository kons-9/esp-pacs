#[doc = "Register `LEDC_CONF` reader"]
pub struct R(crate::R<LEDC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_CONF` writer"]
pub struct W(crate::W<LEDC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_CONF_SPEC>;
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
impl From<crate::W<LEDC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEDC_CLK_EN` reader - Set 1 to enable ledc apb clock"]
pub type LEDC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_CLK_EN` writer - Set 1 to enable ledc apb clock"]
pub type LEDC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_CONF_SPEC, bool, O>;
#[doc = "Field `LEDC_RST_EN` reader - Set 0 to reset ledc module"]
pub type LEDC_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_RST_EN` writer - Set 0 to reset ledc module"]
pub type LEDC_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_CONF_SPEC, bool, O>;
#[doc = "Field `LEDC_READY` reader - Query this field after reset ledc module"]
pub type LEDC_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable ledc apb clock"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset ledc module"]
    #[inline(always)]
    pub fn ledc_rst_en(&self) -> LEDC_RST_EN_R {
        LEDC_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset ledc module"]
    #[inline(always)]
    pub fn ledc_ready(&self) -> LEDC_READY_R {
        LEDC_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable ledc apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<0> {
        LEDC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset ledc module"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_rst_en(&mut self) -> LEDC_RST_EN_W<1> {
        LEDC_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_conf](index.html) module"]
pub struct LEDC_CONF_SPEC;
impl crate::RegisterSpec for LEDC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_conf::R](R) reader structure"]
impl crate::Readable for LEDC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_conf::W](W) writer structure"]
impl crate::Writable for LEDC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LEDC_CONF to value 0x05"]
impl crate::Resettable for LEDC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
