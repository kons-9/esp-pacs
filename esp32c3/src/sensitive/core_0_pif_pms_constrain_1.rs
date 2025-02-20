#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART` reader - core_0_pif_pms_constrain_world_0_uart"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART` writer - core_0_pif_pms_constrain_world_0_uart"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1` reader - core_0_pif_pms_constrain_world_0_g0spi_1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1` writer - core_0_pif_pms_constrain_world_0_g0spi_1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0` reader - core_0_pif_pms_constrain_world_0_g0spi_0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0` writer - core_0_pif_pms_constrain_world_0_g0spi_0"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO` reader - core_0_pif_pms_constrain_world_0_gpio"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO` writer - core_0_pif_pms_constrain_world_0_gpio"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2` reader - core_0_pif_pms_constrain_world_0_fe2"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2` writer - core_0_pif_pms_constrain_world_0_fe2"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE` reader - core_0_pif_pms_constrain_world_0_fe"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE` writer - core_0_pif_pms_constrain_world_0_fe"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER` reader - core_0_pif_pms_constrain_world_0_timer"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER` writer - core_0_pif_pms_constrain_world_0_timer"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC` reader - core_0_pif_pms_constrain_world_0_rtc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC` writer - core_0_pif_pms_constrain_world_0_rtc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX` reader - core_0_pif_pms_constrain_world_0_io_mux"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX` writer - core_0_pif_pms_constrain_world_0_io_mux"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG` reader - core_0_pif_pms_constrain_world_0_wdg"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG` writer - core_0_pif_pms_constrain_world_0_wdg"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC` reader - core_0_pif_pms_constrain_world_0_misc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC` writer - core_0_pif_pms_constrain_world_0_misc"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C` reader - core_0_pif_pms_constrain_world_0_i2c"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C` writer - core_0_pif_pms_constrain_world_0_i2c"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1` reader - core_0_pif_pms_constrain_world_0_uart1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R = crate::FieldReader;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1` writer - core_0_pif_pms_constrain_world_0_uart1"]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_uart"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_0_g0spi_1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_g0spi_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_0_gpio"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_gpio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_0_fe2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe2(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_fe"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_0_timer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_0_rtc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rtc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_0_io_mux"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_io_mux(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_0_wdg"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_wdg(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_0_misc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_misc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_i2c"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_0_uart1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_PIF_PMS_CONSTRAIN_1")
            .field(
                "core_0_pif_pms_constrain_world_0_uart",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_uart().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_g0spi_1",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_g0spi_1().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_g0spi_0",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_g0spi_0().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_gpio",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_gpio().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_fe2",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_fe2().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_fe",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_fe().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_timer",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_timer().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_rtc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_rtc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_io_mux",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_io_mux().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_wdg",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_wdg().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_misc",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_misc().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_i2c",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_i2c().bits()),
            )
            .field(
                "core_0_pif_pms_constrain_world_0_uart1",
                &format_args!("{}", self.core_0_pif_pms_constrain_world_0_uart1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_uart"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_uart(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W::new(self)
    }
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_0_g0spi_1"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W<2> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W::new(self)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_g0spi_0"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W::new(self)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_0_gpio"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_gpio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W::new(self)
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_0_fe2"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_fe2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W::new(self)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_fe"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_fe(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W::new(self)
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_0_timer"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_timer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W<12> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W::new(self)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_0_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_rtc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W::new(self)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_0_io_mux"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_io_mux(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W::new(self)
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_0_wdg"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_wdg(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W<18> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W::new(self)
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_0_misc"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_misc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W<24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W::new(self)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_i2c"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_i2c(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W::new(self)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_0_uart1"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_uart1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W<30> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_1](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_1 to value 0xcf0f_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xcf0f_ffff;
}
