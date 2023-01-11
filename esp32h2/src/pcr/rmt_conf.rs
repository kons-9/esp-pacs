#[doc = "Register `RMT_CONF` reader"]
pub struct R(crate::R<RMT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMT_CONF` writer"]
pub struct W(crate::W<RMT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMT_CONF_SPEC>;
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
impl From<crate::W<RMT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMT_CLK_EN` reader - Set 1 to enable rmt apb clock"]
pub type RMT_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `RMT_CLK_EN` writer - Set 1 to enable rmt apb clock"]
pub type RMT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMT_CONF_SPEC, bool, O>;
#[doc = "Field `RMT_RST_EN` reader - Set 0 to reset rmt module"]
pub type RMT_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RMT_RST_EN` writer - Set 0 to reset rmt module"]
pub type RMT_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RMT_CONF_SPEC, bool, O>;
#[doc = "Field `RMT_READY` reader - Query this field after reset rmt module"]
pub type RMT_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable rmt apb clock"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset rmt module"]
    #[inline(always)]
    pub fn rmt_rst_en(&self) -> RMT_RST_EN_R {
        RMT_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset rmt module"]
    #[inline(always)]
    pub fn rmt_ready(&self) -> RMT_READY_R {
        RMT_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable rmt apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<0> {
        RMT_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset rmt module"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_rst_en(&mut self) -> RMT_RST_EN_W<1> {
        RMT_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmt_conf](index.html) module"]
pub struct RMT_CONF_SPEC;
impl crate::RegisterSpec for RMT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmt_conf::R](R) reader structure"]
impl crate::Readable for RMT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmt_conf::W](W) writer structure"]
impl crate::Writable for RMT_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMT_CONF to value 0x05"]
impl crate::Resettable for RMT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
