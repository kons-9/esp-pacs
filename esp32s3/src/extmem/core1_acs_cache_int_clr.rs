#[doc = "Register `CORE1_ACS_CACHE_INT_CLR` writer"]
pub struct W(crate::W<CORE1_ACS_CACHE_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE1_ACS_CACHE_INT_CLR_SPEC>;
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
impl From<crate::W<CORE1_ACS_CACHE_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE1_ACS_CACHE_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE1_IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
pub type CORE1_IBUS_ACS_MSK_IC_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE1_ACS_CACHE_INT_CLR_SPEC, O>;
#[doc = "Field `CORE1_IBUS_WR_IC_INT_CLR` writer - The bit is used to clear interrupt by ibus trying to write icache"]
pub type CORE1_IBUS_WR_IC_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE1_ACS_CACHE_INT_CLR_SPEC, O>;
#[doc = "Field `CORE1_IBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE1_IBUS_REJECT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE1_ACS_CACHE_INT_CLR_SPEC, O>;
#[doc = "Field `CORE1_DBUS_ACS_MSK_DC_INT_CLR` writer - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
pub type CORE1_DBUS_ACS_MSK_DC_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE1_ACS_CACHE_INT_CLR_SPEC, O>;
#[doc = "Field `CORE1_DBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE1_DBUS_REJECT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE1_ACS_CACHE_INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE1_ACS_CACHE_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_acs_msk_ic_int_clr(&mut self) -> CORE1_IBUS_ACS_MSK_IC_INT_CLR_W<0> {
        CORE1_IBUS_ACS_MSK_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by ibus trying to write icache"]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_wr_ic_int_clr(&mut self) -> CORE1_IBUS_WR_IC_INT_CLR_W<1> {
        CORE1_IBUS_WR_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_reject_int_clr(&mut self) -> CORE1_IBUS_REJECT_INT_CLR_W<2> {
        CORE1_IBUS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_acs_msk_dc_int_clr(&mut self) -> CORE1_DBUS_ACS_MSK_DC_INT_CLR_W<3> {
        CORE1_DBUS_ACS_MSK_DC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_reject_int_clr(&mut self) -> CORE1_DBUS_REJECT_INT_CLR_W<4> {
        CORE1_DBUS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core1_acs_cache_int_clr](index.html) module"]
pub struct CORE1_ACS_CACHE_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE1_ACS_CACHE_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core1_acs_cache_int_clr::W](W) writer structure"]
impl crate::Writable for CORE1_ACS_CACHE_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE1_ACS_CACHE_INT_CLR to value 0"]
impl crate::Resettable for CORE1_ACS_CACHE_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
