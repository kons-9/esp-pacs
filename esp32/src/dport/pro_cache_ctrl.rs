#[doc = "Register `PRO_CACHE_CTRL` reader"]
pub struct R(crate::R<PRO_CACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_CTRL` writer"]
pub struct W(crate::W<PRO_CACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_CTRL_SPEC>;
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
impl From<crate::W<PRO_CACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_MODE` reader - "]
pub type PRO_CACHE_MODE_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MODE` writer - "]
pub type PRO_CACHE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_CACHE_ENABLE` reader - "]
pub type PRO_CACHE_ENABLE_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_ENABLE` writer - "]
pub type PRO_CACHE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_CACHE_FLUSH_ENA` reader - "]
pub type PRO_CACHE_FLUSH_ENA_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_FLUSH_ENA` writer - "]
pub type PRO_CACHE_FLUSH_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_CACHE_FLUSH_DONE` reader - "]
pub type PRO_CACHE_FLUSH_DONE_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_LOCK_0_EN` reader - "]
pub type PRO_CACHE_LOCK_0_EN_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_LOCK_0_EN` writer - "]
pub type PRO_CACHE_LOCK_0_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_CACHE_LOCK_1_EN` reader - "]
pub type PRO_CACHE_LOCK_1_EN_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_LOCK_1_EN` writer - "]
pub type PRO_CACHE_LOCK_1_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_CACHE_LOCK_2_EN` reader - "]
pub type PRO_CACHE_LOCK_2_EN_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_LOCK_2_EN` writer - "]
pub type PRO_CACHE_LOCK_2_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_CACHE_LOCK_3_EN` reader - "]
pub type PRO_CACHE_LOCK_3_EN_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_LOCK_3_EN` writer - "]
pub type PRO_CACHE_LOCK_3_EN_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_SINGLE_IRAM_ENA` reader - "]
pub type PRO_SINGLE_IRAM_ENA_R = crate::BitReader;
#[doc = "Field `PRO_SINGLE_IRAM_ENA` writer - "]
pub type PRO_SINGLE_IRAM_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_DRAM_SPLIT` reader - "]
pub type PRO_DRAM_SPLIT_R = crate::BitReader;
#[doc = "Field `PRO_DRAM_SPLIT` writer - "]
pub type PRO_DRAM_SPLIT_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
#[doc = "Field `PRO_AHB_SPI_REQ` reader - "]
pub type PRO_AHB_SPI_REQ_R = crate::BitReader;
#[doc = "Field `PRO_SLAVE_REQ` reader - "]
pub type PRO_SLAVE_REQ_R = crate::BitReader;
#[doc = "Field `AHB_SPI_REQ` reader - "]
pub type AHB_SPI_REQ_R = crate::BitReader;
#[doc = "Field `SLAVE_REQ` reader - "]
pub type SLAVE_REQ_R = crate::BitReader;
#[doc = "Field `PRO_DRAM_HL` reader - "]
pub type PRO_DRAM_HL_R = crate::BitReader;
#[doc = "Field `PRO_DRAM_HL` writer - "]
pub type PRO_DRAM_HL_W<'a, const O: u8> = crate::BitWriter<'a, PRO_CACHE_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pro_cache_mode(&self) -> PRO_CACHE_MODE_R {
        PRO_CACHE_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pro_cache_enable(&self) -> PRO_CACHE_ENABLE_R {
        PRO_CACHE_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cache_flush_ena(&self) -> PRO_CACHE_FLUSH_ENA_R {
        PRO_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro_cache_flush_done(&self) -> PRO_CACHE_FLUSH_DONE_R {
        PRO_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pro_cache_lock_0_en(&self) -> PRO_CACHE_LOCK_0_EN_R {
        PRO_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pro_cache_lock_1_en(&self) -> PRO_CACHE_LOCK_1_EN_R {
        PRO_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pro_cache_lock_2_en(&self) -> PRO_CACHE_LOCK_2_EN_R {
        PRO_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pro_cache_lock_3_en(&self) -> PRO_CACHE_LOCK_3_EN_R {
        PRO_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pro_single_iram_ena(&self) -> PRO_SINGLE_IRAM_ENA_R {
        PRO_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_dram_split(&self) -> PRO_DRAM_SPLIT_R {
        PRO_DRAM_SPLIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_ahb_spi_req(&self) -> PRO_AHB_SPI_REQ_R {
        PRO_AHB_SPI_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_slave_req(&self) -> PRO_SLAVE_REQ_R {
        PRO_SLAVE_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_spi_req(&self) -> AHB_SPI_REQ_R {
        AHB_SPI_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slave_req(&self) -> SLAVE_REQ_R {
        SLAVE_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pro_dram_hl(&self) -> PRO_DRAM_HL_R {
        PRO_DRAM_HL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_CTRL")
            .field(
                "pro_cache_mode",
                &format_args!("{}", self.pro_cache_mode().bit()),
            )
            .field(
                "pro_cache_enable",
                &format_args!("{}", self.pro_cache_enable().bit()),
            )
            .field(
                "pro_cache_flush_ena",
                &format_args!("{}", self.pro_cache_flush_ena().bit()),
            )
            .field(
                "pro_cache_flush_done",
                &format_args!("{}", self.pro_cache_flush_done().bit()),
            )
            .field(
                "pro_cache_lock_0_en",
                &format_args!("{}", self.pro_cache_lock_0_en().bit()),
            )
            .field(
                "pro_cache_lock_1_en",
                &format_args!("{}", self.pro_cache_lock_1_en().bit()),
            )
            .field(
                "pro_cache_lock_2_en",
                &format_args!("{}", self.pro_cache_lock_2_en().bit()),
            )
            .field(
                "pro_cache_lock_3_en",
                &format_args!("{}", self.pro_cache_lock_3_en().bit()),
            )
            .field(
                "pro_single_iram_ena",
                &format_args!("{}", self.pro_single_iram_ena().bit()),
            )
            .field(
                "pro_dram_split",
                &format_args!("{}", self.pro_dram_split().bit()),
            )
            .field(
                "pro_ahb_spi_req",
                &format_args!("{}", self.pro_ahb_spi_req().bit()),
            )
            .field(
                "pro_slave_req",
                &format_args!("{}", self.pro_slave_req().bit()),
            )
            .field("ahb_spi_req", &format_args!("{}", self.ahb_spi_req().bit()))
            .field("slave_req", &format_args!("{}", self.slave_req().bit()))
            .field("pro_dram_hl", &format_args!("{}", self.pro_dram_hl().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mode(&mut self) -> PRO_CACHE_MODE_W<2> {
        PRO_CACHE_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_enable(&mut self) -> PRO_CACHE_ENABLE_W<3> {
        PRO_CACHE_ENABLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_flush_ena(&mut self) -> PRO_CACHE_FLUSH_ENA_W<4> {
        PRO_CACHE_FLUSH_ENA_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_0_en(&mut self) -> PRO_CACHE_LOCK_0_EN_W<6> {
        PRO_CACHE_LOCK_0_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_1_en(&mut self) -> PRO_CACHE_LOCK_1_EN_W<7> {
        PRO_CACHE_LOCK_1_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_2_en(&mut self) -> PRO_CACHE_LOCK_2_EN_W<8> {
        PRO_CACHE_LOCK_2_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock_3_en(&mut self) -> PRO_CACHE_LOCK_3_EN_W<9> {
        PRO_CACHE_LOCK_3_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pro_single_iram_ena(&mut self) -> PRO_SINGLE_IRAM_ENA_W<10> {
        PRO_SINGLE_IRAM_ENA_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram_split(&mut self) -> PRO_DRAM_SPLIT_W<11> {
        PRO_DRAM_SPLIT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram_hl(&mut self) -> PRO_DRAM_HL_W<16> {
        PRO_DRAM_HL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_ctrl](index.html) module"]
pub struct PRO_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_CACHE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_CTRL to value 0x10"]
impl crate::Resettable for PRO_CACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
