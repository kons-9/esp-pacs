#[doc = "Register `DIG_PWC` reader"]
pub struct R(crate::R<DIG_PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_PWC` writer"]
pub struct W(crate::W<DIG_PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_PWC_SPEC>;
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
impl From<crate::W<DIG_PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSLP_MEM_FORCE_PD` reader - Set this bit to FPD the memories in the digital system in sleep."]
pub type LSLP_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PD` writer - Set this bit to FPD the memories in the digital system in sleep."]
pub type LSLP_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `LSLP_MEM_FORCE_PU` reader - Set this bit to FPU the memories in the digital system."]
pub type LSLP_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_FORCE_PU` writer - Set this bit to FPU the memories in the digital system."]
pub type LSLP_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `ROM0_FORCE_PD` reader - ROM force power down"]
pub type ROM0_FORCE_PD_R = crate::BitReader;
#[doc = "Field `ROM0_FORCE_PD` writer - ROM force power down"]
pub type ROM0_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `ROM0_FORCE_PU` reader - ROM force power up"]
pub type ROM0_FORCE_PU_R = crate::BitReader;
#[doc = "Field `ROM0_FORCE_PU` writer - ROM force power up"]
pub type ROM0_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM0_FORCE_PD` reader - internal SRAM 0 force power down"]
pub type INTER_RAM0_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_FORCE_PD` writer - internal SRAM 0 force power down"]
pub type INTER_RAM0_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM0_FORCE_PU` reader - internal SRAM 0 force power up"]
pub type INTER_RAM0_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_FORCE_PU` writer - internal SRAM 0 force power up"]
pub type INTER_RAM0_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM1_FORCE_PD` reader - internal SRAM 1 force power down"]
pub type INTER_RAM1_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_FORCE_PD` writer - internal SRAM 1 force power down"]
pub type INTER_RAM1_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM1_FORCE_PU` reader - internal SRAM 1 force power up"]
pub type INTER_RAM1_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_FORCE_PU` writer - internal SRAM 1 force power up"]
pub type INTER_RAM1_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM2_FORCE_PD` reader - internal SRAM 2 force power down"]
pub type INTER_RAM2_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_FORCE_PD` writer - internal SRAM 2 force power down"]
pub type INTER_RAM2_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM2_FORCE_PU` reader - internal SRAM 2 force power up"]
pub type INTER_RAM2_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_FORCE_PU` writer - internal SRAM 2 force power up"]
pub type INTER_RAM2_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM3_FORCE_PD` reader - internal SRAM 3 force power down"]
pub type INTER_RAM3_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_FORCE_PD` writer - internal SRAM 3 force power down"]
pub type INTER_RAM3_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM3_FORCE_PU` reader - internal SRAM 3 force power up"]
pub type INTER_RAM3_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_FORCE_PU` writer - internal SRAM 3 force power up"]
pub type INTER_RAM3_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM4_FORCE_PD` reader - internal SRAM 4 force power down"]
pub type INTER_RAM4_FORCE_PD_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_FORCE_PD` writer - internal SRAM 4 force power down"]
pub type INTER_RAM4_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM4_FORCE_PU` reader - internal SRAM 4 force power up"]
pub type INTER_RAM4_FORCE_PU_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_FORCE_PU` writer - internal SRAM 4 force power up"]
pub type INTER_RAM4_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `WIFI_FORCE_PD` reader - Set this bit to FPD the Wi-Fi circuit."]
pub type WIFI_FORCE_PD_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_PD` writer - Set this bit to FPD the Wi-Fi circuit."]
pub type WIFI_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `WIFI_FORCE_PU` reader - Set this bit to FPU the Wi-Fi circuit."]
pub type WIFI_FORCE_PU_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_PU` writer - Set this bit to FPU the Wi-Fi circuit."]
pub type WIFI_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_PD` reader - Set this bit to FPD the digital system."]
pub type DG_WRAP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PD` writer - Set this bit to FPD the digital system."]
pub type DG_WRAP_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_PU` reader - Set this bit to FPD the DC-DC convertor in the digital system."]
pub type DG_WRAP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_PU` writer - Set this bit to FPD the DC-DC convertor in the digital system."]
pub type DG_WRAP_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_DCDC_FORCE_PD` reader - Set this bit to FPD the DC-DC convertor in the digital system."]
pub type DG_DCDC_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DG_DCDC_FORCE_PD` writer - Set this bit to FPD the DC-DC convertor in the digital system."]
pub type DG_DCDC_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_DCDC_FORCE_PU` reader - Set this bit to FPU the DC-DC convertor in the digital system."]
pub type DG_DCDC_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DG_DCDC_FORCE_PU` writer - Set this bit to FPU the DC-DC convertor in the digital system."]
pub type DG_DCDC_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_DCDC_PD_EN` reader - Set this bit to enable PD for the DC-DC convertor in the digital system."]
pub type DG_DCDC_PD_EN_R = crate::BitReader;
#[doc = "Field `DG_DCDC_PD_EN` writer - Set this bit to enable PD for the DC-DC convertor in the digital system."]
pub type DG_DCDC_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `ROM0_PD_EN` reader - enable power down ROM in sleep"]
pub type ROM0_PD_EN_R = crate::BitReader;
#[doc = "Field `ROM0_PD_EN` writer - enable power down ROM in sleep"]
pub type ROM0_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM0_PD_EN` reader - enable power down internal SRAM 0 in sleep"]
pub type INTER_RAM0_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_PD_EN` writer - enable power down internal SRAM 0 in sleep"]
pub type INTER_RAM0_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM1_PD_EN` reader - enable power down internal SRAM 1 in sleep"]
pub type INTER_RAM1_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_PD_EN` writer - enable power down internal SRAM 1 in sleep"]
pub type INTER_RAM1_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM2_PD_EN` reader - enable power down internal SRAM 2 in sleep"]
pub type INTER_RAM2_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_PD_EN` writer - enable power down internal SRAM 2 in sleep"]
pub type INTER_RAM2_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM3_PD_EN` reader - enable power down internal SRAM 3 in sleep"]
pub type INTER_RAM3_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_PD_EN` writer - enable power down internal SRAM 3 in sleep"]
pub type INTER_RAM3_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `INTER_RAM4_PD_EN` reader - enable power down internal SRAM 4 in sleep"]
pub type INTER_RAM4_PD_EN_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_PD_EN` writer - enable power down internal SRAM 4 in sleep"]
pub type INTER_RAM4_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `WIFI_PD_EN` reader - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
pub type WIFI_PD_EN_R = crate::BitReader;
#[doc = "Field `WIFI_PD_EN` writer - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
pub type WIFI_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
#[doc = "Field `DG_WRAP_PD_EN` reader - Set this bit to enable PD for the digital system in sleep."]
pub type DG_WRAP_PD_EN_R = crate::BitReader;
#[doc = "Field `DG_WRAP_PD_EN` writer - Set this bit to enable PD for the digital system in sleep."]
pub type DG_WRAP_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_PWC_SPEC, O>;
impl R {
    #[doc = "Bit 3 - Set this bit to FPD the memories in the digital system in sleep."]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to FPU the memories in the digital system."]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    pub fn rom0_force_pd(&self) -> ROM0_FORCE_PD_R {
        ROM0_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    pub fn rom0_force_pu(&self) -> ROM0_FORCE_PU_R {
        ROM0_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    pub fn inter_ram0_force_pd(&self) -> INTER_RAM0_FORCE_PD_R {
        INTER_RAM0_FORCE_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    pub fn inter_ram0_force_pu(&self) -> INTER_RAM0_FORCE_PU_R {
        INTER_RAM0_FORCE_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    pub fn inter_ram1_force_pd(&self) -> INTER_RAM1_FORCE_PD_R {
        INTER_RAM1_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    pub fn inter_ram1_force_pu(&self) -> INTER_RAM1_FORCE_PU_R {
        INTER_RAM1_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    pub fn inter_ram2_force_pd(&self) -> INTER_RAM2_FORCE_PD_R {
        INTER_RAM2_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    pub fn inter_ram2_force_pu(&self) -> INTER_RAM2_FORCE_PU_R {
        INTER_RAM2_FORCE_PU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    pub fn inter_ram3_force_pd(&self) -> INTER_RAM3_FORCE_PD_R {
        INTER_RAM3_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    pub fn inter_ram3_force_pu(&self) -> INTER_RAM3_FORCE_PU_R {
        INTER_RAM3_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    pub fn inter_ram4_force_pd(&self) -> INTER_RAM4_FORCE_PD_R {
        INTER_RAM4_FORCE_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    pub fn inter_ram4_force_pu(&self) -> INTER_RAM4_FORCE_PU_R {
        INTER_RAM4_FORCE_PU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to FPD the Wi-Fi circuit."]
    #[inline(always)]
    pub fn wifi_force_pd(&self) -> WIFI_FORCE_PD_R {
        WIFI_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to FPU the Wi-Fi circuit."]
    #[inline(always)]
    pub fn wifi_force_pu(&self) -> WIFI_FORCE_PU_R {
        WIFI_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to FPD the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_force_pd(&self) -> DG_DCDC_FORCE_PD_R {
        DG_DCDC_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to FPU the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_force_pu(&self) -> DG_DCDC_FORCE_PU_R {
        DG_DCDC_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable PD for the DC-DC convertor in the digital system."]
    #[inline(always)]
    pub fn dg_dcdc_pd_en(&self) -> DG_DCDC_PD_EN_R {
        DG_DCDC_PD_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    pub fn rom0_pd_en(&self) -> ROM0_PD_EN_R {
        ROM0_PD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    pub fn inter_ram0_pd_en(&self) -> INTER_RAM0_PD_EN_R {
        INTER_RAM0_PD_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    pub fn inter_ram1_pd_en(&self) -> INTER_RAM1_PD_EN_R {
        INTER_RAM1_PD_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    pub fn inter_ram2_pd_en(&self) -> INTER_RAM2_PD_EN_R {
        INTER_RAM2_PD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    pub fn inter_ram3_pd_en(&self) -> INTER_RAM3_PD_EN_R {
        INTER_RAM3_PD_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    pub fn inter_ram4_pd_en(&self) -> INTER_RAM4_PD_EN_R {
        INTER_RAM4_PD_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
    #[inline(always)]
    pub fn wifi_pd_en(&self) -> WIFI_PD_EN_R {
        WIFI_PD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable PD for the digital system in sleep."]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PWC")
            .field(
                "lslp_mem_force_pd",
                &format_args!("{}", self.lslp_mem_force_pd().bit()),
            )
            .field(
                "lslp_mem_force_pu",
                &format_args!("{}", self.lslp_mem_force_pu().bit()),
            )
            .field(
                "rom0_force_pd",
                &format_args!("{}", self.rom0_force_pd().bit()),
            )
            .field(
                "rom0_force_pu",
                &format_args!("{}", self.rom0_force_pu().bit()),
            )
            .field(
                "inter_ram0_force_pd",
                &format_args!("{}", self.inter_ram0_force_pd().bit()),
            )
            .field(
                "inter_ram0_force_pu",
                &format_args!("{}", self.inter_ram0_force_pu().bit()),
            )
            .field(
                "inter_ram1_force_pd",
                &format_args!("{}", self.inter_ram1_force_pd().bit()),
            )
            .field(
                "inter_ram1_force_pu",
                &format_args!("{}", self.inter_ram1_force_pu().bit()),
            )
            .field(
                "inter_ram2_force_pd",
                &format_args!("{}", self.inter_ram2_force_pd().bit()),
            )
            .field(
                "inter_ram2_force_pu",
                &format_args!("{}", self.inter_ram2_force_pu().bit()),
            )
            .field(
                "inter_ram3_force_pd",
                &format_args!("{}", self.inter_ram3_force_pd().bit()),
            )
            .field(
                "inter_ram3_force_pu",
                &format_args!("{}", self.inter_ram3_force_pu().bit()),
            )
            .field(
                "inter_ram4_force_pd",
                &format_args!("{}", self.inter_ram4_force_pd().bit()),
            )
            .field(
                "inter_ram4_force_pu",
                &format_args!("{}", self.inter_ram4_force_pu().bit()),
            )
            .field(
                "wifi_force_pd",
                &format_args!("{}", self.wifi_force_pd().bit()),
            )
            .field(
                "wifi_force_pu",
                &format_args!("{}", self.wifi_force_pu().bit()),
            )
            .field(
                "dg_wrap_force_pd",
                &format_args!("{}", self.dg_wrap_force_pd().bit()),
            )
            .field(
                "dg_wrap_force_pu",
                &format_args!("{}", self.dg_wrap_force_pu().bit()),
            )
            .field(
                "dg_dcdc_force_pd",
                &format_args!("{}", self.dg_dcdc_force_pd().bit()),
            )
            .field(
                "dg_dcdc_force_pu",
                &format_args!("{}", self.dg_dcdc_force_pu().bit()),
            )
            .field(
                "dg_dcdc_pd_en",
                &format_args!("{}", self.dg_dcdc_pd_en().bit()),
            )
            .field("rom0_pd_en", &format_args!("{}", self.rom0_pd_en().bit()))
            .field(
                "inter_ram0_pd_en",
                &format_args!("{}", self.inter_ram0_pd_en().bit()),
            )
            .field(
                "inter_ram1_pd_en",
                &format_args!("{}", self.inter_ram1_pd_en().bit()),
            )
            .field(
                "inter_ram2_pd_en",
                &format_args!("{}", self.inter_ram2_pd_en().bit()),
            )
            .field(
                "inter_ram3_pd_en",
                &format_args!("{}", self.inter_ram3_pd_en().bit()),
            )
            .field(
                "inter_ram4_pd_en",
                &format_args!("{}", self.inter_ram4_pd_en().bit()),
            )
            .field("wifi_pd_en", &format_args!("{}", self.wifi_pd_en().bit()))
            .field(
                "dg_wrap_pd_en",
                &format_args!("{}", self.dg_wrap_pd_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_PWC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Set this bit to FPD the memories in the digital system in sleep."]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W<3> {
        LSLP_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to FPU the memories in the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W<4> {
        LSLP_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 5 - ROM force power down"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_force_pd(&mut self) -> ROM0_FORCE_PD_W<5> {
        ROM0_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 6 - ROM force power up"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_force_pu(&mut self) -> ROM0_FORCE_PU_W<6> {
        ROM0_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 7 - internal SRAM 0 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_force_pd(&mut self) -> INTER_RAM0_FORCE_PD_W<7> {
        INTER_RAM0_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 8 - internal SRAM 0 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_force_pu(&mut self) -> INTER_RAM0_FORCE_PU_W<8> {
        INTER_RAM0_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 9 - internal SRAM 1 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_force_pd(&mut self) -> INTER_RAM1_FORCE_PD_W<9> {
        INTER_RAM1_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 10 - internal SRAM 1 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_force_pu(&mut self) -> INTER_RAM1_FORCE_PU_W<10> {
        INTER_RAM1_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 11 - internal SRAM 2 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_force_pd(&mut self) -> INTER_RAM2_FORCE_PD_W<11> {
        INTER_RAM2_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 12 - internal SRAM 2 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_force_pu(&mut self) -> INTER_RAM2_FORCE_PU_W<12> {
        INTER_RAM2_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 13 - internal SRAM 3 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_force_pd(&mut self) -> INTER_RAM3_FORCE_PD_W<13> {
        INTER_RAM3_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 14 - internal SRAM 3 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_force_pu(&mut self) -> INTER_RAM3_FORCE_PU_W<14> {
        INTER_RAM3_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 15 - internal SRAM 4 force power down"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_force_pd(&mut self) -> INTER_RAM4_FORCE_PD_W<15> {
        INTER_RAM4_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 16 - internal SRAM 4 force power up"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_force_pu(&mut self) -> INTER_RAM4_FORCE_PU_W<16> {
        INTER_RAM4_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to FPD the Wi-Fi circuit."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_pd(&mut self) -> WIFI_FORCE_PD_W<17> {
        WIFI_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to FPU the Wi-Fi circuit."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_pu(&mut self) -> WIFI_FORCE_PU_W<18> {
        WIFI_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to FPD the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W<19> {
        DG_WRAP_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W<20> {
        DG_WRAP_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to FPD the DC-DC convertor in the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_force_pd(&mut self) -> DG_DCDC_FORCE_PD_W<21> {
        DG_DCDC_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to FPU the DC-DC convertor in the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_force_pu(&mut self) -> DG_DCDC_FORCE_PU_W<22> {
        DG_DCDC_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to enable PD for the DC-DC convertor in the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_pd_en(&mut self) -> DG_DCDC_PD_EN_W<23> {
        DG_DCDC_PD_EN_W::new(self)
    }
    #[doc = "Bit 24 - enable power down ROM in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_pd_en(&mut self) -> ROM0_PD_EN_W<24> {
        ROM0_PD_EN_W::new(self)
    }
    #[doc = "Bit 25 - enable power down internal SRAM 0 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_pd_en(&mut self) -> INTER_RAM0_PD_EN_W<25> {
        INTER_RAM0_PD_EN_W::new(self)
    }
    #[doc = "Bit 26 - enable power down internal SRAM 1 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_pd_en(&mut self) -> INTER_RAM1_PD_EN_W<26> {
        INTER_RAM1_PD_EN_W::new(self)
    }
    #[doc = "Bit 27 - enable power down internal SRAM 2 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_pd_en(&mut self) -> INTER_RAM2_PD_EN_W<27> {
        INTER_RAM2_PD_EN_W::new(self)
    }
    #[doc = "Bit 28 - enable power down internal SRAM 3 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_pd_en(&mut self) -> INTER_RAM3_PD_EN_W<28> {
        INTER_RAM3_PD_EN_W::new(self)
    }
    #[doc = "Bit 29 - enable power down internal SRAM 4 in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_pd_en(&mut self) -> INTER_RAM4_PD_EN_W<29> {
        INTER_RAM4_PD_EN_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to enable PD for the Wi-Fi circuit in sleep."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_pd_en(&mut self) -> WIFI_PD_EN_W<30> {
        WIFI_PD_EN_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to enable PD for the digital system in sleep."]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W<31> {
        DG_WRAP_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital system power configuraiton register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pwc](index.html) module"]
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_pwc::R](R) reader structure"]
impl crate::Readable for DIG_PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_pwc::W](W) writer structure"]
impl crate::Writable for DIG_PWC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_PWC to value 0x0055_5550"]
impl crate::Resettable for DIG_PWC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0055_5550;
}
