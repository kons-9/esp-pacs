#[doc = "Register `GPIO%s` reader"]
pub struct R(crate::R<GPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO%s` writer"]
pub struct W(crate::W<GPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_SPEC>;
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
impl From<crate::W<GPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCU_OE` reader - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
pub type MCU_OE_R = crate::BitReader;
#[doc = "Field `MCU_OE` writer - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
pub type MCU_OE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `SLP_SEL` reader - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
pub type SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `MCU_WPD` reader - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type MCU_WPD_R = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type MCU_WPD_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `MCU_WPU` reader - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type MCU_WPU_R = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type MCU_WPU_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `MCU_IE` reader - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
pub type MCU_IE_R = crate::BitReader;
#[doc = "Field `MCU_IE` writer - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
pub type MCU_IE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `MCU_DRV` reader - Select the drive strength of the pad during sleep mode. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
pub type MCU_DRV_R = crate::FieldReader;
#[doc = "Field `MCU_DRV` writer - Select the drive strength of the pad during sleep mode. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
pub type MCU_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_SPEC, 2, O>;
#[doc = "Field `FUN_WPD` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type FUN_WPD_R = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type FUN_WPD_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `FUN_WPU` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type FUN_WPU_R = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type FUN_WPU_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `FUN_IE` reader - Input enable of the pad. 1: input enabled. 0: input disabled."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable of the pad. 1: input enabled. 0: input disabled."]
pub type FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `FUN_DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
pub type FUN_DRV_R = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
pub type FUN_DRV_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_SPEC, 2, O>;
#[doc = "Field `MCU_SEL` reader - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
pub type MCU_SEL_R = crate::FieldReader;
#[doc = "Field `MCU_SEL` writer - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
pub type MCU_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_SPEC, 3, O>;
#[doc = "Field `FILTER_EN` reader - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `HYS_EN` reader - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
pub type HYS_EN_R = crate::BitReader;
#[doc = "Field `HYS_EN` writer - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
pub type HYS_EN_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
#[doc = "Field `HYS_SEL` reader - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
pub type HYS_SEL_R = crate::BitReader;
#[doc = "Field `HYS_SEL` writer - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
pub type HYS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
    #[inline(always)]
    pub fn hys_en(&self) -> HYS_EN_R {
        HYS_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
    #[inline(always)]
    pub fn hys_sel(&self) -> HYS_SEL_R {
        HYS_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO")
            .field("mcu_oe", &format_args!("{}", self.mcu_oe().bit()))
            .field("slp_sel", &format_args!("{}", self.slp_sel().bit()))
            .field("mcu_wpd", &format_args!("{}", self.mcu_wpd().bit()))
            .field("mcu_wpu", &format_args!("{}", self.mcu_wpu().bit()))
            .field("mcu_ie", &format_args!("{}", self.mcu_ie().bit()))
            .field("mcu_drv", &format_args!("{}", self.mcu_drv().bits()))
            .field("fun_wpd", &format_args!("{}", self.fun_wpd().bit()))
            .field("fun_wpu", &format_args!("{}", self.fun_wpu().bit()))
            .field("fun_ie", &format_args!("{}", self.fun_ie().bit()))
            .field("fun_drv", &format_args!("{}", self.fun_drv().bits()))
            .field("mcu_sel", &format_args!("{}", self.mcu_sel().bits()))
            .field("filter_en", &format_args!("{}", self.filter_en().bit()))
            .field("hys_en", &format_args!("{}", self.hys_en().bit()))
            .field("hys_sel", &format_args!("{}", self.hys_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<0> {
        MCU_OE_W::new(self)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<1> {
        SLP_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<2> {
        MCU_WPD_W::new(self)
    }
    #[doc = "Bit 3 - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<3> {
        MCU_WPU_W::new(self)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<4> {
        MCU_IE_W::new(self)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W<5> {
        MCU_DRV_W::new(self)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<7> {
        FUN_WPD_W::new(self)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<8> {
        FUN_WPU_W::new(self)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<9> {
        FUN_IE_W::new(self)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10mA. 2: ~20mA. 3: ~40mA."]
    #[inline(always)]
    #[must_use]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<10> {
        FUN_DRV_W::new(self)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<12> {
        MCU_SEL_W::new(self)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<15> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bit 16 - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
    #[inline(always)]
    #[must_use]
    pub fn hys_en(&mut self) -> HYS_EN_W<16> {
        HYS_EN_W::new(self)
    }
    #[doc = "Bit 17 - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
    #[inline(always)]
    #[must_use]
    pub fn hys_sel(&mut self) -> HYS_SEL_W<17> {
        HYS_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO MUX Configure Register for pad GPIO0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio](index.html) module"]
pub struct GPIO_SPEC;
impl crate::RegisterSpec for GPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio::R](R) reader structure"]
impl crate::Readable for GPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio::W](W) writer structure"]
impl crate::Writable for GPIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO%s to value 0x0800"]
impl crate::Resettable for GPIO_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
