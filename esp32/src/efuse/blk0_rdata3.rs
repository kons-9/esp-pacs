#[doc = "Register `BLK0_RDATA3` reader"]
pub struct R(crate::R<BLK0_RDATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_RDATA3` writer"]
pub struct W(crate::W<BLK0_RDATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_RDATA3_SPEC>;
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
impl From<crate::W<BLK0_RDATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_RDATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_DISABLE_APP_CPU` reader - "]
pub type RD_DISABLE_APP_CPU_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_BT` reader - "]
pub type RD_DISABLE_BT_R = crate::BitReader;
#[doc = "Field `RD_CHIP_PACKAGE_4BIT` reader - "]
pub type RD_CHIP_PACKAGE_4BIT_R = crate::BitReader;
#[doc = "Field `RD_DIS_CACHE` reader - "]
pub type RD_DIS_CACHE_R = crate::BitReader;
#[doc = "Field `RD_SPI_PAD_CONFIG_HD` reader - "]
pub type RD_SPI_PAD_CONFIG_HD_R = crate::FieldReader;
#[doc = "Field `RD_CHIP_PACKAGE` reader - "]
pub type RD_CHIP_PACKAGE_R = crate::FieldReader;
#[doc = "Field `RD_CHIP_PACKAGE` writer - "]
pub type RD_CHIP_PACKAGE_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_RDATA3_SPEC, 3, O>;
#[doc = "Field `RD_CHIP_CPU_FREQ_LOW` reader - "]
pub type RD_CHIP_CPU_FREQ_LOW_R = crate::BitReader;
#[doc = "Field `RD_CHIP_CPU_FREQ_LOW` writer - "]
pub type RD_CHIP_CPU_FREQ_LOW_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_RDATA3_SPEC, O>;
#[doc = "Field `RD_CHIP_CPU_FREQ_RATED` reader - "]
pub type RD_CHIP_CPU_FREQ_RATED_R = crate::BitReader;
#[doc = "Field `RD_CHIP_CPU_FREQ_RATED` writer - "]
pub type RD_CHIP_CPU_FREQ_RATED_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_RDATA3_SPEC, O>;
#[doc = "Field `RD_BLK3_PART_RESERVE` reader - "]
pub type RD_BLK3_PART_RESERVE_R = crate::BitReader;
#[doc = "Field `RD_BLK3_PART_RESERVE` writer - "]
pub type RD_BLK3_PART_RESERVE_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_RDATA3_SPEC, O>;
#[doc = "Field `RD_CHIP_VER_REV1` reader - "]
pub type RD_CHIP_VER_REV1_R = crate::BitReader;
#[doc = "Field `RD_CHIP_VER_REV1` writer - "]
pub type RD_CHIP_VER_REV1_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_RDATA3_SPEC, O>;
#[doc = "Field `RD_RESERVE_0_112` reader - "]
pub type RD_RESERVE_0_112_R = crate::FieldReader<u16>;
#[doc = "Field `RD_RESERVE_0_112` writer - "]
pub type RD_RESERVE_0_112_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_RDATA3_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rd_disable_app_cpu(&self) -> RD_DISABLE_APP_CPU_R {
        RD_DISABLE_APP_CPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rd_disable_bt(&self) -> RD_DISABLE_BT_R {
        RD_DISABLE_BT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rd_chip_package_4bit(&self) -> RD_CHIP_PACKAGE_4BIT_R {
        RD_CHIP_PACKAGE_4BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_dis_cache(&self) -> RD_DIS_CACHE_R {
        RD_DIS_CACHE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn rd_spi_pad_config_hd(&self) -> RD_SPI_PAD_CONFIG_HD_R {
        RD_SPI_PAD_CONFIG_HD_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn rd_chip_package(&self) -> RD_CHIP_PACKAGE_R {
        RD_CHIP_PACKAGE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rd_chip_cpu_freq_low(&self) -> RD_CHIP_CPU_FREQ_LOW_R {
        RD_CHIP_CPU_FREQ_LOW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rd_chip_cpu_freq_rated(&self) -> RD_CHIP_CPU_FREQ_RATED_R {
        RD_CHIP_CPU_FREQ_RATED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rd_blk3_part_reserve(&self) -> RD_BLK3_PART_RESERVE_R {
        RD_BLK3_PART_RESERVE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rd_chip_ver_rev1(&self) -> RD_CHIP_VER_REV1_R {
        RD_CHIP_VER_REV1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rd_reserve_0_112(&self) -> RD_RESERVE_0_112_R {
        RD_RESERVE_0_112_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA3")
            .field(
                "rd_disable_app_cpu",
                &format_args!("{}", self.rd_disable_app_cpu().bit()),
            )
            .field(
                "rd_disable_bt",
                &format_args!("{}", self.rd_disable_bt().bit()),
            )
            .field(
                "rd_chip_package_4bit",
                &format_args!("{}", self.rd_chip_package_4bit().bit()),
            )
            .field(
                "rd_dis_cache",
                &format_args!("{}", self.rd_dis_cache().bit()),
            )
            .field(
                "rd_spi_pad_config_hd",
                &format_args!("{}", self.rd_spi_pad_config_hd().bits()),
            )
            .field(
                "rd_chip_package",
                &format_args!("{}", self.rd_chip_package().bits()),
            )
            .field(
                "rd_chip_cpu_freq_low",
                &format_args!("{}", self.rd_chip_cpu_freq_low().bit()),
            )
            .field(
                "rd_chip_cpu_freq_rated",
                &format_args!("{}", self.rd_chip_cpu_freq_rated().bit()),
            )
            .field(
                "rd_blk3_part_reserve",
                &format_args!("{}", self.rd_blk3_part_reserve().bit()),
            )
            .field(
                "rd_chip_ver_rev1",
                &format_args!("{}", self.rd_chip_ver_rev1().bit()),
            )
            .field(
                "rd_reserve_0_112",
                &format_args!("{}", self.rd_reserve_0_112().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn rd_chip_package(&mut self) -> RD_CHIP_PACKAGE_W<9> {
        RD_CHIP_PACKAGE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rd_chip_cpu_freq_low(&mut self) -> RD_CHIP_CPU_FREQ_LOW_W<12> {
        RD_CHIP_CPU_FREQ_LOW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn rd_chip_cpu_freq_rated(&mut self) -> RD_CHIP_CPU_FREQ_RATED_W<13> {
        RD_CHIP_CPU_FREQ_RATED_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn rd_blk3_part_reserve(&mut self) -> RD_BLK3_PART_RESERVE_W<14> {
        RD_BLK3_PART_RESERVE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rd_chip_ver_rev1(&mut self) -> RD_CHIP_VER_REV1_W<15> {
        RD_CHIP_VER_REV1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_112(&mut self) -> RD_RESERVE_0_112_W<16> {
        RD_RESERVE_0_112_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata3](index.html) module"]
pub struct BLK0_RDATA3_SPEC;
impl crate::RegisterSpec for BLK0_RDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata3::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_rdata3::W](W) writer structure"]
impl crate::Writable for BLK0_RDATA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_RDATA3 to value 0"]
impl crate::Resettable for BLK0_RDATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
