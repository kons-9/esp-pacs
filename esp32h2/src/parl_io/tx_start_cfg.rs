#[doc = "Register `TX_START_CFG` reader"]
pub struct R(crate::R<TX_START_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_START_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_START_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_START_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_START_CFG` writer"]
pub struct W(crate::W<TX_START_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_START_CFG_SPEC>;
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
impl From<crate::W<TX_START_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_START_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_START` reader - Set this bit to start tx data transmit."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - Set this bit to start tx data transmit."]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, TX_START_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 31 - Set this bit to start tx data transmit."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_START_CFG")
            .field("tx_start", &format_args!("{}", self.tx_start().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_START_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to start tx data transmit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<31> {
        TX_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel TX Start configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_start_cfg](index.html) module"]
pub struct TX_START_CFG_SPEC;
impl crate::RegisterSpec for TX_START_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_start_cfg::R](R) reader structure"]
impl crate::Readable for TX_START_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_start_cfg::W](W) writer structure"]
impl crate::Writable for TX_START_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_START_CFG to value 0"]
impl crate::Resettable for TX_START_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
