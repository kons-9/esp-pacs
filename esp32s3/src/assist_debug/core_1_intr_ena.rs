#[doc = "Register `CORE_1_INTR_ENA` reader"]
pub struct R(crate::R<CORE_1_INTR_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_INTR_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_INTR_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_INTR_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_INTR_ENA` writer"]
pub struct W(crate::W<CORE_1_INTR_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_INTR_ENA_SPEC>;
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
impl From<crate::W<CORE_1_INTR_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_INTR_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_INTR_ENA` reader - Core1 dram0 area0 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_INTR_ENA` writer - Core1 dram0 area0 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_RD_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_INTR_ENA` reader - Core1 dram0 area0 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_INTR_ENA` writer - Core1 dram0 area0 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_0_WR_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_INTR_ENA` reader - Core1 dram0 area1 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_INTR_ENA` writer - Core1 dram0 area1 read monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_RD_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_INTR_ENA` reader - Core1 dram0 area1 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_INTR_ENA` writer - Core1 dram0 area1 write monitor interrupt enable"]
pub type CORE_1_AREA_DRAM0_1_WR_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_INTR_ENA` reader - Core1 PIF area0 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_INTR_ENA` writer - Core1 PIF area0 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_RD_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_INTR_ENA` reader - Core1 PIF area0 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_INTR_ENA` writer - Core1 PIF area0 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_0_WR_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_INTR_ENA` reader - Core1 PIF area1 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_RD_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_INTR_ENA` writer - Core1 PIF area1 read monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_RD_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_INTR_ENA` reader - Core1 PIF area1 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_WR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_INTR_ENA` writer - Core1 PIF area1 write monitor interrupt enable"]
pub type CORE_1_AREA_PIF_1_WR_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_SP_SPILL_MIN_INTR_ENA` reader - Core1 stackpoint overflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MIN_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MIN_INTR_ENA` writer - Core1 stackpoint overflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MIN_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_SP_SPILL_MAX_INTR_ENA` reader - Core1 stackpoint underflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MAX_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MAX_INTR_ENA` writer - Core1 stackpoint underflow monitor interrupt enable"]
pub type CORE_1_SP_SPILL_MAX_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA` reader - IBUS busy monitor interrupt enable"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA` writer - IBUS busy monitor interrupt enable"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA` reader - DBUS busy monitor interrupt enbale"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA` writer - DBUS busy monitor interrupt enbale"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_1_INTR_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_intr_ena(&self) -> CORE_1_AREA_DRAM0_0_RD_INTR_ENA_R {
        CORE_1_AREA_DRAM0_0_RD_INTR_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_intr_ena(&self) -> CORE_1_AREA_DRAM0_0_WR_INTR_ENA_R {
        CORE_1_AREA_DRAM0_0_WR_INTR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_intr_ena(&self) -> CORE_1_AREA_DRAM0_1_RD_INTR_ENA_R {
        CORE_1_AREA_DRAM0_1_RD_INTR_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_intr_ena(&self) -> CORE_1_AREA_DRAM0_1_WR_INTR_ENA_R {
        CORE_1_AREA_DRAM0_1_WR_INTR_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_intr_ena(&self) -> CORE_1_AREA_PIF_0_RD_INTR_ENA_R {
        CORE_1_AREA_PIF_0_RD_INTR_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_intr_ena(&self) -> CORE_1_AREA_PIF_0_WR_INTR_ENA_R {
        CORE_1_AREA_PIF_0_WR_INTR_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_intr_ena(&self) -> CORE_1_AREA_PIF_1_RD_INTR_ENA_R {
        CORE_1_AREA_PIF_1_RD_INTR_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_intr_ena(&self) -> CORE_1_AREA_PIF_1_WR_INTR_ENA_R {
        CORE_1_AREA_PIF_1_WR_INTR_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core1 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_intr_ena(&self) -> CORE_1_SP_SPILL_MIN_INTR_ENA_R {
        CORE_1_SP_SPILL_MIN_INTR_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core1 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_intr_ena(&self) -> CORE_1_SP_SPILL_MAX_INTR_ENA_R {
        CORE_1_SP_SPILL_MAX_INTR_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_intr_ena(
        &self,
    ) -> CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_R {
        CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_intr_ena(
        &self,
    ) -> CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_R {
        CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_INTR_ENA")
            .field(
                "core_1_area_dram0_0_rd_intr_ena",
                &format_args!("{}", self.core_1_area_dram0_0_rd_intr_ena().bit()),
            )
            .field(
                "core_1_area_dram0_0_wr_intr_ena",
                &format_args!("{}", self.core_1_area_dram0_0_wr_intr_ena().bit()),
            )
            .field(
                "core_1_area_dram0_1_rd_intr_ena",
                &format_args!("{}", self.core_1_area_dram0_1_rd_intr_ena().bit()),
            )
            .field(
                "core_1_area_dram0_1_wr_intr_ena",
                &format_args!("{}", self.core_1_area_dram0_1_wr_intr_ena().bit()),
            )
            .field(
                "core_1_area_pif_0_rd_intr_ena",
                &format_args!("{}", self.core_1_area_pif_0_rd_intr_ena().bit()),
            )
            .field(
                "core_1_area_pif_0_wr_intr_ena",
                &format_args!("{}", self.core_1_area_pif_0_wr_intr_ena().bit()),
            )
            .field(
                "core_1_area_pif_1_rd_intr_ena",
                &format_args!("{}", self.core_1_area_pif_1_rd_intr_ena().bit()),
            )
            .field(
                "core_1_area_pif_1_wr_intr_ena",
                &format_args!("{}", self.core_1_area_pif_1_wr_intr_ena().bit()),
            )
            .field(
                "core_1_sp_spill_min_intr_ena",
                &format_args!("{}", self.core_1_sp_spill_min_intr_ena().bit()),
            )
            .field(
                "core_1_sp_spill_max_intr_ena",
                &format_args!("{}", self.core_1_sp_spill_max_intr_ena().bit()),
            )
            .field(
                "core_1_iram0_exception_monitor_intr_ena",
                &format_args!("{}", self.core_1_iram0_exception_monitor_intr_ena().bit()),
            )
            .field(
                "core_1_dram0_exception_monitor_intr_ena",
                &format_args!("{}", self.core_1_dram0_exception_monitor_intr_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_INTR_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_rd_intr_ena(&mut self) -> CORE_1_AREA_DRAM0_0_RD_INTR_ENA_W<0> {
        CORE_1_AREA_DRAM0_0_RD_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_0_wr_intr_ena(&mut self) -> CORE_1_AREA_DRAM0_0_WR_INTR_ENA_W<1> {
        CORE_1_AREA_DRAM0_0_WR_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_1_rd_intr_ena(&mut self) -> CORE_1_AREA_DRAM0_1_RD_INTR_ENA_W<2> {
        CORE_1_AREA_DRAM0_1_RD_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_dram0_1_wr_intr_ena(&mut self) -> CORE_1_AREA_DRAM0_1_WR_INTR_ENA_W<3> {
        CORE_1_AREA_DRAM0_1_WR_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_rd_intr_ena(&mut self) -> CORE_1_AREA_PIF_0_RD_INTR_ENA_W<4> {
        CORE_1_AREA_PIF_0_RD_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_0_wr_intr_ena(&mut self) -> CORE_1_AREA_PIF_0_WR_INTR_ENA_W<5> {
        CORE_1_AREA_PIF_0_WR_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_rd_intr_ena(&mut self) -> CORE_1_AREA_PIF_1_RD_INTR_ENA_W<6> {
        CORE_1_AREA_PIF_1_RD_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_area_pif_1_wr_intr_ena(&mut self) -> CORE_1_AREA_PIF_1_WR_INTR_ENA_W<7> {
        CORE_1_AREA_PIF_1_WR_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 8 - Core1 stackpoint overflow monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_spill_min_intr_ena(&mut self) -> CORE_1_SP_SPILL_MIN_INTR_ENA_W<8> {
        CORE_1_SP_SPILL_MIN_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 9 - Core1 stackpoint underflow monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_sp_spill_max_intr_ena(&mut self) -> CORE_1_SP_SPILL_MAX_INTR_ENA_W<9> {
        CORE_1_SP_SPILL_MAX_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_iram0_exception_monitor_intr_ena(
        &mut self,
    ) -> CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_W<10> {
        CORE_1_IRAM0_EXCEPTION_MONITOR_INTR_ENA_W::new(self)
    }
    #[doc = "Bit 11 - DBUS busy monitor interrupt enbale"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_dram0_exception_monitor_intr_ena(
        &mut self,
    ) -> CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_W<11> {
        CORE_1_DRAM0_EXCEPTION_MONITOR_INTR_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core1 monitor interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_intr_ena](index.html) module"]
pub struct CORE_1_INTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_1_INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_intr_ena::R](R) reader structure"]
impl crate::Readable for CORE_1_INTR_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_intr_ena::W](W) writer structure"]
impl crate::Writable for CORE_1_INTR_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_INTR_ENA to value 0"]
impl crate::Resettable for CORE_1_INTR_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
