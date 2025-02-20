#[doc = "Register `L1_CACHE_PRELOAD_RST_CTRL` reader"]
pub struct R(crate::R<L1_CACHE_PRELOAD_RST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_PRELOAD_RST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_PRELOAD_RST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_PRELOAD_RST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_PRELOAD_RST_CTRL` writer"]
pub struct W(crate::W<L1_CACHE_PRELOAD_RST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_PRELOAD_RST_CTRL_SPEC>;
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
impl From<crate::W<L1_CACHE_PRELOAD_RST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_PRELOAD_RST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_ICACHE0_PLD_RST` reader - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1_ICACHE0_PLD_RST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_RST` reader - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1_ICACHE1_PLD_RST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PLD_RST` reader - Reserved"]
pub type L1_ICACHE2_PLD_RST_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_RST` reader - Reserved"]
pub type L1_ICACHE3_PLD_RST_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_RST` reader - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1_CACHE_PLD_RST_R = crate::BitReader;
#[doc = "Field `L1_CACHE_PLD_RST` writer - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1_CACHE_PLD_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_PRELOAD_RST_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_icache0_pld_rst(&self) -> L1_ICACHE0_PLD_RST_R {
        L1_ICACHE0_PLD_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_icache1_pld_rst(&self) -> L1_ICACHE1_PLD_RST_R {
        L1_ICACHE1_PLD_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_rst(&self) -> L1_ICACHE2_PLD_RST_R {
        L1_ICACHE2_PLD_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_rst(&self) -> L1_ICACHE3_PLD_RST_R {
        L1_ICACHE3_PLD_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_cache_pld_rst(&self) -> L1_CACHE_PLD_RST_R {
        L1_CACHE_PLD_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_PRELOAD_RST_CTRL")
            .field(
                "l1_icache0_pld_rst",
                &format_args!("{}", self.l1_icache0_pld_rst().bit()),
            )
            .field(
                "l1_icache1_pld_rst",
                &format_args!("{}", self.l1_icache1_pld_rst().bit()),
            )
            .field(
                "l1_icache2_pld_rst",
                &format_args!("{}", self.l1_icache2_pld_rst().bit()),
            )
            .field(
                "l1_icache3_pld_rst",
                &format_args!("{}", self.l1_icache3_pld_rst().bit()),
            )
            .field(
                "l1_cache_pld_rst",
                &format_args!("{}", self.l1_cache_pld_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_PRELOAD_RST_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_pld_rst(&mut self) -> L1_CACHE_PLD_RST_W<4> {
        L1_CACHE_PLD_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Preload Reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_preload_rst_ctrl](index.html) module"]
pub struct L1_CACHE_PRELOAD_RST_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_preload_rst_ctrl::R](R) reader structure"]
impl crate::Readable for L1_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_preload_rst_ctrl::W](W) writer structure"]
impl crate::Writable for L1_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_PRELOAD_RST_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_PRELOAD_RST_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
