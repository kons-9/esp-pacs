#[doc = "Register `SPI_MEM_FSM` reader"]
pub struct R(crate::R<SPI_MEM_FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_FSM` writer"]
pub struct W(crate::W<SPI_MEM_FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_FSM_SPEC>;
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
impl From<crate::W<SPI_MEM_FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_LOCK_DELAY_TIME` reader - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type SPI_MEM_LOCK_DELAY_TIME_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_LOCK_DELAY_TIME` writer - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type SPI_MEM_LOCK_DELAY_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_FSM_SPEC, 5, O>;
impl R {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn spi_mem_lock_delay_time(&self) -> SPI_MEM_LOCK_DELAY_TIME_R {
        SPI_MEM_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_FSM")
            .field(
                "spi_mem_lock_delay_time",
                &format_args!("{}", self.spi_mem_lock_delay_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_lock_delay_time(&mut self) -> SPI_MEM_LOCK_DELAY_TIME_W<7> {
        SPI_MEM_LOCK_DELAY_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 FSM status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_fsm](index.html) module"]
pub struct SPI_MEM_FSM_SPEC;
impl crate::RegisterSpec for SPI_MEM_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_fsm::R](R) reader structure"]
impl crate::Readable for SPI_MEM_FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_fsm::W](W) writer structure"]
impl crate::Writable for SPI_MEM_FSM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_FSM to value 0x0200"]
impl crate::Resettable for SPI_MEM_FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
