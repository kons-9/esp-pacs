#[doc = "Register `CORE_0_INTR_CLR` reader"]
pub struct R(crate::R<CORE_0_INTR_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_INTR_CLR` writer"]
pub struct W(crate::W<CORE_0_INTR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTR_CLR_SPEC>;
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
impl From<crate::W<CORE_0_INTR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` reader - reg_core_0_area_dram0_0_rd_clr"]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` writer - reg_core_0_area_dram0_0_rd_clr"]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` reader - reg_core_0_area_dram0_0_wr_clr"]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` writer - reg_core_0_area_dram0_0_wr_clr"]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` reader - reg_core_0_area_dram0_1_rd_clr"]
pub type CORE_0_AREA_DRAM0_1_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_CLR` writer - reg_core_0_area_dram0_1_rd_clr"]
pub type CORE_0_AREA_DRAM0_1_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` reader - reg_core_0_area_dram0_1_wr_clr"]
pub type CORE_0_AREA_DRAM0_1_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_CLR` writer - reg_core_0_area_dram0_1_wr_clr"]
pub type CORE_0_AREA_DRAM0_1_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` reader - reg_core_0_area_pif_0_rd_clr"]
pub type CORE_0_AREA_PIF_0_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_RD_CLR` writer - reg_core_0_area_pif_0_rd_clr"]
pub type CORE_0_AREA_PIF_0_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` reader - reg_core_0_area_pif_0_wr_clr"]
pub type CORE_0_AREA_PIF_0_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_0_WR_CLR` writer - reg_core_0_area_pif_0_wr_clr"]
pub type CORE_0_AREA_PIF_0_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` reader - reg_core_0_area_pif_1_rd_clr"]
pub type CORE_0_AREA_PIF_1_RD_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_RD_CLR` writer - reg_core_0_area_pif_1_rd_clr"]
pub type CORE_0_AREA_PIF_1_RD_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` reader - reg_core_0_area_pif_1_wr_clr"]
pub type CORE_0_AREA_PIF_1_WR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_AREA_PIF_1_WR_CLR` writer - reg_core_0_area_pif_1_wr_clr"]
pub type CORE_0_AREA_PIF_1_WR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` reader - reg_core_0_sp_spill_min_clr"]
pub type CORE_0_SP_SPILL_MIN_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MIN_CLR` writer - reg_core_0_sp_spill_min_clr"]
pub type CORE_0_SP_SPILL_MIN_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` reader - reg_core_0_sp_spill_max_clr"]
pub type CORE_0_SP_SPILL_MAX_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_SP_SPILL_MAX_CLR` writer - reg_core_0_sp_spill_max_clr"]
pub type CORE_0_SP_SPILL_MAX_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` reader - reg_core_0_iram0_exception_monitor_clr"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_CLR` writer - reg_core_0_iram0_exception_monitor_clr"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` reader - reg_core_0_dram0_exception_monitor_clr"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R = crate::BitReader;
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_CLR` writer - reg_core_0_dram0_exception_monitor_clr"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_0_INTR_CLR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - reg_core_0_area_dram0_0_rd_clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_clr(&self) -> CORE_0_AREA_DRAM0_0_RD_CLR_R {
        CORE_0_AREA_DRAM0_0_RD_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_core_0_area_dram0_0_wr_clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_clr(&self) -> CORE_0_AREA_DRAM0_0_WR_CLR_R {
        CORE_0_AREA_DRAM0_0_WR_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_core_0_area_dram0_1_rd_clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_clr(&self) -> CORE_0_AREA_DRAM0_1_RD_CLR_R {
        CORE_0_AREA_DRAM0_1_RD_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_core_0_area_dram0_1_wr_clr"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_clr(&self) -> CORE_0_AREA_DRAM0_1_WR_CLR_R {
        CORE_0_AREA_DRAM0_1_WR_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_core_0_area_pif_0_rd_clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_clr(&self) -> CORE_0_AREA_PIF_0_RD_CLR_R {
        CORE_0_AREA_PIF_0_RD_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_core_0_area_pif_0_wr_clr"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_clr(&self) -> CORE_0_AREA_PIF_0_WR_CLR_R {
        CORE_0_AREA_PIF_0_WR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_core_0_area_pif_1_rd_clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_clr(&self) -> CORE_0_AREA_PIF_1_RD_CLR_R {
        CORE_0_AREA_PIF_1_RD_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_core_0_area_pif_1_wr_clr"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_clr(&self) -> CORE_0_AREA_PIF_1_WR_CLR_R {
        CORE_0_AREA_PIF_1_WR_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_core_0_sp_spill_min_clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_clr(&self) -> CORE_0_SP_SPILL_MIN_CLR_R {
        CORE_0_SP_SPILL_MIN_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_core_0_sp_spill_max_clr"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_clr(&self) -> CORE_0_SP_SPILL_MAX_CLR_R {
        CORE_0_SP_SPILL_MAX_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_core_0_iram0_exception_monitor_clr"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_clr(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_core_0_dram0_exception_monitor_clr"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_clr(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_CLR")
            .field(
                "core_0_area_dram0_0_rd_clr",
                &format_args!("{}", self.core_0_area_dram0_0_rd_clr().bit()),
            )
            .field(
                "core_0_area_dram0_0_wr_clr",
                &format_args!("{}", self.core_0_area_dram0_0_wr_clr().bit()),
            )
            .field(
                "core_0_area_dram0_1_rd_clr",
                &format_args!("{}", self.core_0_area_dram0_1_rd_clr().bit()),
            )
            .field(
                "core_0_area_dram0_1_wr_clr",
                &format_args!("{}", self.core_0_area_dram0_1_wr_clr().bit()),
            )
            .field(
                "core_0_area_pif_0_rd_clr",
                &format_args!("{}", self.core_0_area_pif_0_rd_clr().bit()),
            )
            .field(
                "core_0_area_pif_0_wr_clr",
                &format_args!("{}", self.core_0_area_pif_0_wr_clr().bit()),
            )
            .field(
                "core_0_area_pif_1_rd_clr",
                &format_args!("{}", self.core_0_area_pif_1_rd_clr().bit()),
            )
            .field(
                "core_0_area_pif_1_wr_clr",
                &format_args!("{}", self.core_0_area_pif_1_wr_clr().bit()),
            )
            .field(
                "core_0_sp_spill_min_clr",
                &format_args!("{}", self.core_0_sp_spill_min_clr().bit()),
            )
            .field(
                "core_0_sp_spill_max_clr",
                &format_args!("{}", self.core_0_sp_spill_max_clr().bit()),
            )
            .field(
                "core_0_iram0_exception_monitor_clr",
                &format_args!("{}", self.core_0_iram0_exception_monitor_clr().bit()),
            )
            .field(
                "core_0_dram0_exception_monitor_clr",
                &format_args!("{}", self.core_0_dram0_exception_monitor_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_core_0_area_dram0_0_rd_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_0_RD_CLR_W<0> {
        CORE_0_AREA_DRAM0_0_RD_CLR_W::new(self)
    }
    #[doc = "Bit 1 - reg_core_0_area_dram0_0_wr_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_0_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_0_WR_CLR_W<1> {
        CORE_0_AREA_DRAM0_0_WR_CLR_W::new(self)
    }
    #[doc = "Bit 2 - reg_core_0_area_dram0_1_rd_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_1_RD_CLR_W<2> {
        CORE_0_AREA_DRAM0_1_RD_CLR_W::new(self)
    }
    #[doc = "Bit 3 - reg_core_0_area_dram0_1_wr_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_dram0_1_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_1_WR_CLR_W<3> {
        CORE_0_AREA_DRAM0_1_WR_CLR_W::new(self)
    }
    #[doc = "Bit 4 - reg_core_0_area_pif_0_rd_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_rd_clr(&mut self) -> CORE_0_AREA_PIF_0_RD_CLR_W<4> {
        CORE_0_AREA_PIF_0_RD_CLR_W::new(self)
    }
    #[doc = "Bit 5 - reg_core_0_area_pif_0_wr_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_0_wr_clr(&mut self) -> CORE_0_AREA_PIF_0_WR_CLR_W<5> {
        CORE_0_AREA_PIF_0_WR_CLR_W::new(self)
    }
    #[doc = "Bit 6 - reg_core_0_area_pif_1_rd_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_rd_clr(&mut self) -> CORE_0_AREA_PIF_1_RD_CLR_W<6> {
        CORE_0_AREA_PIF_1_RD_CLR_W::new(self)
    }
    #[doc = "Bit 7 - reg_core_0_area_pif_1_wr_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_area_pif_1_wr_clr(&mut self) -> CORE_0_AREA_PIF_1_WR_CLR_W<7> {
        CORE_0_AREA_PIF_1_WR_CLR_W::new(self)
    }
    #[doc = "Bit 8 - reg_core_0_sp_spill_min_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_min_clr(&mut self) -> CORE_0_SP_SPILL_MIN_CLR_W<8> {
        CORE_0_SP_SPILL_MIN_CLR_W::new(self)
    }
    #[doc = "Bit 9 - reg_core_0_sp_spill_max_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_sp_spill_max_clr(&mut self) -> CORE_0_SP_SPILL_MAX_CLR_W<9> {
        CORE_0_SP_SPILL_MAX_CLR_W::new(self)
    }
    #[doc = "Bit 10 - reg_core_0_iram0_exception_monitor_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_iram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W<10> {
        CORE_0_IRAM0_EXCEPTION_MONITOR_CLR_W::new(self)
    }
    #[doc = "Bit 11 - reg_core_0_dram0_exception_monitor_clr"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_dram0_exception_monitor_clr(
        &mut self,
    ) -> CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W<11> {
        CORE_0_DRAM0_EXCEPTION_MONITOR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_clr](index.html) module"]
pub struct CORE_0_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_clr::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_intr_clr::W](W) writer structure"]
impl crate::Writable for CORE_0_INTR_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_INTR_CLR to value 0"]
impl crate::Resettable for CORE_0_INTR_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
