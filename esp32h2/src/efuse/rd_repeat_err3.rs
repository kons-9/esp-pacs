#[doc = "Register `RD_REPEAT_ERR3` reader"]
pub struct R(crate::R<RD_REPEAT_ERR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - Indicates a programming error of DIS_DOWNLOAD_MODE."]
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT_ERR` reader - Indicates a programming error of DIS_DIRECT_BOOT."]
pub type DIS_DIRECT_BOOT_ERR_R = crate::BitReader;
#[doc = "Field `USB_PRINT_ERR` reader - Indicates a programming error of UART_PRINT_CHANNEL."]
pub type USB_PRINT_ERR_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED3_ERR_5` reader - Reserved."]
pub type RPT4_RESERVED3_ERR_5_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR` reader - Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE."]
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - Indicates a programming error of ENABLE_SECURITY_DOWNLOAD."]
pub type ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - Indicates a programming error of UART_PRINT_CONTROL."]
pub type UART_PRINT_CONTROL_ERR_R = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - Indicates a programming error of FORCE_SEND_RESUME."]
pub type FORCE_SEND_RESUME_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_VERSION_ERR` reader - Indicates a programming error of SECURE VERSION."]
pub type SECURE_VERSION_ERR_R = crate::FieldReader<u16>;
#[doc = "Field `SECURE_BOOT_DISABLE_FAST_WAKE_ERR` reader - Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE."]
pub type SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R = crate::BitReader;
#[doc = "Field `HYS_EN_PAD0_ERR` reader - Indicates a programming error of HYS_EN_PAD0."]
pub type HYS_EN_PAD0_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates a programming error of DIS_DOWNLOAD_MODE."]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates a programming error of DIS_DIRECT_BOOT."]
    #[inline(always)]
    pub fn dis_direct_boot_err(&self) -> DIS_DIRECT_BOOT_ERR_R {
        DIS_DIRECT_BOOT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a programming error of UART_PRINT_CHANNEL."]
    #[inline(always)]
    pub fn usb_print_err(&self) -> USB_PRINT_ERR_R {
        USB_PRINT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved3_err_5(&self) -> RPT4_RESERVED3_ERR_5_R {
        RPT4_RESERVED3_ERR_5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE."]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode_err(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates a programming error of ENABLE_SECURITY_DOWNLOAD."]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Indicates a programming error of UART_PRINT_CONTROL."]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Indicates a programming error of FORCE_SEND_RESUME."]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - Indicates a programming error of SECURE VERSION."]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE."]
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake_err(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Indicates a programming error of HYS_EN_PAD0."]
    #[inline(always)]
    pub fn hys_en_pad0_err(&self) -> HYS_EN_PAD0_ERR_R {
        HYS_EN_PAD0_ERR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR3")
            .field(
                "dis_download_mode_err",
                &format_args!("{}", self.dis_download_mode_err().bit()),
            )
            .field(
                "dis_direct_boot_err",
                &format_args!("{}", self.dis_direct_boot_err().bit()),
            )
            .field(
                "usb_print_err",
                &format_args!("{}", self.usb_print_err().bit()),
            )
            .field(
                "rpt4_reserved3_err_5",
                &format_args!("{}", self.rpt4_reserved3_err_5().bit()),
            )
            .field(
                "dis_usb_serial_jtag_download_mode_err",
                &format_args!("{}", self.dis_usb_serial_jtag_download_mode_err().bit()),
            )
            .field(
                "enable_security_download_err",
                &format_args!("{}", self.enable_security_download_err().bit()),
            )
            .field(
                "uart_print_control_err",
                &format_args!("{}", self.uart_print_control_err().bits()),
            )
            .field(
                "force_send_resume_err",
                &format_args!("{}", self.force_send_resume_err().bit()),
            )
            .field(
                "secure_version_err",
                &format_args!("{}", self.secure_version_err().bits()),
            )
            .field(
                "secure_boot_disable_fast_wake_err",
                &format_args!("{}", self.secure_boot_disable_fast_wake_err().bit()),
            )
            .field(
                "hys_en_pad0_err",
                &format_args!("{}", self.hys_en_pad0_err().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 3 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err3](index.html) module"]
pub struct RD_REPEAT_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err3::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR3 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
