#[doc = "Register `POWER_PD_HPWIFI_CNTL` reader"]
pub struct R(crate::R<POWER_PD_HPWIFI_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_PD_HPWIFI_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_PD_HPWIFI_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_PD_HPWIFI_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_PD_HPWIFI_CNTL` writer"]
pub struct W(crate::W<POWER_PD_HPWIFI_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_PD_HPWIFI_CNTL_SPEC>;
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
impl From<crate::W<POWER_PD_HPWIFI_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_PD_HPWIFI_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_HP_WIFI_RESET` reader - need_des"]
pub type FORCE_HP_WIFI_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_HP_WIFI_RESET` writer - need_des"]
pub type FORCE_HP_WIFI_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_HP_WIFI_ISO` reader - need_des"]
pub type FORCE_HP_WIFI_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_HP_WIFI_ISO` writer - need_des"]
pub type FORCE_HP_WIFI_ISO_W<'a, const O: u8> = crate::BitWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_HP_WIFI_PU` reader - need_des"]
pub type FORCE_HP_WIFI_PU_R = crate::BitReader;
#[doc = "Field `FORCE_HP_WIFI_PU` writer - need_des"]
pub type FORCE_HP_WIFI_PU_W<'a, const O: u8> = crate::BitWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_HP_WIFI_NO_RESET` reader - need_des"]
pub type FORCE_HP_WIFI_NO_RESET_R = crate::BitReader;
#[doc = "Field `FORCE_HP_WIFI_NO_RESET` writer - need_des"]
pub type FORCE_HP_WIFI_NO_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_HP_WIFI_NO_ISO` reader - need_des"]
pub type FORCE_HP_WIFI_NO_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_HP_WIFI_NO_ISO` writer - need_des"]
pub type FORCE_HP_WIFI_NO_ISO_W<'a, const O: u8> =
    crate::BitWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, O>;
#[doc = "Field `FORCE_HP_WIFI_PD` reader - need_des"]
pub type FORCE_HP_WIFI_PD_R = crate::BitReader;
#[doc = "Field `FORCE_HP_WIFI_PD` writer - need_des"]
pub type FORCE_HP_WIFI_PD_W<'a, const O: u8> = crate::BitWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, O>;
#[doc = "Field `PD_HP_WIFI_MASK` reader - need_des"]
pub type PD_HP_WIFI_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_WIFI_MASK` writer - need_des"]
pub type PD_HP_WIFI_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, 5, O>;
#[doc = "Field `PD_HP_WIFI_PD_MASK` reader - need_des"]
pub type PD_HP_WIFI_PD_MASK_R = crate::FieldReader;
#[doc = "Field `PD_HP_WIFI_PD_MASK` writer - need_des"]
pub type PD_HP_WIFI_PD_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, POWER_PD_HPWIFI_CNTL_SPEC, 5, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_wifi_reset(&self) -> FORCE_HP_WIFI_RESET_R {
        FORCE_HP_WIFI_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_wifi_iso(&self) -> FORCE_HP_WIFI_ISO_R {
        FORCE_HP_WIFI_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_hp_wifi_pu(&self) -> FORCE_HP_WIFI_PU_R {
        FORCE_HP_WIFI_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn force_hp_wifi_no_reset(&self) -> FORCE_HP_WIFI_NO_RESET_R {
        FORCE_HP_WIFI_NO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn force_hp_wifi_no_iso(&self) -> FORCE_HP_WIFI_NO_ISO_R {
        FORCE_HP_WIFI_NO_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn force_hp_wifi_pd(&self) -> FORCE_HP_WIFI_PD_R {
        FORCE_HP_WIFI_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - need_des"]
    #[inline(always)]
    pub fn pd_hp_wifi_mask(&self) -> PD_HP_WIFI_MASK_R {
        PD_HP_WIFI_MASK_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_hp_wifi_pd_mask(&self) -> PD_HP_WIFI_PD_MASK_R {
        PD_HP_WIFI_PD_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_HPWIFI_CNTL")
            .field(
                "force_hp_wifi_reset",
                &format_args!("{}", self.force_hp_wifi_reset().bit()),
            )
            .field(
                "force_hp_wifi_iso",
                &format_args!("{}", self.force_hp_wifi_iso().bit()),
            )
            .field(
                "force_hp_wifi_pu",
                &format_args!("{}", self.force_hp_wifi_pu().bit()),
            )
            .field(
                "force_hp_wifi_no_reset",
                &format_args!("{}", self.force_hp_wifi_no_reset().bit()),
            )
            .field(
                "force_hp_wifi_no_iso",
                &format_args!("{}", self.force_hp_wifi_no_iso().bit()),
            )
            .field(
                "force_hp_wifi_pd",
                &format_args!("{}", self.force_hp_wifi_pd().bit()),
            )
            .field(
                "pd_hp_wifi_mask",
                &format_args!("{}", self.pd_hp_wifi_mask().bits()),
            )
            .field(
                "pd_hp_wifi_pd_mask",
                &format_args!("{}", self.pd_hp_wifi_pd_mask().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_PD_HPWIFI_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_wifi_reset(&mut self) -> FORCE_HP_WIFI_RESET_W<0> {
        FORCE_HP_WIFI_RESET_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_wifi_iso(&mut self) -> FORCE_HP_WIFI_ISO_W<1> {
        FORCE_HP_WIFI_ISO_W::new(self)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_wifi_pu(&mut self) -> FORCE_HP_WIFI_PU_W<2> {
        FORCE_HP_WIFI_PU_W::new(self)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_wifi_no_reset(&mut self) -> FORCE_HP_WIFI_NO_RESET_W<3> {
        FORCE_HP_WIFI_NO_RESET_W::new(self)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_wifi_no_iso(&mut self) -> FORCE_HP_WIFI_NO_ISO_W<4> {
        FORCE_HP_WIFI_NO_ISO_W::new(self)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_wifi_pd(&mut self) -> FORCE_HP_WIFI_PD_W<5> {
        FORCE_HP_WIFI_PD_W::new(self)
    }
    #[doc = "Bits 6:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_wifi_mask(&mut self) -> PD_HP_WIFI_MASK_W<6> {
        PD_HP_WIFI_MASK_W::new(self)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_hp_wifi_pd_mask(&mut self) -> PD_HP_WIFI_PD_MASK_W<27> {
        PD_HP_WIFI_PD_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_pd_hpwifi_cntl](index.html) module"]
pub struct POWER_PD_HPWIFI_CNTL_SPEC;
impl crate::RegisterSpec for POWER_PD_HPWIFI_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_pd_hpwifi_cntl::R](R) reader structure"]
impl crate::Readable for POWER_PD_HPWIFI_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_pd_hpwifi_cntl::W](W) writer structure"]
impl crate::Writable for POWER_PD_HPWIFI_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_PD_HPWIFI_CNTL to value 0x1c"]
impl crate::Resettable for POWER_PD_HPWIFI_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c;
}
