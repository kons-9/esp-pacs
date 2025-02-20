#[doc = "Register `SWD_CONF` reader"]
pub struct R(crate::R<SWD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWD_CONF` writer"]
pub struct W(crate::W<SWD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWD_CONF_SPEC>;
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
impl From<crate::W<SWD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWD_RESET_FLAG` reader - need_des"]
pub type SWD_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` reader - need_des"]
pub type SWD_AUTO_FEED_EN_R = crate::BitReader;
#[doc = "Field `SWD_AUTO_FEED_EN` writer - need_des"]
pub type SWD_AUTO_FEED_EN_W<'a, const O: u8> = crate::BitWriter<'a, SWD_CONF_SPEC, O>;
#[doc = "Field `SWD_RST_FLAG_CLR` writer - need_des"]
pub type SWD_RST_FLAG_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SWD_CONF_SPEC, O>;
#[doc = "Field `SWD_SIGNAL_WIDTH` reader - need_des"]
pub type SWD_SIGNAL_WIDTH_R = crate::FieldReader<u16>;
#[doc = "Field `SWD_SIGNAL_WIDTH` writer - need_des"]
pub type SWD_SIGNAL_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, SWD_CONF_SPEC, 10, O, u16>;
#[doc = "Field `SWD_DISABLE` reader - need_des"]
pub type SWD_DISABLE_R = crate::BitReader;
#[doc = "Field `SWD_DISABLE` writer - need_des"]
pub type SWD_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, SWD_CONF_SPEC, O>;
#[doc = "Field `SWD_FEED` writer - need_des"]
pub type SWD_FEED_W<'a, const O: u8> = crate::BitWriter<'a, SWD_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> SWD_RESET_FLAG_R {
        SWD_RESET_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> SWD_AUTO_FEED_EN_R {
        SWD_AUTO_FEED_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn swd_signal_width(&self) -> SWD_SIGNAL_WIDTH_R {
        SWD_SIGNAL_WIDTH_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_CONF")
            .field(
                "swd_reset_flag",
                &format_args!("{}", self.swd_reset_flag().bit()),
            )
            .field(
                "swd_auto_feed_en",
                &format_args!("{}", self.swd_auto_feed_en().bit()),
            )
            .field(
                "swd_signal_width",
                &format_args!("{}", self.swd_signal_width().bits()),
            )
            .field("swd_disable", &format_args!("{}", self.swd_disable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWD_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_auto_feed_en(&mut self) -> SWD_AUTO_FEED_EN_W<18> {
        SWD_AUTO_FEED_EN_W::new(self)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_rst_flag_clr(&mut self) -> SWD_RST_FLAG_CLR_W<19> {
        SWD_RST_FLAG_CLR_W::new(self)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_signal_width(&mut self) -> SWD_SIGNAL_WIDTH_W<20> {
        SWD_SIGNAL_WIDTH_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_disable(&mut self) -> SWD_DISABLE_W<30> {
        SWD_DISABLE_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_feed(&mut self) -> SWD_FEED_W<31> {
        SWD_FEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_conf](index.html) module"]
pub struct SWD_CONF_SPEC;
impl crate::RegisterSpec for SWD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swd_conf::R](R) reader structure"]
impl crate::Readable for SWD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swd_conf::W](W) writer structure"]
impl crate::Writable for SWD_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWD_CONF to value 0x12c0_0000"]
impl crate::Resettable for SWD_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x12c0_0000;
}
